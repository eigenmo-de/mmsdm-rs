
create or alter procedure mmsdm_proc.InsertAncilliaryServicesContractagc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AncilliaryServicesContractagc1 as tgt 
using (
    select 
        d.[contractid],
        d.[versionno],
        d.[startdate],
        d.[enddate],
        d.[participantid],
        d.[duid],
        d.[crr],
        d.[crl],
        d.[rlprice],
        d.[ccprice],
        d.[bs],
        d.[authorisedby],
        d.[authoriseddate],
        d.[lastchanged]
    from openjson(@data) with (
        [contractid] varchar(10),
        [versionno] decimal(3,0),
        [startdate] datetime2,
        [enddate] datetime2,
        [participantid] varchar(10),
        [duid] varchar(10),
        [crr] decimal(4,0),
        [crl] decimal(4,0),
        [rlprice] decimal(10,2),
        [ccprice] decimal(10,2),
        [bs] decimal(10,2),
        [authorisedby] varchar(15),
        [authoriseddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[participantid] = src.[participantid],
        tgt.[duid] = src.[duid],
        tgt.[crr] = src.[crr],
        tgt.[crl] = src.[crl],
        tgt.[rlprice] = src.[rlprice],
        tgt.[ccprice] = src.[ccprice],
        tgt.[bs] = src.[bs],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [versionno],
        [startdate],
        [enddate],
        [participantid],
        [duid],
        [crr],
        [crl],
        [rlprice],
        [ccprice],
        [bs],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[startdate],
        src.[enddate],
        src.[participantid],
        src.[duid],
        src.[crr],
        src.[crl],
        src.[rlprice],
        src.[ccprice],
        src.[bs],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAncilliaryServicesContractloadshed2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AncilliaryServicesContractloadshed2 as tgt 
using (
    select 
        d.[contractid],
        d.[versionno],
        d.[startdate],
        d.[enddate],
        d.[participantid],
        d.[duid],
        d.[lseprice],
        d.[mcpprice],
        d.[tenderedprice],
        d.[lscr],
        d.[ilscalingfactor],
        d.[lower60secbreakpoint],
        d.[lower60secmax],
        d.[lower6secbreakpoint],
        d.[lower6secmax],
        d.[raise60secbreakpoint],
        d.[raise60seccapacity],
        d.[raise60secmax],
        d.[raise6secbreakpoint],
        d.[raise6seccapacity],
        d.[raise6secmax],
        d.[price6secraisemandatory],
        d.[quant6secraisemandatory],
        d.[price6secraisecontract],
        d.[quant6secraisecontract],
        d.[price60secraisemandatory],
        d.[quant60secraisemandatory],
        d.[price60secraisecontract],
        d.[quant60secraisecontract],
        d.[price6seclowermandatory],
        d.[quant6seclowermandatory],
        d.[price6seclowercontract],
        d.[quant6seclowercontract],
        d.[price60seclowermandatory],
        d.[quant60seclowermandatory],
        d.[price60seclowercontract],
        d.[quant60seclowercontract],
        d.[authorisedby],
        d.[authoriseddate],
        d.[lastchanged],
        d.[default_testingpayment_amount],
        d.[service_start_date]
    from openjson(@data) with (
        [contractid] varchar(10),
        [versionno] decimal(3,0),
        [startdate] datetime2,
        [enddate] datetime2,
        [participantid] varchar(10),
        [duid] varchar(10),
        [lseprice] decimal(6,2),
        [mcpprice] decimal(12,2),
        [tenderedprice] decimal(6,2),
        [lscr] decimal(6,2),
        [ilscalingfactor] decimal(15,5),
        [lower60secbreakpoint] decimal(9,6),
        [lower60secmax] decimal(9,6),
        [lower6secbreakpoint] decimal(9,6),
        [lower6secmax] decimal(9,6),
        [raise60secbreakpoint] decimal(9,6),
        [raise60seccapacity] decimal(9,6),
        [raise60secmax] decimal(9,6),
        [raise6secbreakpoint] decimal(9,6),
        [raise6seccapacity] decimal(9,6),
        [raise6secmax] decimal(9,6),
        [price6secraisemandatory] decimal(16,6),
        [quant6secraisemandatory] decimal(9,6),
        [price6secraisecontract] decimal(16,6),
        [quant6secraisecontract] decimal(9,6),
        [price60secraisemandatory] decimal(16,6),
        [quant60secraisemandatory] decimal(9,6),
        [price60secraisecontract] decimal(16,6),
        [quant60secraisecontract] decimal(9,6),
        [price6seclowermandatory] decimal(16,6),
        [quant6seclowermandatory] decimal(9,6),
        [price6seclowercontract] decimal(16,6),
        [quant6seclowercontract] decimal(9,6),
        [price60seclowermandatory] decimal(16,6),
        [quant60seclowermandatory] decimal(9,6),
        [price60seclowercontract] decimal(16,6),
        [quant60seclowercontract] decimal(9,6),
        [authorisedby] varchar(15),
        [authoriseddate] datetime2,
        [lastchanged] datetime2,
        [default_testingpayment_amount] decimal(18,8),
        [service_start_date] datetime2
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[participantid] = src.[participantid],
        tgt.[duid] = src.[duid],
        tgt.[lseprice] = src.[lseprice],
        tgt.[mcpprice] = src.[mcpprice],
        tgt.[tenderedprice] = src.[tenderedprice],
        tgt.[lscr] = src.[lscr],
        tgt.[ilscalingfactor] = src.[ilscalingfactor],
        tgt.[lower60secbreakpoint] = src.[lower60secbreakpoint],
        tgt.[lower60secmax] = src.[lower60secmax],
        tgt.[lower6secbreakpoint] = src.[lower6secbreakpoint],
        tgt.[lower6secmax] = src.[lower6secmax],
        tgt.[raise60secbreakpoint] = src.[raise60secbreakpoint],
        tgt.[raise60seccapacity] = src.[raise60seccapacity],
        tgt.[raise60secmax] = src.[raise60secmax],
        tgt.[raise6secbreakpoint] = src.[raise6secbreakpoint],
        tgt.[raise6seccapacity] = src.[raise6seccapacity],
        tgt.[raise6secmax] = src.[raise6secmax],
        tgt.[price6secraisemandatory] = src.[price6secraisemandatory],
        tgt.[quant6secraisemandatory] = src.[quant6secraisemandatory],
        tgt.[price6secraisecontract] = src.[price6secraisecontract],
        tgt.[quant6secraisecontract] = src.[quant6secraisecontract],
        tgt.[price60secraisemandatory] = src.[price60secraisemandatory],
        tgt.[quant60secraisemandatory] = src.[quant60secraisemandatory],
        tgt.[price60secraisecontract] = src.[price60secraisecontract],
        tgt.[quant60secraisecontract] = src.[quant60secraisecontract],
        tgt.[price6seclowermandatory] = src.[price6seclowermandatory],
        tgt.[quant6seclowermandatory] = src.[quant6seclowermandatory],
        tgt.[price6seclowercontract] = src.[price6seclowercontract],
        tgt.[quant6seclowercontract] = src.[quant6seclowercontract],
        tgt.[price60seclowermandatory] = src.[price60seclowermandatory],
        tgt.[quant60seclowermandatory] = src.[quant60seclowermandatory],
        tgt.[price60seclowercontract] = src.[price60seclowercontract],
        tgt.[quant60seclowercontract] = src.[quant60seclowercontract],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[default_testingpayment_amount] = src.[default_testingpayment_amount],
        tgt.[service_start_date] = src.[service_start_date]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [versionno],
        [startdate],
        [enddate],
        [participantid],
        [duid],
        [lseprice],
        [mcpprice],
        [tenderedprice],
        [lscr],
        [ilscalingfactor],
        [lower60secbreakpoint],
        [lower60secmax],
        [lower6secbreakpoint],
        [lower6secmax],
        [raise60secbreakpoint],
        [raise60seccapacity],
        [raise60secmax],
        [raise6secbreakpoint],
        [raise6seccapacity],
        [raise6secmax],
        [price6secraisemandatory],
        [quant6secraisemandatory],
        [price6secraisecontract],
        [quant6secraisecontract],
        [price60secraisemandatory],
        [quant60secraisemandatory],
        [price60secraisecontract],
        [quant60secraisecontract],
        [price6seclowermandatory],
        [quant6seclowermandatory],
        [price6seclowercontract],
        [quant6seclowercontract],
        [price60seclowermandatory],
        [quant60seclowermandatory],
        [price60seclowercontract],
        [quant60seclowercontract],
        [authorisedby],
        [authoriseddate],
        [lastchanged],
        [default_testingpayment_amount],
        [service_start_date]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[startdate],
        src.[enddate],
        src.[participantid],
        src.[duid],
        src.[lseprice],
        src.[mcpprice],
        src.[tenderedprice],
        src.[lscr],
        src.[ilscalingfactor],
        src.[lower60secbreakpoint],
        src.[lower60secmax],
        src.[lower6secbreakpoint],
        src.[lower6secmax],
        src.[raise60secbreakpoint],
        src.[raise60seccapacity],
        src.[raise60secmax],
        src.[raise6secbreakpoint],
        src.[raise6seccapacity],
        src.[raise6secmax],
        src.[price6secraisemandatory],
        src.[quant6secraisemandatory],
        src.[price6secraisecontract],
        src.[quant6secraisecontract],
        src.[price60secraisemandatory],
        src.[quant60secraisemandatory],
        src.[price60secraisecontract],
        src.[quant60secraisecontract],
        src.[price6seclowermandatory],
        src.[quant6seclowermandatory],
        src.[price6seclowercontract],
        src.[quant6seclowercontract],
        src.[price60seclowermandatory],
        src.[quant60seclowermandatory],
        src.[price60seclowercontract],
        src.[quant60seclowercontract],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged],
        src.[default_testingpayment_amount],
        src.[service_start_date]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAncilliaryServicesContractreactivepower4
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AncilliaryServicesContractreactivepower4 as tgt 
using (
    select 
        d.[contractid],
        d.[versionno],
        d.[startdate],
        d.[enddate],
        d.[participantid],
        d.[duid],
        d.[synccompensation],
        d.[mvaraprice],
        d.[mvareprice],
        d.[mvargprice],
        d.[ccprice],
        d.[mta],
        d.[mtg],
        d.[mmca],
        d.[mmcg],
        d.[eu],
        d.[pp],
        d.[bs],
        d.[authorisedby],
        d.[authoriseddate],
        d.[lastchanged],
        d.[default_testingpayment_amount],
        d.[service_start_date],
        d.[availability_mwh_threshold],
        d.[mvar_threshold],
        d.[rebate_cap],
        d.[rebate_amount_per_mvar],
        d.[isrebateapplicable]
    from openjson(@data) with (
        [contractid] varchar(10),
        [versionno] decimal(3,0),
        [startdate] datetime2,
        [enddate] datetime2,
        [participantid] varchar(10),
        [duid] varchar(10),
        [synccompensation] varchar(1),
        [mvaraprice] decimal(10,2),
        [mvareprice] decimal(10,2),
        [mvargprice] decimal(10,2),
        [ccprice] decimal(10,2),
        [mta] decimal(10,2),
        [mtg] decimal(10,2),
        [mmca] decimal(10,2),
        [mmcg] decimal(10,2),
        [eu] decimal(10,2),
        [pp] decimal(10,2),
        [bs] decimal(10,2),
        [authorisedby] varchar(15),
        [authoriseddate] datetime2,
        [lastchanged] datetime2,
        [default_testingpayment_amount] decimal(18,8),
        [service_start_date] datetime2,
        [availability_mwh_threshold] decimal(18,8),
        [mvar_threshold] decimal(18,8),
        [rebate_cap] decimal(18,8),
        [rebate_amount_per_mvar] decimal(18,8),
        [isrebateapplicable] decimal(1,0)
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[participantid] = src.[participantid],
        tgt.[duid] = src.[duid],
        tgt.[synccompensation] = src.[synccompensation],
        tgt.[mvaraprice] = src.[mvaraprice],
        tgt.[mvareprice] = src.[mvareprice],
        tgt.[mvargprice] = src.[mvargprice],
        tgt.[ccprice] = src.[ccprice],
        tgt.[mta] = src.[mta],
        tgt.[mtg] = src.[mtg],
        tgt.[mmca] = src.[mmca],
        tgt.[mmcg] = src.[mmcg],
        tgt.[eu] = src.[eu],
        tgt.[pp] = src.[pp],
        tgt.[bs] = src.[bs],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[default_testingpayment_amount] = src.[default_testingpayment_amount],
        tgt.[service_start_date] = src.[service_start_date],
        tgt.[availability_mwh_threshold] = src.[availability_mwh_threshold],
        tgt.[mvar_threshold] = src.[mvar_threshold],
        tgt.[rebate_cap] = src.[rebate_cap],
        tgt.[rebate_amount_per_mvar] = src.[rebate_amount_per_mvar],
        tgt.[isrebateapplicable] = src.[isrebateapplicable]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [versionno],
        [startdate],
        [enddate],
        [participantid],
        [duid],
        [synccompensation],
        [mvaraprice],
        [mvareprice],
        [mvargprice],
        [ccprice],
        [mta],
        [mtg],
        [mmca],
        [mmcg],
        [eu],
        [pp],
        [bs],
        [authorisedby],
        [authoriseddate],
        [lastchanged],
        [default_testingpayment_amount],
        [service_start_date],
        [availability_mwh_threshold],
        [mvar_threshold],
        [rebate_cap],
        [rebate_amount_per_mvar],
        [isrebateapplicable]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[startdate],
        src.[enddate],
        src.[participantid],
        src.[duid],
        src.[synccompensation],
        src.[mvaraprice],
        src.[mvareprice],
        src.[mvargprice],
        src.[ccprice],
        src.[mta],
        src.[mtg],
        src.[mmca],
        src.[mmcg],
        src.[eu],
        src.[pp],
        src.[bs],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged],
        src.[default_testingpayment_amount],
        src.[service_start_date],
        src.[availability_mwh_threshold],
        src.[mvar_threshold],
        src.[rebate_cap],
        src.[rebate_amount_per_mvar],
        src.[isrebateapplicable]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAncilliaryServicesContractrestartservices2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AncilliaryServicesContractrestartservices2 as tgt 
using (
    select 
        d.[contractid],
        d.[versionno],
        d.[startdate],
        d.[enddate],
        d.[participantid],
        d.[restarttype],
        d.[rcprice],
        d.[triptohouselevel],
        d.[authorisedby],
        d.[authoriseddate],
        d.[lastchanged],
        d.[default_testingpayment_amount],
        d.[service_start_date]
    from openjson(@data) with (
        [contractid] varchar(10),
        [versionno] decimal(3,0),
        [startdate] datetime2,
        [enddate] datetime2,
        [participantid] varchar(10),
        [restarttype] decimal(1,0),
        [rcprice] decimal(6,2),
        [triptohouselevel] decimal(5,0),
        [authorisedby] varchar(15),
        [authoriseddate] datetime2,
        [lastchanged] datetime2,
        [default_testingpayment_amount] decimal(18,8),
        [service_start_date] datetime2
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[participantid] = src.[participantid],
        tgt.[restarttype] = src.[restarttype],
        tgt.[rcprice] = src.[rcprice],
        tgt.[triptohouselevel] = src.[triptohouselevel],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[default_testingpayment_amount] = src.[default_testingpayment_amount],
        tgt.[service_start_date] = src.[service_start_date]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [versionno],
        [startdate],
        [enddate],
        [participantid],
        [restarttype],
        [rcprice],
        [triptohouselevel],
        [authorisedby],
        [authoriseddate],
        [lastchanged],
        [default_testingpayment_amount],
        [service_start_date]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[startdate],
        src.[enddate],
        src.[participantid],
        src.[restarttype],
        src.[rcprice],
        src.[triptohouselevel],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged],
        src.[default_testingpayment_amount],
        src.[service_start_date]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAncilliaryServicesContractrestartunits1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AncilliaryServicesContractrestartunits1 as tgt 
using (
    select 
        d.[contractid],
        d.[versionno],
        d.[duid],
        d.[lastchanged],
        d.[authorisedby],
        d.[authoriseddate]
    from openjson(@data) with (
        [contractid] varchar(10),
        [versionno] decimal(3,0),
        [duid] varchar(10),
        [lastchanged] datetime2,
        [authorisedby] varchar(15),
        [authoriseddate] datetime2
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[duid] = src.[duid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[duid] = src.[duid],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [versionno],
        [duid],
        [lastchanged],
        [authorisedby],
        [authoriseddate]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[duid],
        src.[lastchanged],
        src.[authorisedby],
        src.[authoriseddate]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAsofferOfferagcdata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AsofferOfferagcdata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[availability] = src.[availability],
        tgt.[upperlimit] = src.[upperlimit],
        tgt.[lowerlimit] = src.[lowerlimit],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[filename] = src.[filename],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[periodid] = src.[periodid],
        tgt.[agcup] = src.[agcup],
        tgt.[agcdown] = src.[agcdown]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[effectivedate],
        src.[versionno],
        src.[availability],
        src.[upperlimit],
        src.[lowerlimit],
        src.[authoriseddate],
        src.[authorisedby],
        src.[filename],
        src.[lastchanged],
        src.[periodid],
        src.[agcup],
        src.[agcdown]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAsofferOfferastrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AsofferOfferastrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[filename] = src.[filename],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [participantid],
        [filename],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[participantid],
        src.[filename],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAsofferOfferlsheddata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AsofferOfferlsheddata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[availableload] = src.[availableload],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[filename] = src.[filename],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[periodid] = src.[periodid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[effectivedate],
        src.[versionno],
        src.[availableload],
        src.[authoriseddate],
        src.[authorisedby],
        src.[filename],
        src.[lastchanged],
        src.[periodid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAsofferOfferrestartdata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AsofferOfferrestartdata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[availability] = src.[availability],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[filename] = src.[filename],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[periodid] = src.[periodid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[offerdate],
        src.[versionno],
        src.[availability],
        src.[authoriseddate],
        src.[authorisedby],
        src.[filename],
        src.[lastchanged],
        src.[periodid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertAsofferOfferrpowerdata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.AsofferOfferrpowerdata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[availability] = src.[availability],
        tgt.[mta] = src.[mta],
        tgt.[mtg] = src.[mtg],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[filename] = src.[filename],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[effectivedate],
        src.[versionno],
        src.[periodid],
        src.[availability],
        src.[mta],
        src.[mtg],
        src.[authoriseddate],
        src.[authorisedby],
        src.[filename],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertOfferBiddayoffer3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.OfferBiddayoffer3 as tgt 
using (
    select 
        d.[duid],
        d.[bidtype],
        d.[settlementdate],
        d.[offerdate],
        d.[direction],
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
        d.[entrytype],
        d.[rebid_event_time],
        d.[rebid_aware_time],
        d.[rebid_decision_time],
        d.[rebid_category],
        d.[reference_id]
    from openjson(@data) with (
        [duid] varchar(10),
        [bidtype] varchar(10),
        [settlementdate] datetime2,
        [offerdate] datetime2,
        [direction] varchar(20),
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
        [entrytype] varchar(20),
        [rebid_event_time] varchar(20),
        [rebid_aware_time] varchar(20),
        [rebid_decision_time] varchar(20),
        [rebid_category] varchar(1),
        [reference_id] varchar(100)
    ) d
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[direction] = src.[direction]
    and tgt.[duid] = src.[duid]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[direction] = src.[direction],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[dailyenergyconstraint] = src.[dailyenergyconstraint],
        tgt.[rebidexplanation] = src.[rebidexplanation],
        tgt.[priceband1] = src.[priceband1],
        tgt.[priceband2] = src.[priceband2],
        tgt.[priceband3] = src.[priceband3],
        tgt.[priceband4] = src.[priceband4],
        tgt.[priceband5] = src.[priceband5],
        tgt.[priceband6] = src.[priceband6],
        tgt.[priceband7] = src.[priceband7],
        tgt.[priceband8] = src.[priceband8],
        tgt.[priceband9] = src.[priceband9],
        tgt.[priceband10] = src.[priceband10],
        tgt.[minimumload] = src.[minimumload],
        tgt.[t1] = src.[t1],
        tgt.[t2] = src.[t2],
        tgt.[t3] = src.[t3],
        tgt.[t4] = src.[t4],
        tgt.[normalstatus] = src.[normalstatus],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[mr_factor] = src.[mr_factor],
        tgt.[entrytype] = src.[entrytype],
        tgt.[rebid_event_time] = src.[rebid_event_time],
        tgt.[rebid_aware_time] = src.[rebid_aware_time],
        tgt.[rebid_decision_time] = src.[rebid_decision_time],
        tgt.[rebid_category] = src.[rebid_category],
        tgt.[reference_id] = src.[reference_id]
when not matched
    then insert (
        file_log_id,
        [duid],
        [bidtype],
        [settlementdate],
        [offerdate],
        [direction],
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
        [entrytype],
        [rebid_event_time],
        [rebid_aware_time],
        [rebid_decision_time],
        [rebid_category],
        [reference_id]
    ) values (
        @file_log_id,
        src.[duid],
        src.[bidtype],
        src.[settlementdate],
        src.[offerdate],
        src.[direction],
        src.[versionno],
        src.[participantid],
        src.[dailyenergyconstraint],
        src.[rebidexplanation],
        src.[priceband1],
        src.[priceband2],
        src.[priceband3],
        src.[priceband4],
        src.[priceband5],
        src.[priceband6],
        src.[priceband7],
        src.[priceband8],
        src.[priceband9],
        src.[priceband10],
        src.[minimumload],
        src.[t1],
        src.[t2],
        src.[t3],
        src.[t4],
        src.[normalstatus],
        src.[lastchanged],
        src.[mr_factor],
        src.[entrytype],
        src.[rebid_event_time],
        src.[rebid_aware_time],
        src.[rebid_decision_time],
        src.[rebid_category],
        src.[reference_id]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidBiddayofferD3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidBiddayofferD3 as tgt 
using (
    select 
        d.[settlementdate],
        d.[duid],
        d.[bidtype],
        d.[direction],
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
        [direction] varchar(20),
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[direction] = src.[direction]
    and tgt.[duid] = src.[duid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[duid] = src.[duid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[direction] = src.[direction],
        tgt.[bidsettlementdate] = src.[bidsettlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[dailyenergyconstraint] = src.[dailyenergyconstraint],
        tgt.[rebidexplanation] = src.[rebidexplanation],
        tgt.[priceband1] = src.[priceband1],
        tgt.[priceband2] = src.[priceband2],
        tgt.[priceband3] = src.[priceband3],
        tgt.[priceband4] = src.[priceband4],
        tgt.[priceband5] = src.[priceband5],
        tgt.[priceband6] = src.[priceband6],
        tgt.[priceband7] = src.[priceband7],
        tgt.[priceband8] = src.[priceband8],
        tgt.[priceband9] = src.[priceband9],
        tgt.[priceband10] = src.[priceband10],
        tgt.[minimumload] = src.[minimumload],
        tgt.[t1] = src.[t1],
        tgt.[t2] = src.[t2],
        tgt.[t3] = src.[t3],
        tgt.[t4] = src.[t4],
        tgt.[normalstatus] = src.[normalstatus],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[mr_factor] = src.[mr_factor],
        tgt.[entrytype] = src.[entrytype]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [duid],
        [bidtype],
        [direction],
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[duid],
        src.[bidtype],
        src.[direction],
        src.[bidsettlementdate],
        src.[offerdate],
        src.[versionno],
        src.[participantid],
        src.[dailyenergyconstraint],
        src.[rebidexplanation],
        src.[priceband1],
        src.[priceband2],
        src.[priceband3],
        src.[priceband4],
        src.[priceband5],
        src.[priceband6],
        src.[priceband7],
        src.[priceband8],
        src.[priceband9],
        src.[priceband10],
        src.[minimumload],
        src.[t1],
        src.[t2],
        src.[t3],
        src.[t4],
        src.[normalstatus],
        src.[lastchanged],
        src.[mr_factor],
        src.[entrytype]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidsBidofferfiletrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidsBidofferfiletrk1 as tgt 
using (
    select 
        d.[participantid],
        d.[offerdate],
        d.[filename],
        d.[status],
        d.[lastchanged],
        d.[authorisedby],
        d.[authoriseddate],
        d.[transaction_id],
        d.[reference_id],
        d.[submission_timestamp],
        d.[comments],
        d.[submission_method]
    from openjson(@data) with (
        [participantid] varchar(10),
        [offerdate] datetime2,
        [filename] varchar(80),
        [status] varchar(10),
        [lastchanged] datetime2,
        [authorisedby] varchar(20),
        [authoriseddate] datetime2,
        [transaction_id] varchar(100),
        [reference_id] varchar(100),
        [submission_timestamp] datetime2,
        [comments] varchar(1000),
        [submission_method] varchar(20)
    ) d
) as src 
on (
    tgt.[offerdate] = src.[offerdate]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[offerdate] = src.[offerdate],
        tgt.[filename] = src.[filename],
        tgt.[status] = src.[status],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[transaction_id] = src.[transaction_id],
        tgt.[reference_id] = src.[reference_id],
        tgt.[submission_timestamp] = src.[submission_timestamp],
        tgt.[comments] = src.[comments],
        tgt.[submission_method] = src.[submission_method]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [offerdate],
        [filename],
        [status],
        [lastchanged],
        [authorisedby],
        [authoriseddate],
        [transaction_id],
        [reference_id],
        [submission_timestamp],
        [comments],
        [submission_method]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[offerdate],
        src.[filename],
        src.[status],
        src.[lastchanged],
        src.[authorisedby],
        src.[authoriseddate],
        src.[transaction_id],
        src.[reference_id],
        src.[submission_timestamp],
        src.[comments],
        src.[submission_method]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidsBidofferperiod2
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BidsBidofferperiod2(
file_log_id,
[duid],
        [bidtype],
        [tradingdate],
        [offerdatetime],
        [direction],
        [periodid],
        [maxavail],
        [fixedload],
        [rampuprate],
        [rampdownrate],
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
        [pasaavailability],
        [energylimit],
        [periodidto]
)
select 
@file_log_id,
d.[duid],
        d.[bidtype],
        d.[tradingdate],
        d.[offerdatetime],
        d.[direction],
        d.[periodid],
        d.[maxavail],
        d.[fixedload],
        d.[rampuprate],
        d.[rampdownrate],
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
        d.[pasaavailability],
        d.[energylimit],
        d.[periodidto]
from openjson(@data) with (
[duid] varchar(20),
        [bidtype] varchar(10),
        [tradingdate] datetime2,
        [offerdatetime] datetime2,
        [direction] varchar(20),
        [periodid] decimal(3,0),
        [maxavail] decimal(8,3),
        [fixedload] decimal(8,3),
        [rampuprate] decimal(6,0),
        [rampdownrate] decimal(6,0),
        [enablementmin] decimal(8,3),
        [enablementmax] decimal(8,3),
        [lowbreakpoint] decimal(8,3),
        [highbreakpoint] decimal(8,3),
        [bandavail1] decimal(8,3),
        [bandavail2] decimal(8,3),
        [bandavail3] decimal(8,3),
        [bandavail4] decimal(8,3),
        [bandavail5] decimal(8,3),
        [bandavail6] decimal(8,3),
        [bandavail7] decimal(8,3),
        [bandavail8] decimal(8,3),
        [bandavail9] decimal(8,3),
        [bandavail10] decimal(8,3),
        [pasaavailability] decimal(8,3),
        [energylimit] decimal(15,5),
        [periodidto] decimal(3,0)
) d
end
go
create or alter procedure mmsdm_proc.InsertBidBidperofferD3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidBidperofferD3 as tgt 
using (
    select 
        d.[settlementdate],
        d.[duid],
        d.[bidtype],
        d.[direction],
        d.[interval_datetime],
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
        d.[mr_capacity],
        d.[energylimit]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [duid] varchar(10),
        [bidtype] varchar(10),
        [direction] varchar(20),
        [interval_datetime] datetime2,
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
        [mr_capacity] decimal(6,0),
        [energylimit] decimal(15,5)
    ) d
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[direction] = src.[direction]
    and tgt.[duid] = src.[duid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[duid] = src.[duid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[direction] = src.[direction],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[bidsettlementdate] = src.[bidsettlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[periodid] = src.[periodid],
        tgt.[versionno] = src.[versionno],
        tgt.[maxavail] = src.[maxavail],
        tgt.[fixedload] = src.[fixedload],
        tgt.[rocup] = src.[rocup],
        tgt.[rocdown] = src.[rocdown],
        tgt.[enablementmin] = src.[enablementmin],
        tgt.[enablementmax] = src.[enablementmax],
        tgt.[lowbreakpoint] = src.[lowbreakpoint],
        tgt.[highbreakpoint] = src.[highbreakpoint],
        tgt.[bandavail1] = src.[bandavail1],
        tgt.[bandavail2] = src.[bandavail2],
        tgt.[bandavail3] = src.[bandavail3],
        tgt.[bandavail4] = src.[bandavail4],
        tgt.[bandavail5] = src.[bandavail5],
        tgt.[bandavail6] = src.[bandavail6],
        tgt.[bandavail7] = src.[bandavail7],
        tgt.[bandavail8] = src.[bandavail8],
        tgt.[bandavail9] = src.[bandavail9],
        tgt.[bandavail10] = src.[bandavail10],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[pasaavailability] = src.[pasaavailability],
        tgt.[mr_capacity] = src.[mr_capacity],
        tgt.[energylimit] = src.[energylimit]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [duid],
        [bidtype],
        [direction],
        [interval_datetime],
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
        [mr_capacity],
        [energylimit]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[duid],
        src.[bidtype],
        src.[direction],
        src.[interval_datetime],
        src.[bidsettlementdate],
        src.[offerdate],
        src.[periodid],
        src.[versionno],
        src.[maxavail],
        src.[fixedload],
        src.[rocup],
        src.[rocdown],
        src.[enablementmin],
        src.[enablementmax],
        src.[lowbreakpoint],
        src.[highbreakpoint],
        src.[bandavail1],
        src.[bandavail2],
        src.[bandavail3],
        src.[bandavail4],
        src.[bandavail5],
        src.[bandavail6],
        src.[bandavail7],
        src.[bandavail8],
        src.[bandavail9],
        src.[bandavail10],
        src.[lastchanged],
        src.[pasaavailability],
        src.[mr_capacity],
        src.[energylimit]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidsMnspBidofferperiod1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BidsMnspBidofferperiod1(
file_log_id,
[linkid],
        [tradingdate],
        [offerdatetime],
        [periodid],
        [maxavail],
        [fixedload],
        [rampuprate],
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
        [pasaavailability]
)
select 
@file_log_id,
d.[linkid],
        d.[tradingdate],
        d.[offerdatetime],
        d.[periodid],
        d.[maxavail],
        d.[fixedload],
        d.[rampuprate],
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
        d.[pasaavailability]
from openjson(@data) with (
[linkid] varchar(20),
        [tradingdate] datetime2,
        [offerdatetime] datetime2,
        [periodid] decimal(3,0),
        [maxavail] decimal(8,3),
        [fixedload] decimal(8,3),
        [rampuprate] decimal(6,0),
        [bandavail1] decimal(8,3),
        [bandavail2] decimal(8,3),
        [bandavail3] decimal(8,3),
        [bandavail4] decimal(8,3),
        [bandavail5] decimal(8,3),
        [bandavail6] decimal(8,3),
        [bandavail7] decimal(8,3),
        [bandavail8] decimal(8,3),
        [bandavail9] decimal(8,3),
        [bandavail10] decimal(8,3),
        [pasaavailability] decimal(8,3)
) d
end
go
create or alter procedure mmsdm_proc.InsertBidMnspDayoffer2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidMnspDayoffer2 as tgt 
using (
    select 
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
        d.[mr_factor],
        d.[rebid_event_time],
        d.[rebid_aware_time],
        d.[rebid_decision_time],
        d.[rebid_category],
        d.[reference_id]
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
        [mr_factor] decimal(16,6),
        [rebid_event_time] varchar(20),
        [rebid_aware_time] varchar(20),
        [rebid_decision_time] varchar(20),
        [rebid_category] varchar(1),
        [reference_id] varchar(100)
    ) d
) as src 
on (
    tgt.[linkid] = src.[linkid]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[linkid] = src.[linkid],
        tgt.[entrytype] = src.[entrytype],
        tgt.[rebidexplanation] = src.[rebidexplanation],
        tgt.[priceband1] = src.[priceband1],
        tgt.[priceband2] = src.[priceband2],
        tgt.[priceband3] = src.[priceband3],
        tgt.[priceband4] = src.[priceband4],
        tgt.[priceband5] = src.[priceband5],
        tgt.[priceband6] = src.[priceband6],
        tgt.[priceband7] = src.[priceband7],
        tgt.[priceband8] = src.[priceband8],
        tgt.[priceband9] = src.[priceband9],
        tgt.[priceband10] = src.[priceband10],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[mr_factor] = src.[mr_factor],
        tgt.[rebid_event_time] = src.[rebid_event_time],
        tgt.[rebid_aware_time] = src.[rebid_aware_time],
        tgt.[rebid_decision_time] = src.[rebid_decision_time],
        tgt.[rebid_category] = src.[rebid_category],
        tgt.[reference_id] = src.[reference_id]
when not matched
    then insert (
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
        [mr_factor],
        [rebid_event_time],
        [rebid_aware_time],
        [rebid_decision_time],
        [rebid_category],
        [reference_id]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[offerdate],
        src.[versionno],
        src.[participantid],
        src.[linkid],
        src.[entrytype],
        src.[rebidexplanation],
        src.[priceband1],
        src.[priceband2],
        src.[priceband3],
        src.[priceband4],
        src.[priceband5],
        src.[priceband6],
        src.[priceband7],
        src.[priceband8],
        src.[priceband9],
        src.[priceband10],
        src.[lastchanged],
        src.[mr_factor],
        src.[rebid_event_time],
        src.[rebid_aware_time],
        src.[rebid_decision_time],
        src.[rebid_category],
        src.[reference_id]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertOfferMtpasaOfferdata2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.OfferMtpasaOfferdata2 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[unitstate1],
        d.[unitstate2],
        d.[unitstate3],
        d.[unitstate4],
        d.[unitstate5],
        d.[unitstate6],
        d.[unitstate7],
        d.[recalltime1],
        d.[recalltime2],
        d.[recalltime3],
        d.[recalltime4],
        d.[recalltime5],
        d.[recalltime6],
        d.[recalltime7]
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
        [lastchanged] datetime2,
        [unitstate1] varchar(20),
        [unitstate2] varchar(20),
        [unitstate3] varchar(20),
        [unitstate4] varchar(20),
        [unitstate5] varchar(20),
        [unitstate6] varchar(20),
        [unitstate7] varchar(20),
        [recalltime1] decimal(4,0),
        [recalltime2] decimal(4,0),
        [recalltime3] decimal(4,0),
        [recalltime4] decimal(4,0),
        [recalltime5] decimal(4,0),
        [recalltime6] decimal(4,0),
        [recalltime7] decimal(4,0)
    ) d
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[offerdatetime] = src.[offerdatetime]
    and tgt.[participantid] = src.[participantid]
    and tgt.[unitid] = src.[unitid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[offerdatetime] = src.[offerdatetime],
        tgt.[unitid] = src.[unitid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[energy] = src.[energy],
        tgt.[capacity1] = src.[capacity1],
        tgt.[capacity2] = src.[capacity2],
        tgt.[capacity3] = src.[capacity3],
        tgt.[capacity4] = src.[capacity4],
        tgt.[capacity5] = src.[capacity5],
        tgt.[capacity6] = src.[capacity6],
        tgt.[capacity7] = src.[capacity7],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[unitstate1] = src.[unitstate1],
        tgt.[unitstate2] = src.[unitstate2],
        tgt.[unitstate3] = src.[unitstate3],
        tgt.[unitstate4] = src.[unitstate4],
        tgt.[unitstate5] = src.[unitstate5],
        tgt.[unitstate6] = src.[unitstate6],
        tgt.[unitstate7] = src.[unitstate7],
        tgt.[recalltime1] = src.[recalltime1],
        tgt.[recalltime2] = src.[recalltime2],
        tgt.[recalltime3] = src.[recalltime3],
        tgt.[recalltime4] = src.[recalltime4],
        tgt.[recalltime5] = src.[recalltime5],
        tgt.[recalltime6] = src.[recalltime6],
        tgt.[recalltime7] = src.[recalltime7]
when not matched
    then insert (
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
        [lastchanged],
        [unitstate1],
        [unitstate2],
        [unitstate3],
        [unitstate4],
        [unitstate5],
        [unitstate6],
        [unitstate7],
        [recalltime1],
        [recalltime2],
        [recalltime3],
        [recalltime4],
        [recalltime5],
        [recalltime6],
        [recalltime7]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[offerdatetime],
        src.[unitid],
        src.[effectivedate],
        src.[energy],
        src.[capacity1],
        src.[capacity2],
        src.[capacity3],
        src.[capacity4],
        src.[capacity5],
        src.[capacity6],
        src.[capacity7],
        src.[lastchanged],
        src.[unitstate1],
        src.[unitstate2],
        src.[unitstate3],
        src.[unitstate4],
        src.[unitstate5],
        src.[unitstate6],
        src.[unitstate7],
        src.[recalltime1],
        src.[recalltime2],
        src.[recalltime3],
        src.[recalltime4],
        src.[recalltime5],
        src.[recalltime6],
        src.[recalltime7]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertOfferMtpasaOfferfiletrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.OfferMtpasaOfferfiletrk1(
file_log_id,
[participantid],
        [offerdatetime],
        [filename]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingConfigBillingcalendar2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingConfigBillingcalendar2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[preliminarystatementdate] = src.[preliminarystatementdate],
        tgt.[finalstatementdate] = src.[finalstatementdate],
        tgt.[paymentdate] = src.[paymentdate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[revision1_statementdate] = src.[revision1_statementdate],
        tgt.[revision2_statementdate] = src.[revision2_statementdate]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[startdate],
        src.[enddate],
        src.[preliminarystatementdate],
        src.[finalstatementdate],
        src.[paymentdate],
        src.[lastchanged],
        src.[revision1_statementdate],
        src.[revision2_statementdate]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingConfigGstBasClass1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingConfigGstBasClass1 as tgt 
using (
    select 
        d.[bas_class],
        d.[description],
        d.[lastchanged]
    from openjson(@data) with (
        [bas_class] varchar(30),
        [description] varchar(100),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[bas_class] = src.[bas_class]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[bas_class] = src.[bas_class],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [bas_class],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[bas_class],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingConfigGstRate1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingConfigGstRate1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bas_class] = src.[bas_class]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[bas_class] = src.[bas_class],
        tgt.[gst_rate] = src.[gst_rate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [bas_class],
        [gst_rate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[bas_class],
        src.[gst_rate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingConfigGstTransactionClass1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingConfigGstTransactionClass1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bas_class] = src.[bas_class]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[transaction_type] = src.[transaction_type]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[transaction_type] = src.[transaction_type],
        tgt.[bas_class] = src.[bas_class],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [transaction_type],
        [bas_class],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[transaction_type],
        src.[bas_class],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingConfigGstTransactionType1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingConfigGstTransactionType1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[transaction_type] = src.[transaction_type]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[transaction_type] = src.[transaction_type],
        tgt.[description] = src.[description],
        tgt.[gl_financialcode] = src.[gl_financialcode],
        tgt.[gl_tcode] = src.[gl_tcode],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [transaction_type],
        [description],
        [gl_financialcode],
        [gl_tcode],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[transaction_type],
        src.[description],
        src.[gl_financialcode],
        src.[gl_tcode],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingConfigSecdepositInterestRate1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingConfigSecdepositInterestRate1(
file_log_id,
[interest_acct_id],
        [effectivedate],
        [version_datetime],
        [interest_rate]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingConfigSecdepositProvision1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingConfigSecdepositProvision1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingAspayments7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingAspayments7 as tgt 
using (
    select 
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
        d.[availability_reactive_rbt],
        d.[raise1sec],
        d.[lower1sec]
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
        [availability_reactive_rbt] decimal(18,8),
        [raise1sec] decimal(18,8),
        [lower1sec] decimal(18,8)
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[agc] = src.[agc],
        tgt.[fcascomp] = src.[fcascomp],
        tgt.[loadshed] = src.[loadshed],
        tgt.[rgul] = src.[rgul],
        tgt.[rguu] = src.[rguu],
        tgt.[reactivepower] = src.[reactivepower],
        tgt.[systemrestart] = src.[systemrestart],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[lower5min] = src.[lower5min],
        tgt.[raise5min] = src.[raise5min],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[availability_reactive] = src.[availability_reactive],
        tgt.[availability_reactive_rbt] = src.[availability_reactive_rbt],
        tgt.[raise1sec] = src.[raise1sec],
        tgt.[lower1sec] = src.[lower1sec]
when not matched
    then insert (
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
        [availability_reactive_rbt],
        [raise1sec],
        [lower1sec]
    ) values (
        @file_log_id,
        src.[regionid],
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[connectionpointid],
        src.[raise6sec],
        src.[lower6sec],
        src.[raise60sec],
        src.[lower60sec],
        src.[agc],
        src.[fcascomp],
        src.[loadshed],
        src.[rgul],
        src.[rguu],
        src.[reactivepower],
        src.[systemrestart],
        src.[lastchanged],
        src.[lower5min],
        src.[raise5min],
        src.[lowerreg],
        src.[raisereg],
        src.[availability_reactive],
        src.[availability_reactive_rbt],
        src.[raise1sec],
        src.[lower1sec]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingAsrecovery9
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingAsrecovery9 as tgt 
using (
    select 
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
        d.[availability_reactive_rbt_gen],
        d.[raise1sec],
        d.[lower1sec],
        d.[raise1sec_gen],
        d.[lower1sec_gen],
        d.[lowerreg_ace],
        d.[raisereg_ace],
        d.[raise1sec_ace],
        d.[raise1sec_asoe],
        d.[lower1sec_ace],
        d.[lower1sec_asoe]
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
        [lowerreg] decimal(18,8),
        [raisereg] decimal(18,8),
        [lower5min_gen] decimal(16,6),
        [raise5min_gen] decimal(16,6),
        [lowerreg_gen] decimal(16,6),
        [raisereg_gen] decimal(16,6),
        [availability_reactive] decimal(18,8),
        [availability_reactive_rbt] decimal(18,8),
        [availability_reactive_gen] decimal(18,8),
        [availability_reactive_rbt_gen] decimal(18,8),
        [raise1sec] decimal(18,8),
        [lower1sec] decimal(18,8),
        [raise1sec_gen] decimal(18,8),
        [lower1sec_gen] decimal(18,8),
        [lowerreg_ace] decimal(18,8),
        [raisereg_ace] decimal(18,8),
        [raise1sec_ace] decimal(18,8),
        [raise1sec_asoe] decimal(18,8),
        [lower1sec_ace] decimal(18,8),
        [lower1sec_asoe] decimal(18,8)
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[agc] = src.[agc],
        tgt.[fcascomp] = src.[fcascomp],
        tgt.[loadshed] = src.[loadshed],
        tgt.[rgul] = src.[rgul],
        tgt.[rguu] = src.[rguu],
        tgt.[reactivepower] = src.[reactivepower],
        tgt.[systemrestart] = src.[systemrestart],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[raise6sec_gen] = src.[raise6sec_gen],
        tgt.[lower6sec_gen] = src.[lower6sec_gen],
        tgt.[raise60sec_gen] = src.[raise60sec_gen],
        tgt.[lower60sec_gen] = src.[lower60sec_gen],
        tgt.[agc_gen] = src.[agc_gen],
        tgt.[fcascomp_gen] = src.[fcascomp_gen],
        tgt.[loadshed_gen] = src.[loadshed_gen],
        tgt.[rgul_gen] = src.[rgul_gen],
        tgt.[rguu_gen] = src.[rguu_gen],
        tgt.[reactivepower_gen] = src.[reactivepower_gen],
        tgt.[systemrestart_gen] = src.[systemrestart_gen],
        tgt.[lower5min] = src.[lower5min],
        tgt.[raise5min] = src.[raise5min],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[lower5min_gen] = src.[lower5min_gen],
        tgt.[raise5min_gen] = src.[raise5min_gen],
        tgt.[lowerreg_gen] = src.[lowerreg_gen],
        tgt.[raisereg_gen] = src.[raisereg_gen],
        tgt.[availability_reactive] = src.[availability_reactive],
        tgt.[availability_reactive_rbt] = src.[availability_reactive_rbt],
        tgt.[availability_reactive_gen] = src.[availability_reactive_gen],
        tgt.[availability_reactive_rbt_gen] = src.[availability_reactive_rbt_gen],
        tgt.[raise1sec] = src.[raise1sec],
        tgt.[lower1sec] = src.[lower1sec],
        tgt.[raise1sec_gen] = src.[raise1sec_gen],
        tgt.[lower1sec_gen] = src.[lower1sec_gen],
        tgt.[lowerreg_ace] = src.[lowerreg_ace],
        tgt.[raisereg_ace] = src.[raisereg_ace],
        tgt.[raise1sec_ace] = src.[raise1sec_ace],
        tgt.[raise1sec_asoe] = src.[raise1sec_asoe],
        tgt.[lower1sec_ace] = src.[lower1sec_ace],
        tgt.[lower1sec_asoe] = src.[lower1sec_asoe]
when not matched
    then insert (
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
        [availability_reactive_rbt_gen],
        [raise1sec],
        [lower1sec],
        [raise1sec_gen],
        [lower1sec_gen],
        [lowerreg_ace],
        [raisereg_ace],
        [raise1sec_ace],
        [raise1sec_asoe],
        [lower1sec_ace],
        [lower1sec_asoe]
    ) values (
        @file_log_id,
        src.[regionid],
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[raise6sec],
        src.[lower6sec],
        src.[raise60sec],
        src.[lower60sec],
        src.[agc],
        src.[fcascomp],
        src.[loadshed],
        src.[rgul],
        src.[rguu],
        src.[reactivepower],
        src.[systemrestart],
        src.[lastchanged],
        src.[raise6sec_gen],
        src.[lower6sec_gen],
        src.[raise60sec_gen],
        src.[lower60sec_gen],
        src.[agc_gen],
        src.[fcascomp_gen],
        src.[loadshed_gen],
        src.[rgul_gen],
        src.[rguu_gen],
        src.[reactivepower_gen],
        src.[systemrestart_gen],
        src.[lower5min],
        src.[raise5min],
        src.[lowerreg],
        src.[raisereg],
        src.[lower5min_gen],
        src.[raise5min_gen],
        src.[lowerreg_gen],
        src.[raisereg_gen],
        src.[availability_reactive],
        src.[availability_reactive_rbt],
        src.[availability_reactive_gen],
        src.[availability_reactive_rbt_gen],
        src.[raise1sec],
        src.[lower1sec],
        src.[raise1sec_gen],
        src.[lower1sec_gen],
        src.[lowerreg_ace],
        src.[raisereg_ace],
        src.[raise1sec_ace],
        src.[raise1sec_asoe],
        src.[lower1sec_ace],
        src.[lower1sec_asoe]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingCpdata7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingCpdata7 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[participantid],
        d.[connectionpointid],
        d.[aggregateenergy],
        d.[purchases],
        d.[lastchanged],
        d.[mda],
        d.[afe],
        d.[dme],
        d.[ufea],
        d.[age],
        d.[soldenergy],
        d.[sales],
        d.[purchasedenergy]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [participantid] varchar(10),
        [connectionpointid] varchar(10),
        [aggregateenergy] decimal(16,6),
        [purchases] decimal(16,6),
        [lastchanged] datetime2,
        [mda] varchar(10),
        [afe] decimal(18,8),
        [dme] decimal(18,8),
        [ufea] decimal(18,8),
        [age] decimal(18,8),
        [soldenergy] decimal(18,8),
        [sales] decimal(18,8),
        [purchasedenergy] decimal(18,8)
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[mda] = src.[mda]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[aggregateenergy] = src.[aggregateenergy],
        tgt.[purchases] = src.[purchases],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[mda] = src.[mda],
        tgt.[afe] = src.[afe],
        tgt.[dme] = src.[dme],
        tgt.[ufea] = src.[ufea],
        tgt.[age] = src.[age],
        tgt.[soldenergy] = src.[soldenergy],
        tgt.[sales] = src.[sales],
        tgt.[purchasedenergy] = src.[purchasedenergy]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [connectionpointid],
        [aggregateenergy],
        [purchases],
        [lastchanged],
        [mda],
        [afe],
        [dme],
        [ufea],
        [age],
        [soldenergy],
        [sales],
        [purchasedenergy]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[connectionpointid],
        src.[aggregateenergy],
        src.[purchases],
        src.[lastchanged],
        src.[mda],
        src.[afe],
        src.[dme],
        src.[ufea],
        src.[age],
        src.[soldenergy],
        src.[sales],
        src.[purchasedenergy]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingDaytrk5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingDaytrk5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [settlementdate],
        [runno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[settlementdate],
        src.[runno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingFees5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingFees5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[marketfeeid] = src.[marketfeeid]
    and tgt.[participantcategoryid] = src.[participantcategoryid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[rate] = src.[rate],
        tgt.[energy] = src.[energy],
        tgt.[value] = src.[value],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[participantcategoryid] = src.[participantcategoryid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[marketfeeid],
        src.[rate],
        src.[energy],
        src.[value],
        src.[lastchanged],
        src.[participantcategoryid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingFinancialadjustments5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingFinancialadjustments5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[adjustmentitem] = src.[adjustmentitem]
    and tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[participanttype] = src.[participanttype],
        tgt.[adjustmentitem] = src.[adjustmentitem],
        tgt.[amount] = src.[amount],
        tgt.[value] = src.[value],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[financialcode] = src.[financialcode],
        tgt.[bas_class] = src.[bas_class]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[participanttype],
        src.[adjustmentitem],
        src.[amount],
        src.[value],
        src.[lastchanged],
        src.[financialcode],
        src.[bas_class]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingGendata5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingGendata5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[stationid] = src.[stationid],
        tgt.[duid] = src.[duid],
        tgt.[aggregateenergy] = src.[aggregateenergy],
        tgt.[sales] = src.[sales],
        tgt.[purchases] = src.[purchases],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[purchasedenergy] = src.[purchasedenergy],
        tgt.[mda] = src.[mda]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[connectionpointid],
        src.[stationid],
        src.[duid],
        src.[aggregateenergy],
        src.[sales],
        src.[purchases],
        src.[lastchanged],
        src.[purchasedenergy],
        src.[mda]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingInterresidues5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingInterresidues5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[allocation] = src.[allocation],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[regionid] = src.[regionid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[allocation],
        src.[totalsurplus],
        src.[interconnectorid],
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[surplusvalue],
        src.[lastchanged],
        src.[regionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIntraresidues5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIntraresidues5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[allocation] = src.[allocation],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[regionid] = src.[regionid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[allocation],
        src.[totalsurplus],
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[surplusvalue],
        src.[lastchanged],
        src.[regionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIraucsurplus5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIraucsurplus5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractid] = src.[contractid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[residueyear] = src.[residueyear],
        tgt.[quarter] = src.[quarter],
        tgt.[billrunno] = src.[billrunno],
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[totalresidues] = src.[totalresidues],
        tgt.[adjustment] = src.[adjustment],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[residueyear],
        src.[quarter],
        src.[billrunno],
        src.[contractid],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[totalresidues],
        src.[adjustment],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIraucsurplussum7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIraucsurplussum7 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[quarter] = src.[quarter]
    and tgt.[residueyear] = src.[residueyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[residueyear] = src.[residueyear],
        tgt.[quarter] = src.[quarter],
        tgt.[billrunno] = src.[billrunno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[participantid] = src.[participantid],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[auctionfees] = src.[auctionfees],
        tgt.[actualpayment] = src.[actualpayment],
        tgt.[auctionfees_gst] = src.[auctionfees_gst],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr],
        tgt.[negative_residues] = src.[negative_residues]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[residueyear],
        src.[quarter],
        src.[billrunno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[participantid],
        src.[totalsurplus],
        src.[auctionfees],
        src.[actualpayment],
        src.[auctionfees_gst],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr],
        src.[negative_residues]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIrnspsurplus5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIrnspsurplus5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractid] = src.[contractid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[residueyear] = src.[residueyear],
        tgt.[quarter] = src.[quarter],
        tgt.[billrunno] = src.[billrunno],
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[totalresidues] = src.[totalresidues],
        tgt.[adjustment] = src.[adjustment],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[residueyear],
        src.[quarter],
        src.[billrunno],
        src.[contractid],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[totalresidues],
        src.[adjustment],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIrnspsurplussum6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIrnspsurplussum6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[quarter] = src.[quarter]
    and tgt.[residueyear] = src.[residueyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[residueyear] = src.[residueyear],
        tgt.[quarter] = src.[quarter],
        tgt.[billrunno] = src.[billrunno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[participantid] = src.[participantid],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[auctionfees] = src.[auctionfees],
        tgt.[auctionfees_gst] = src.[auctionfees_gst],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[residueyear],
        src.[quarter],
        src.[billrunno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[participantid],
        src.[totalsurplus],
        src.[auctionfees],
        src.[auctionfees_gst],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIrpartsurplus5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIrpartsurplus5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractid] = src.[contractid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[residueyear] = src.[residueyear],
        tgt.[quarter] = src.[quarter],
        tgt.[billrunno] = src.[billrunno],
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[totalresidues] = src.[totalresidues],
        tgt.[adjustment] = src.[adjustment],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[actualpayment] = src.[actualpayment]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[residueyear],
        src.[quarter],
        src.[billrunno],
        src.[contractid],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[totalresidues],
        src.[adjustment],
        src.[lastchanged],
        src.[actualpayment]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingIrpartsurplussum7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingIrpartsurplussum7 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[quarter] = src.[quarter]
    and tgt.[residueyear] = src.[residueyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[residueyear] = src.[residueyear],
        tgt.[quarter] = src.[quarter],
        tgt.[billrunno] = src.[billrunno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[participantid] = src.[participantid],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[auctionfees] = src.[auctionfees],
        tgt.[actualpayment] = src.[actualpayment],
        tgt.[auctionfees_gst] = src.[auctionfees_gst],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr],
        tgt.[auctionfees_totalgross_adj] = src.[auctionfees_totalgross_adj]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[residueyear],
        src.[quarter],
        src.[billrunno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[participantid],
        src.[totalsurplus],
        src.[auctionfees],
        src.[actualpayment],
        src.[auctionfees_gst],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr],
        src.[auctionfees_totalgross_adj]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingPrioradjustments5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingPrioradjustments5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[adjbillrunno] = src.[adjbillrunno]
    and tgt.[adjcontractyear] = src.[adjcontractyear]
    and tgt.[adjweekno] = src.[adjweekno]
    and tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[adjcontractyear] = src.[adjcontractyear],
        tgt.[adjweekno] = src.[adjweekno],
        tgt.[adjbillrunno] = src.[adjbillrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[prevamount] = src.[prevamount],
        tgt.[adjamount] = src.[adjamount],
        tgt.[irn] = src.[irn],
        tgt.[irp] = src.[irp],
        tgt.[interestamount] = src.[interestamount],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[irsr_prevamount] = src.[irsr_prevamount],
        tgt.[irsr_adjamount] = src.[irsr_adjamount],
        tgt.[irsr_interestamount] = src.[irsr_interestamount]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[adjcontractyear],
        src.[adjweekno],
        src.[adjbillrunno],
        src.[participantid],
        src.[prevamount],
        src.[adjamount],
        src.[irn],
        src.[irp],
        src.[interestamount],
        src.[lastchanged],
        src.[irsr_prevamount],
        src.[irsr_adjamount],
        src.[irsr_interestamount]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingRealloc5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingRealloc5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[counterparty] = src.[counterparty]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[counterparty] = src.[counterparty],
        tgt.[value] = src.[value],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [counterparty],
        [value],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[counterparty],
        src.[value],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingReallocDetail5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingReallocDetail5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[counterparty] = src.[counterparty]
    and tgt.[participantid] = src.[participantid]
    and tgt.[reallocationid] = src.[reallocationid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[counterparty] = src.[counterparty],
        tgt.[reallocationid] = src.[reallocationid],
        tgt.[value] = src.[value],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [counterparty],
        [reallocationid],
        [value],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[counterparty],
        src.[reallocationid],
        src.[value],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingRegionexports5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingRegionexports5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[exportto] = src.[exportto]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[regionid] = src.[regionid],
        tgt.[exportto] = src.[exportto],
        tgt.[energy] = src.[energy],
        tgt.[value] = src.[value],
        tgt.[surplusenergy] = src.[surplusenergy],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[regionid],
        src.[exportto],
        src.[energy],
        src.[value],
        src.[surplusenergy],
        src.[surplusvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingRegionfigures6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingRegionfigures6 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[wdrsq],
        d.[wdrta]
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
        [lastchanged] datetime2,
        [wdrsq] decimal(18,8),
        [wdrta] decimal(18,8)
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[regionid] = src.[regionid],
        tgt.[energyout] = src.[energyout],
        tgt.[valueout] = src.[valueout],
        tgt.[energypurchased] = src.[energypurchased],
        tgt.[valuepurchased] = src.[valuepurchased],
        tgt.[excessgen] = src.[excessgen],
        tgt.[reservetrading] = src.[reservetrading],
        tgt.[intcompo] = src.[intcompo],
        tgt.[adminpricecompo] = src.[adminpricecompo],
        tgt.[settsurplus] = src.[settsurplus],
        tgt.[aspayment] = src.[aspayment],
        tgt.[poolfees] = src.[poolfees],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[wdrsq] = src.[wdrsq],
        tgt.[wdrta] = src.[wdrta]
when not matched
    then insert (
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
        [lastchanged],
        [wdrsq],
        [wdrta]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[regionid],
        src.[energyout],
        src.[valueout],
        src.[energypurchased],
        src.[valuepurchased],
        src.[excessgen],
        src.[reservetrading],
        src.[intcompo],
        src.[adminpricecompo],
        src.[settsurplus],
        src.[aspayment],
        src.[poolfees],
        src.[lastchanged],
        src.[wdrsq],
        src.[wdrta]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingRegionimports5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingRegionimports5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[importfrom] = src.[importfrom]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[regionid] = src.[regionid],
        tgt.[importfrom] = src.[importfrom],
        tgt.[energy] = src.[energy],
        tgt.[value] = src.[value],
        tgt.[surplusenergy] = src.[surplusenergy],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[regionid],
        src.[importfrom],
        src.[energy],
        src.[value],
        src.[surplusenergy],
        src.[surplusvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingRuntrk5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingRuntrk5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[status] = src.[status],
        tgt.[adj_cleared] = src.[adj_cleared],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[postdate] = src.[postdate],
        tgt.[postby] = src.[postby],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[receiptpostdate] = src.[receiptpostdate],
        tgt.[receiptpostby] = src.[receiptpostby],
        tgt.[paymentpostdate] = src.[paymentpostdate],
        tgt.[paymentpostby] = src.[paymentpostby],
        tgt.[shortfall] = src.[shortfall],
        tgt.[makeup] = src.[makeup]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[status],
        src.[adj_cleared],
        src.[authoriseddate],
        src.[authorisedby],
        src.[postdate],
        src.[postby],
        src.[lastchanged],
        src.[receiptpostdate],
        src.[receiptpostby],
        src.[paymentpostdate],
        src.[paymentpostby],
        src.[shortfall],
        src.[makeup]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingApcCompensation2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingApcCompensation2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[apeventid] = src.[apeventid]
    and tgt.[billrunno] = src.[billrunno]
    and tgt.[claimid] = src.[claimid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[apeventid] = src.[apeventid],
        tgt.[claimid] = src.[claimid],
        tgt.[participantid] = src.[participantid],
        tgt.[compensation_amount] = src.[compensation_amount],
        tgt.[event_type] = src.[event_type],
        tgt.[compensation_type] = src.[compensation_type],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[apeventid],
        src.[claimid],
        src.[participantid],
        src.[compensation_amount],
        src.[event_type],
        src.[compensation_type],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingApcRecovery3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingApcRecovery3 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[participant_ace_mwh],
        d.[region_ace_mwh]
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
        [lastchanged] datetime2,
        [participant_ace_mwh] decimal(18,8),
        [region_ace_mwh] decimal(18,8)
    ) d
) as src 
on (
    tgt.[apeventid] = src.[apeventid]
    and tgt.[billrunno] = src.[billrunno]
    and tgt.[claimid] = src.[claimid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[apeventid] = src.[apeventid],
        tgt.[claimid] = src.[claimid],
        tgt.[participantid] = src.[participantid],
        tgt.[regionid] = src.[regionid],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[eligibility_start_interval] = src.[eligibility_start_interval],
        tgt.[eligibility_end_interval] = src.[eligibility_end_interval],
        tgt.[participant_demand] = src.[participant_demand],
        tgt.[region_demand] = src.[region_demand],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[participant_ace_mwh] = src.[participant_ace_mwh],
        tgt.[region_ace_mwh] = src.[region_ace_mwh]
when not matched
    then insert (
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
        [lastchanged],
        [participant_ace_mwh],
        [region_ace_mwh]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[apeventid],
        src.[claimid],
        src.[participantid],
        src.[regionid],
        src.[recovery_amount],
        src.[eligibility_start_interval],
        src.[eligibility_end_interval],
        src.[participant_demand],
        src.[region_demand],
        src.[lastchanged],
        src.[participant_ace_mwh],
        src.[region_ace_mwh]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingBillingCo2ePublication1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingBillingCo2ePublication1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingBillingCo2ePublicationTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingBillingCo2ePublicationTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingDailyEnergySummary2
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingDailyEnergySummary2(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [settlementdate],
        [participantid],
        [regionid],
        [customer_energy_purchased],
        [generator_energy_sold],
        [generator_energy_purchased],
        [ace_mwh],
        [asoe_mwh],
        [ace_amount],
        [asoe_amount],
        [ce_mwh],
        [ufea_mwh],
        [total_mwh],
        [total_amount]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[settlementdate],
        d.[participantid],
        d.[regionid],
        d.[customer_energy_purchased],
        d.[generator_energy_sold],
        d.[generator_energy_purchased],
        d.[ace_mwh],
        d.[asoe_mwh],
        d.[ace_amount],
        d.[asoe_amount],
        d.[ce_mwh],
        d.[ufea_mwh],
        d.[total_mwh],
        d.[total_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [settlementdate] datetime2,
        [participantid] varchar(20),
        [regionid] varchar(20),
        [customer_energy_purchased] decimal(18,8),
        [generator_energy_sold] decimal(18,8),
        [generator_energy_purchased] decimal(18,8),
        [ace_mwh] decimal(18,8),
        [asoe_mwh] decimal(18,8),
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8),
        [ce_mwh] decimal(18,8),
        [ufea_mwh] decimal(18,8),
        [total_mwh] decimal(18,8),
        [total_amount] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingBillingDirectionReconOther2
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingBillingDirectionReconOther2(
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
        [regional_benefit_factor],
        [region_ace_mwh],
        [region_asoe_mwh],
        [direction_service_id]
)
select 
@file_log_id,
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
        d.[regional_benefit_factor],
        d.[region_ace_mwh],
        d.[region_asoe_mwh],
        d.[direction_service_id]
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
        [regional_benefit_factor] decimal(18,8),
        [region_ace_mwh] decimal(18,8),
        [region_asoe_mwh] decimal(18,8),
        [direction_service_id] varchar(20)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingDirFinalAmount1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingDirFinalAmount1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[direction_id],
        d.[participantid],
        d.[compensation_type],
        d.[provisional_amount],
        d.[final_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [direction_id] varchar(20),
        [participantid] varchar(20),
        [compensation_type] varchar(40),
        [provisional_amount] decimal(18,8),
        [final_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[compensation_type] = src.[compensation_type]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[direction_id] = src.[direction_id]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[direction_id] = src.[direction_id],
        tgt.[participantid] = src.[participantid],
        tgt.[compensation_type] = src.[compensation_type],
        tgt.[provisional_amount] = src.[provisional_amount],
        tgt.[final_amount] = src.[final_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [direction_id],
        [participantid],
        [compensation_type],
        [provisional_amount],
        [final_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[direction_id],
        src.[participantid],
        src.[compensation_type],
        src.[provisional_amount],
        src.[final_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingDirFinalRecovery1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingDirFinalRecovery1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[direction_id],
        d.[participantid],
        d.[cra_amount],
        d.[provisional_amount],
        d.[final_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [direction_id] varchar(20),
        [participantid] varchar(20),
        [cra_amount] decimal(18,8),
        [provisional_amount] decimal(18,8),
        [final_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[direction_id] = src.[direction_id]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[direction_id] = src.[direction_id],
        tgt.[participantid] = src.[participantid],
        tgt.[cra_amount] = src.[cra_amount],
        tgt.[provisional_amount] = src.[provisional_amount],
        tgt.[final_amount] = src.[final_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [direction_id],
        [participantid],
        [cra_amount],
        [provisional_amount],
        [final_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[direction_id],
        src.[participantid],
        src.[cra_amount],
        src.[provisional_amount],
        src.[final_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingDirProvAmount1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingDirProvAmount1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[direction_id],
        d.[participantid],
        d.[compensation_type],
        d.[compensation_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [direction_id] varchar(20),
        [participantid] varchar(20),
        [compensation_type] varchar(40),
        [compensation_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[compensation_type] = src.[compensation_type]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[direction_id] = src.[direction_id]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[direction_id] = src.[direction_id],
        tgt.[participantid] = src.[participantid],
        tgt.[compensation_type] = src.[compensation_type],
        tgt.[compensation_amount] = src.[compensation_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [direction_id],
        [participantid],
        [compensation_type],
        [compensation_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[direction_id],
        src.[participantid],
        src.[compensation_type],
        src.[compensation_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingDirProvRecovery1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingDirProvRecovery1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[direction_id],
        d.[participantid],
        d.[cra_amount],
        d.[recovery_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [direction_id] varchar(20),
        [participantid] varchar(20),
        [cra_amount] decimal(18,8),
        [recovery_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[direction_id] = src.[direction_id]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[direction_id] = src.[direction_id],
        tgt.[participantid] = src.[participantid],
        tgt.[cra_amount] = src.[cra_amount],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [direction_id],
        [participantid],
        [cra_amount],
        [recovery_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[direction_id],
        src.[participantid],
        src.[cra_amount],
        src.[recovery_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingDirRecoveryDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingDirRecoveryDetail1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[direction_id],
        d.[participantid],
        d.[participantcategoryid],
        d.[regionid],
        d.[recovery_amount],
        d.[recovery_energy],
        d.[region_energy],
        d.[excluded_energy],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [direction_id] varchar(20),
        [participantid] varchar(20),
        [participantcategoryid] varchar(20),
        [regionid] varchar(20),
        [recovery_amount] decimal(18,8),
        [recovery_energy] decimal(18,8),
        [region_energy] decimal(18,8),
        [excluded_energy] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[direction_id] = src.[direction_id]
    and tgt.[participantcategoryid] = src.[participantcategoryid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[direction_id] = src.[direction_id],
        tgt.[participantid] = src.[participantid],
        tgt.[participantcategoryid] = src.[participantcategoryid],
        tgt.[regionid] = src.[regionid],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[recovery_energy] = src.[recovery_energy],
        tgt.[region_energy] = src.[region_energy],
        tgt.[excluded_energy] = src.[excluded_energy],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [direction_id],
        [participantid],
        [participantcategoryid],
        [regionid],
        [recovery_amount],
        [recovery_energy],
        [region_energy],
        [excluded_energy],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[direction_id],
        src.[participantid],
        src.[participantcategoryid],
        src.[regionid],
        src.[recovery_amount],
        src.[recovery_energy],
        src.[region_energy],
        src.[excluded_energy],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingEftshortfallAmount1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingEftshortfallAmount1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingEftshortfallDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingEftshortfallDetail1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [transaction_type],
        [amount]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingEnergyGensetDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingEnergyGensetDetail1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[participantid],
        d.[stationid],
        d.[duid],
        d.[gensetid],
        d.[regionid],
        d.[connectionpointid],
        d.[meterid],
        d.[ce_mwh],
        d.[ufea_mwh],
        d.[ace_mwh],
        d.[asoe_mwh],
        d.[total_mwh],
        d.[dme_mwh],
        d.[ace_amount],
        d.[asoe_amount],
        d.[total_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(4,0),
        [participantid] varchar(20),
        [stationid] varchar(20),
        [duid] varchar(20),
        [gensetid] varchar(20),
        [regionid] varchar(20),
        [connectionpointid] varchar(20),
        [meterid] varchar(20),
        [ce_mwh] decimal(18,8),
        [ufea_mwh] decimal(18,8),
        [ace_mwh] decimal(18,8),
        [asoe_mwh] decimal(18,8),
        [total_mwh] decimal(18,8),
        [dme_mwh] decimal(18,8),
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8),
        [total_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[duid] = src.[duid]
    and tgt.[gensetid] = src.[gensetid]
    and tgt.[meterid] = src.[meterid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[stationid] = src.[stationid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[stationid] = src.[stationid],
        tgt.[duid] = src.[duid],
        tgt.[gensetid] = src.[gensetid],
        tgt.[regionid] = src.[regionid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[meterid] = src.[meterid],
        tgt.[ce_mwh] = src.[ce_mwh],
        tgt.[ufea_mwh] = src.[ufea_mwh],
        tgt.[ace_mwh] = src.[ace_mwh],
        tgt.[asoe_mwh] = src.[asoe_mwh],
        tgt.[total_mwh] = src.[total_mwh],
        tgt.[dme_mwh] = src.[dme_mwh],
        tgt.[ace_amount] = src.[ace_amount],
        tgt.[asoe_amount] = src.[asoe_amount],
        tgt.[total_amount] = src.[total_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [stationid],
        [duid],
        [gensetid],
        [regionid],
        [connectionpointid],
        [meterid],
        [ce_mwh],
        [ufea_mwh],
        [ace_mwh],
        [asoe_mwh],
        [total_mwh],
        [dme_mwh],
        [ace_amount],
        [asoe_amount],
        [total_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[stationid],
        src.[duid],
        src.[gensetid],
        src.[regionid],
        src.[connectionpointid],
        src.[meterid],
        src.[ce_mwh],
        src.[ufea_mwh],
        src.[ace_mwh],
        src.[asoe_mwh],
        src.[total_mwh],
        src.[dme_mwh],
        src.[ace_amount],
        src.[asoe_amount],
        src.[total_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingEnergyTransaction1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingEnergyTransaction1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[participantid],
        d.[connectionpointid],
        d.[regionid],
        d.[ce_mwh],
        d.[ufea_mwh],
        d.[ace_mwh],
        d.[asoe_mwh],
        d.[ace_amount],
        d.[asoe_amount],
        d.[total_mwh],
        d.[total_amount],
        d.[dme_mwh],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(4,0),
        [participantid] varchar(20),
        [connectionpointid] varchar(20),
        [regionid] varchar(20),
        [ce_mwh] decimal(18,8),
        [ufea_mwh] decimal(18,8),
        [ace_mwh] decimal(18,8),
        [asoe_mwh] decimal(18,8),
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8),
        [total_mwh] decimal(18,8),
        [total_amount] decimal(18,8),
        [dme_mwh] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[regionid] = src.[regionid],
        tgt.[ce_mwh] = src.[ce_mwh],
        tgt.[ufea_mwh] = src.[ufea_mwh],
        tgt.[ace_mwh] = src.[ace_mwh],
        tgt.[asoe_mwh] = src.[asoe_mwh],
        tgt.[ace_amount] = src.[ace_amount],
        tgt.[asoe_amount] = src.[asoe_amount],
        tgt.[total_mwh] = src.[total_mwh],
        tgt.[total_amount] = src.[total_amount],
        tgt.[dme_mwh] = src.[dme_mwh],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [connectionpointid],
        [regionid],
        [ce_mwh],
        [ufea_mwh],
        [ace_mwh],
        [asoe_mwh],
        [ace_amount],
        [asoe_amount],
        [total_mwh],
        [total_amount],
        [dme_mwh],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[connectionpointid],
        src.[regionid],
        src.[ce_mwh],
        src.[ufea_mwh],
        src.[ace_mwh],
        src.[asoe_mwh],
        src.[ace_amount],
        src.[asoe_amount],
        src.[total_mwh],
        src.[total_amount],
        src.[dme_mwh],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingEnergyTranSaps1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingEnergyTranSaps1 as tgt 
using (
    select 
        d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[participantid],
        d.[tni],
        d.[regionid],
        d.[consumed_energy_mwh],
        d.[sentout_energy_mwh],
        d.[consumed_energy_cost],
        d.[sentout_energy_cost],
        d.[lastchanged]
    from openjson(@data) with (
        [contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [participantid] varchar(20),
        [tni] varchar(20),
        [regionid] varchar(20),
        [consumed_energy_mwh] decimal(18,8),
        [sentout_energy_mwh] decimal(18,8),
        [consumed_energy_cost] decimal(18,8),
        [sentout_energy_cost] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[tni] = src.[tni]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[tni] = src.[tni],
        tgt.[regionid] = src.[regionid],
        tgt.[consumed_energy_mwh] = src.[consumed_energy_mwh],
        tgt.[sentout_energy_mwh] = src.[sentout_energy_mwh],
        tgt.[consumed_energy_cost] = src.[consumed_energy_cost],
        tgt.[sentout_energy_cost] = src.[sentout_energy_cost],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [tni],
        [regionid],
        [consumed_energy_mwh],
        [sentout_energy_mwh],
        [consumed_energy_cost],
        [sentout_energy_cost],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[tni],
        src.[regionid],
        src.[consumed_energy_mwh],
        src.[sentout_energy_mwh],
        src.[consumed_energy_cost],
        src.[sentout_energy_cost],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingGstDetail5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingGstDetail5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bas_class] = src.[bas_class]
    and tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[transaction_type] = src.[transaction_type]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[bas_class] = src.[bas_class],
        tgt.[transaction_type] = src.[transaction_type],
        tgt.[gst_exclusive_amount] = src.[gst_exclusive_amount],
        tgt.[gst_amount] = src.[gst_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[bas_class],
        src.[transaction_type],
        src.[gst_exclusive_amount],
        src.[gst_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingGstSummary5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingGstSummary5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bas_class] = src.[bas_class]
    and tgt.[billrunno] = src.[billrunno]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[bas_class] = src.[bas_class],
        tgt.[gst_exclusive_amount] = src.[gst_exclusive_amount],
        tgt.[gst_amount] = src.[gst_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [bas_class],
        [gst_exclusive_amount],
        [gst_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[bas_class],
        src.[gst_exclusive_amount],
        src.[gst_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingNmasTstPayments1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingNmasTstPayments1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingNmasTstRecovery2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingNmasTstRecovery2 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[participant_ace_mwh],
        d.[region_ace_mwh],
        d.[ace_portion],
        d.[asoe_portion],
        d.[participant_asoe_mwh],
        d.[region_asoe_mwh],
        d.[recoveryamount_ace],
        d.[recoveryamount_asoe]
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
        [lastchanged] datetime2,
        [participant_ace_mwh] decimal(18,8),
        [region_ace_mwh] decimal(18,8),
        [ace_portion] decimal(18,8),
        [asoe_portion] decimal(18,8),
        [participant_asoe_mwh] decimal(18,8),
        [region_asoe_mwh] decimal(18,8),
        [recoveryamount_ace] decimal(18,8),
        [recoveryamount_asoe] decimal(18,8)
    ) d
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractid] = src.[contractid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[service] = src.[service]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[service] = src.[service],
        tgt.[contractid] = src.[contractid],
        tgt.[regionid] = src.[regionid],
        tgt.[rbf] = src.[rbf],
        tgt.[test_payment] = src.[test_payment],
        tgt.[recovery_start_date] = src.[recovery_start_date],
        tgt.[recovery_end_date] = src.[recovery_end_date],
        tgt.[participant_energy] = src.[participant_energy],
        tgt.[region_energy] = src.[region_energy],
        tgt.[nem_energy] = src.[nem_energy],
        tgt.[customer_proportion] = src.[customer_proportion],
        tgt.[generator_proportion] = src.[generator_proportion],
        tgt.[participant_generation] = src.[participant_generation],
        tgt.[nem_generation] = src.[nem_generation],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[participant_ace_mwh] = src.[participant_ace_mwh],
        tgt.[region_ace_mwh] = src.[region_ace_mwh],
        tgt.[ace_portion] = src.[ace_portion],
        tgt.[asoe_portion] = src.[asoe_portion],
        tgt.[participant_asoe_mwh] = src.[participant_asoe_mwh],
        tgt.[region_asoe_mwh] = src.[region_asoe_mwh],
        tgt.[recoveryamount_ace] = src.[recoveryamount_ace],
        tgt.[recoveryamount_asoe] = src.[recoveryamount_asoe]
when not matched
    then insert (
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
        [lastchanged],
        [participant_ace_mwh],
        [region_ace_mwh],
        [ace_portion],
        [asoe_portion],
        [participant_asoe_mwh],
        [region_asoe_mwh],
        [recoveryamount_ace],
        [recoveryamount_asoe]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[participantid],
        src.[service],
        src.[contractid],
        src.[regionid],
        src.[rbf],
        src.[test_payment],
        src.[recovery_start_date],
        src.[recovery_end_date],
        src.[participant_energy],
        src.[region_energy],
        src.[nem_energy],
        src.[customer_proportion],
        src.[generator_proportion],
        src.[participant_generation],
        src.[nem_generation],
        src.[recovery_amount],
        src.[lastchanged],
        src.[participant_ace_mwh],
        src.[region_ace_mwh],
        src.[ace_portion],
        src.[asoe_portion],
        src.[participant_asoe_mwh],
        src.[region_asoe_mwh],
        src.[recoveryamount_ace],
        src.[recoveryamount_asoe]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingNmasTstRecvryRbf1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BillingNmasTstRecvryRbf1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[billrunno] = src.[billrunno]
    and tgt.[contractid] = src.[contractid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[regionid] = src.[regionid]
    and tgt.[service] = src.[service]
    and tgt.[weekno] = src.[weekno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[weekno] = src.[weekno],
        tgt.[billrunno] = src.[billrunno],
        tgt.[service] = src.[service],
        tgt.[contractid] = src.[contractid],
        tgt.[regionid] = src.[regionid],
        tgt.[rbf] = src.[rbf],
        tgt.[payment_amount] = src.[payment_amount],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[weekno],
        src.[billrunno],
        src.[service],
        src.[contractid],
        src.[regionid],
        src.[rbf],
        src.[payment_amount],
        src.[recovery_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBillingNmasTstRecvryTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingNmasTstRecvryTrk1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [recovery_contractyear],
        [recovery_weekno],
        [recovery_billrunno]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingSecdepositApplication1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingSecdepositApplication1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [application_amount]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingSecdepInterestPay1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingSecdepInterestPay1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingSecdepInterestRate1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingSecdepInterestRate1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [interest_acct_id],
        [effectivedate],
        [interest_rate]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertBillingSubstDemand1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingSubstDemand1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [settlementdate],
        [tni],
        [participantid],
        [regionid],
        [substitutedemand]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[settlementdate],
        d.[tni],
        d.[participantid],
        d.[regionid],
        d.[substitutedemand]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [settlementdate] datetime2,
        [tni] varchar(20),
        [participantid] varchar(20),
        [regionid] varchar(20),
        [substitutedemand] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingSubstRunVersion1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingSubstRunVersion1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [referencesettlementdate],
        [referencesettlementrunno]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[referencesettlementdate],
        d.[referencesettlementrunno]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [referencesettlementdate] datetime2,
        [referencesettlementrunno] decimal(3,0)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingWdr1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingWdr1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [wdr_credit_amount],
        [wdr_debit_amount]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[participantid],
        d.[wdr_credit_amount],
        d.[wdr_debit_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [participantid] varchar(20),
        [wdr_credit_amount] decimal(18,8),
        [wdr_debit_amount] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingWdrDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingWdrDetail1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [wdrrrperiod],
        [regionid],
        [frmp],
        [drsp],
        [wdrsq],
        [wdrrr],
        [wdrta]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[wdrrrperiod],
        d.[regionid],
        d.[frmp],
        d.[drsp],
        d.[wdrsq],
        d.[wdrrr],
        d.[wdrta]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [wdrrrperiod] varchar(20),
        [regionid] varchar(20),
        [frmp] varchar(20),
        [drsp] varchar(20),
        [wdrsq] decimal(18,8),
        [wdrrr] decimal(18,8),
        [wdrta] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingReservetraderpayment1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingReservetraderpayment1(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [participantid],
        [contractid],
        [payment_id],
        [payment_type],
        [payment_amount]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[participantid],
        d.[contractid],
        d.[payment_id],
        d.[payment_type],
        d.[payment_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [participantid] varchar(20),
        [contractid] varchar(20),
        [payment_id] decimal(3,0),
        [payment_type] varchar(40),
        [payment_amount] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertBillingReservetraderrecovery3
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.BillingReservetraderrecovery3(
file_log_id,
[contractyear],
        [weekno],
        [billrunno],
        [publication_id],
        [payment_id],
        [payment_amount],
        [participantid],
        [regionid],
        [participant_demand],
        [region_demand],
        [eligibility_start_interval],
        [eligibility_end_interval],
        [recovery_amount],
        [excluded_energy],
        [participant_ace_mwh],
        [region_ace_mwh]
)
select 
@file_log_id,
d.[contractyear],
        d.[weekno],
        d.[billrunno],
        d.[publication_id],
        d.[payment_id],
        d.[payment_amount],
        d.[participantid],
        d.[regionid],
        d.[participant_demand],
        d.[region_demand],
        d.[eligibility_start_interval],
        d.[eligibility_end_interval],
        d.[recovery_amount],
        d.[excluded_energy],
        d.[participant_ace_mwh],
        d.[region_ace_mwh]
from openjson(@data) with (
[contractyear] decimal(4,0),
        [weekno] decimal(3,0),
        [billrunno] decimal(3,0),
        [publication_id] varchar(40),
        [payment_id] decimal(3,0),
        [payment_amount] decimal(18,8),
        [participantid] varchar(20),
        [regionid] varchar(20),
        [participant_demand] decimal(18,8),
        [region_demand] decimal(18,8),
        [eligibility_start_interval] datetime2,
        [eligibility_end_interval] datetime2,
        [recovery_amount] decimal(18,8),
        [excluded_energy] decimal(18,8),
        [participant_ace_mwh] decimal(18,8),
        [region_ace_mwh] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertOperationalDemandActual3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.OperationalDemandActual3 as tgt 
using (
    select 
        d.[interval_datetime],
        d.[regionid],
        d.[operational_demand],
        d.[lastchanged],
        d.[operational_demand_adjustment],
        d.[wdr_estimate]
    from openjson(@data) with (
        [interval_datetime] datetime2,
        [regionid] varchar(20),
        [operational_demand] decimal(10,0),
        [lastchanged] datetime2,
        [operational_demand_adjustment] decimal(10,0),
        [wdr_estimate] decimal(10,0)
    ) d
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[operational_demand] = src.[operational_demand],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[operational_demand_adjustment] = src.[operational_demand_adjustment],
        tgt.[wdr_estimate] = src.[wdr_estimate]
when not matched
    then insert (
        file_log_id,
        [interval_datetime],
        [regionid],
        [operational_demand],
        [lastchanged],
        [operational_demand_adjustment],
        [wdr_estimate]
    ) values (
        @file_log_id,
        src.[interval_datetime],
        src.[regionid],
        src.[operational_demand],
        src.[lastchanged],
        src.[operational_demand_adjustment],
        src.[wdr_estimate]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertOperationalDemandForecast1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.OperationalDemandForecast1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[load_date] = src.[load_date],
        tgt.[operational_demand_poe10] = src.[operational_demand_poe10],
        tgt.[operational_demand_poe50] = src.[operational_demand_poe50],
        tgt.[operational_demand_poe90] = src.[operational_demand_poe90],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [interval_datetime],
        [regionid],
        [load_date],
        [operational_demand_poe10],
        [operational_demand_poe50],
        [operational_demand_poe90],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[interval_datetime],
        src.[regionid],
        src.[load_date],
        src.[operational_demand_poe10],
        src.[operational_demand_poe50],
        src.[operational_demand_poe90],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDemandIntermittentClusterAvail2
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DemandIntermittentClusterAvail2(
file_log_id,
[tradingdate],
        [duid],
        [offerdatetime],
        [clusterid],
        [periodid],
        [elements_unavailable],
        [elements_available]
)
select 
@file_log_id,
d.[tradingdate],
        d.[duid],
        d.[offerdatetime],
        d.[clusterid],
        d.[periodid],
        d.[elements_unavailable],
        d.[elements_available]
from openjson(@data) with (
[tradingdate] datetime2,
        [duid] varchar(20),
        [offerdatetime] datetime2,
        [clusterid] varchar(20),
        [periodid] decimal(3,0),
        [elements_unavailable] decimal(5,0),
        [elements_available] decimal(5,0)
) d
end
go
create or alter procedure mmsdm_proc.InsertDemandIntermittentClusterAvailDay1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DemandIntermittentClusterAvailDay1(
file_log_id,
[tradingdate],
        [duid],
        [offerdatetime],
        [clusterid]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDemandIntermittentDsPred1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DemandIntermittentDsPred1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDemandIntermittentDsRun1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DemandIntermittentDsRun1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[forecast_priority] = src.[forecast_priority]
    and tgt.[offerdatetime] = src.[offerdatetime]
    and tgt.[origin] = src.[origin]
    and tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[duid] = src.[duid],
        tgt.[offerdatetime] = src.[offerdatetime],
        tgt.[origin] = src.[origin],
        tgt.[forecast_priority] = src.[forecast_priority],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[comments] = src.[comments],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[model] = src.[model],
        tgt.[participant_timestamp] = src.[participant_timestamp],
        tgt.[suppressed_aemo] = src.[suppressed_aemo],
        tgt.[suppressed_participant] = src.[suppressed_participant],
        tgt.[transaction_id] = src.[transaction_id]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[duid],
        src.[offerdatetime],
        src.[origin],
        src.[forecast_priority],
        src.[authorisedby],
        src.[comments],
        src.[lastchanged],
        src.[model],
        src.[participant_timestamp],
        src.[suppressed_aemo],
        src.[suppressed_participant],
        src.[transaction_id]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForecastIntermittentGen1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForecastIntermittentGen1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[duid] = src.[duid],
        tgt.[start_interval_datetime] = src.[start_interval_datetime],
        tgt.[end_interval_datetime] = src.[end_interval_datetime],
        tgt.[versionno] = src.[versionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [duid],
        [start_interval_datetime],
        [end_interval_datetime],
        [versionno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[duid],
        src.[start_interval_datetime],
        src.[end_interval_datetime],
        src.[versionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForecastIntermittentGenData1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForecastIntermittentGenData1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[duid] = src.[duid],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[powermean] = src.[powermean],
        tgt.[powerpoe50] = src.[powerpoe50],
        tgt.[powerpoelow] = src.[powerpoelow],
        tgt.[powerpoehigh] = src.[powerpoehigh],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [duid],
        [interval_datetime],
        [powermean],
        [powerpoe50],
        [powerpoelow],
        [powerpoehigh],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[duid],
        src.[interval_datetime],
        src.[powermean],
        src.[powerpoe50],
        src.[powerpoelow],
        src.[powerpoehigh],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDemandIntermittentGenLimit1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DemandIntermittentGenLimit1(
file_log_id,
[tradingdate],
        [duid],
        [offerdatetime],
        [periodid],
        [uppermwlimit]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDemandIntermittentGenLimitDay1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DemandIntermittentGenLimitDay1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[offerdatetime] = src.[offerdatetime]
    and tgt.[tradingdate] = src.[tradingdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[tradingdate] = src.[tradingdate],
        tgt.[duid] = src.[duid],
        tgt.[offerdatetime] = src.[offerdatetime],
        tgt.[participantid] = src.[participantid],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[authorisedbyuser] = src.[authorisedbyuser],
        tgt.[authorisedbyparticipantid] = src.[authorisedbyparticipantid]
when not matched
    then insert (
        file_log_id,
        [tradingdate],
        [duid],
        [offerdatetime],
        [participantid],
        [lastchanged],
        [authorisedbyuser],
        [authorisedbyparticipantid]
    ) values (
        @file_log_id,
        src.[tradingdate],
        src.[duid],
        src.[offerdatetime],
        src.[participantid],
        src.[lastchanged],
        src.[authorisedbyuser],
        src.[authorisedbyparticipantid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDemandIntermittentGenScada1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DemandIntermittentGenScada1(
file_log_id,
[run_datetime],
        [duid],
        [scada_type],
        [scada_value],
        [scada_quality]
)
select 
@file_log_id,
d.[run_datetime],
        d.[duid],
        d.[scada_type],
        d.[scada_value],
        d.[scada_quality]
from openjson(@data) with (
[run_datetime] datetime2,
        [duid] varchar(20),
        [scada_type] varchar(20),
        [scada_value] decimal(15,5),
        [scada_quality] varchar(20)
) d
end
go
create or alter procedure mmsdm_proc.InsertDemandMtpasaIntermittentAvail2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DemandMtpasaIntermittentAvail2 as tgt 
using (
    select 
        d.[tradingdate],
        d.[duid],
        d.[offerdatetime],
        d.[clusterid],
        d.[lastchanged],
        d.[elements_unavailable],
        d.[elements_available]
    from openjson(@data) with (
        [tradingdate] datetime2,
        [duid] varchar(20),
        [offerdatetime] datetime2,
        [clusterid] varchar(20),
        [lastchanged] datetime2,
        [elements_unavailable] decimal(5,0),
        [elements_available] decimal(5,0)
    ) d
) as src 
on (
    tgt.[clusterid] = src.[clusterid]
    and tgt.[duid] = src.[duid]
    and tgt.[offerdatetime] = src.[offerdatetime]
    and tgt.[tradingdate] = src.[tradingdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[tradingdate] = src.[tradingdate],
        tgt.[duid] = src.[duid],
        tgt.[offerdatetime] = src.[offerdatetime],
        tgt.[clusterid] = src.[clusterid],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[elements_unavailable] = src.[elements_unavailable],
        tgt.[elements_available] = src.[elements_available]
when not matched
    then insert (
        file_log_id,
        [tradingdate],
        [duid],
        [offerdatetime],
        [clusterid],
        [lastchanged],
        [elements_unavailable],
        [elements_available]
    ) values (
        @file_log_id,
        src.[tradingdate],
        src.[duid],
        src.[offerdatetime],
        src.[clusterid],
        src.[lastchanged],
        src.[elements_unavailable],
        src.[elements_available]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDemandMtpasaIntermittentLimit1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DemandMtpasaIntermittentLimit1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[offerdatetime] = src.[offerdatetime]
    and tgt.[tradingdate] = src.[tradingdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[tradingdate] = src.[tradingdate],
        tgt.[duid] = src.[duid],
        tgt.[offerdatetime] = src.[offerdatetime],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[uppermwlimit] = src.[uppermwlimit],
        tgt.[authorisedbyuser] = src.[authorisedbyuser],
        tgt.[authorisedbyparticipantid] = src.[authorisedbyparticipantid]
when not matched
    then insert (
        file_log_id,
        [tradingdate],
        [duid],
        [offerdatetime],
        [lastchanged],
        [uppermwlimit],
        [authorisedbyuser],
        [authorisedbyparticipantid]
    ) values (
        @file_log_id,
        src.[tradingdate],
        src.[duid],
        src.[offerdatetime],
        src.[lastchanged],
        src.[uppermwlimit],
        src.[authorisedbyuser],
        src.[authorisedbyparticipantid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDemandPeriod1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DemandPeriod1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[offerdate] = src.[offerdate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[regionid] = src.[regionid],
        tgt.[offerdate] = src.[offerdate],
        tgt.[periodid] = src.[periodid],
        tgt.[versionno] = src.[versionno],
        tgt.[resdemand] = src.[resdemand],
        tgt.[demand90probability] = src.[demand90probability],
        tgt.[demand10probability] = src.[demand10probability],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[mr_schedule] = src.[mr_schedule]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[settlementdate],
        src.[regionid],
        src.[offerdate],
        src.[periodid],
        src.[versionno],
        src.[resdemand],
        src.[demand90probability],
        src.[demand10probability],
        src.[lastchanged],
        src.[mr_schedule]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDemandTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DemandTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[regionid] = src.[regionid],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[filename] = src.[filename],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [regionid],
        [offerdate],
        [versionno],
        [filename],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[regionid],
        src.[offerdate],
        src.[versionno],
        src.[filename],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertRooftopActual2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.RooftopActual2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[type] = src.[type]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[type] = src.[type],
        tgt.[regionid] = src.[regionid],
        tgt.[power] = src.[power],
        tgt.[qi] = src.[qi],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [interval_datetime],
        [type],
        [regionid],
        [power],
        [qi],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[interval_datetime],
        src.[type],
        src.[regionid],
        src.[power],
        src.[qi],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertRooftopForecast1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.RooftopForecast1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[powermean] = src.[powermean],
        tgt.[powerpoe50] = src.[powerpoe50],
        tgt.[powerpoelow] = src.[powerpoelow],
        tgt.[powerpoehigh] = src.[powerpoehigh],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [version_datetime],
        [regionid],
        [interval_datetime],
        [powermean],
        [powerpoe50],
        [powerpoelow],
        [powerpoehigh],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[version_datetime],
        src.[regionid],
        src.[interval_datetime],
        src.[powermean],
        src.[powerpoe50],
        src.[powerpoelow],
        src.[powerpoehigh],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchocdConstraintrelaxation2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchocdConstraintrelaxation2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[constraintid] = src.[constraintid],
        tgt.[rhs] = src.[rhs],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[versionno] = src.[versionno]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [constraintid],
        [rhs],
        [lastchanged],
        [versionno]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[constraintid],
        src.[rhs],
        src.[lastchanged],
        src.[versionno]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchBlockedConstraints1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DispatchBlockedConstraints1(
file_log_id,
[settlementdate],
        [runno],
        [constraintid]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDispatchCaseSolution2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchCaseSolution2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[casesubtype] = src.[casesubtype],
        tgt.[solutionstatus] = src.[solutionstatus],
        tgt.[spdversion] = src.[spdversion],
        tgt.[nonphysicallosses] = src.[nonphysicallosses],
        tgt.[totalobjective] = src.[totalobjective],
        tgt.[totalareagenviolation] = src.[totalareagenviolation],
        tgt.[totalinterconnectorviolation] = src.[totalinterconnectorviolation],
        tgt.[totalgenericviolation] = src.[totalgenericviolation],
        tgt.[totalramprateviolation] = src.[totalramprateviolation],
        tgt.[totalunitmwcapacityviolation] = src.[totalunitmwcapacityviolation],
        tgt.[total5minviolation] = src.[total5minviolation],
        tgt.[totalregviolation] = src.[totalregviolation],
        tgt.[total6secviolation] = src.[total6secviolation],
        tgt.[total60secviolation] = src.[total60secviolation],
        tgt.[totalasprofileviolation] = src.[totalasprofileviolation],
        tgt.[totalfaststartviolation] = src.[totalfaststartviolation],
        tgt.[totalenergyofferviolation] = src.[totalenergyofferviolation],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[switchruninitialstatus] = src.[switchruninitialstatus],
        tgt.[switchrunbeststatus] = src.[switchrunbeststatus],
        tgt.[switchrunbeststatus_int] = src.[switchrunbeststatus_int]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[intervention],
        src.[casesubtype],
        src.[solutionstatus],
        src.[spdversion],
        src.[nonphysicallosses],
        src.[totalobjective],
        src.[totalareagenviolation],
        src.[totalinterconnectorviolation],
        src.[totalgenericviolation],
        src.[totalramprateviolation],
        src.[totalunitmwcapacityviolation],
        src.[total5minviolation],
        src.[totalregviolation],
        src.[total6secviolation],
        src.[total60secviolation],
        src.[totalasprofileviolation],
        src.[totalfaststartviolation],
        src.[totalenergyofferviolation],
        src.[lastchanged],
        src.[switchruninitialstatus],
        src.[switchrunbeststatus],
        src.[switchrunbeststatus_int]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchConstraint5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchConstraint5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[dispatchinterval] = src.[dispatchinterval]
    and tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[constraintid] = src.[constraintid],
        tgt.[dispatchinterval] = src.[dispatchinterval],
        tgt.[intervention] = src.[intervention],
        tgt.[rhs] = src.[rhs],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[duid] = src.[duid],
        tgt.[genconid_effectivedate] = src.[genconid_effectivedate],
        tgt.[genconid_versionno] = src.[genconid_versionno],
        tgt.[lhs] = src.[lhs]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[constraintid],
        src.[dispatchinterval],
        src.[intervention],
        src.[rhs],
        src.[marginalvalue],
        src.[violationdegree],
        src.[lastchanged],
        src.[duid],
        src.[genconid_effectivedate],
        src.[genconid_versionno],
        src.[lhs]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchInterconnectorres3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchInterconnectorres3 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[dispatchinterval] = src.[dispatchinterval]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[dispatchinterval] = src.[dispatchinterval],
        tgt.[intervention] = src.[intervention],
        tgt.[meteredmwflow] = src.[meteredmwflow],
        tgt.[mwflow] = src.[mwflow],
        tgt.[mwlosses] = src.[mwlosses],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[exportlimit] = src.[exportlimit],
        tgt.[importlimit] = src.[importlimit],
        tgt.[marginalloss] = src.[marginalloss],
        tgt.[exportgenconid] = src.[exportgenconid],
        tgt.[importgenconid] = src.[importgenconid],
        tgt.[fcasexportlimit] = src.[fcasexportlimit],
        tgt.[fcasimportlimit] = src.[fcasimportlimit],
        tgt.[local_price_adjustment_export] = src.[local_price_adjustment_export],
        tgt.[locally_constrained_export] = src.[locally_constrained_export],
        tgt.[local_price_adjustment_import] = src.[local_price_adjustment_import],
        tgt.[locally_constrained_import] = src.[locally_constrained_import]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[interconnectorid],
        src.[dispatchinterval],
        src.[intervention],
        src.[meteredmwflow],
        src.[mwflow],
        src.[mwlosses],
        src.[marginalvalue],
        src.[violationdegree],
        src.[lastchanged],
        src.[exportlimit],
        src.[importlimit],
        src.[marginalloss],
        src.[exportgenconid],
        src.[importgenconid],
        src.[fcasexportlimit],
        src.[fcasimportlimit],
        src.[local_price_adjustment_export],
        src.[locally_constrained_export],
        src.[local_price_adjustment_import],
        src.[locally_constrained_import]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchUnitSolution5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchUnitSolution5 as tgt 
using (
    select 
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
        d.[semidispatchcap],
        d.[dispatchmodetime],
        d.[conformance_mode],
        d.[uigf],
        d.[raise1sec],
        d.[raise1secflags],
        d.[lower1sec],
        d.[lower1secflags],
        d.[raise1secactualavailability],
        d.[lower1secactualavailability],
        d.[initial_energy_storage],
        d.[energy_storage],
        d.[min_availability]
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
        [semidispatchcap] decimal(3,0),
        [dispatchmodetime] decimal(4,0),
        [conformance_mode] decimal(6,0),
        [uigf] decimal(15,5),
        [raise1sec] decimal(15,5),
        [raise1secflags] decimal(3,0),
        [lower1sec] decimal(15,5),
        [lower1secflags] decimal(3,0),
        [raise1secactualavailability] decimal(16,6),
        [lower1secactualavailability] decimal(16,6),
        [initial_energy_storage] decimal(15,5),
        [energy_storage] decimal(15,5),
        [min_availability] decimal(15,5)
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[duid] = src.[duid],
        tgt.[tradetype] = src.[tradetype],
        tgt.[dispatchinterval] = src.[dispatchinterval],
        tgt.[intervention] = src.[intervention],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[dispatchmode] = src.[dispatchmode],
        tgt.[agcstatus] = src.[agcstatus],
        tgt.[initialmw] = src.[initialmw],
        tgt.[totalcleared] = src.[totalcleared],
        tgt.[rampdownrate] = src.[rampdownrate],
        tgt.[rampuprate] = src.[rampuprate],
        tgt.[lower5min] = src.[lower5min],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise5min] = src.[raise5min],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[downepf] = src.[downepf],
        tgt.[upepf] = src.[upepf],
        tgt.[marginal5minvalue] = src.[marginal5minvalue],
        tgt.[marginal60secvalue] = src.[marginal60secvalue],
        tgt.[marginal6secvalue] = src.[marginal6secvalue],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violation5mindegree] = src.[violation5mindegree],
        tgt.[violation60secdegree] = src.[violation60secdegree],
        tgt.[violation6secdegree] = src.[violation6secdegree],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[availability] = src.[availability],
        tgt.[raise6secflags] = src.[raise6secflags],
        tgt.[raise60secflags] = src.[raise60secflags],
        tgt.[raise5minflags] = src.[raise5minflags],
        tgt.[raiseregflags] = src.[raiseregflags],
        tgt.[lower6secflags] = src.[lower6secflags],
        tgt.[lower60secflags] = src.[lower60secflags],
        tgt.[lower5minflags] = src.[lower5minflags],
        tgt.[lowerregflags] = src.[lowerregflags],
        tgt.[raiseregavailability] = src.[raiseregavailability],
        tgt.[raiseregenablementmax] = src.[raiseregenablementmax],
        tgt.[raiseregenablementmin] = src.[raiseregenablementmin],
        tgt.[lowerregavailability] = src.[lowerregavailability],
        tgt.[lowerregenablementmax] = src.[lowerregenablementmax],
        tgt.[lowerregenablementmin] = src.[lowerregenablementmin],
        tgt.[raise6secactualavailability] = src.[raise6secactualavailability],
        tgt.[raise60secactualavailability] = src.[raise60secactualavailability],
        tgt.[raise5minactualavailability] = src.[raise5minactualavailability],
        tgt.[raiseregactualavailability] = src.[raiseregactualavailability],
        tgt.[lower6secactualavailability] = src.[lower6secactualavailability],
        tgt.[lower60secactualavailability] = src.[lower60secactualavailability],
        tgt.[lower5minactualavailability] = src.[lower5minactualavailability],
        tgt.[lowerregactualavailability] = src.[lowerregactualavailability],
        tgt.[semidispatchcap] = src.[semidispatchcap],
        tgt.[dispatchmodetime] = src.[dispatchmodetime],
        tgt.[conformance_mode] = src.[conformance_mode],
        tgt.[uigf] = src.[uigf],
        tgt.[raise1sec] = src.[raise1sec],
        tgt.[raise1secflags] = src.[raise1secflags],
        tgt.[lower1sec] = src.[lower1sec],
        tgt.[lower1secflags] = src.[lower1secflags],
        tgt.[raise1secactualavailability] = src.[raise1secactualavailability],
        tgt.[lower1secactualavailability] = src.[lower1secactualavailability],
        tgt.[initial_energy_storage] = src.[initial_energy_storage],
        tgt.[energy_storage] = src.[energy_storage],
        tgt.[min_availability] = src.[min_availability]
when not matched
    then insert (
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
        [semidispatchcap],
        [dispatchmodetime],
        [conformance_mode],
        [uigf],
        [raise1sec],
        [raise1secflags],
        [lower1sec],
        [lower1secflags],
        [raise1secactualavailability],
        [lower1secactualavailability],
        [initial_energy_storage],
        [energy_storage],
        [min_availability]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[duid],
        src.[tradetype],
        src.[dispatchinterval],
        src.[intervention],
        src.[connectionpointid],
        src.[dispatchmode],
        src.[agcstatus],
        src.[initialmw],
        src.[totalcleared],
        src.[rampdownrate],
        src.[rampuprate],
        src.[lower5min],
        src.[lower60sec],
        src.[lower6sec],
        src.[raise5min],
        src.[raise60sec],
        src.[raise6sec],
        src.[downepf],
        src.[upepf],
        src.[marginal5minvalue],
        src.[marginal60secvalue],
        src.[marginal6secvalue],
        src.[marginalvalue],
        src.[violation5mindegree],
        src.[violation60secdegree],
        src.[violation6secdegree],
        src.[violationdegree],
        src.[lastchanged],
        src.[lowerreg],
        src.[raisereg],
        src.[availability],
        src.[raise6secflags],
        src.[raise60secflags],
        src.[raise5minflags],
        src.[raiseregflags],
        src.[lower6secflags],
        src.[lower60secflags],
        src.[lower5minflags],
        src.[lowerregflags],
        src.[raiseregavailability],
        src.[raiseregenablementmax],
        src.[raiseregenablementmin],
        src.[lowerregavailability],
        src.[lowerregenablementmax],
        src.[lowerregenablementmin],
        src.[raise6secactualavailability],
        src.[raise60secactualavailability],
        src.[raise5minactualavailability],
        src.[raiseregactualavailability],
        src.[lower6secactualavailability],
        src.[lower60secactualavailability],
        src.[lower5minactualavailability],
        src.[lowerregactualavailability],
        src.[semidispatchcap],
        src.[dispatchmodetime],
        src.[conformance_mode],
        src.[uigf],
        src.[raise1sec],
        src.[raise1secflags],
        src.[lower1sec],
        src.[lower1secflags],
        src.[raise1secactualavailability],
        src.[lower1secactualavailability],
        src.[initial_energy_storage],
        src.[energy_storage],
        src.[min_availability]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchOffertrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchOffertrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[duid] = src.[duid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[duid] = src.[duid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[bidsettlementdate] = src.[bidsettlementdate],
        tgt.[bidofferdate] = src.[bidofferdate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [duid],
        [bidtype],
        [bidsettlementdate],
        [bidofferdate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[duid],
        src.[bidtype],
        src.[bidsettlementdate],
        src.[bidofferdate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchPrice5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchPrice5 as tgt 
using (
    select 
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
        d.[mii_status],
        d.[raise1secrrp],
        d.[raise1secrop],
        d.[raise1secapcflag],
        d.[lower1secrrp],
        d.[lower1secrop],
        d.[lower1secapcflag],
        d.[pre_ap_raise1_price],
        d.[pre_ap_lower1_price],
        d.[cumul_pre_ap_raise1_price],
        d.[cumul_pre_ap_lower1_price]
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
        [mii_status] varchar(21),
        [raise1secrrp] decimal(15,5),
        [raise1secrop] decimal(15,5),
        [raise1secapcflag] decimal(3,0),
        [lower1secrrp] decimal(15,5),
        [lower1secrop] decimal(15,5),
        [lower1secapcflag] decimal(3,0),
        [pre_ap_raise1_price] decimal(15,5),
        [pre_ap_lower1_price] decimal(15,5),
        [cumul_pre_ap_raise1_price] decimal(15,5),
        [cumul_pre_ap_lower1_price] decimal(15,5)
    ) d
) as src 
on (
    tgt.[dispatchinterval] = src.[dispatchinterval]
    and tgt.[intervention] = src.[intervention]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[dispatchinterval] = src.[dispatchinterval],
        tgt.[intervention] = src.[intervention],
        tgt.[rrp] = src.[rrp],
        tgt.[eep] = src.[eep],
        tgt.[rop] = src.[rop],
        tgt.[apcflag] = src.[apcflag],
        tgt.[marketsuspendedflag] = src.[marketsuspendedflag],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[raise6secrrp] = src.[raise6secrrp],
        tgt.[raise6secrop] = src.[raise6secrop],
        tgt.[raise6secapcflag] = src.[raise6secapcflag],
        tgt.[raise60secrrp] = src.[raise60secrrp],
        tgt.[raise60secrop] = src.[raise60secrop],
        tgt.[raise60secapcflag] = src.[raise60secapcflag],
        tgt.[raise5minrrp] = src.[raise5minrrp],
        tgt.[raise5minrop] = src.[raise5minrop],
        tgt.[raise5minapcflag] = src.[raise5minapcflag],
        tgt.[raiseregrrp] = src.[raiseregrrp],
        tgt.[raiseregrop] = src.[raiseregrop],
        tgt.[raiseregapcflag] = src.[raiseregapcflag],
        tgt.[lower6secrrp] = src.[lower6secrrp],
        tgt.[lower6secrop] = src.[lower6secrop],
        tgt.[lower6secapcflag] = src.[lower6secapcflag],
        tgt.[lower60secrrp] = src.[lower60secrrp],
        tgt.[lower60secrop] = src.[lower60secrop],
        tgt.[lower60secapcflag] = src.[lower60secapcflag],
        tgt.[lower5minrrp] = src.[lower5minrrp],
        tgt.[lower5minrop] = src.[lower5minrop],
        tgt.[lower5minapcflag] = src.[lower5minapcflag],
        tgt.[lowerregrrp] = src.[lowerregrrp],
        tgt.[lowerregrop] = src.[lowerregrop],
        tgt.[lowerregapcflag] = src.[lowerregapcflag],
        tgt.[price_status] = src.[price_status],
        tgt.[pre_ap_energy_price] = src.[pre_ap_energy_price],
        tgt.[pre_ap_raise6_price] = src.[pre_ap_raise6_price],
        tgt.[pre_ap_raise60_price] = src.[pre_ap_raise60_price],
        tgt.[pre_ap_raise5min_price] = src.[pre_ap_raise5min_price],
        tgt.[pre_ap_raisereg_price] = src.[pre_ap_raisereg_price],
        tgt.[pre_ap_lower6_price] = src.[pre_ap_lower6_price],
        tgt.[pre_ap_lower60_price] = src.[pre_ap_lower60_price],
        tgt.[pre_ap_lower5min_price] = src.[pre_ap_lower5min_price],
        tgt.[pre_ap_lowerreg_price] = src.[pre_ap_lowerreg_price],
        tgt.[cumul_pre_ap_energy_price] = src.[cumul_pre_ap_energy_price],
        tgt.[cumul_pre_ap_raise6_price] = src.[cumul_pre_ap_raise6_price],
        tgt.[cumul_pre_ap_raise60_price] = src.[cumul_pre_ap_raise60_price],
        tgt.[cumul_pre_ap_raise5min_price] = src.[cumul_pre_ap_raise5min_price],
        tgt.[cumul_pre_ap_raisereg_price] = src.[cumul_pre_ap_raisereg_price],
        tgt.[cumul_pre_ap_lower6_price] = src.[cumul_pre_ap_lower6_price],
        tgt.[cumul_pre_ap_lower60_price] = src.[cumul_pre_ap_lower60_price],
        tgt.[cumul_pre_ap_lower5min_price] = src.[cumul_pre_ap_lower5min_price],
        tgt.[cumul_pre_ap_lowerreg_price] = src.[cumul_pre_ap_lowerreg_price],
        tgt.[ocd_status] = src.[ocd_status],
        tgt.[mii_status] = src.[mii_status],
        tgt.[raise1secrrp] = src.[raise1secrrp],
        tgt.[raise1secrop] = src.[raise1secrop],
        tgt.[raise1secapcflag] = src.[raise1secapcflag],
        tgt.[lower1secrrp] = src.[lower1secrrp],
        tgt.[lower1secrop] = src.[lower1secrop],
        tgt.[lower1secapcflag] = src.[lower1secapcflag],
        tgt.[pre_ap_raise1_price] = src.[pre_ap_raise1_price],
        tgt.[pre_ap_lower1_price] = src.[pre_ap_lower1_price],
        tgt.[cumul_pre_ap_raise1_price] = src.[cumul_pre_ap_raise1_price],
        tgt.[cumul_pre_ap_lower1_price] = src.[cumul_pre_ap_lower1_price]
when not matched
    then insert (
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
        [mii_status],
        [raise1secrrp],
        [raise1secrop],
        [raise1secapcflag],
        [lower1secrrp],
        [lower1secrop],
        [lower1secapcflag],
        [pre_ap_raise1_price],
        [pre_ap_lower1_price],
        [cumul_pre_ap_raise1_price],
        [cumul_pre_ap_lower1_price]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[regionid],
        src.[dispatchinterval],
        src.[intervention],
        src.[rrp],
        src.[eep],
        src.[rop],
        src.[apcflag],
        src.[marketsuspendedflag],
        src.[lastchanged],
        src.[raise6secrrp],
        src.[raise6secrop],
        src.[raise6secapcflag],
        src.[raise60secrrp],
        src.[raise60secrop],
        src.[raise60secapcflag],
        src.[raise5minrrp],
        src.[raise5minrop],
        src.[raise5minapcflag],
        src.[raiseregrrp],
        src.[raiseregrop],
        src.[raiseregapcflag],
        src.[lower6secrrp],
        src.[lower6secrop],
        src.[lower6secapcflag],
        src.[lower60secrrp],
        src.[lower60secrop],
        src.[lower60secapcflag],
        src.[lower5minrrp],
        src.[lower5minrop],
        src.[lower5minapcflag],
        src.[lowerregrrp],
        src.[lowerregrop],
        src.[lowerregapcflag],
        src.[price_status],
        src.[pre_ap_energy_price],
        src.[pre_ap_raise6_price],
        src.[pre_ap_raise60_price],
        src.[pre_ap_raise5min_price],
        src.[pre_ap_raisereg_price],
        src.[pre_ap_lower6_price],
        src.[pre_ap_lower60_price],
        src.[pre_ap_lower5min_price],
        src.[pre_ap_lowerreg_price],
        src.[cumul_pre_ap_energy_price],
        src.[cumul_pre_ap_raise6_price],
        src.[cumul_pre_ap_raise60_price],
        src.[cumul_pre_ap_raise5min_price],
        src.[cumul_pre_ap_raisereg_price],
        src.[cumul_pre_ap_lower6_price],
        src.[cumul_pre_ap_lower60_price],
        src.[cumul_pre_ap_lower5min_price],
        src.[cumul_pre_ap_lowerreg_price],
        src.[ocd_status],
        src.[mii_status],
        src.[raise1secrrp],
        src.[raise1secrop],
        src.[raise1secapcflag],
        src.[lower1secrrp],
        src.[lower1secrop],
        src.[lower1secapcflag],
        src.[pre_ap_raise1_price],
        src.[pre_ap_lower1_price],
        src.[cumul_pre_ap_raise1_price],
        src.[cumul_pre_ap_lower1_price]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchRegionsum8
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchRegionsum8 as tgt 
using (
    select 
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
        d.[ss_wind_compliancemw],
        d.[wdr_initialmw],
        d.[wdr_available],
        d.[wdr_dispatched],
        d.[ss_solar_availability],
        d.[ss_wind_availability],
        d.[raise1seclocaldispatch],
        d.[lower1seclocaldispatch],
        d.[raise1secactualavailability],
        d.[lower1secactualavailability],
        d.[bdu_energy_storage],
        d.[bdu_min_avail],
        d.[bdu_max_avail],
        d.[bdu_clearedmw_gen],
        d.[bdu_clearedmw_load]
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
        [ss_wind_compliancemw] decimal(15,5),
        [wdr_initialmw] decimal(15,5),
        [wdr_available] decimal(15,5),
        [wdr_dispatched] decimal(15,5),
        [ss_solar_availability] decimal(15,5),
        [ss_wind_availability] decimal(15,5),
        [raise1seclocaldispatch] decimal(15,5),
        [lower1seclocaldispatch] decimal(15,5),
        [raise1secactualavailability] decimal(16,6),
        [lower1secactualavailability] decimal(16,6),
        [bdu_energy_storage] decimal(15,5),
        [bdu_min_avail] decimal(15,5),
        [bdu_max_avail] decimal(15,5),
        [bdu_clearedmw_gen] decimal(15,5),
        [bdu_clearedmw_load] decimal(15,5)
    ) d
) as src 
on (
    tgt.[dispatchinterval] = src.[dispatchinterval]
    and tgt.[intervention] = src.[intervention]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[dispatchinterval] = src.[dispatchinterval],
        tgt.[intervention] = src.[intervention],
        tgt.[totaldemand] = src.[totaldemand],
        tgt.[availablegeneration] = src.[availablegeneration],
        tgt.[availableload] = src.[availableload],
        tgt.[demandforecast] = src.[demandforecast],
        tgt.[dispatchablegeneration] = src.[dispatchablegeneration],
        tgt.[dispatchableload] = src.[dispatchableload],
        tgt.[netinterchange] = src.[netinterchange],
        tgt.[excessgeneration] = src.[excessgeneration],
        tgt.[lower5mindispatch] = src.[lower5mindispatch],
        tgt.[lower5minimport] = src.[lower5minimport],
        tgt.[lower5minlocaldispatch] = src.[lower5minlocaldispatch],
        tgt.[lower5minlocalprice] = src.[lower5minlocalprice],
        tgt.[lower5minlocalreq] = src.[lower5minlocalreq],
        tgt.[lower5minprice] = src.[lower5minprice],
        tgt.[lower5minreq] = src.[lower5minreq],
        tgt.[lower5minsupplyprice] = src.[lower5minsupplyprice],
        tgt.[lower60secdispatch] = src.[lower60secdispatch],
        tgt.[lower60secimport] = src.[lower60secimport],
        tgt.[lower60seclocaldispatch] = src.[lower60seclocaldispatch],
        tgt.[lower60seclocalprice] = src.[lower60seclocalprice],
        tgt.[lower60seclocalreq] = src.[lower60seclocalreq],
        tgt.[lower60secprice] = src.[lower60secprice],
        tgt.[lower60secreq] = src.[lower60secreq],
        tgt.[lower60secsupplyprice] = src.[lower60secsupplyprice],
        tgt.[lower6secdispatch] = src.[lower6secdispatch],
        tgt.[lower6secimport] = src.[lower6secimport],
        tgt.[lower6seclocaldispatch] = src.[lower6seclocaldispatch],
        tgt.[lower6seclocalprice] = src.[lower6seclocalprice],
        tgt.[lower6seclocalreq] = src.[lower6seclocalreq],
        tgt.[lower6secprice] = src.[lower6secprice],
        tgt.[lower6secreq] = src.[lower6secreq],
        tgt.[lower6secsupplyprice] = src.[lower6secsupplyprice],
        tgt.[raise5mindispatch] = src.[raise5mindispatch],
        tgt.[raise5minimport] = src.[raise5minimport],
        tgt.[raise5minlocaldispatch] = src.[raise5minlocaldispatch],
        tgt.[raise5minlocalprice] = src.[raise5minlocalprice],
        tgt.[raise5minlocalreq] = src.[raise5minlocalreq],
        tgt.[raise5minprice] = src.[raise5minprice],
        tgt.[raise5minreq] = src.[raise5minreq],
        tgt.[raise5minsupplyprice] = src.[raise5minsupplyprice],
        tgt.[raise60secdispatch] = src.[raise60secdispatch],
        tgt.[raise60secimport] = src.[raise60secimport],
        tgt.[raise60seclocaldispatch] = src.[raise60seclocaldispatch],
        tgt.[raise60seclocalprice] = src.[raise60seclocalprice],
        tgt.[raise60seclocalreq] = src.[raise60seclocalreq],
        tgt.[raise60secprice] = src.[raise60secprice],
        tgt.[raise60secreq] = src.[raise60secreq],
        tgt.[raise60secsupplyprice] = src.[raise60secsupplyprice],
        tgt.[raise6secdispatch] = src.[raise6secdispatch],
        tgt.[raise6secimport] = src.[raise6secimport],
        tgt.[raise6seclocaldispatch] = src.[raise6seclocaldispatch],
        tgt.[raise6seclocalprice] = src.[raise6seclocalprice],
        tgt.[raise6seclocalreq] = src.[raise6seclocalreq],
        tgt.[raise6secprice] = src.[raise6secprice],
        tgt.[raise6secreq] = src.[raise6secreq],
        tgt.[raise6secsupplyprice] = src.[raise6secsupplyprice],
        tgt.[aggegatedispatcherror] = src.[aggegatedispatcherror],
        tgt.[aggregatedispatcherror] = src.[aggregatedispatcherror],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[initialsupply] = src.[initialsupply],
        tgt.[clearedsupply] = src.[clearedsupply],
        tgt.[lowerregimport] = src.[lowerregimport],
        tgt.[lowerreglocaldispatch] = src.[lowerreglocaldispatch],
        tgt.[lowerreglocalreq] = src.[lowerreglocalreq],
        tgt.[lowerregreq] = src.[lowerregreq],
        tgt.[raiseregimport] = src.[raiseregimport],
        tgt.[raisereglocaldispatch] = src.[raisereglocaldispatch],
        tgt.[raisereglocalreq] = src.[raisereglocalreq],
        tgt.[raiseregreq] = src.[raiseregreq],
        tgt.[raise5minlocalviolation] = src.[raise5minlocalviolation],
        tgt.[raisereglocalviolation] = src.[raisereglocalviolation],
        tgt.[raise60seclocalviolation] = src.[raise60seclocalviolation],
        tgt.[raise6seclocalviolation] = src.[raise6seclocalviolation],
        tgt.[lower5minlocalviolation] = src.[lower5minlocalviolation],
        tgt.[lowerreglocalviolation] = src.[lowerreglocalviolation],
        tgt.[lower60seclocalviolation] = src.[lower60seclocalviolation],
        tgt.[lower6seclocalviolation] = src.[lower6seclocalviolation],
        tgt.[raise5minviolation] = src.[raise5minviolation],
        tgt.[raiseregviolation] = src.[raiseregviolation],
        tgt.[raise60secviolation] = src.[raise60secviolation],
        tgt.[raise6secviolation] = src.[raise6secviolation],
        tgt.[lower5minviolation] = src.[lower5minviolation],
        tgt.[lowerregviolation] = src.[lowerregviolation],
        tgt.[lower60secviolation] = src.[lower60secviolation],
        tgt.[lower6secviolation] = src.[lower6secviolation],
        tgt.[raise6secactualavailability] = src.[raise6secactualavailability],
        tgt.[raise60secactualavailability] = src.[raise60secactualavailability],
        tgt.[raise5minactualavailability] = src.[raise5minactualavailability],
        tgt.[raiseregactualavailability] = src.[raiseregactualavailability],
        tgt.[lower6secactualavailability] = src.[lower6secactualavailability],
        tgt.[lower60secactualavailability] = src.[lower60secactualavailability],
        tgt.[lower5minactualavailability] = src.[lower5minactualavailability],
        tgt.[lowerregactualavailability] = src.[lowerregactualavailability],
        tgt.[lorsurplus] = src.[lorsurplus],
        tgt.[lrcsurplus] = src.[lrcsurplus],
        tgt.[totalintermittentgeneration] = src.[totalintermittentgeneration],
        tgt.[demand_and_nonschedgen] = src.[demand_and_nonschedgen],
        tgt.[uigf] = src.[uigf],
        tgt.[semischedule_clearedmw] = src.[semischedule_clearedmw],
        tgt.[semischedule_compliancemw] = src.[semischedule_compliancemw],
        tgt.[ss_solar_uigf] = src.[ss_solar_uigf],
        tgt.[ss_wind_uigf] = src.[ss_wind_uigf],
        tgt.[ss_solar_clearedmw] = src.[ss_solar_clearedmw],
        tgt.[ss_wind_clearedmw] = src.[ss_wind_clearedmw],
        tgt.[ss_solar_compliancemw] = src.[ss_solar_compliancemw],
        tgt.[ss_wind_compliancemw] = src.[ss_wind_compliancemw],
        tgt.[wdr_initialmw] = src.[wdr_initialmw],
        tgt.[wdr_available] = src.[wdr_available],
        tgt.[wdr_dispatched] = src.[wdr_dispatched],
        tgt.[ss_solar_availability] = src.[ss_solar_availability],
        tgt.[ss_wind_availability] = src.[ss_wind_availability],
        tgt.[raise1seclocaldispatch] = src.[raise1seclocaldispatch],
        tgt.[lower1seclocaldispatch] = src.[lower1seclocaldispatch],
        tgt.[raise1secactualavailability] = src.[raise1secactualavailability],
        tgt.[lower1secactualavailability] = src.[lower1secactualavailability],
        tgt.[bdu_energy_storage] = src.[bdu_energy_storage],
        tgt.[bdu_min_avail] = src.[bdu_min_avail],
        tgt.[bdu_max_avail] = src.[bdu_max_avail],
        tgt.[bdu_clearedmw_gen] = src.[bdu_clearedmw_gen],
        tgt.[bdu_clearedmw_load] = src.[bdu_clearedmw_load]
when not matched
    then insert (
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
        [ss_wind_compliancemw],
        [wdr_initialmw],
        [wdr_available],
        [wdr_dispatched],
        [ss_solar_availability],
        [ss_wind_availability],
        [raise1seclocaldispatch],
        [lower1seclocaldispatch],
        [raise1secactualavailability],
        [lower1secactualavailability],
        [bdu_energy_storage],
        [bdu_min_avail],
        [bdu_max_avail],
        [bdu_clearedmw_gen],
        [bdu_clearedmw_load]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[regionid],
        src.[dispatchinterval],
        src.[intervention],
        src.[totaldemand],
        src.[availablegeneration],
        src.[availableload],
        src.[demandforecast],
        src.[dispatchablegeneration],
        src.[dispatchableload],
        src.[netinterchange],
        src.[excessgeneration],
        src.[lower5mindispatch],
        src.[lower5minimport],
        src.[lower5minlocaldispatch],
        src.[lower5minlocalprice],
        src.[lower5minlocalreq],
        src.[lower5minprice],
        src.[lower5minreq],
        src.[lower5minsupplyprice],
        src.[lower60secdispatch],
        src.[lower60secimport],
        src.[lower60seclocaldispatch],
        src.[lower60seclocalprice],
        src.[lower60seclocalreq],
        src.[lower60secprice],
        src.[lower60secreq],
        src.[lower60secsupplyprice],
        src.[lower6secdispatch],
        src.[lower6secimport],
        src.[lower6seclocaldispatch],
        src.[lower6seclocalprice],
        src.[lower6seclocalreq],
        src.[lower6secprice],
        src.[lower6secreq],
        src.[lower6secsupplyprice],
        src.[raise5mindispatch],
        src.[raise5minimport],
        src.[raise5minlocaldispatch],
        src.[raise5minlocalprice],
        src.[raise5minlocalreq],
        src.[raise5minprice],
        src.[raise5minreq],
        src.[raise5minsupplyprice],
        src.[raise60secdispatch],
        src.[raise60secimport],
        src.[raise60seclocaldispatch],
        src.[raise60seclocalprice],
        src.[raise60seclocalreq],
        src.[raise60secprice],
        src.[raise60secreq],
        src.[raise60secsupplyprice],
        src.[raise6secdispatch],
        src.[raise6secimport],
        src.[raise6seclocaldispatch],
        src.[raise6seclocalprice],
        src.[raise6seclocalreq],
        src.[raise6secprice],
        src.[raise6secreq],
        src.[raise6secsupplyprice],
        src.[aggegatedispatcherror],
        src.[aggregatedispatcherror],
        src.[lastchanged],
        src.[initialsupply],
        src.[clearedsupply],
        src.[lowerregimport],
        src.[lowerreglocaldispatch],
        src.[lowerreglocalreq],
        src.[lowerregreq],
        src.[raiseregimport],
        src.[raisereglocaldispatch],
        src.[raisereglocalreq],
        src.[raiseregreq],
        src.[raise5minlocalviolation],
        src.[raisereglocalviolation],
        src.[raise60seclocalviolation],
        src.[raise6seclocalviolation],
        src.[lower5minlocalviolation],
        src.[lowerreglocalviolation],
        src.[lower60seclocalviolation],
        src.[lower6seclocalviolation],
        src.[raise5minviolation],
        src.[raiseregviolation],
        src.[raise60secviolation],
        src.[raise6secviolation],
        src.[lower5minviolation],
        src.[lowerregviolation],
        src.[lower60secviolation],
        src.[lower6secviolation],
        src.[raise6secactualavailability],
        src.[raise60secactualavailability],
        src.[raise5minactualavailability],
        src.[raiseregactualavailability],
        src.[lower6secactualavailability],
        src.[lower60secactualavailability],
        src.[lower5minactualavailability],
        src.[lowerregactualavailability],
        src.[lorsurplus],
        src.[lrcsurplus],
        src.[totalintermittentgeneration],
        src.[demand_and_nonschedgen],
        src.[uigf],
        src.[semischedule_clearedmw],
        src.[semischedule_compliancemw],
        src.[ss_solar_uigf],
        src.[ss_wind_uigf],
        src.[ss_solar_clearedmw],
        src.[ss_wind_clearedmw],
        src.[ss_solar_compliancemw],
        src.[ss_wind_compliancemw],
        src.[wdr_initialmw],
        src.[wdr_available],
        src.[wdr_dispatched],
        src.[ss_solar_availability],
        src.[ss_wind_availability],
        src.[raise1seclocaldispatch],
        src.[lower1seclocaldispatch],
        src.[raise1secactualavailability],
        src.[lower1secactualavailability],
        src.[bdu_energy_storage],
        src.[bdu_min_avail],
        src.[bdu_max_avail],
        src.[bdu_clearedmw_gen],
        src.[bdu_clearedmw_load]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchocdConstraintFcasOcd1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchocdConstraintFcasOcd1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[constraintid] = src.[constraintid],
        tgt.[versionno] = src.[versionno],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[rhs] = src.[rhs],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[intervention],
        src.[constraintid],
        src.[versionno],
        src.[lastchanged],
        src.[rhs],
        src.[marginalvalue],
        src.[violationdegree]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchFcasReq2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchFcasReq2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[genconid] = src.[genconid]
    and tgt.[intervention] = src.[intervention]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[genconid] = src.[genconid],
        tgt.[regionid] = src.[regionid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[genconeffectivedate] = src.[genconeffectivedate],
        tgt.[genconversionno] = src.[genconversionno],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[base_cost] = src.[base_cost],
        tgt.[adjusted_cost] = src.[adjusted_cost],
        tgt.[estimated_cmpf] = src.[estimated_cmpf],
        tgt.[estimated_crmpf] = src.[estimated_crmpf],
        tgt.[recovery_factor_cmpf] = src.[recovery_factor_cmpf],
        tgt.[recovery_factor_crmpf] = src.[recovery_factor_crmpf]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[intervention],
        src.[genconid],
        src.[regionid],
        src.[bidtype],
        src.[genconeffectivedate],
        src.[genconversionno],
        src.[marginalvalue],
        src.[lastchanged],
        src.[base_cost],
        src.[adjusted_cost],
        src.[estimated_cmpf],
        src.[estimated_crmpf],
        src.[recovery_factor_cmpf],
        src.[recovery_factor_crmpf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchInterconnection1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchInterconnection1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[from_regionid] = src.[from_regionid]
    and tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[to_regionid] = src.[to_regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[from_regionid] = src.[from_regionid],
        tgt.[to_regionid] = src.[to_regionid],
        tgt.[dispatchinterval] = src.[dispatchinterval],
        tgt.[irlf] = src.[irlf],
        tgt.[mwflow] = src.[mwflow],
        tgt.[meteredmwflow] = src.[meteredmwflow],
        tgt.[from_region_mw_losses] = src.[from_region_mw_losses],
        tgt.[to_region_mw_losses] = src.[to_region_mw_losses],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[intervention],
        src.[from_regionid],
        src.[to_regionid],
        src.[dispatchinterval],
        src.[irlf],
        src.[mwflow],
        src.[meteredmwflow],
        src.[from_region_mw_losses],
        src.[to_region_mw_losses],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchLocalPrice1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DispatchLocalPrice1(
file_log_id,
[settlementdate],
        [duid],
        [local_price_adjustment],
        [locally_constrained]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDispatchMnspbidtrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchMnspbidtrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[linkid] = src.[linkid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[participantid] = src.[participantid],
        tgt.[linkid] = src.[linkid],
        tgt.[offersettlementdate] = src.[offersettlementdate],
        tgt.[offereffectivedate] = src.[offereffectivedate],
        tgt.[offerversionno] = src.[offerversionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [participantid],
        [linkid],
        [offersettlementdate],
        [offereffectivedate],
        [offerversionno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[participantid],
        src.[linkid],
        src.[offersettlementdate],
        src.[offereffectivedate],
        src.[offerversionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchMrScheduleTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchMrScheduleTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[regionid] = src.[regionid],
        tgt.[mr_date] = src.[mr_date],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [regionid],
        [mr_date],
        [version_datetime],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[regionid],
        src.[mr_date],
        src.[version_datetime],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPriceloadPriceRevision1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PriceloadPriceRevision1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[intervention] = src.[intervention]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[regionid] = src.[regionid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[versionno] = src.[versionno],
        tgt.[rrp_new] = src.[rrp_new],
        tgt.[rrp_old] = src.[rrp_old],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[intervention],
        src.[regionid],
        src.[bidtype],
        src.[versionno],
        src.[rrp_new],
        src.[rrp_old],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchUnitConformance2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchUnitConformance2 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[adg_id],
        d.[semidispatchcap],
        d.[conformance_mode]
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
        [lastchanged] datetime2,
        [adg_id] varchar(20),
        [semidispatchcap] decimal(3,0),
        [conformance_mode] decimal(6,0)
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[interval_datetime] = src.[interval_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[duid] = src.[duid],
        tgt.[totalcleared] = src.[totalcleared],
        tgt.[actualmw] = src.[actualmw],
        tgt.[roc] = src.[roc],
        tgt.[availability] = src.[availability],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[striglm] = src.[striglm],
        tgt.[ltriglm] = src.[ltriglm],
        tgt.[mwerror] = src.[mwerror],
        tgt.[max_mwerror] = src.[max_mwerror],
        tgt.[lecount] = src.[lecount],
        tgt.[secount] = src.[secount],
        tgt.[status] = src.[status],
        tgt.[participant_status_action] = src.[participant_status_action],
        tgt.[operating_mode] = src.[operating_mode],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[adg_id] = src.[adg_id],
        tgt.[semidispatchcap] = src.[semidispatchcap],
        tgt.[conformance_mode] = src.[conformance_mode]
when not matched
    then insert (
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
        [lastchanged],
        [adg_id],
        [semidispatchcap],
        [conformance_mode]
    ) values (
        @file_log_id,
        src.[interval_datetime],
        src.[duid],
        src.[totalcleared],
        src.[actualmw],
        src.[roc],
        src.[availability],
        src.[lowerreg],
        src.[raisereg],
        src.[striglm],
        src.[ltriglm],
        src.[mwerror],
        src.[max_mwerror],
        src.[lecount],
        src.[secount],
        src.[status],
        src.[participant_status_action],
        src.[operating_mode],
        src.[lastchanged],
        src.[adg_id],
        src.[semidispatchcap],
        src.[conformance_mode]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchUnitScada1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DispatchUnitScada1(
file_log_id,
[settlementdate],
        [duid],
        [scadavalue]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDispatchIntermittentForecastTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DispatchIntermittentForecastTrk1(
file_log_id,
[settlementdate],
        [duid],
        [origin],
        [forecast_priority],
        [offerdatetime]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertDispatchNegativeResidue1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.DispatchNegativeResidue1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertApApevent1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ApApevent1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[apeventid] = src.[apeventid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[apeventid] = src.[apeventid],
        tgt.[effectivefrominterval] = src.[effectivefrominterval],
        tgt.[effectivetointerval] = src.[effectivetointerval],
        tgt.[reason] = src.[reason],
        tgt.[startauthorisedby] = src.[startauthorisedby],
        tgt.[startauthoriseddate] = src.[startauthoriseddate],
        tgt.[endauthorisedby] = src.[endauthorisedby],
        tgt.[endauthoriseddate] = src.[endauthoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[apeventid],
        src.[effectivefrominterval],
        src.[effectivetointerval],
        src.[reason],
        src.[startauthorisedby],
        src.[startauthoriseddate],
        src.[endauthorisedby],
        src.[endauthoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertApApeventregion2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ApApeventregion2 as tgt 
using (
    select 
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
        d.[lowerregapflag],
        d.[raise1secapflag],
        d.[lower1secapflag]
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
        [lowerregapflag] decimal(1,0),
        [raise1secapflag] decimal(3,0),
        [lower1secapflag] decimal(3,0)
    ) d
) as src 
on (
    tgt.[apeventid] = src.[apeventid]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[apeventid] = src.[apeventid],
        tgt.[regionid] = src.[regionid],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[energyapflag] = src.[energyapflag],
        tgt.[raise6secapflag] = src.[raise6secapflag],
        tgt.[raise60secapflag] = src.[raise60secapflag],
        tgt.[raise5minapflag] = src.[raise5minapflag],
        tgt.[raiseregapflag] = src.[raiseregapflag],
        tgt.[lower6secapflag] = src.[lower6secapflag],
        tgt.[lower60secapflag] = src.[lower60secapflag],
        tgt.[lower5minapflag] = src.[lower5minapflag],
        tgt.[lowerregapflag] = src.[lowerregapflag],
        tgt.[raise1secapflag] = src.[raise1secapflag],
        tgt.[lower1secapflag] = src.[lower1secapflag]
when not matched
    then insert (
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
        [lowerregapflag],
        [raise1secapflag],
        [lower1secapflag]
    ) values (
        @file_log_id,
        src.[apeventid],
        src.[regionid],
        src.[lastchanged],
        src.[energyapflag],
        src.[raise6secapflag],
        src.[raise60secapflag],
        src.[raise5minapflag],
        src.[raiseregapflag],
        src.[lower6secapflag],
        src.[lower60secapflag],
        src.[lower5minapflag],
        src.[lowerregapflag],
        src.[raise1secapflag],
        src.[lower1secapflag]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureIrfmamount1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureIrfmamount1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[irfmid] = src.[irfmid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[irfmid] = src.[irfmid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[amount] = src.[amount],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [irfmid],
        [effectivedate],
        [versionno],
        [periodid],
        [amount],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[irfmid],
        src.[effectivedate],
        src.[versionno],
        src.[periodid],
        src.[amount],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureIrfmevents1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureIrfmevents1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[irfmid] = src.[irfmid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[irfmid] = src.[irfmid],
        tgt.[startdate] = src.[startdate],
        tgt.[startperiod] = src.[startperiod],
        tgt.[enddate] = src.[enddate],
        tgt.[endperiod] = src.[endperiod],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [irfmid],
        [startdate],
        [startperiod],
        [enddate],
        [endperiod],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[irfmid],
        src.[startdate],
        src.[startperiod],
        src.[enddate],
        src.[endperiod],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureMarketSuspendRegimeSum1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureMarketSuspendRegimeSum1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[regionid] = src.[regionid]
    and tgt.[start_interval] = src.[start_interval]
    and tgt.[suspension_id] = src.[suspension_id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[suspension_id] = src.[suspension_id],
        tgt.[regionid] = src.[regionid],
        tgt.[start_interval] = src.[start_interval],
        tgt.[end_interval] = src.[end_interval],
        tgt.[pricing_regime] = src.[pricing_regime],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [suspension_id],
        [regionid],
        [start_interval],
        [end_interval],
        [pricing_regime],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[suspension_id],
        src.[regionid],
        src.[start_interval],
        src.[end_interval],
        src.[pricing_regime],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureMarketSuspendRegionSum1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureMarketSuspendRegionSum1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[regionid] = src.[regionid]
    and tgt.[suspension_id] = src.[suspension_id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[suspension_id] = src.[suspension_id],
        tgt.[regionid] = src.[regionid],
        tgt.[initial_interval] = src.[initial_interval],
        tgt.[end_region_interval] = src.[end_region_interval],
        tgt.[end_suspension_interval] = src.[end_suspension_interval],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [suspension_id],
        [regionid],
        [initial_interval],
        [end_region_interval],
        [end_suspension_interval],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[suspension_id],
        src.[regionid],
        src.[initial_interval],
        src.[end_region_interval],
        src.[end_suspension_interval],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureMarketSuspendSchedule2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureMarketSuspendSchedule2 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[l1_rrp],
        d.[r1_rrp]
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
        [lastchanged] datetime2,
        [l1_rrp] decimal(15,5),
        [r1_rrp] decimal(15,5)
    ) d
) as src 
on (
    tgt.[day_type] = src.[day_type]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[day_type] = src.[day_type],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[energy_rrp] = src.[energy_rrp],
        tgt.[r6_rrp] = src.[r6_rrp],
        tgt.[r60_rrp] = src.[r60_rrp],
        tgt.[r5_rrp] = src.[r5_rrp],
        tgt.[rreg_rrp] = src.[rreg_rrp],
        tgt.[l6_rrp] = src.[l6_rrp],
        tgt.[l60_rrp] = src.[l60_rrp],
        tgt.[l5_rrp] = src.[l5_rrp],
        tgt.[lreg_rrp] = src.[lreg_rrp],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[l1_rrp] = src.[l1_rrp],
        tgt.[r1_rrp] = src.[r1_rrp]
when not matched
    then insert (
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
        [lastchanged],
        [l1_rrp],
        [r1_rrp]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[day_type],
        src.[regionid],
        src.[periodid],
        src.[energy_rrp],
        src.[r6_rrp],
        src.[r60_rrp],
        src.[r5_rrp],
        src.[rreg_rrp],
        src.[l6_rrp],
        src.[l60_rrp],
        src.[l5_rrp],
        src.[lreg_rrp],
        src.[lastchanged],
        src.[l1_rrp],
        src.[r1_rrp]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureMarketSuspendScheduleTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureMarketSuspendScheduleTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[source_start_date] = src.[source_start_date],
        tgt.[source_end_date] = src.[source_end_date],
        tgt.[comments] = src.[comments],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [source_start_date],
        [source_end_date],
        [comments],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[source_start_date],
        src.[source_end_date],
        src.[comments],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertForceMajeureOverriderrp1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ForceMajeureOverriderrp1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[regionid] = src.[regionid]
    and tgt.[startdate] = src.[startdate]
    and tgt.[startperiod] = src.[startperiod]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[startdate] = src.[startdate],
        tgt.[startperiod] = src.[startperiod],
        tgt.[enddate] = src.[enddate],
        tgt.[endperiod] = src.[endperiod],
        tgt.[rrp] = src.[rrp],
        tgt.[description] = src.[description],
        tgt.[authorisestart] = src.[authorisestart],
        tgt.[authoriseend] = src.[authoriseend],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[regionid],
        src.[startdate],
        src.[startperiod],
        src.[enddate],
        src.[endperiod],
        src.[rrp],
        src.[description],
        src.[authorisestart],
        src.[authoriseend],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertApRegionapc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ApRegionapc1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [regionid],
        [effectivedate],
        [versionno],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[regionid],
        src.[effectivedate],
        src.[versionno],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertApRegionapcintervals1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ApRegionapcintervals1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[apcvalue] = src.[apcvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[apctype] = src.[apctype],
        tgt.[fcasapcvalue] = src.[fcasapcvalue],
        tgt.[apfvalue] = src.[apfvalue]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[regionid],
        src.[effectivedate],
        src.[versionno],
        src.[periodid],
        src.[apcvalue],
        src.[lastchanged],
        src.[apctype],
        src.[fcasapcvalue],
        src.[apfvalue]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGdInstructGdinstruct1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GdInstructGdinstruct1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[id] = src.[id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[stationid] = src.[stationid],
        tgt.[regionid] = src.[regionid],
        tgt.[id] = src.[id],
        tgt.[instructiontypeid] = src.[instructiontypeid],
        tgt.[instructionsubtypeid] = src.[instructionsubtypeid],
        tgt.[instructionclassid] = src.[instructionclassid],
        tgt.[reason] = src.[reason],
        tgt.[instlevel] = src.[instlevel],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[participantid] = src.[participantid],
        tgt.[issuedtime] = src.[issuedtime],
        tgt.[targettime] = src.[targettime],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[duid],
        src.[stationid],
        src.[regionid],
        src.[id],
        src.[instructiontypeid],
        src.[instructionsubtypeid],
        src.[instructionclassid],
        src.[reason],
        src.[instlevel],
        src.[authoriseddate],
        src.[authorisedby],
        src.[participantid],
        src.[issuedtime],
        src.[targettime],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGdInstructInstructionsubtype1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GdInstructInstructionsubtype1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[instructionsubtypeid] = src.[instructionsubtypeid]
    and tgt.[instructiontypeid] = src.[instructiontypeid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[instructiontypeid] = src.[instructiontypeid],
        tgt.[instructionsubtypeid] = src.[instructionsubtypeid],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [instructiontypeid],
        [instructionsubtypeid],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[instructiontypeid],
        src.[instructionsubtypeid],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGdInstructInstructiontype1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GdInstructInstructiontype1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[instructiontypeid] = src.[instructiontypeid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[instructiontypeid] = src.[instructiontypeid],
        tgt.[description] = src.[description],
        tgt.[regionid] = src.[regionid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [instructiontypeid],
        [description],
        [regionid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[instructiontypeid],
        src.[description],
        src.[regionid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGenericConstraintEmsmaster1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GenericConstraintEmsmaster1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[spd_id] = src.[spd_id]
    and tgt.[spd_type] = src.[spd_type]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[spd_id] = src.[spd_id],
        tgt.[spd_type] = src.[spd_type],
        tgt.[description] = src.[description],
        tgt.[grouping_id] = src.[grouping_id],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [spd_id],
        [spd_type],
        [description],
        [grouping_id],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[spd_id],
        src.[spd_type],
        src.[description],
        src.[grouping_id],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGencondataNull6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GencondataNull6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconid] = src.[genconid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[genconid] = src.[genconid],
        tgt.[constrainttype] = src.[constrainttype],
        tgt.[constraintvalue] = src.[constraintvalue],
        tgt.[description] = src.[description],
        tgt.[status] = src.[status],
        tgt.[genericconstraintweight] = src.[genericconstraintweight],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[dynamicrhs] = src.[dynamicrhs],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[dispatch] = src.[dispatch],
        tgt.[predispatch] = src.[predispatch],
        tgt.[stpasa] = src.[stpasa],
        tgt.[mtpasa] = src.[mtpasa],
        tgt.[impact] = src.[impact],
        tgt.[source] = src.[source],
        tgt.[limittype] = src.[limittype],
        tgt.[reason] = src.[reason],
        tgt.[modifications] = src.[modifications],
        tgt.[additionalnotes] = src.[additionalnotes],
        tgt.[p5min_scope_override] = src.[p5min_scope_override],
        tgt.[lrc] = src.[lrc],
        tgt.[lor] = src.[lor],
        tgt.[force_scada] = src.[force_scada]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[genconid],
        src.[constrainttype],
        src.[constraintvalue],
        src.[description],
        src.[status],
        src.[genericconstraintweight],
        src.[authoriseddate],
        src.[authorisedby],
        src.[dynamicrhs],
        src.[lastchanged],
        src.[dispatch],
        src.[predispatch],
        src.[stpasa],
        src.[mtpasa],
        src.[impact],
        src.[source],
        src.[limittype],
        src.[reason],
        src.[modifications],
        src.[additionalnotes],
        src.[p5min_scope_override],
        src.[lrc],
        src.[lor],
        src.[force_scada]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGenconsetNull1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GenconsetNull1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconid] = src.[genconid]
    and tgt.[genconsetid] = src.[genconsetid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[genconsetid] = src.[genconsetid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[genconid] = src.[genconid],
        tgt.[genconeffdate] = src.[genconeffdate],
        tgt.[genconversionno] = src.[genconversionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [genconsetid],
        [effectivedate],
        [versionno],
        [genconid],
        [genconeffdate],
        [genconversionno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[genconsetid],
        src.[effectivedate],
        src.[versionno],
        src.[genconid],
        src.[genconeffdate],
        src.[genconversionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGenconsetinvokeNull2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GenconsetinvokeNull2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[invocation_id] = src.[invocation_id]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[invocation_id] = src.[invocation_id],
        tgt.[startdate] = src.[startdate],
        tgt.[startperiod] = src.[startperiod],
        tgt.[genconsetid] = src.[genconsetid],
        tgt.[enddate] = src.[enddate],
        tgt.[endperiod] = src.[endperiod],
        tgt.[startauthorisedby] = src.[startauthorisedby],
        tgt.[endauthorisedby] = src.[endauthorisedby],
        tgt.[intervention] = src.[intervention],
        tgt.[asconstrainttype] = src.[asconstrainttype],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[startintervaldatetime] = src.[startintervaldatetime],
        tgt.[endintervaldatetime] = src.[endintervaldatetime],
        tgt.[systemnormal] = src.[systemnormal]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[invocation_id],
        src.[startdate],
        src.[startperiod],
        src.[genconsetid],
        src.[enddate],
        src.[endperiod],
        src.[startauthorisedby],
        src.[endauthorisedby],
        src.[intervention],
        src.[asconstrainttype],
        src.[lastchanged],
        src.[startintervaldatetime],
        src.[endintervaldatetime],
        src.[systemnormal]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGenconsettrkNull2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GenconsettrkNull2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconsetid] = src.[genconsetid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[genconsetid] = src.[genconsetid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[description] = src.[description],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[coverage] = src.[coverage],
        tgt.[modifications] = src.[modifications],
        tgt.[systemnormal] = src.[systemnormal],
        tgt.[outage] = src.[outage]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[genconsetid],
        src.[effectivedate],
        src.[versionno],
        src.[description],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged],
        src.[coverage],
        src.[modifications],
        src.[systemnormal],
        src.[outage]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGcrhsNull1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GcrhsNull1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconid] = src.[genconid]
    and tgt.[scope] = src.[scope]
    and tgt.[termid] = src.[termid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[genconid] = src.[genconid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[scope] = src.[scope],
        tgt.[termid] = src.[termid],
        tgt.[groupid] = src.[groupid],
        tgt.[spd_id] = src.[spd_id],
        tgt.[spd_type] = src.[spd_type],
        tgt.[factor] = src.[factor],
        tgt.[operation] = src.[operation],
        tgt.[defaultvalue] = src.[defaultvalue],
        tgt.[parameterterm1] = src.[parameterterm1],
        tgt.[parameterterm2] = src.[parameterterm2],
        tgt.[parameterterm3] = src.[parameterterm3],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[genconid],
        src.[effectivedate],
        src.[versionno],
        src.[scope],
        src.[termid],
        src.[groupid],
        src.[spd_id],
        src.[spd_type],
        src.[factor],
        src.[operation],
        src.[defaultvalue],
        src.[parameterterm1],
        src.[parameterterm2],
        src.[parameterterm3],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGeqdescNull2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GeqdescNull2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[equationid] = src.[equationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[equationid] = src.[equationid],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[impact] = src.[impact],
        tgt.[source] = src.[source],
        tgt.[limittype] = src.[limittype],
        tgt.[reason] = src.[reason],
        tgt.[modifications] = src.[modifications],
        tgt.[additionalnotes] = src.[additionalnotes]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[equationid],
        src.[description],
        src.[lastchanged],
        src.[impact],
        src.[source],
        src.[limittype],
        src.[reason],
        src.[modifications],
        src.[additionalnotes]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertGeqrhsNull1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.GeqrhsNull1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[equationid] = src.[equationid]
    and tgt.[termid] = src.[termid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[equationid] = src.[equationid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[termid] = src.[termid],
        tgt.[groupid] = src.[groupid],
        tgt.[spd_id] = src.[spd_id],
        tgt.[spd_type] = src.[spd_type],
        tgt.[factor] = src.[factor],
        tgt.[operation] = src.[operation],
        tgt.[defaultvalue] = src.[defaultvalue],
        tgt.[parameterterm1] = src.[parameterterm1],
        tgt.[parameterterm2] = src.[parameterterm2],
        tgt.[parameterterm3] = src.[parameterterm3],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[equationid],
        src.[effectivedate],
        src.[versionno],
        src.[termid],
        src.[groupid],
        src.[spd_id],
        src.[spd_type],
        src.[factor],
        src.[operation],
        src.[defaultvalue],
        src.[parameterterm1],
        src.[parameterterm2],
        src.[parameterterm3],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSpdcpcNull2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SpdcpcNull2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconid] = src.[genconid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[genconid] = src.[genconid],
        tgt.[factor] = src.[factor],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[bidtype] = src.[bidtype]
when not matched
    then insert (
        file_log_id,
        [connectionpointid],
        [effectivedate],
        [versionno],
        [genconid],
        [factor],
        [lastchanged],
        [bidtype]
    ) values (
        @file_log_id,
        src.[connectionpointid],
        src.[effectivedate],
        src.[versionno],
        src.[genconid],
        src.[factor],
        src.[lastchanged],
        src.[bidtype]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSpdiccNull1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SpdiccNull1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconid] = src.[genconid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[genconid] = src.[genconid],
        tgt.[factor] = src.[factor],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [interconnectorid],
        [effectivedate],
        [versionno],
        [genconid],
        [factor],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[interconnectorid],
        src.[effectivedate],
        src.[versionno],
        src.[genconid],
        src.[factor],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSpdrcNull2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SpdrcNull2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[genconid] = src.[genconid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[genconid] = src.[genconid],
        tgt.[factor] = src.[factor],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[bidtype] = src.[bidtype]
when not matched
    then insert (
        file_log_id,
        [regionid],
        [effectivedate],
        [versionno],
        [genconid],
        [factor],
        [lastchanged],
        [bidtype]
    ) values (
        @file_log_id,
        src.[regionid],
        src.[effectivedate],
        src.[versionno],
        src.[genconid],
        src.[factor],
        src.[lastchanged],
        src.[bidtype]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertOfferBidperoffer1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.OfferBidperoffer1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[duid] = src.[duid]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[periodid] = src.[periodid],
        tgt.[versionno] = src.[versionno],
        tgt.[maxavail] = src.[maxavail],
        tgt.[fixedload] = src.[fixedload],
        tgt.[rocup] = src.[rocup],
        tgt.[rocdown] = src.[rocdown],
        tgt.[enablementmin] = src.[enablementmin],
        tgt.[enablementmax] = src.[enablementmax],
        tgt.[lowbreakpoint] = src.[lowbreakpoint],
        tgt.[highbreakpoint] = src.[highbreakpoint],
        tgt.[bandavail1] = src.[bandavail1],
        tgt.[bandavail2] = src.[bandavail2],
        tgt.[bandavail3] = src.[bandavail3],
        tgt.[bandavail4] = src.[bandavail4],
        tgt.[bandavail5] = src.[bandavail5],
        tgt.[bandavail6] = src.[bandavail6],
        tgt.[bandavail7] = src.[bandavail7],
        tgt.[bandavail8] = src.[bandavail8],
        tgt.[bandavail9] = src.[bandavail9],
        tgt.[bandavail10] = src.[bandavail10],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[pasaavailability] = src.[pasaavailability],
        tgt.[mr_capacity] = src.[mr_capacity]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[duid],
        src.[bidtype],
        src.[settlementdate],
        src.[offerdate],
        src.[periodid],
        src.[versionno],
        src.[maxavail],
        src.[fixedload],
        src.[rocup],
        src.[rocdown],
        src.[enablementmin],
        src.[enablementmax],
        src.[lowbreakpoint],
        src.[highbreakpoint],
        src.[bandavail1],
        src.[bandavail2],
        src.[bandavail3],
        src.[bandavail4],
        src.[bandavail5],
        src.[bandavail6],
        src.[bandavail7],
        src.[bandavail8],
        src.[bandavail9],
        src.[bandavail10],
        src.[lastchanged],
        src.[pasaavailability],
        src.[mr_capacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchbncCasesolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchbncCasesolution1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[runno],
        d.[intervention],
        d.[casesubtype],
        d.[solutionstatus],
        d.[spdversion],
        d.[startperiod],
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
        d.[totalenergyconstrviolation],
        d.[totalenergyofferviolation],
        d.[totalasprofileviolation],
        d.[totalfaststartviolation],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [runno] decimal(3,0),
        [intervention] decimal(2,0),
        [casesubtype] varchar(3),
        [solutionstatus] decimal(2,0),
        [spdversion] decimal(10,3),
        [startperiod] varchar(20),
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
        [totalenergyconstrviolation] decimal(15,5),
        [totalenergyofferviolation] decimal(15,5),
        [totalasprofileviolation] decimal(15,5),
        [totalfaststartviolation] decimal(15,5),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[casesubtype] = src.[casesubtype],
        tgt.[solutionstatus] = src.[solutionstatus],
        tgt.[spdversion] = src.[spdversion],
        tgt.[startperiod] = src.[startperiod],
        tgt.[nonphysicallosses] = src.[nonphysicallosses],
        tgt.[totalobjective] = src.[totalobjective],
        tgt.[totalareagenviolation] = src.[totalareagenviolation],
        tgt.[totalinterconnectorviolation] = src.[totalinterconnectorviolation],
        tgt.[totalgenericviolation] = src.[totalgenericviolation],
        tgt.[totalramprateviolation] = src.[totalramprateviolation],
        tgt.[totalunitmwcapacityviolation] = src.[totalunitmwcapacityviolation],
        tgt.[total5minviolation] = src.[total5minviolation],
        tgt.[totalregviolation] = src.[totalregviolation],
        tgt.[total6secviolation] = src.[total6secviolation],
        tgt.[total60secviolation] = src.[total60secviolation],
        tgt.[totalenergyconstrviolation] = src.[totalenergyconstrviolation],
        tgt.[totalenergyofferviolation] = src.[totalenergyofferviolation],
        tgt.[totalasprofileviolation] = src.[totalasprofileviolation],
        tgt.[totalfaststartviolation] = src.[totalfaststartviolation],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [intervention],
        [casesubtype],
        [solutionstatus],
        [spdversion],
        [startperiod],
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
        [totalenergyconstrviolation],
        [totalenergyofferviolation],
        [totalasprofileviolation],
        [totalfaststartviolation],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[intervention],
        src.[casesubtype],
        src.[solutionstatus],
        src.[spdversion],
        src.[startperiod],
        src.[nonphysicallosses],
        src.[totalobjective],
        src.[totalareagenviolation],
        src.[totalinterconnectorviolation],
        src.[totalgenericviolation],
        src.[totalramprateviolation],
        src.[totalunitmwcapacityviolation],
        src.[total5minviolation],
        src.[totalregviolation],
        src.[total6secviolation],
        src.[total60secviolation],
        src.[totalenergyconstrviolation],
        src.[totalenergyofferviolation],
        src.[totalasprofileviolation],
        src.[totalfaststartviolation],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchocdCase1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchocdCase1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[runno],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [runno] decimal(3,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertDispatchbncUnitsolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.DispatchbncUnitsolution1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[runno],
        d.[duid],
        d.[intervention],
        d.[connectionpointid],
        d.[dispatchmode],
        d.[totalcleared],
        d.[raisereg],
        d.[raise5min],
        d.[raise60sec],
        d.[raise6sec],
        d.[lowerreg],
        d.[lower5min],
        d.[lower60sec],
        d.[lower6sec],
        d.[raiseregflags],
        d.[raise5minflags],
        d.[raise60secflags],
        d.[raise6secflags],
        d.[lowerregflags],
        d.[lower5minflags],
        d.[lower60secflags],
        d.[lower6secflags],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [runno] decimal(3,0),
        [duid] varchar(10),
        [intervention] decimal(2,0),
        [connectionpointid] varchar(12),
        [dispatchmode] decimal(2,0),
        [totalcleared] decimal(15,5),
        [raisereg] decimal(15,5),
        [raise5min] decimal(15,5),
        [raise60sec] decimal(15,5),
        [raise6sec] decimal(15,5),
        [lowerreg] decimal(15,5),
        [lower5min] decimal(15,5),
        [lower60sec] decimal(15,5),
        [lower6sec] decimal(15,5),
        [raiseregflags] decimal(3,0),
        [raise5minflags] decimal(3,0),
        [raise60secflags] decimal(3,0),
        [raise6secflags] decimal(3,0),
        [lowerregflags] decimal(3,0),
        [lower5minflags] decimal(3,0),
        [lower60secflags] decimal(3,0),
        [lower6secflags] decimal(3,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[intervention] = src.[intervention]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[duid] = src.[duid],
        tgt.[intervention] = src.[intervention],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[dispatchmode] = src.[dispatchmode],
        tgt.[totalcleared] = src.[totalcleared],
        tgt.[raisereg] = src.[raisereg],
        tgt.[raise5min] = src.[raise5min],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[lower5min] = src.[lower5min],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raiseregflags] = src.[raiseregflags],
        tgt.[raise5minflags] = src.[raise5minflags],
        tgt.[raise60secflags] = src.[raise60secflags],
        tgt.[raise6secflags] = src.[raise6secflags],
        tgt.[lowerregflags] = src.[lowerregflags],
        tgt.[lower5minflags] = src.[lower5minflags],
        tgt.[lower60secflags] = src.[lower60secflags],
        tgt.[lower6secflags] = src.[lower6secflags],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [duid],
        [intervention],
        [connectionpointid],
        [dispatchmode],
        [totalcleared],
        [raisereg],
        [raise5min],
        [raise60sec],
        [raise6sec],
        [lowerreg],
        [lower5min],
        [lower60sec],
        [lower6sec],
        [raiseregflags],
        [raise5minflags],
        [raise60secflags],
        [raise6secflags],
        [lowerregflags],
        [lower5minflags],
        [lower60secflags],
        [lower6secflags],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[duid],
        src.[intervention],
        src.[connectionpointid],
        src.[dispatchmode],
        src.[totalcleared],
        src.[raisereg],
        src.[raise5min],
        src.[raise60sec],
        src.[raise6sec],
        src.[lowerreg],
        src.[lower5min],
        src.[lower60sec],
        src.[lower6sec],
        src.[raiseregflags],
        src.[raise5minflags],
        src.[raise60secflags],
        src.[raise6secflags],
        src.[lowerregflags],
        src.[lower5minflags],
        src.[lower60secflags],
        src.[lower6secflags],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterDataCustomer1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterDataCustomer1 as tgt 
using (
    select 
        d.[participantid],
        d.[periodid],
        d.[settlementdate],
        d.[meterrunno],
        d.[connectionpointid],
        d.[importenergyvalue],
        d.[exportenergyvalue],
        d.[importreactivevalue],
        d.[exportreactivevalue],
        d.[hostdistributor],
        d.[lastchanged],
        d.[mda]
    from openjson(@data) with (
        [participantid] varchar(10),
        [periodid] decimal(3,0),
        [settlementdate] datetime2,
        [meterrunno] decimal(6,0),
        [connectionpointid] varchar(10),
        [importenergyvalue] decimal(9,6),
        [exportenergyvalue] decimal(9,6),
        [importreactivevalue] decimal(9,6),
        [exportreactivevalue] decimal(9,6),
        [hostdistributor] varchar(10),
        [lastchanged] datetime2,
        [mda] varchar(10)
    ) d
) as src 
on (
    tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[hostdistributor] = src.[hostdistributor]
    and tgt.[mda] = src.[mda]
    and tgt.[meterrunno] = src.[meterrunno]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[periodid] = src.[periodid],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[meterrunno] = src.[meterrunno],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[importenergyvalue] = src.[importenergyvalue],
        tgt.[exportenergyvalue] = src.[exportenergyvalue],
        tgt.[importreactivevalue] = src.[importreactivevalue],
        tgt.[exportreactivevalue] = src.[exportreactivevalue],
        tgt.[hostdistributor] = src.[hostdistributor],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[mda] = src.[mda]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [periodid],
        [settlementdate],
        [meterrunno],
        [connectionpointid],
        [importenergyvalue],
        [exportenergyvalue],
        [importreactivevalue],
        [exportreactivevalue],
        [hostdistributor],
        [lastchanged],
        [mda]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[periodid],
        src.[settlementdate],
        src.[meterrunno],
        src.[connectionpointid],
        src.[importenergyvalue],
        src.[exportenergyvalue],
        src.[importreactivevalue],
        src.[exportreactivevalue],
        src.[hostdistributor],
        src.[lastchanged],
        src.[mda]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterDataCustomerTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterDataCustomerTrk1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[meterrunno],
        d.[participantid],
        d.[filename],
        d.[ackfilename],
        d.[connectionpointid],
        d.[authoriseddate],
        d.[authorisedby],
        d.[meteringdataagent],
        d.[hostdistributor],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [meterrunno] decimal(6,0),
        [participantid] varchar(10),
        [filename] varchar(40),
        [ackfilename] varchar(40),
        [connectionpointid] varchar(10),
        [authoriseddate] datetime2,
        [authorisedby] varchar(15),
        [meteringdataagent] varchar(10),
        [hostdistributor] varchar(10),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[hostdistributor] = src.[hostdistributor]
    and tgt.[meteringdataagent] = src.[meteringdataagent]
    and tgt.[meterrunno] = src.[meterrunno]
    and tgt.[participantid] = src.[participantid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[meterrunno] = src.[meterrunno],
        tgt.[participantid] = src.[participantid],
        tgt.[filename] = src.[filename],
        tgt.[ackfilename] = src.[ackfilename],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[meteringdataagent] = src.[meteringdataagent],
        tgt.[hostdistributor] = src.[hostdistributor],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [meterrunno],
        [participantid],
        [filename],
        [ackfilename],
        [connectionpointid],
        [authoriseddate],
        [authorisedby],
        [meteringdataagent],
        [hostdistributor],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[meterrunno],
        src.[participantid],
        src.[filename],
        src.[ackfilename],
        src.[connectionpointid],
        src.[authoriseddate],
        src.[authorisedby],
        src.[meteringdataagent],
        src.[hostdistributor],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterDataGenDuid1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterDataGenDuid1 as tgt 
using (
    select 
        d.[interval_datetime],
        d.[duid],
        d.[mwh_reading],
        d.[lastchanged]
    from openjson(@data) with (
        [interval_datetime] datetime2,
        [duid] varchar(10),
        [mwh_reading] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[interval_datetime] = src.[interval_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[duid] = src.[duid],
        tgt.[mwh_reading] = src.[mwh_reading],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [interval_datetime],
        [duid],
        [mwh_reading],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[interval_datetime],
        src.[duid],
        src.[mwh_reading],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterdataTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterdataTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[case_id] = src.[case_id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[case_id] = src.[case_id],
        tgt.[aggregate_reads_load_datetime] = src.[aggregate_reads_load_datetime],
        tgt.[individual_reads_load_datetime] = src.[individual_reads_load_datetime],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [case_id],
        [aggregate_reads_load_datetime],
        [individual_reads_load_datetime],
        [startdate],
        [enddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[case_id],
        src.[aggregate_reads_load_datetime],
        src.[individual_reads_load_datetime],
        src.[startdate],
        src.[enddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidMnspFiletrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidMnspFiletrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[filename] = src.[filename]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[participantid] = src.[participantid],
        tgt.[filename] = src.[filename],
        tgt.[status] = src.[status],
        tgt.[ackfilename] = src.[ackfilename],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [offerdate],
        [participantid],
        [filename],
        [status],
        [ackfilename],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[offerdate],
        src.[participantid],
        src.[filename],
        src.[status],
        src.[ackfilename],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidMnspOffertrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidMnspOffertrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[filename] = src.[filename]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[filename] = src.[filename],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [offerdate],
        [versionno],
        [participantid],
        [filename],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[offerdate],
        src.[versionno],
        src.[participantid],
        src.[filename],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertBidMnspPeroffer1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.BidMnspPeroffer1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[linkid] = src.[linkid]
    and tgt.[offerdate] = src.[offerdate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[linkid] = src.[linkid],
        tgt.[periodid] = src.[periodid],
        tgt.[maxavail] = src.[maxavail],
        tgt.[bandavail1] = src.[bandavail1],
        tgt.[bandavail2] = src.[bandavail2],
        tgt.[bandavail3] = src.[bandavail3],
        tgt.[bandavail4] = src.[bandavail4],
        tgt.[bandavail5] = src.[bandavail5],
        tgt.[bandavail6] = src.[bandavail6],
        tgt.[bandavail7] = src.[bandavail7],
        tgt.[bandavail8] = src.[bandavail8],
        tgt.[bandavail9] = src.[bandavail9],
        tgt.[bandavail10] = src.[bandavail10],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[fixedload] = src.[fixedload],
        tgt.[rampuprate] = src.[rampuprate],
        tgt.[pasaavailability] = src.[pasaavailability],
        tgt.[mr_capacity] = src.[mr_capacity]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[offerdate],
        src.[versionno],
        src.[participantid],
        src.[linkid],
        src.[periodid],
        src.[maxavail],
        src.[bandavail1],
        src.[bandavail2],
        src.[bandavail3],
        src.[bandavail4],
        src.[bandavail5],
        src.[bandavail6],
        src.[bandavail7],
        src.[bandavail8],
        src.[bandavail9],
        src.[bandavail10],
        src.[lastchanged],
        src.[fixedload],
        src.[rampuprate],
        src.[pasaavailability],
        src.[mr_capacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMrDayofferStack1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MrDayofferStack1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[mr_date] = src.[mr_date]
    and tgt.[regionid] = src.[regionid]
    and tgt.[stack_position] = src.[stack_position]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[mr_date] = src.[mr_date],
        tgt.[regionid] = src.[regionid],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[stack_position] = src.[stack_position],
        tgt.[duid] = src.[duid],
        tgt.[authorised] = src.[authorised],
        tgt.[offer_settlementdate] = src.[offer_settlementdate],
        tgt.[offer_offerdate] = src.[offer_offerdate],
        tgt.[offer_versionno] = src.[offer_versionno],
        tgt.[offer_type] = src.[offer_type],
        tgt.[laof] = src.[laof],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[mr_date],
        src.[regionid],
        src.[version_datetime],
        src.[stack_position],
        src.[duid],
        src.[authorised],
        src.[offer_settlementdate],
        src.[offer_offerdate],
        src.[offer_versionno],
        src.[offer_type],
        src.[laof],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMrEvent1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MrEvent1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[mr_date] = src.[mr_date]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[mr_date] = src.[mr_date],
        tgt.[regionid] = src.[regionid],
        tgt.[description] = src.[description],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[offer_cut_off_time] = src.[offer_cut_off_time],
        tgt.[settlement_complete] = src.[settlement_complete],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [mr_date],
        [regionid],
        [description],
        [authoriseddate],
        [authorisedby],
        [offer_cut_off_time],
        [settlement_complete],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[mr_date],
        src.[regionid],
        src.[description],
        src.[authoriseddate],
        src.[authorisedby],
        src.[offer_cut_off_time],
        src.[settlement_complete],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMrEventSchedule1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MrEventSchedule1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[mr_date] = src.[mr_date]
    and tgt.[regionid] = src.[regionid]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[mr_date] = src.[mr_date],
        tgt.[regionid] = src.[regionid],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[demand_effectivedate] = src.[demand_effectivedate],
        tgt.[demand_offerdate] = src.[demand_offerdate],
        tgt.[demand_versionno] = src.[demand_versionno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[mr_date],
        src.[regionid],
        src.[version_datetime],
        src.[demand_effectivedate],
        src.[demand_offerdate],
        src.[demand_versionno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMrPerofferStack1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MrPerofferStack1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[mr_date] = src.[mr_date]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[stack_position] = src.[stack_position]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[mr_date] = src.[mr_date],
        tgt.[regionid] = src.[regionid],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[stack_position] = src.[stack_position],
        tgt.[periodid] = src.[periodid],
        tgt.[duid] = src.[duid],
        tgt.[accepted_capacity] = src.[accepted_capacity],
        tgt.[deducted_capacity] = src.[deducted_capacity],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[mr_date],
        src.[regionid],
        src.[version_datetime],
        src.[stack_position],
        src.[periodid],
        src.[duid],
        src.[accepted_capacity],
        src.[deducted_capacity],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertTradingUnitSolution2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.TradingUnitSolution2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[tradetype] = src.[tradetype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[duid] = src.[duid],
        tgt.[tradetype] = src.[tradetype],
        tgt.[periodid] = src.[periodid],
        tgt.[initialmw] = src.[initialmw],
        tgt.[totalcleared] = src.[totalcleared],
        tgt.[rampdownrate] = src.[rampdownrate],
        tgt.[rampuprate] = src.[rampuprate],
        tgt.[lower5min] = src.[lower5min],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise5min] = src.[raise5min],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[availability] = src.[availability],
        tgt.[semidispatchcap] = src.[semidispatchcap]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[duid],
        src.[tradetype],
        src.[periodid],
        src.[initialmw],
        src.[totalcleared],
        src.[rampdownrate],
        src.[rampuprate],
        src.[lower5min],
        src.[lower60sec],
        src.[lower6sec],
        src.[raise5min],
        src.[raise60sec],
        src.[raise6sec],
        src.[lastchanged],
        src.[lowerreg],
        src.[raisereg],
        src.[availability],
        src.[semidispatchcap]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertTradingRegionsum4
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.TradingRegionsum4 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[totaldemand] = src.[totaldemand],
        tgt.[availablegeneration] = src.[availablegeneration],
        tgt.[availableload] = src.[availableload],
        tgt.[demandforecast] = src.[demandforecast],
        tgt.[dispatchablegeneration] = src.[dispatchablegeneration],
        tgt.[dispatchableload] = src.[dispatchableload],
        tgt.[netinterchange] = src.[netinterchange],
        tgt.[excessgeneration] = src.[excessgeneration],
        tgt.[lower5mindispatch] = src.[lower5mindispatch],
        tgt.[lower5minimport] = src.[lower5minimport],
        tgt.[lower5minlocaldispatch] = src.[lower5minlocaldispatch],
        tgt.[lower5minlocalprice] = src.[lower5minlocalprice],
        tgt.[lower5minlocalreq] = src.[lower5minlocalreq],
        tgt.[lower5minprice] = src.[lower5minprice],
        tgt.[lower5minreq] = src.[lower5minreq],
        tgt.[lower5minsupplyprice] = src.[lower5minsupplyprice],
        tgt.[lower60secdispatch] = src.[lower60secdispatch],
        tgt.[lower60secimport] = src.[lower60secimport],
        tgt.[lower60seclocaldispatch] = src.[lower60seclocaldispatch],
        tgt.[lower60seclocalprice] = src.[lower60seclocalprice],
        tgt.[lower60seclocalreq] = src.[lower60seclocalreq],
        tgt.[lower60secprice] = src.[lower60secprice],
        tgt.[lower60secreq] = src.[lower60secreq],
        tgt.[lower60secsupplyprice] = src.[lower60secsupplyprice],
        tgt.[lower6secdispatch] = src.[lower6secdispatch],
        tgt.[lower6secimport] = src.[lower6secimport],
        tgt.[lower6seclocaldispatch] = src.[lower6seclocaldispatch],
        tgt.[lower6seclocalprice] = src.[lower6seclocalprice],
        tgt.[lower6seclocalreq] = src.[lower6seclocalreq],
        tgt.[lower6secprice] = src.[lower6secprice],
        tgt.[lower6secreq] = src.[lower6secreq],
        tgt.[lower6secsupplyprice] = src.[lower6secsupplyprice],
        tgt.[raise5mindispatch] = src.[raise5mindispatch],
        tgt.[raise5minimport] = src.[raise5minimport],
        tgt.[raise5minlocaldispatch] = src.[raise5minlocaldispatch],
        tgt.[raise5minlocalprice] = src.[raise5minlocalprice],
        tgt.[raise5minlocalreq] = src.[raise5minlocalreq],
        tgt.[raise5minprice] = src.[raise5minprice],
        tgt.[raise5minreq] = src.[raise5minreq],
        tgt.[raise5minsupplyprice] = src.[raise5minsupplyprice],
        tgt.[raise60secdispatch] = src.[raise60secdispatch],
        tgt.[raise60secimport] = src.[raise60secimport],
        tgt.[raise60seclocaldispatch] = src.[raise60seclocaldispatch],
        tgt.[raise60seclocalprice] = src.[raise60seclocalprice],
        tgt.[raise60seclocalreq] = src.[raise60seclocalreq],
        tgt.[raise60secprice] = src.[raise60secprice],
        tgt.[raise60secreq] = src.[raise60secreq],
        tgt.[raise60secsupplyprice] = src.[raise60secsupplyprice],
        tgt.[raise6secdispatch] = src.[raise6secdispatch],
        tgt.[raise6secimport] = src.[raise6secimport],
        tgt.[raise6seclocaldispatch] = src.[raise6seclocaldispatch],
        tgt.[raise6seclocalprice] = src.[raise6seclocalprice],
        tgt.[raise6seclocalreq] = src.[raise6seclocalreq],
        tgt.[raise6secprice] = src.[raise6secprice],
        tgt.[raise6secreq] = src.[raise6secreq],
        tgt.[raise6secsupplyprice] = src.[raise6secsupplyprice],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[initialsupply] = src.[initialsupply],
        tgt.[clearedsupply] = src.[clearedsupply],
        tgt.[lowerregimport] = src.[lowerregimport],
        tgt.[lowerreglocaldispatch] = src.[lowerreglocaldispatch],
        tgt.[lowerreglocalreq] = src.[lowerreglocalreq],
        tgt.[lowerregreq] = src.[lowerregreq],
        tgt.[raiseregimport] = src.[raiseregimport],
        tgt.[raisereglocaldispatch] = src.[raisereglocaldispatch],
        tgt.[raisereglocalreq] = src.[raisereglocalreq],
        tgt.[raiseregreq] = src.[raiseregreq],
        tgt.[raise5minlocalviolation] = src.[raise5minlocalviolation],
        tgt.[raisereglocalviolation] = src.[raisereglocalviolation],
        tgt.[raise60seclocalviolation] = src.[raise60seclocalviolation],
        tgt.[raise6seclocalviolation] = src.[raise6seclocalviolation],
        tgt.[lower5minlocalviolation] = src.[lower5minlocalviolation],
        tgt.[lowerreglocalviolation] = src.[lowerreglocalviolation],
        tgt.[lower60seclocalviolation] = src.[lower60seclocalviolation],
        tgt.[lower6seclocalviolation] = src.[lower6seclocalviolation],
        tgt.[raise5minviolation] = src.[raise5minviolation],
        tgt.[raiseregviolation] = src.[raiseregviolation],
        tgt.[raise60secviolation] = src.[raise60secviolation],
        tgt.[raise6secviolation] = src.[raise6secviolation],
        tgt.[lower5minviolation] = src.[lower5minviolation],
        tgt.[lowerregviolation] = src.[lowerregviolation],
        tgt.[lower60secviolation] = src.[lower60secviolation],
        tgt.[lower6secviolation] = src.[lower6secviolation],
        tgt.[totalintermittentgeneration] = src.[totalintermittentgeneration],
        tgt.[demand_and_nonschedgen] = src.[demand_and_nonschedgen],
        tgt.[uigf] = src.[uigf]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[regionid],
        src.[periodid],
        src.[totaldemand],
        src.[availablegeneration],
        src.[availableload],
        src.[demandforecast],
        src.[dispatchablegeneration],
        src.[dispatchableload],
        src.[netinterchange],
        src.[excessgeneration],
        src.[lower5mindispatch],
        src.[lower5minimport],
        src.[lower5minlocaldispatch],
        src.[lower5minlocalprice],
        src.[lower5minlocalreq],
        src.[lower5minprice],
        src.[lower5minreq],
        src.[lower5minsupplyprice],
        src.[lower60secdispatch],
        src.[lower60secimport],
        src.[lower60seclocaldispatch],
        src.[lower60seclocalprice],
        src.[lower60seclocalreq],
        src.[lower60secprice],
        src.[lower60secreq],
        src.[lower60secsupplyprice],
        src.[lower6secdispatch],
        src.[lower6secimport],
        src.[lower6seclocaldispatch],
        src.[lower6seclocalprice],
        src.[lower6seclocalreq],
        src.[lower6secprice],
        src.[lower6secreq],
        src.[lower6secsupplyprice],
        src.[raise5mindispatch],
        src.[raise5minimport],
        src.[raise5minlocaldispatch],
        src.[raise5minlocalprice],
        src.[raise5minlocalreq],
        src.[raise5minprice],
        src.[raise5minreq],
        src.[raise5minsupplyprice],
        src.[raise60secdispatch],
        src.[raise60secimport],
        src.[raise60seclocaldispatch],
        src.[raise60seclocalprice],
        src.[raise60seclocalreq],
        src.[raise60secprice],
        src.[raise60secreq],
        src.[raise60secsupplyprice],
        src.[raise6secdispatch],
        src.[raise6secimport],
        src.[raise6seclocaldispatch],
        src.[raise6seclocalprice],
        src.[raise6seclocalreq],
        src.[raise6secprice],
        src.[raise6secreq],
        src.[raise6secsupplyprice],
        src.[lastchanged],
        src.[initialsupply],
        src.[clearedsupply],
        src.[lowerregimport],
        src.[lowerreglocaldispatch],
        src.[lowerreglocalreq],
        src.[lowerregreq],
        src.[raiseregimport],
        src.[raisereglocaldispatch],
        src.[raisereglocalreq],
        src.[raiseregreq],
        src.[raise5minlocalviolation],
        src.[raisereglocalviolation],
        src.[raise60seclocalviolation],
        src.[raise6seclocalviolation],
        src.[lower5minlocalviolation],
        src.[lowerreglocalviolation],
        src.[lower60seclocalviolation],
        src.[lower6seclocalviolation],
        src.[raise5minviolation],
        src.[raiseregviolation],
        src.[raise60secviolation],
        src.[raise6secviolation],
        src.[lower5minviolation],
        src.[lowerregviolation],
        src.[lower60secviolation],
        src.[lower6secviolation],
        src.[totalintermittentgeneration],
        src.[demand_and_nonschedgen],
        src.[uigf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuction1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuction1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[auctionid] = src.[auctionid],
        tgt.[auctiondate] = src.[auctiondate],
        tgt.[notifydate] = src.[notifydate],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[description] = src.[description],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[auctionid],
        src.[auctiondate],
        src.[notifydate],
        src.[startdate],
        src.[enddate],
        src.[description],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuctionCalendar2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuctionCalendar2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[quarter] = src.[quarter]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[notifydate] = src.[notifydate],
        tgt.[paymentdate] = src.[paymentdate],
        tgt.[reconciliationdate] = src.[reconciliationdate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[prelimpurchasestmtdate] = src.[prelimpurchasestmtdate],
        tgt.[prelimproceedsstmtdate] = src.[prelimproceedsstmtdate],
        tgt.[finalpurchasestmtdate] = src.[finalpurchasestmtdate],
        tgt.[finalproceedsstmtdate] = src.[finalproceedsstmtdate]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[startdate],
        src.[enddate],
        src.[notifydate],
        src.[paymentdate],
        src.[reconciliationdate],
        src.[lastchanged],
        src.[prelimpurchasestmtdate],
        src.[prelimproceedsstmtdate],
        src.[finalpurchasestmtdate],
        src.[finalproceedsstmtdate]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuctionIcAllocations2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuctionIcAllocations2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[quarter] = src.[quarter]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[maximumunits] = src.[maximumunits],
        tgt.[proportion] = src.[proportion],
        tgt.[auctionfee] = src.[auctionfee],
        tgt.[changedate] = src.[changedate],
        tgt.[changedby] = src.[changedby],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[auctionfee_sales] = src.[auctionfee_sales]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[versionno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[maximumunits],
        src.[proportion],
        src.[auctionfee],
        src.[changedate],
        src.[changedby],
        src.[lastchanged],
        src.[auctionfee_sales]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuctionRevenueEstimate1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuctionRevenueEstimate1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[monthno] = src.[monthno]
    and tgt.[quarter] = src.[quarter]
    and tgt.[valuationid] = src.[valuationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[valuationid] = src.[valuationid],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[monthno] = src.[monthno],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[revenue] = src.[revenue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[valuationid],
        src.[versionno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[monthno],
        src.[startdate],
        src.[enddate],
        src.[revenue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuctionRevenueTrack1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuctionRevenueTrack1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[quarter] = src.[quarter]
    and tgt.[valuationid] = src.[valuationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[valuationid] = src.[valuationid],
        tgt.[versionno] = src.[versionno],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[status] = src.[status],
        tgt.[documentref] = src.[documentref],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[valuationid],
        src.[versionno],
        src.[effectivedate],
        src.[status],
        src.[documentref],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuctionRpEstimate1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuctionRpEstimate1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[quarter] = src.[quarter]
    and tgt.[valuationid] = src.[valuationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[valuationid] = src.[valuationid],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[rpestimate] = src.[rpestimate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractyear],
        [quarter],
        [valuationid],
        [versionno],
        [interconnectorid],
        [fromregionid],
        [rpestimate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[valuationid],
        src.[versionno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[rpestimate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionConfigAuctionTranche1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionConfigAuctionTranche1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[quarter] = src.[quarter]
    and tgt.[tranche] = src.[tranche]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[versionno] = src.[versionno],
        tgt.[tranche] = src.[tranche],
        tgt.[auctiondate] = src.[auctiondate],
        tgt.[notifydate] = src.[notifydate],
        tgt.[unitallocation] = src.[unitallocation],
        tgt.[changedate] = src.[changedate],
        tgt.[changedby] = src.[changedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[versionno],
        src.[tranche],
        src.[auctiondate],
        src.[notifydate],
        src.[unitallocation],
        src.[changedate],
        src.[changedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigResiduecontractpayments1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigResiduecontractpayments1 as tgt 
using (
    select 
        d.[contractid],
        d.[participantid],
        d.[lastchanged]
    from openjson(@data) with (
        [contractid] varchar(30),
        [participantid] varchar(10),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [participantid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[participantid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionBidsFileTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionBidsFileTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[loaddate] = src.[loaddate]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[loaddate] = src.[loaddate],
        tgt.[filename] = src.[filename],
        tgt.[ackfilename] = src.[ackfilename],
        tgt.[status] = src.[status],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[auctionid] = src.[auctionid]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [participantid],
        [loaddate],
        [filename],
        [ackfilename],
        [status],
        [lastchanged],
        [auctionid]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[participantid],
        src.[loaddate],
        src.[filename],
        src.[ackfilename],
        src.[status],
        src.[lastchanged],
        src.[auctionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResidueBidTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResidueBidTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[bidloaddate] = src.[bidloaddate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[auctionid] = src.[auctionid]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [versionno],
        [participantid],
        [bidloaddate],
        [lastchanged],
        [auctionid]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[participantid],
        src.[bidloaddate],
        src.[lastchanged],
        src.[auctionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResidueContracts1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResidueContracts1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractyear] = src.[contractyear]
    and tgt.[quarter] = src.[quarter]
    and tgt.[tranche] = src.[tranche]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[tranche] = src.[tranche],
        tgt.[contractid] = src.[contractid],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[notifydate] = src.[notifydate],
        tgt.[auctiondate] = src.[auctiondate],
        tgt.[calcmethod] = src.[calcmethod],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[notifypostdate] = src.[notifypostdate],
        tgt.[notifyby] = src.[notifyby],
        tgt.[postdate] = src.[postdate],
        tgt.[postedby] = src.[postedby],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[description] = src.[description],
        tgt.[auctionid] = src.[auctionid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractyear],
        src.[quarter],
        src.[tranche],
        src.[contractid],
        src.[startdate],
        src.[enddate],
        src.[notifydate],
        src.[auctiondate],
        src.[calcmethod],
        src.[authoriseddate],
        src.[authorisedby],
        src.[notifypostdate],
        src.[notifyby],
        src.[postdate],
        src.[postedby],
        src.[lastchanged],
        src.[description],
        src.[auctionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResidueConData2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResidueConData2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[unitspurchased] = src.[unitspurchased],
        tgt.[linkpayment] = src.[linkpayment],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[secondary_units_sold] = src.[secondary_units_sold]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[unitspurchased],
        src.[linkpayment],
        src.[lastchanged],
        src.[secondary_units_sold]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResidueConEstimatesTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResidueConEstimatesTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[contractyear] = src.[contractyear]
    and tgt.[quarter] = src.[quarter]
    and tgt.[valuationid] = src.[valuationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[contractyear] = src.[contractyear],
        tgt.[quarter] = src.[quarter],
        tgt.[valuationid] = src.[valuationid],
        tgt.[versionno] = src.[versionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [contractyear],
        [quarter],
        [valuationid],
        [versionno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[contractyear],
        src.[quarter],
        src.[valuationid],
        src.[versionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResidueConFunds1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResidueConFunds1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[defaultunits] = src.[defaultunits],
        tgt.[rolloverunits] = src.[rolloverunits],
        tgt.[reallocatedunits] = src.[reallocatedunits],
        tgt.[unitsoffered] = src.[unitsoffered],
        tgt.[meanreserveprice] = src.[meanreserveprice],
        tgt.[scalefactor] = src.[scalefactor],
        tgt.[actualreserveprice] = src.[actualreserveprice],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[defaultunits],
        src.[rolloverunits],
        src.[reallocatedunits],
        src.[unitsoffered],
        src.[meanreserveprice],
        src.[scalefactor],
        src.[actualreserveprice],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionBidsFundsBid1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionBidsFundsBid1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[loaddate] = src.[loaddate]
    and tgt.[optionid] = src.[optionid]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[loaddate] = src.[loaddate],
        tgt.[optionid] = src.[optionid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[units] = src.[units],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [participantid],
        [loaddate],
        [optionid],
        [interconnectorid],
        [fromregionid],
        [units],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[participantid],
        src.[loaddate],
        src.[optionid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[units],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResiduePriceBid1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResiduePriceBid1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[loaddate] = src.[loaddate]
    and tgt.[optionid] = src.[optionid]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[participantid] = src.[participantid],
        tgt.[loaddate] = src.[loaddate],
        tgt.[optionid] = src.[optionid],
        tgt.[bidprice] = src.[bidprice],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[auctionid] = src.[auctionid]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [participantid],
        [loaddate],
        [optionid],
        [bidprice],
        [lastchanged],
        [auctionid]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[participantid],
        src.[loaddate],
        src.[optionid],
        src.[bidprice],
        src.[lastchanged],
        src.[auctionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResiduePriceFundsBid1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResiduePriceFundsBid1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[linkedbidflag] = src.[linkedbidflag]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[units] = src.[units],
        tgt.[bidprice] = src.[bidprice],
        tgt.[linkedbidflag] = src.[linkedbidflag],
        tgt.[auctionid] = src.[auctionid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [contractid],
        [interconnectorid],
        [fromregionid],
        [units],
        [bidprice],
        [linkedbidflag],
        [auctionid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[contractid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[units],
        src.[bidprice],
        src.[linkedbidflag],
        src.[auctionid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResiduePublicData1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResiduePublicData1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[unitsoffered] = src.[unitsoffered],
        tgt.[unitssold] = src.[unitssold],
        tgt.[clearingprice] = src.[clearingprice],
        tgt.[reserveprice] = src.[reserveprice],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[interconnectorid],
        src.[fromregionid],
        src.[unitsoffered],
        src.[unitssold],
        src.[clearingprice],
        src.[reserveprice],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionResidueTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionResidueTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[contractid] = src.[contractid],
        tgt.[versionno] = src.[versionno],
        tgt.[rundate] = src.[rundate],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[postdate] = src.[postdate],
        tgt.[postedby] = src.[postedby],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[status] = src.[status],
        tgt.[auctionid] = src.[auctionid]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[contractid],
        src.[versionno],
        src.[rundate],
        src.[authoriseddate],
        src.[authorisedby],
        src.[postdate],
        src.[postedby],
        src.[lastchanged],
        src.[status],
        src.[auctionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraCashSecurity1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraCashSecurity1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[cash_security_id] = src.[cash_security_id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[cash_security_id] = src.[cash_security_id],
        tgt.[participantid] = src.[participantid],
        tgt.[provision_date] = src.[provision_date],
        tgt.[cash_amount] = src.[cash_amount],
        tgt.[interest_acct_id] = src.[interest_acct_id],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[finalreturndate] = src.[finalreturndate],
        tgt.[cash_security_returned] = src.[cash_security_returned],
        tgt.[deletiondate] = src.[deletiondate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[cash_security_id],
        src.[participantid],
        src.[provision_date],
        src.[cash_amount],
        src.[interest_acct_id],
        src.[authoriseddate],
        src.[finalreturndate],
        src.[cash_security_returned],
        src.[deletiondate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraFinancialAucpayDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraFinancialAucpayDetail1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[sra_quarter] = src.[sra_quarter]
    and tgt.[sra_runno] = src.[sra_runno]
    and tgt.[sra_year] = src.[sra_year]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[sra_year] = src.[sra_year],
        tgt.[sra_quarter] = src.[sra_quarter],
        tgt.[sra_runno] = src.[sra_runno],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[contractid] = src.[contractid],
        tgt.[maximum_units] = src.[maximum_units],
        tgt.[units_sold] = src.[units_sold],
        tgt.[shortfall_units] = src.[shortfall_units],
        tgt.[reserve_price] = src.[reserve_price],
        tgt.[clearing_price] = src.[clearing_price],
        tgt.[payment_amount] = src.[payment_amount],
        tgt.[shortfall_amount] = src.[shortfall_amount],
        tgt.[allocation] = src.[allocation],
        tgt.[net_payment_amount] = src.[net_payment_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[sra_year],
        src.[sra_quarter],
        src.[sra_runno],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[contractid],
        src.[maximum_units],
        src.[units_sold],
        src.[shortfall_units],
        src.[reserve_price],
        src.[clearing_price],
        src.[payment_amount],
        src.[shortfall_amount],
        src.[allocation],
        src.[net_payment_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraFinancialAucpaySum1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraFinancialAucpaySum1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[participantid] = src.[participantid]
    and tgt.[sra_quarter] = src.[sra_quarter]
    and tgt.[sra_runno] = src.[sra_runno]
    and tgt.[sra_year] = src.[sra_year]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[sra_year] = src.[sra_year],
        tgt.[sra_quarter] = src.[sra_quarter],
        tgt.[sra_runno] = src.[sra_runno],
        tgt.[participantid] = src.[participantid],
        tgt.[gross_proceeds_amount] = src.[gross_proceeds_amount],
        tgt.[total_gross_proceeds_amount] = src.[total_gross_proceeds_amount],
        tgt.[shortfall_amount] = src.[shortfall_amount],
        tgt.[total_shortfall_amount] = src.[total_shortfall_amount],
        tgt.[net_payment_amount] = src.[net_payment_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[sra_year],
        src.[sra_quarter],
        src.[sra_runno],
        src.[participantid],
        src.[gross_proceeds_amount],
        src.[total_gross_proceeds_amount],
        src.[shortfall_amount],
        src.[total_shortfall_amount],
        src.[net_payment_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraFinancialAucMardetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.IrauctionSraFinancialAucMardetail1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertIrauctionSraFinancialAucMargin1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.IrauctionSraFinancialAucMargin1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertIrauctionSraFinancialAucReceipts1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraFinancialAucReceipts1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[sra_quarter] = src.[sra_quarter]
    and tgt.[sra_runno] = src.[sra_runno]
    and tgt.[sra_year] = src.[sra_year]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[sra_year] = src.[sra_year],
        tgt.[sra_quarter] = src.[sra_quarter],
        tgt.[sra_runno] = src.[sra_runno],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[contractid] = src.[contractid],
        tgt.[units_purchased] = src.[units_purchased],
        tgt.[clearing_price] = src.[clearing_price],
        tgt.[receipt_amount] = src.[receipt_amount],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[proceeds_amount] = src.[proceeds_amount],
        tgt.[units_sold] = src.[units_sold]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[sra_year],
        src.[sra_quarter],
        src.[sra_runno],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[contractid],
        src.[units_purchased],
        src.[clearing_price],
        src.[receipt_amount],
        src.[lastchanged],
        src.[proceeds_amount],
        src.[units_sold]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraFinancialRuntrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraFinancialRuntrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[sra_quarter] = src.[sra_quarter]
    and tgt.[sra_runno] = src.[sra_runno]
    and tgt.[sra_year] = src.[sra_year]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[sra_year] = src.[sra_year],
        tgt.[sra_quarter] = src.[sra_quarter],
        tgt.[sra_runno] = src.[sra_runno],
        tgt.[runtype] = src.[runtype],
        tgt.[rundate] = src.[rundate],
        tgt.[posteddate] = src.[posteddate],
        tgt.[interest_versionno] = src.[interest_versionno],
        tgt.[makeup_versionno] = src.[makeup_versionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[sra_year],
        src.[sra_quarter],
        src.[sra_runno],
        src.[runtype],
        src.[rundate],
        src.[posteddate],
        src.[interest_versionno],
        src.[makeup_versionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraOfferProduct1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraOfferProduct1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[loaddate] = src.[loaddate]
    and tgt.[optionid] = src.[optionid]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[auctionid] = src.[auctionid],
        tgt.[participantid] = src.[participantid],
        tgt.[loaddate] = src.[loaddate],
        tgt.[optionid] = src.[optionid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[offer_quantity] = src.[offer_quantity],
        tgt.[offer_price] = src.[offer_price],
        tgt.[trancheid] = src.[trancheid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[auctionid],
        src.[participantid],
        src.[loaddate],
        src.[optionid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[offer_quantity],
        src.[offer_price],
        src.[trancheid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraOfferProfile1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionSraOfferProfile1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[auctionid] = src.[auctionid]
    and tgt.[loaddate] = src.[loaddate]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[auctionid] = src.[auctionid],
        tgt.[participantid] = src.[participantid],
        tgt.[loaddate] = src.[loaddate],
        tgt.[filename] = src.[filename],
        tgt.[ackfilename] = src.[ackfilename],
        tgt.[transactionid] = src.[transactionid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [auctionid],
        [participantid],
        [loaddate],
        [filename],
        [ackfilename],
        [transactionid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[auctionid],
        src.[participantid],
        src.[loaddate],
        src.[filename],
        src.[ackfilename],
        src.[transactionid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertIrauctionSraPrudentialCashSecurity1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.IrauctionSraPrudentialCashSecurity1(
file_log_id,
[prudential_date],
        [prudential_runno],
        [participantid],
        [cash_security_id],
        [cash_security_amount]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertIrauctionSraPrudentialCompPosition1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.IrauctionSraPrudentialCompPosition1(
file_log_id,
[prudential_date],
        [prudential_runno],
        [participantid],
        [trading_limit],
        [prudential_exposure_amount],
        [trading_margin]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertIrauctionSraPrudentialExposure1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.IrauctionSraPrudentialExposure1(
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
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertIrauctionSraPrudentialRun1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.IrauctionSraPrudentialRun1(
file_log_id,
[prudential_date],
        [prudential_runno]
)
select 
@file_log_id,
d.[prudential_date],
        d.[prudential_runno]
from openjson(@data) with (
[prudential_date] datetime2,
        [prudential_runno] decimal(8,0)
) d
end
go
create or alter procedure mmsdm_proc.InsertIrauctionValuationid1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.IrauctionValuationid1 as tgt 
using (
    select 
        d.[valuationid],
        d.[description],
        d.[lastchanged]
    from openjson(@data) with (
        [valuationid] varchar(15),
        [description] varchar(80),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[valuationid] = src.[valuationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[valuationid] = src.[valuationid],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [valuationid],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[valuationid],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigBidtypes1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigBidtypes1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[bidtype] = src.[bidtype],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[description] = src.[description],
        tgt.[numberofbands] = src.[numberofbands],
        tgt.[numdaysaheadpricelocked] = src.[numdaysaheadpricelocked],
        tgt.[validationrule] = src.[validationrule],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[spdalias] = src.[spdalias]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[bidtype],
        src.[effectivedate],
        src.[versionno],
        src.[description],
        src.[numberofbands],
        src.[numdaysaheadpricelocked],
        src.[validationrule],
        src.[lastchanged],
        src.[spdalias]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigBidtypestrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigBidtypestrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigFcasregusefactors1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigFcasregusefactors1 as tgt 
using (
    select 
        d.[effectivedate],
        d.[versionno],
        d.[regionid],
        d.[bidtype],
        d.[periodid],
        d.[usage_factor],
        d.[lastchanged]
    from openjson(@data) with (
        [effectivedate] datetime2,
        [versionno] decimal(3,0),
        [regionid] varchar(20),
        [bidtype] varchar(20),
        [periodid] decimal(3,0),
        [usage_factor] decimal(8,3),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[regionid] = src.[regionid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[periodid] = src.[periodid],
        tgt.[usage_factor] = src.[usage_factor],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [regionid],
        [bidtype],
        [periodid],
        [usage_factor],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[regionid],
        src.[bidtype],
        src.[periodid],
        src.[usage_factor],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigFcasregusefactorsTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigFcasregusefactorsTrk1 as tgt 
using (
    select 
        d.[effectivedate],
        d.[versionno],
        d.[authoriseddate],
        d.[lastchanged]
    from openjson(@data) with (
        [effectivedate] datetime2,
        [versionno] decimal(3,0),
        [authoriseddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigInterconnector1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigInterconnector1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[regionfrom] = src.[regionfrom],
        tgt.[rsoid] = src.[rsoid],
        tgt.[regionto] = src.[regionto],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [interconnectorid],
        [regionfrom],
        [rsoid],
        [regionto],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[interconnectorid],
        src.[regionfrom],
        src.[rsoid],
        src.[regionto],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigInterconnectoralloc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigInterconnectoralloc1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[regionid] = src.[regionid],
        tgt.[participantid] = src.[participantid],
        tgt.[allocation] = src.[allocation],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [interconnectorid],
        [regionid],
        [participantid],
        [allocation],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[interconnectorid],
        src.[regionid],
        src.[participantid],
        src.[allocation],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigInterconnectorconstraint1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigInterconnectorconstraint1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[reserveoverallloadfactor] = src.[reserveoverallloadfactor],
        tgt.[fromregionlossshare] = src.[fromregionlossshare],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[maxmwin] = src.[maxmwin],
        tgt.[maxmwout] = src.[maxmwout],
        tgt.[lossconstant] = src.[lossconstant],
        tgt.[lossflowcoefficient] = src.[lossflowcoefficient],
        tgt.[emsmeasurand] = src.[emsmeasurand],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[dynamicrhs] = src.[dynamicrhs],
        tgt.[importlimit] = src.[importlimit],
        tgt.[exportlimit] = src.[exportlimit],
        tgt.[outagederationfactor] = src.[outagederationfactor],
        tgt.[nonphysicallossfactor] = src.[nonphysicallossfactor],
        tgt.[overloadfactor60sec] = src.[overloadfactor60sec],
        tgt.[overloadfactor6sec] = src.[overloadfactor6sec],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[fcassupportunavailable] = src.[fcassupportunavailable],
        tgt.[ictype] = src.[ictype]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[reserveoverallloadfactor],
        src.[fromregionlossshare],
        src.[effectivedate],
        src.[versionno],
        src.[interconnectorid],
        src.[maxmwin],
        src.[maxmwout],
        src.[lossconstant],
        src.[lossflowcoefficient],
        src.[emsmeasurand],
        src.[authorisedby],
        src.[authoriseddate],
        src.[dynamicrhs],
        src.[importlimit],
        src.[exportlimit],
        src.[outagederationfactor],
        src.[nonphysicallossfactor],
        src.[overloadfactor60sec],
        src.[overloadfactor6sec],
        src.[lastchanged],
        src.[fcassupportunavailable],
        src.[ictype]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigIntraregionalloc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigIntraregionalloc1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[regionid] = src.[regionid],
        tgt.[participantid] = src.[participantid],
        tgt.[allocation] = src.[allocation],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [regionid],
        [participantid],
        [allocation],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[regionid],
        src.[participantid],
        src.[allocation],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigLossfactormodel1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigLossfactormodel1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[regionid] = src.[regionid],
        tgt.[demandcoefficient] = src.[demandcoefficient],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [interconnectorid],
        [regionid],
        [demandcoefficient],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[interconnectorid],
        src.[regionid],
        src.[demandcoefficient],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigLossmodel1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigLossmodel1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[losssegment] = src.[losssegment]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[periodid] = src.[periodid],
        tgt.[losssegment] = src.[losssegment],
        tgt.[mwbreakpoint] = src.[mwbreakpoint],
        tgt.[lossfactor] = src.[lossfactor],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [interconnectorid],
        [periodid],
        [losssegment],
        [mwbreakpoint],
        [lossfactor],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[interconnectorid],
        src.[periodid],
        src.[losssegment],
        src.[mwbreakpoint],
        src.[lossfactor],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigMarketPriceThresholds1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigMarketPriceThresholds1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[voll] = src.[voll],
        tgt.[marketpricefloor] = src.[marketpricefloor],
        tgt.[administered_price_threshold] = src.[administered_price_threshold],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [voll],
        [marketpricefloor],
        [administered_price_threshold],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[voll],
        src.[marketpricefloor],
        src.[administered_price_threshold],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigRegion1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigRegion1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[regionid] = src.[regionid],
        tgt.[description] = src.[description],
        tgt.[regionstatus] = src.[regionstatus],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [regionid],
        [description],
        [regionstatus],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[regionid],
        src.[description],
        src.[regionstatus],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigRegionstandingdata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigRegionstandingdata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[regionid] = src.[regionid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[regionid] = src.[regionid],
        tgt.[rsoid] = src.[rsoid],
        tgt.[regionalreferencepointid] = src.[regionalreferencepointid],
        tgt.[peaktradingperiod] = src.[peaktradingperiod],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[scalingfactor] = src.[scalingfactor],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[regionid],
        src.[rsoid],
        src.[regionalreferencepointid],
        src.[peaktradingperiod],
        src.[authoriseddate],
        src.[authorisedby],
        src.[scalingfactor],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketConfigTransmissionlossfactor2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketConfigTransmissionlossfactor2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[transmissionlossfactor] = src.[transmissionlossfactor],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[regionid] = src.[regionid],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[secondary_tlf] = src.[secondary_tlf]
when not matched
    then insert (
        file_log_id,
        [transmissionlossfactor],
        [effectivedate],
        [versionno],
        [connectionpointid],
        [regionid],
        [lastchanged],
        [secondary_tlf]
    ) values (
        @file_log_id,
        src.[transmissionlossfactor],
        src.[effectivedate],
        src.[versionno],
        src.[connectionpointid],
        src.[regionid],
        src.[lastchanged],
        src.[secondary_tlf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketNoticeMarketnoticedata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketNoticeMarketnoticedata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[noticeid] = src.[noticeid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[noticeid] = src.[noticeid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[typeid] = src.[typeid],
        tgt.[noticetype] = src.[noticetype],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[reason] = src.[reason],
        tgt.[externalreference] = src.[externalreference]
when not matched
    then insert (
        file_log_id,
        [noticeid],
        [effectivedate],
        [typeid],
        [noticetype],
        [lastchanged],
        [reason],
        [externalreference]
    ) values (
        @file_log_id,
        src.[noticeid],
        src.[effectivedate],
        src.[typeid],
        src.[noticetype],
        src.[lastchanged],
        src.[reason],
        src.[externalreference]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketNoticeMarketnoticetype1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketNoticeMarketnoticetype1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[typeid] = src.[typeid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[typeid] = src.[typeid],
        tgt.[description] = src.[description],
        tgt.[raisedby] = src.[raisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [typeid],
        [description],
        [raisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[typeid],
        src.[description],
        src.[raisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMarketNoticeParticipantnoticetrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MarketNoticeParticipantnoticetrk1 as tgt 
using (
    select 
        d.[participantid],
        d.[noticeid],
        d.[lastchanged]
    from openjson(@data) with (
        [participantid] varchar(10),
        [noticeid] decimal(10,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[noticeid] = src.[noticeid]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[noticeid] = src.[noticeid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [noticeid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[noticeid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMccCasesolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.MccCasesolution1(
file_log_id,
[run_datetime]
)
select 
@file_log_id,
d.[run_datetime]
from openjson(@data) with (
[run_datetime] datetime2
) d
end
go
create or alter procedure mmsdm_proc.InsertMccConstraintsolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.MccConstraintsolution1(
file_log_id,
[run_datetime],
        [constraintid],
        [rhs],
        [marginalvalue]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertMeterdataAggregateReads1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterdataAggregateReads1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[case_id] = src.[case_id]
    and tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[frmp] = src.[frmp]
    and tgt.[lr] = src.[lr]
    and tgt.[meter_type] = src.[meter_type]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[case_id] = src.[case_id],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[meter_type] = src.[meter_type],
        tgt.[frmp] = src.[frmp],
        tgt.[lr] = src.[lr],
        tgt.[periodid] = src.[periodid],
        tgt.[importvalue] = src.[importvalue],
        tgt.[exportvalue] = src.[exportvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[case_id],
        src.[settlementdate],
        src.[connectionpointid],
        src.[meter_type],
        src.[frmp],
        src.[lr],
        src.[periodid],
        src.[importvalue],
        src.[exportvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterdataIndividualReads1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterdataIndividualReads1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[case_id] = src.[case_id]
    and tgt.[meter_id] = src.[meter_id]
    and tgt.[meter_id_suffix] = src.[meter_id_suffix]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[case_id] = src.[case_id],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[meter_id] = src.[meter_id],
        tgt.[meter_id_suffix] = src.[meter_id_suffix],
        tgt.[frmp] = src.[frmp],
        tgt.[lr] = src.[lr],
        tgt.[periodid] = src.[periodid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[meter_type] = src.[meter_type],
        tgt.[importvalue] = src.[importvalue],
        tgt.[exportvalue] = src.[exportvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[case_id],
        src.[settlementdate],
        src.[meter_id],
        src.[meter_id_suffix],
        src.[frmp],
        src.[lr],
        src.[periodid],
        src.[connectionpointid],
        src.[meter_type],
        src.[importvalue],
        src.[exportvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterdataInterconnector1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterdataInterconnector1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[case_id] = src.[case_id]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[case_id] = src.[case_id],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[periodid] = src.[periodid],
        tgt.[importvalue] = src.[importvalue],
        tgt.[exportvalue] = src.[exportvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [case_id],
        [settlementdate],
        [interconnectorid],
        [periodid],
        [importvalue],
        [exportvalue],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[case_id],
        src.[settlementdate],
        src.[interconnectorid],
        src.[periodid],
        src.[importvalue],
        src.[exportvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterdataMeterdataSaps1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MeterdataMeterdataSaps1 as tgt 
using (
    select 
        d.[case_id],
        d.[settlementdate],
        d.[connectionpoint_id],
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
        [connectionpoint_id] varchar(20),
        [meter_type] varchar(20),
        [frmp] varchar(20),
        [lr] varchar(20),
        [periodid] decimal(4,0),
        [importvalue] decimal(18,8),
        [exportvalue] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[case_id] = src.[case_id]
    and tgt.[connectionpoint_id] = src.[connectionpoint_id]
    and tgt.[frmp] = src.[frmp]
    and tgt.[lr] = src.[lr]
    and tgt.[meter_type] = src.[meter_type]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[case_id] = src.[case_id],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[connectionpoint_id] = src.[connectionpoint_id],
        tgt.[meter_type] = src.[meter_type],
        tgt.[frmp] = src.[frmp],
        tgt.[lr] = src.[lr],
        tgt.[periodid] = src.[periodid],
        tgt.[importvalue] = src.[importvalue],
        tgt.[exportvalue] = src.[exportvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [case_id],
        [settlementdate],
        [connectionpoint_id],
        [meter_type],
        [frmp],
        [lr],
        [periodid],
        [importvalue],
        [exportvalue],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[case_id],
        src.[settlementdate],
        src.[connectionpoint_id],
        src.[meter_type],
        src.[frmp],
        src.[lr],
        src.[periodid],
        src.[importvalue],
        src.[exportvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMeterdataWdrReads1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.MeterdataWdrReads1(
file_log_id,
[market_id],
        [case_id],
        [settlementdate],
        [meter_id],
        [tni],
        [frmp],
        [drsp],
        [periodid],
        [meteredquantityimport],
        [meteredquantityexport],
        [baselinequantity],
        [qualityflag],
        [isnoncompliant],
        [baselinecalculationid]
)
select 
@file_log_id,
d.[market_id],
        d.[case_id],
        d.[settlementdate],
        d.[meter_id],
        d.[tni],
        d.[frmp],
        d.[drsp],
        d.[periodid],
        d.[meteredquantityimport],
        d.[meteredquantityexport],
        d.[baselinequantity],
        d.[qualityflag],
        d.[isnoncompliant],
        d.[baselinecalculationid]
from openjson(@data) with (
[market_id] varchar(20),
        [case_id] decimal(15,0),
        [settlementdate] datetime2,
        [meter_id] varchar(20),
        [tni] varchar(20),
        [frmp] varchar(20),
        [drsp] varchar(20),
        [periodid] decimal(3,0),
        [meteredquantityimport] decimal(18,8),
        [meteredquantityexport] decimal(18,8),
        [baselinequantity] decimal(18,8),
        [qualityflag] varchar(20),
        [isnoncompliant] decimal(1,0),
        [baselinecalculationid] varchar(100)
) d
end
go
create or alter procedure mmsdm_proc.InsertMtpasaCaseresult1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaCaseresult1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[plexos_version],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [plexos_version] varchar(20),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[plexos_version] = src.[plexos_version],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [plexos_version],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[plexos_version],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaConstraintresult1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaConstraintresult1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[demand_poe_type],
        d.[day],
        d.[constraintid],
        d.[effectivedate],
        d.[versionno],
        d.[periodid],
        d.[probabilityofbinding],
        d.[probabilityofviolation],
        d.[constraintviolation90],
        d.[constraintviolation50],
        d.[constraintviolation10],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [demand_poe_type] varchar(20),
        [day] datetime2,
        [constraintid] varchar(20),
        [effectivedate] datetime2,
        [versionno] decimal(3,0),
        [periodid] decimal(3,0),
        [probabilityofbinding] decimal(8,5),
        [probabilityofviolation] decimal(8,5),
        [constraintviolation90] decimal(12,2),
        [constraintviolation50] decimal(12,2),
        [constraintviolation10] decimal(12,2),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[day] = src.[day]
    and tgt.[demand_poe_type] = src.[demand_poe_type]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[demand_poe_type] = src.[demand_poe_type],
        tgt.[day] = src.[day],
        tgt.[constraintid] = src.[constraintid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[probabilityofbinding] = src.[probabilityofbinding],
        tgt.[probabilityofviolation] = src.[probabilityofviolation],
        tgt.[constraintviolation90] = src.[constraintviolation90],
        tgt.[constraintviolation50] = src.[constraintviolation50],
        tgt.[constraintviolation10] = src.[constraintviolation10],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [demand_poe_type],
        [day],
        [constraintid],
        [effectivedate],
        [versionno],
        [periodid],
        [probabilityofbinding],
        [probabilityofviolation],
        [constraintviolation90],
        [constraintviolation50],
        [constraintviolation10],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[demand_poe_type],
        src.[day],
        src.[constraintid],
        src.[effectivedate],
        src.[versionno],
        src.[periodid],
        src.[probabilityofbinding],
        src.[probabilityofviolation],
        src.[constraintviolation90],
        src.[constraintviolation50],
        src.[constraintviolation10],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaConstraintsummary1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaConstraintsummary1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[demand_poe_type],
        d.[day],
        d.[constraintid],
        d.[effectivedate],
        d.[versionno],
        d.[aggregation_period],
        d.[constrainthoursbinding],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [demand_poe_type] varchar(20),
        [day] datetime2,
        [constraintid] varchar(20),
        [effectivedate] datetime2,
        [versionno] decimal(3,0),
        [aggregation_period] varchar(20),
        [constrainthoursbinding] decimal(12,2),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[aggregation_period] = src.[aggregation_period]
    and tgt.[constraintid] = src.[constraintid]
    and tgt.[day] = src.[day]
    and tgt.[demand_poe_type] = src.[demand_poe_type]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[demand_poe_type] = src.[demand_poe_type],
        tgt.[day] = src.[day],
        tgt.[constraintid] = src.[constraintid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[aggregation_period] = src.[aggregation_period],
        tgt.[constrainthoursbinding] = src.[constrainthoursbinding],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [demand_poe_type],
        [day],
        [constraintid],
        [effectivedate],
        [versionno],
        [aggregation_period],
        [constrainthoursbinding],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[demand_poe_type],
        src.[day],
        src.[constraintid],
        src.[effectivedate],
        src.[versionno],
        src.[aggregation_period],
        src.[constrainthoursbinding],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaDuidavailability3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaDuidavailability3 as tgt 
using (
    select 
        d.[publish_datetime],
        d.[day],
        d.[regionid],
        d.[duid],
        d.[pasaavailability],
        d.[latest_offer_datetime],
        d.[lastchanged],
        d.[carryoverstatus],
        d.[pasaunitstate],
        d.[pasarecalltime]
    from openjson(@data) with (
        [publish_datetime] datetime2,
        [day] datetime2,
        [regionid] varchar(20),
        [duid] varchar(20),
        [pasaavailability] decimal(12,0),
        [latest_offer_datetime] datetime2,
        [lastchanged] datetime2,
        [carryoverstatus] decimal(1,0),
        [pasaunitstate] varchar(20),
        [pasarecalltime] decimal(4,0)
    ) d
) as src 
on (
    tgt.[day] = src.[day]
    and tgt.[duid] = src.[duid]
    and tgt.[publish_datetime] = src.[publish_datetime]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[publish_datetime] = src.[publish_datetime],
        tgt.[day] = src.[day],
        tgt.[regionid] = src.[regionid],
        tgt.[duid] = src.[duid],
        tgt.[pasaavailability] = src.[pasaavailability],
        tgt.[latest_offer_datetime] = src.[latest_offer_datetime],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[carryoverstatus] = src.[carryoverstatus],
        tgt.[pasaunitstate] = src.[pasaunitstate],
        tgt.[pasarecalltime] = src.[pasarecalltime]
when not matched
    then insert (
        file_log_id,
        [publish_datetime],
        [day],
        [regionid],
        [duid],
        [pasaavailability],
        [latest_offer_datetime],
        [lastchanged],
        [carryoverstatus],
        [pasaunitstate],
        [pasarecalltime]
    ) values (
        @file_log_id,
        src.[publish_datetime],
        src.[day],
        src.[regionid],
        src.[duid],
        src.[pasaavailability],
        src.[latest_offer_datetime],
        src.[lastchanged],
        src.[carryoverstatus],
        src.[pasaunitstate],
        src.[pasarecalltime]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaInterconnectorresult1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaInterconnectorresult1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[demand_poe_type],
        d.[day],
        d.[interconnectorid],
        d.[periodid],
        d.[flow90],
        d.[flow50],
        d.[flow10],
        d.[probabilityofbindingexport],
        d.[probabilityofbindingimport],
        d.[calculatedexportlimit],
        d.[calculatedimportlimit],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [demand_poe_type] varchar(20),
        [day] datetime2,
        [interconnectorid] varchar(20),
        [periodid] decimal(3,0),
        [flow90] decimal(12,2),
        [flow50] decimal(12,2),
        [flow10] decimal(12,2),
        [probabilityofbindingexport] decimal(8,5),
        [probabilityofbindingimport] decimal(8,5),
        [calculatedexportlimit] decimal(12,2),
        [calculatedimportlimit] decimal(12,2),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[day] = src.[day]
    and tgt.[demand_poe_type] = src.[demand_poe_type]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[demand_poe_type] = src.[demand_poe_type],
        tgt.[day] = src.[day],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[periodid] = src.[periodid],
        tgt.[flow90] = src.[flow90],
        tgt.[flow50] = src.[flow50],
        tgt.[flow10] = src.[flow10],
        tgt.[probabilityofbindingexport] = src.[probabilityofbindingexport],
        tgt.[probabilityofbindingimport] = src.[probabilityofbindingimport],
        tgt.[calculatedexportlimit] = src.[calculatedexportlimit],
        tgt.[calculatedimportlimit] = src.[calculatedimportlimit],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [demand_poe_type],
        [day],
        [interconnectorid],
        [periodid],
        [flow90],
        [flow50],
        [flow10],
        [probabilityofbindingexport],
        [probabilityofbindingimport],
        [calculatedexportlimit],
        [calculatedimportlimit],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[demand_poe_type],
        src.[day],
        src.[interconnectorid],
        src.[periodid],
        src.[flow90],
        src.[flow50],
        src.[flow10],
        src.[probabilityofbindingexport],
        src.[probabilityofbindingimport],
        src.[calculatedexportlimit],
        src.[calculatedimportlimit],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaLolpresult1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaLolpresult1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[day],
        d.[regionid],
        d.[worst_interval_periodid],
        d.[worst_interval_demand],
        d.[worst_interval_intgen],
        d.[worst_interval_dsp],
        d.[lossofloadprobability],
        d.[lossofloadmagnitude],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [day] datetime2,
        [regionid] varchar(20),
        [worst_interval_periodid] decimal(3,0),
        [worst_interval_demand] decimal(12,2),
        [worst_interval_intgen] decimal(12,2),
        [worst_interval_dsp] decimal(12,2),
        [lossofloadprobability] decimal(8,5),
        [lossofloadmagnitude] varchar(20),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[day] = src.[day]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[day] = src.[day],
        tgt.[regionid] = src.[regionid],
        tgt.[worst_interval_periodid] = src.[worst_interval_periodid],
        tgt.[worst_interval_demand] = src.[worst_interval_demand],
        tgt.[worst_interval_intgen] = src.[worst_interval_intgen],
        tgt.[worst_interval_dsp] = src.[worst_interval_dsp],
        tgt.[lossofloadprobability] = src.[lossofloadprobability],
        tgt.[lossofloadmagnitude] = src.[lossofloadmagnitude],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [day],
        [regionid],
        [worst_interval_periodid],
        [worst_interval_demand],
        [worst_interval_intgen],
        [worst_interval_dsp],
        [lossofloadprobability],
        [lossofloadmagnitude],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[day],
        src.[regionid],
        src.[worst_interval_periodid],
        src.[worst_interval_demand],
        src.[worst_interval_intgen],
        src.[worst_interval_dsp],
        src.[lossofloadprobability],
        src.[lossofloadmagnitude],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaRegionavailability4
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaRegionavailability4 as tgt 
using (
    select 
        d.[publish_datetime],
        d.[day],
        d.[regionid],
        d.[pasaavailability_scheduled],
        d.[latest_offer_datetime],
        d.[energyunconstrainedcapacity],
        d.[energyconstrainedcapacity],
        d.[nonscheduledgeneration],
        d.[demand10],
        d.[demand50],
        d.[energyreqdemand10],
        d.[energyreqdemand50],
        d.[lastchanged],
        d.[demand10min],
        d.[demand10max],
        d.[demand50min],
        d.[demand50max],
        d.[carryovercapacity]
    from openjson(@data) with (
        [publish_datetime] datetime2,
        [day] datetime2,
        [regionid] varchar(20),
        [pasaavailability_scheduled] decimal(12,0),
        [latest_offer_datetime] datetime2,
        [energyunconstrainedcapacity] decimal(12,0),
        [energyconstrainedcapacity] decimal(12,0),
        [nonscheduledgeneration] decimal(12,2),
        [demand10] decimal(12,2),
        [demand50] decimal(12,2),
        [energyreqdemand10] decimal(12,2),
        [energyreqdemand50] decimal(12,2),
        [lastchanged] datetime2,
        [demand10min] decimal(12,2),
        [demand10max] decimal(12,2),
        [demand50min] decimal(12,2),
        [demand50max] decimal(12,2),
        [carryovercapacity] decimal(12,0)
    ) d
) as src 
on (
    tgt.[day] = src.[day]
    and tgt.[publish_datetime] = src.[publish_datetime]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[publish_datetime] = src.[publish_datetime],
        tgt.[day] = src.[day],
        tgt.[regionid] = src.[regionid],
        tgt.[pasaavailability_scheduled] = src.[pasaavailability_scheduled],
        tgt.[latest_offer_datetime] = src.[latest_offer_datetime],
        tgt.[energyunconstrainedcapacity] = src.[energyunconstrainedcapacity],
        tgt.[energyconstrainedcapacity] = src.[energyconstrainedcapacity],
        tgt.[nonscheduledgeneration] = src.[nonscheduledgeneration],
        tgt.[demand10] = src.[demand10],
        tgt.[demand50] = src.[demand50],
        tgt.[energyreqdemand10] = src.[energyreqdemand10],
        tgt.[energyreqdemand50] = src.[energyreqdemand50],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[demand10min] = src.[demand10min],
        tgt.[demand10max] = src.[demand10max],
        tgt.[demand50min] = src.[demand50min],
        tgt.[demand50max] = src.[demand50max],
        tgt.[carryovercapacity] = src.[carryovercapacity]
when not matched
    then insert (
        file_log_id,
        [publish_datetime],
        [day],
        [regionid],
        [pasaavailability_scheduled],
        [latest_offer_datetime],
        [energyunconstrainedcapacity],
        [energyconstrainedcapacity],
        [nonscheduledgeneration],
        [demand10],
        [demand50],
        [energyreqdemand10],
        [energyreqdemand50],
        [lastchanged],
        [demand10min],
        [demand10max],
        [demand50min],
        [demand50max],
        [carryovercapacity]
    ) values (
        @file_log_id,
        src.[publish_datetime],
        src.[day],
        src.[regionid],
        src.[pasaavailability_scheduled],
        src.[latest_offer_datetime],
        src.[energyunconstrainedcapacity],
        src.[energyconstrainedcapacity],
        src.[nonscheduledgeneration],
        src.[demand10],
        src.[demand50],
        src.[energyreqdemand10],
        src.[energyreqdemand50],
        src.[lastchanged],
        src.[demand10min],
        src.[demand10max],
        src.[demand50min],
        src.[demand50max],
        src.[carryovercapacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaRegionavailtrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.MtpasaRegionavailtrk1(
file_log_id,
[publish_datetime],
        [startdate],
        [enddate],
        [latest_offer_datetime]
)
select 
@file_log_id,
d.[publish_datetime],
        d.[startdate],
        d.[enddate],
        d.[latest_offer_datetime]
from openjson(@data) with (
[publish_datetime] datetime2,
        [startdate] datetime2,
        [enddate] datetime2,
        [latest_offer_datetime] datetime2
) d
end
go
create or alter procedure mmsdm_proc.InsertMtpasaRegioniteration1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaRegioniteration1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[demand_poe_type],
        d.[aggregation_period],
        d.[period_ending],
        d.[regionid],
        d.[use_iteration_id],
        d.[use_iteration_event_number],
        d.[use_iteration_event_average],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [demand_poe_type] varchar(20),
        [aggregation_period] varchar(20),
        [period_ending] datetime2,
        [regionid] varchar(20),
        [use_iteration_id] decimal(5,0),
        [use_iteration_event_number] decimal(12,2),
        [use_iteration_event_average] decimal(12,2),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[aggregation_period] = src.[aggregation_period]
    and tgt.[demand_poe_type] = src.[demand_poe_type]
    and tgt.[period_ending] = src.[period_ending]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]
    and tgt.[use_iteration_id] = src.[use_iteration_id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[demand_poe_type] = src.[demand_poe_type],
        tgt.[aggregation_period] = src.[aggregation_period],
        tgt.[period_ending] = src.[period_ending],
        tgt.[regionid] = src.[regionid],
        tgt.[use_iteration_id] = src.[use_iteration_id],
        tgt.[use_iteration_event_number] = src.[use_iteration_event_number],
        tgt.[use_iteration_event_average] = src.[use_iteration_event_average],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [demand_poe_type],
        [aggregation_period],
        [period_ending],
        [regionid],
        [use_iteration_id],
        [use_iteration_event_number],
        [use_iteration_event_average],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[demand_poe_type],
        src.[aggregation_period],
        src.[period_ending],
        src.[regionid],
        src.[use_iteration_id],
        src.[use_iteration_event_number],
        src.[use_iteration_event_average],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaRegionresult2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaRegionresult2 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[demand_poe_type],
        d.[day],
        d.[regionid],
        d.[periodid],
        d.[demand],
        d.[aggregateinstalledcapacity],
        d.[numberofiterations],
        d.[use_numberofiterations],
        d.[use_max],
        d.[use_upperquartile],
        d.[use_median],
        d.[use_lowerquartile],
        d.[use_min],
        d.[use_average],
        d.[use_event_average],
        d.[totalscheduledgen90],
        d.[totalscheduledgen50],
        d.[totalscheduledgen10],
        d.[totalintermittentgen90],
        d.[totalintermittentgen50],
        d.[totalintermittentgen10],
        d.[demandsideparticipation90],
        d.[demandsideparticipation50],
        d.[demandsideparticipation10],
        d.[lastchanged],
        d.[totalsemischedulegen90],
        d.[totalsemischedulegen50],
        d.[totalsemischedulegen10],
        d.[totalavailablegenmin],
        d.[totalavailablegen10],
        d.[totalavailablegen50],
        d.[totalavailablegen90],
        d.[totalavailablegenmax]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [demand_poe_type] varchar(20),
        [day] datetime2,
        [regionid] varchar(20),
        [periodid] decimal(3,0),
        [demand] decimal(12,2),
        [aggregateinstalledcapacity] decimal(12,2),
        [numberofiterations] decimal(12,2),
        [use_numberofiterations] decimal(12,2),
        [use_max] decimal(12,2),
        [use_upperquartile] decimal(12,2),
        [use_median] decimal(12,2),
        [use_lowerquartile] decimal(12,2),
        [use_min] decimal(12,2),
        [use_average] decimal(12,2),
        [use_event_average] decimal(12,2),
        [totalscheduledgen90] decimal(12,2),
        [totalscheduledgen50] decimal(12,2),
        [totalscheduledgen10] decimal(12,2),
        [totalintermittentgen90] decimal(12,2),
        [totalintermittentgen50] decimal(12,2),
        [totalintermittentgen10] decimal(12,2),
        [demandsideparticipation90] decimal(12,2),
        [demandsideparticipation50] decimal(12,2),
        [demandsideparticipation10] decimal(12,2),
        [lastchanged] datetime2,
        [totalsemischedulegen90] decimal(12,2),
        [totalsemischedulegen50] decimal(12,2),
        [totalsemischedulegen10] decimal(12,2),
        [totalavailablegenmin] decimal(12,2),
        [totalavailablegen10] decimal(12,2),
        [totalavailablegen50] decimal(12,2),
        [totalavailablegen90] decimal(12,2),
        [totalavailablegenmax] decimal(12,2)
    ) d
) as src 
on (
    tgt.[day] = src.[day]
    and tgt.[demand_poe_type] = src.[demand_poe_type]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[demand_poe_type] = src.[demand_poe_type],
        tgt.[day] = src.[day],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[demand] = src.[demand],
        tgt.[aggregateinstalledcapacity] = src.[aggregateinstalledcapacity],
        tgt.[numberofiterations] = src.[numberofiterations],
        tgt.[use_numberofiterations] = src.[use_numberofiterations],
        tgt.[use_max] = src.[use_max],
        tgt.[use_upperquartile] = src.[use_upperquartile],
        tgt.[use_median] = src.[use_median],
        tgt.[use_lowerquartile] = src.[use_lowerquartile],
        tgt.[use_min] = src.[use_min],
        tgt.[use_average] = src.[use_average],
        tgt.[use_event_average] = src.[use_event_average],
        tgt.[totalscheduledgen90] = src.[totalscheduledgen90],
        tgt.[totalscheduledgen50] = src.[totalscheduledgen50],
        tgt.[totalscheduledgen10] = src.[totalscheduledgen10],
        tgt.[totalintermittentgen90] = src.[totalintermittentgen90],
        tgt.[totalintermittentgen50] = src.[totalintermittentgen50],
        tgt.[totalintermittentgen10] = src.[totalintermittentgen10],
        tgt.[demandsideparticipation90] = src.[demandsideparticipation90],
        tgt.[demandsideparticipation50] = src.[demandsideparticipation50],
        tgt.[demandsideparticipation10] = src.[demandsideparticipation10],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[totalsemischedulegen90] = src.[totalsemischedulegen90],
        tgt.[totalsemischedulegen50] = src.[totalsemischedulegen50],
        tgt.[totalsemischedulegen10] = src.[totalsemischedulegen10],
        tgt.[totalavailablegenmin] = src.[totalavailablegenmin],
        tgt.[totalavailablegen10] = src.[totalavailablegen10],
        tgt.[totalavailablegen50] = src.[totalavailablegen50],
        tgt.[totalavailablegen90] = src.[totalavailablegen90],
        tgt.[totalavailablegenmax] = src.[totalavailablegenmax]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [demand_poe_type],
        [day],
        [regionid],
        [periodid],
        [demand],
        [aggregateinstalledcapacity],
        [numberofiterations],
        [use_numberofiterations],
        [use_max],
        [use_upperquartile],
        [use_median],
        [use_lowerquartile],
        [use_min],
        [use_average],
        [use_event_average],
        [totalscheduledgen90],
        [totalscheduledgen50],
        [totalscheduledgen10],
        [totalintermittentgen90],
        [totalintermittentgen50],
        [totalintermittentgen10],
        [demandsideparticipation90],
        [demandsideparticipation50],
        [demandsideparticipation10],
        [lastchanged],
        [totalsemischedulegen90],
        [totalsemischedulegen50],
        [totalsemischedulegen10],
        [totalavailablegenmin],
        [totalavailablegen10],
        [totalavailablegen50],
        [totalavailablegen90],
        [totalavailablegenmax]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[demand_poe_type],
        src.[day],
        src.[regionid],
        src.[periodid],
        src.[demand],
        src.[aggregateinstalledcapacity],
        src.[numberofiterations],
        src.[use_numberofiterations],
        src.[use_max],
        src.[use_upperquartile],
        src.[use_median],
        src.[use_lowerquartile],
        src.[use_min],
        src.[use_average],
        src.[use_event_average],
        src.[totalscheduledgen90],
        src.[totalscheduledgen50],
        src.[totalscheduledgen10],
        src.[totalintermittentgen90],
        src.[totalintermittentgen50],
        src.[totalintermittentgen10],
        src.[demandsideparticipation90],
        src.[demandsideparticipation50],
        src.[demandsideparticipation10],
        src.[lastchanged],
        src.[totalsemischedulegen90],
        src.[totalsemischedulegen50],
        src.[totalsemischedulegen10],
        src.[totalavailablegenmin],
        src.[totalavailablegen10],
        src.[totalavailablegen50],
        src.[totalavailablegen90],
        src.[totalavailablegenmax]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaRegionsummary1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaRegionsummary1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[run_no],
        d.[runtype],
        d.[demand_poe_type],
        d.[aggregation_period],
        d.[period_ending],
        d.[regionid],
        d.[nativedemand],
        d.[use_percentile10],
        d.[use_percentile20],
        d.[use_percentile30],
        d.[use_percentile40],
        d.[use_percentile50],
        d.[use_percentile60],
        d.[use_percentile70],
        d.[use_percentile80],
        d.[use_percentile90],
        d.[use_percentile100],
        d.[use_average],
        d.[numberofiterations],
        d.[use_numberofiterations],
        d.[use_event_max],
        d.[use_event_upperquartile],
        d.[use_event_median],
        d.[use_event_lowerquartile],
        d.[use_event_min],
        d.[weight],
        d.[use_weighted_avg],
        d.[lrc],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [run_no] decimal(4,0),
        [runtype] varchar(20),
        [demand_poe_type] varchar(20),
        [aggregation_period] varchar(20),
        [period_ending] datetime2,
        [regionid] varchar(20),
        [nativedemand] decimal(12,2),
        [use_percentile10] decimal(12,2),
        [use_percentile20] decimal(12,2),
        [use_percentile30] decimal(12,2),
        [use_percentile40] decimal(12,2),
        [use_percentile50] decimal(12,2),
        [use_percentile60] decimal(12,2),
        [use_percentile70] decimal(12,2),
        [use_percentile80] decimal(12,2),
        [use_percentile90] decimal(12,2),
        [use_percentile100] decimal(12,2),
        [use_average] decimal(12,2),
        [numberofiterations] decimal(12,2),
        [use_numberofiterations] decimal(12,2),
        [use_event_max] decimal(12,2),
        [use_event_upperquartile] decimal(12,2),
        [use_event_median] decimal(12,2),
        [use_event_lowerquartile] decimal(12,2),
        [use_event_min] decimal(12,2),
        [weight] decimal(16,6),
        [use_weighted_avg] decimal(16,6),
        [lrc] decimal(12,2),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[aggregation_period] = src.[aggregation_period]
    and tgt.[demand_poe_type] = src.[demand_poe_type]
    and tgt.[period_ending] = src.[period_ending]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[run_no] = src.[run_no]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[run_no] = src.[run_no],
        tgt.[runtype] = src.[runtype],
        tgt.[demand_poe_type] = src.[demand_poe_type],
        tgt.[aggregation_period] = src.[aggregation_period],
        tgt.[period_ending] = src.[period_ending],
        tgt.[regionid] = src.[regionid],
        tgt.[nativedemand] = src.[nativedemand],
        tgt.[use_percentile10] = src.[use_percentile10],
        tgt.[use_percentile20] = src.[use_percentile20],
        tgt.[use_percentile30] = src.[use_percentile30],
        tgt.[use_percentile40] = src.[use_percentile40],
        tgt.[use_percentile50] = src.[use_percentile50],
        tgt.[use_percentile60] = src.[use_percentile60],
        tgt.[use_percentile70] = src.[use_percentile70],
        tgt.[use_percentile80] = src.[use_percentile80],
        tgt.[use_percentile90] = src.[use_percentile90],
        tgt.[use_percentile100] = src.[use_percentile100],
        tgt.[use_average] = src.[use_average],
        tgt.[numberofiterations] = src.[numberofiterations],
        tgt.[use_numberofiterations] = src.[use_numberofiterations],
        tgt.[use_event_max] = src.[use_event_max],
        tgt.[use_event_upperquartile] = src.[use_event_upperquartile],
        tgt.[use_event_median] = src.[use_event_median],
        tgt.[use_event_lowerquartile] = src.[use_event_lowerquartile],
        tgt.[use_event_min] = src.[use_event_min],
        tgt.[weight] = src.[weight],
        tgt.[use_weighted_avg] = src.[use_weighted_avg],
        tgt.[lrc] = src.[lrc],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [run_no],
        [runtype],
        [demand_poe_type],
        [aggregation_period],
        [period_ending],
        [regionid],
        [nativedemand],
        [use_percentile10],
        [use_percentile20],
        [use_percentile30],
        [use_percentile40],
        [use_percentile50],
        [use_percentile60],
        [use_percentile70],
        [use_percentile80],
        [use_percentile90],
        [use_percentile100],
        [use_average],
        [numberofiterations],
        [use_numberofiterations],
        [use_event_max],
        [use_event_upperquartile],
        [use_event_median],
        [use_event_lowerquartile],
        [use_event_min],
        [weight],
        [use_weighted_avg],
        [lrc],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[run_no],
        src.[runtype],
        src.[demand_poe_type],
        src.[aggregation_period],
        src.[period_ending],
        src.[regionid],
        src.[nativedemand],
        src.[use_percentile10],
        src.[use_percentile20],
        src.[use_percentile30],
        src.[use_percentile40],
        src.[use_percentile50],
        src.[use_percentile60],
        src.[use_percentile70],
        src.[use_percentile80],
        src.[use_percentile90],
        src.[use_percentile100],
        src.[use_average],
        src.[numberofiterations],
        src.[use_numberofiterations],
        src.[use_event_max],
        src.[use_event_upperquartile],
        src.[use_event_median],
        src.[use_event_lowerquartile],
        src.[use_event_min],
        src.[weight],
        src.[use_weighted_avg],
        src.[lrc],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertNetworkEquipmentdetail2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.NetworkEquipmentdetail2 as tgt 
using (
    select 
        d.[substationid],
        d.[equipmenttype],
        d.[equipmentid],
        d.[validfrom],
        d.[validto],
        d.[voltage],
        d.[description],
        d.[lastchanged],
        d.[elementid]
    from openjson(@data) with (
        [substationid] varchar(30),
        [equipmenttype] varchar(10),
        [equipmentid] varchar(30),
        [validfrom] datetime2,
        [validto] datetime2,
        [voltage] varchar(20),
        [description] varchar(100),
        [lastchanged] datetime2,
        [elementid] decimal(15,0)
    ) d
) as src 
on (
    tgt.[elementid] = src.[elementid]
    and tgt.[equipmentid] = src.[equipmentid]
    and tgt.[equipmenttype] = src.[equipmenttype]
    and tgt.[substationid] = src.[substationid]
    and tgt.[validfrom] = src.[validfrom]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[substationid] = src.[substationid],
        tgt.[equipmenttype] = src.[equipmenttype],
        tgt.[equipmentid] = src.[equipmentid],
        tgt.[validfrom] = src.[validfrom],
        tgt.[validto] = src.[validto],
        tgt.[voltage] = src.[voltage],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[elementid] = src.[elementid]
when not matched
    then insert (
        file_log_id,
        [substationid],
        [equipmenttype],
        [equipmentid],
        [validfrom],
        [validto],
        [voltage],
        [description],
        [lastchanged],
        [elementid]
    ) values (
        @file_log_id,
        src.[substationid],
        src.[equipmenttype],
        src.[equipmentid],
        src.[validfrom],
        src.[validto],
        src.[voltage],
        src.[description],
        src.[lastchanged],
        src.[elementid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertNetworkOutageconstraintset1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.NetworkOutageconstraintset1(
file_log_id,
[outageid],
        [genconsetid],
        [startinterval],
        [endinterval]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertNetworkOutagedetail4
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.NetworkOutagedetail4 as tgt 
using (
    select 
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
        d.[companyrefcode],
        d.[elementid]
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
        [companyrefcode] varchar(20),
        [elementid] decimal(15,0)
    ) d
) as src 
on (
    tgt.[elementid] = src.[elementid]
    and tgt.[equipmentid] = src.[equipmentid]
    and tgt.[equipmenttype] = src.[equipmenttype]
    and tgt.[outageid] = src.[outageid]
    and tgt.[starttime] = src.[starttime]
    and tgt.[substationid] = src.[substationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[outageid] = src.[outageid],
        tgt.[substationid] = src.[substationid],
        tgt.[equipmenttype] = src.[equipmenttype],
        tgt.[equipmentid] = src.[equipmentid],
        tgt.[starttime] = src.[starttime],
        tgt.[endtime] = src.[endtime],
        tgt.[submitteddate] = src.[submitteddate],
        tgt.[outagestatuscode] = src.[outagestatuscode],
        tgt.[resubmitreason] = src.[resubmitreason],
        tgt.[resubmitoutageid] = src.[resubmitoutageid],
        tgt.[recalltimeday] = src.[recalltimeday],
        tgt.[recalltimenight] = src.[recalltimenight],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[reason] = src.[reason],
        tgt.[issecondary] = src.[issecondary],
        tgt.[actual_starttime] = src.[actual_starttime],
        tgt.[actual_endtime] = src.[actual_endtime],
        tgt.[companyrefcode] = src.[companyrefcode],
        tgt.[elementid] = src.[elementid]
when not matched
    then insert (
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
        [companyrefcode],
        [elementid]
    ) values (
        @file_log_id,
        src.[outageid],
        src.[substationid],
        src.[equipmenttype],
        src.[equipmentid],
        src.[starttime],
        src.[endtime],
        src.[submitteddate],
        src.[outagestatuscode],
        src.[resubmitreason],
        src.[resubmitoutageid],
        src.[recalltimeday],
        src.[recalltimenight],
        src.[lastchanged],
        src.[reason],
        src.[issecondary],
        src.[actual_starttime],
        src.[actual_endtime],
        src.[companyrefcode],
        src.[elementid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertNetworkOutagestatuscode1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.NetworkOutagestatuscode1 as tgt 
using (
    select 
        d.[outagestatuscode],
        d.[description],
        d.[lastchanged]
    from openjson(@data) with (
        [outagestatuscode] varchar(10),
        [description] varchar(100),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[outagestatuscode] = src.[outagestatuscode]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[outagestatuscode] = src.[outagestatuscode],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [outagestatuscode],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[outagestatuscode],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertNetworkRating1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.NetworkRating1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[spd_id] = src.[spd_id]
    and tgt.[validfrom] = src.[validfrom]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[spd_id] = src.[spd_id],
        tgt.[validfrom] = src.[validfrom],
        tgt.[validto] = src.[validto],
        tgt.[regionid] = src.[regionid],
        tgt.[substationid] = src.[substationid],
        tgt.[equipmenttype] = src.[equipmenttype],
        tgt.[equipmentid] = src.[equipmentid],
        tgt.[ratinglevel] = src.[ratinglevel],
        tgt.[isdynamic] = src.[isdynamic],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[spd_id],
        src.[validfrom],
        src.[validto],
        src.[regionid],
        src.[substationid],
        src.[equipmenttype],
        src.[equipmentid],
        src.[ratinglevel],
        src.[isdynamic],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertNetworkRealtimerating1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.NetworkRealtimerating1(
file_log_id,
[settlementdate],
        [spd_id],
        [ratingvalue]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertNetworkStaticrating1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.NetworkStaticrating1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[applicationid] = src.[applicationid]
    and tgt.[equipmentid] = src.[equipmentid]
    and tgt.[equipmenttype] = src.[equipmenttype]
    and tgt.[ratinglevel] = src.[ratinglevel]
    and tgt.[substationid] = src.[substationid]
    and tgt.[validfrom] = src.[validfrom]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[substationid] = src.[substationid],
        tgt.[equipmenttype] = src.[equipmenttype],
        tgt.[equipmentid] = src.[equipmentid],
        tgt.[ratinglevel] = src.[ratinglevel],
        tgt.[applicationid] = src.[applicationid],
        tgt.[validfrom] = src.[validfrom],
        tgt.[validto] = src.[validto],
        tgt.[ratingvalue] = src.[ratingvalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[substationid],
        src.[equipmenttype],
        src.[equipmentid],
        src.[ratinglevel],
        src.[applicationid],
        src.[validfrom],
        src.[validto],
        src.[ratingvalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertNetworkSubstationdetail2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.NetworkSubstationdetail2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[substationid] = src.[substationid]
    and tgt.[validfrom] = src.[validfrom]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[substationid] = src.[substationid],
        tgt.[validfrom] = src.[validfrom],
        tgt.[validto] = src.[validto],
        tgt.[description] = src.[description],
        tgt.[regionid] = src.[regionid],
        tgt.[ownerid] = src.[ownerid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [substationid],
        [validfrom],
        [validto],
        [description],
        [regionid],
        [ownerid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[substationid],
        src.[validfrom],
        src.[validto],
        src.[description],
        src.[regionid],
        src.[ownerid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minBlockedConstraints1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.P5minBlockedConstraints1(
file_log_id,
[run_datetime],
        [constraintid]
)
select 
@file_log_id,
d.[run_datetime],
        d.[constraintid]
from openjson(@data) with (
[run_datetime] datetime2,
        [constraintid] varchar(20)
) d
end
go
create or alter procedure mmsdm_proc.InsertP5minCasesolution2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minCasesolution2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[startinterval_datetime] = src.[startinterval_datetime],
        tgt.[totalobjective] = src.[totalobjective],
        tgt.[nonphysicallosses] = src.[nonphysicallosses],
        tgt.[totalareagenviolation] = src.[totalareagenviolation],
        tgt.[totalinterconnectorviolation] = src.[totalinterconnectorviolation],
        tgt.[totalgenericviolation] = src.[totalgenericviolation],
        tgt.[totalramprateviolation] = src.[totalramprateviolation],
        tgt.[totalunitmwcapacityviolation] = src.[totalunitmwcapacityviolation],
        tgt.[total5minviolation] = src.[total5minviolation],
        tgt.[totalregviolation] = src.[totalregviolation],
        tgt.[total6secviolation] = src.[total6secviolation],
        tgt.[total60secviolation] = src.[total60secviolation],
        tgt.[totalenergyconstrviolation] = src.[totalenergyconstrviolation],
        tgt.[totalenergyofferviolation] = src.[totalenergyofferviolation],
        tgt.[totalasprofileviolation] = src.[totalasprofileviolation],
        tgt.[totalfaststartviolation] = src.[totalfaststartviolation],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[intervention] = src.[intervention]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[startinterval_datetime],
        src.[totalobjective],
        src.[nonphysicallosses],
        src.[totalareagenviolation],
        src.[totalinterconnectorviolation],
        src.[totalgenericviolation],
        src.[totalramprateviolation],
        src.[totalunitmwcapacityviolation],
        src.[total5minviolation],
        src.[totalregviolation],
        src.[total6secviolation],
        src.[total60secviolation],
        src.[totalenergyconstrviolation],
        src.[totalenergyofferviolation],
        src.[totalasprofileviolation],
        src.[totalfaststartviolation],
        src.[lastchanged],
        src.[intervention]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minConstraintsolution6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minConstraintsolution6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[constraintid] = src.[constraintid],
        tgt.[rhs] = src.[rhs],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[duid] = src.[duid],
        tgt.[genconid_effectivedate] = src.[genconid_effectivedate],
        tgt.[genconid_versionno] = src.[genconid_versionno],
        tgt.[lhs] = src.[lhs],
        tgt.[intervention] = src.[intervention]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[constraintid],
        src.[rhs],
        src.[marginalvalue],
        src.[violationdegree],
        src.[lastchanged],
        src.[duid],
        src.[genconid_effectivedate],
        src.[genconid_versionno],
        src.[lhs],
        src.[intervention]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minFcasRequirment1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minFcasRequirment1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[interval_datetime],
        d.[constraintid],
        d.[regionid],
        d.[bidtype],
        d.[intervention],
        d.[constraint_effectivedate],
        d.[constraint_versionno],
        d.[marginalvalue],
        d.[base_cost],
        d.[adjusted_cost],
        d.[estimated_cmpf],
        d.[estimated_crmpf],
        d.[recovery_factor_cmpf],
        d.[recovery_factor_crmpf],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [interval_datetime] datetime2,
        [constraintid] varchar(20),
        [regionid] varchar(20),
        [bidtype] varchar(10),
        [intervention] decimal(2,0),
        [constraint_effectivedate] datetime2,
        [constraint_versionno] decimal(3,0),
        [marginalvalue] decimal(18,8),
        [base_cost] decimal(18,8),
        [adjusted_cost] decimal(18,8),
        [estimated_cmpf] decimal(18,8),
        [estimated_crmpf] decimal(18,8),
        [recovery_factor_cmpf] decimal(18,8),
        [recovery_factor_crmpf] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[constraintid] = src.[constraintid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[constraintid] = src.[constraintid],
        tgt.[regionid] = src.[regionid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[intervention] = src.[intervention],
        tgt.[constraint_effectivedate] = src.[constraint_effectivedate],
        tgt.[constraint_versionno] = src.[constraint_versionno],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[base_cost] = src.[base_cost],
        tgt.[adjusted_cost] = src.[adjusted_cost],
        tgt.[estimated_cmpf] = src.[estimated_cmpf],
        tgt.[estimated_crmpf] = src.[estimated_crmpf],
        tgt.[recovery_factor_cmpf] = src.[recovery_factor_cmpf],
        tgt.[recovery_factor_crmpf] = src.[recovery_factor_crmpf],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [interval_datetime],
        [constraintid],
        [regionid],
        [bidtype],
        [intervention],
        [constraint_effectivedate],
        [constraint_versionno],
        [marginalvalue],
        [base_cost],
        [adjusted_cost],
        [estimated_cmpf],
        [estimated_crmpf],
        [recovery_factor_cmpf],
        [recovery_factor_crmpf],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[constraintid],
        src.[regionid],
        src.[bidtype],
        src.[intervention],
        src.[constraint_effectivedate],
        src.[constraint_versionno],
        src.[marginalvalue],
        src.[base_cost],
        src.[adjusted_cost],
        src.[estimated_cmpf],
        src.[estimated_crmpf],
        src.[recovery_factor_cmpf],
        src.[recovery_factor_crmpf],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minInterconnectorsoln4
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minInterconnectorsoln4 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[meteredmwflow] = src.[meteredmwflow],
        tgt.[mwflow] = src.[mwflow],
        tgt.[mwlosses] = src.[mwlosses],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[mnsp] = src.[mnsp],
        tgt.[exportlimit] = src.[exportlimit],
        tgt.[importlimit] = src.[importlimit],
        tgt.[marginalloss] = src.[marginalloss],
        tgt.[exportgenconid] = src.[exportgenconid],
        tgt.[importgenconid] = src.[importgenconid],
        tgt.[fcasexportlimit] = src.[fcasexportlimit],
        tgt.[fcasimportlimit] = src.[fcasimportlimit],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[local_price_adjustment_export] = src.[local_price_adjustment_export],
        tgt.[locally_constrained_export] = src.[locally_constrained_export],
        tgt.[local_price_adjustment_import] = src.[local_price_adjustment_import],
        tgt.[locally_constrained_import] = src.[locally_constrained_import],
        tgt.[intervention] = src.[intervention]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interconnectorid],
        src.[interval_datetime],
        src.[meteredmwflow],
        src.[mwflow],
        src.[mwlosses],
        src.[marginalvalue],
        src.[violationdegree],
        src.[mnsp],
        src.[exportlimit],
        src.[importlimit],
        src.[marginalloss],
        src.[exportgenconid],
        src.[importgenconid],
        src.[fcasexportlimit],
        src.[fcasimportlimit],
        src.[lastchanged],
        src.[local_price_adjustment_export],
        src.[locally_constrained_export],
        src.[local_price_adjustment_import],
        src.[locally_constrained_import],
        src.[intervention]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minIntersensitivities1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minIntersensitivities1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[interconnectorid],
        d.[interval_datetime],
        d.[intervention],
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
        [run_datetime] datetime2,
        [interconnectorid] varchar(20),
        [interval_datetime] datetime2,
        [intervention] decimal(1,0),
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
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[intervention] = src.[intervention],
        tgt.[intervention_active] = src.[intervention_active],
        tgt.[mwflow1] = src.[mwflow1],
        tgt.[mwflow2] = src.[mwflow2],
        tgt.[mwflow3] = src.[mwflow3],
        tgt.[mwflow4] = src.[mwflow4],
        tgt.[mwflow5] = src.[mwflow5],
        tgt.[mwflow6] = src.[mwflow6],
        tgt.[mwflow7] = src.[mwflow7],
        tgt.[mwflow8] = src.[mwflow8],
        tgt.[mwflow9] = src.[mwflow9],
        tgt.[mwflow10] = src.[mwflow10],
        tgt.[mwflow11] = src.[mwflow11],
        tgt.[mwflow12] = src.[mwflow12],
        tgt.[mwflow13] = src.[mwflow13],
        tgt.[mwflow14] = src.[mwflow14],
        tgt.[mwflow15] = src.[mwflow15],
        tgt.[mwflow16] = src.[mwflow16],
        tgt.[mwflow17] = src.[mwflow17],
        tgt.[mwflow18] = src.[mwflow18],
        tgt.[mwflow19] = src.[mwflow19],
        tgt.[mwflow20] = src.[mwflow20],
        tgt.[mwflow21] = src.[mwflow21],
        tgt.[mwflow22] = src.[mwflow22],
        tgt.[mwflow23] = src.[mwflow23],
        tgt.[mwflow24] = src.[mwflow24],
        tgt.[mwflow25] = src.[mwflow25],
        tgt.[mwflow26] = src.[mwflow26],
        tgt.[mwflow27] = src.[mwflow27],
        tgt.[mwflow28] = src.[mwflow28],
        tgt.[mwflow29] = src.[mwflow29],
        tgt.[mwflow30] = src.[mwflow30],
        tgt.[mwflow31] = src.[mwflow31],
        tgt.[mwflow32] = src.[mwflow32],
        tgt.[mwflow33] = src.[mwflow33],
        tgt.[mwflow34] = src.[mwflow34],
        tgt.[mwflow35] = src.[mwflow35],
        tgt.[mwflow36] = src.[mwflow36],
        tgt.[mwflow37] = src.[mwflow37],
        tgt.[mwflow38] = src.[mwflow38],
        tgt.[mwflow39] = src.[mwflow39],
        tgt.[mwflow40] = src.[mwflow40],
        tgt.[mwflow41] = src.[mwflow41],
        tgt.[mwflow42] = src.[mwflow42],
        tgt.[mwflow43] = src.[mwflow43],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [interconnectorid],
        [interval_datetime],
        [intervention],
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interconnectorid],
        src.[interval_datetime],
        src.[intervention],
        src.[intervention_active],
        src.[mwflow1],
        src.[mwflow2],
        src.[mwflow3],
        src.[mwflow4],
        src.[mwflow5],
        src.[mwflow6],
        src.[mwflow7],
        src.[mwflow8],
        src.[mwflow9],
        src.[mwflow10],
        src.[mwflow11],
        src.[mwflow12],
        src.[mwflow13],
        src.[mwflow14],
        src.[mwflow15],
        src.[mwflow16],
        src.[mwflow17],
        src.[mwflow18],
        src.[mwflow19],
        src.[mwflow20],
        src.[mwflow21],
        src.[mwflow22],
        src.[mwflow23],
        src.[mwflow24],
        src.[mwflow25],
        src.[mwflow26],
        src.[mwflow27],
        src.[mwflow28],
        src.[mwflow29],
        src.[mwflow30],
        src.[mwflow31],
        src.[mwflow32],
        src.[mwflow33],
        src.[mwflow34],
        src.[mwflow35],
        src.[mwflow36],
        src.[mwflow37],
        src.[mwflow38],
        src.[mwflow39],
        src.[mwflow40],
        src.[mwflow41],
        src.[mwflow42],
        src.[mwflow43],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minLocalPrice1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.P5minLocalPrice1(
file_log_id,
[run_datetime],
        [interval_datetime],
        [duid],
        [local_price_adjustment],
        [locally_constrained]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertP5minPricesensitivities1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minPricesensitivities1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[regionid],
        d.[interval_datetime],
        d.[intervention],
        d.[intervention_active],
        d.[rrp1],
        d.[rrp2],
        d.[rrp3],
        d.[rrp4],
        d.[rrp5],
        d.[rrp6],
        d.[rrp7],
        d.[rrp8],
        d.[rrp9],
        d.[rrp10],
        d.[rrp11],
        d.[rrp12],
        d.[rrp13],
        d.[rrp14],
        d.[rrp15],
        d.[rrp16],
        d.[rrp17],
        d.[rrp18],
        d.[rrp19],
        d.[rrp20],
        d.[rrp21],
        d.[rrp22],
        d.[rrp23],
        d.[rrp24],
        d.[rrp25],
        d.[rrp26],
        d.[rrp27],
        d.[rrp28],
        d.[rrp29],
        d.[rrp30],
        d.[rrp31],
        d.[rrp32],
        d.[rrp33],
        d.[rrp34],
        d.[rrp35],
        d.[rrp36],
        d.[rrp37],
        d.[rrp38],
        d.[rrp39],
        d.[rrp40],
        d.[rrp41],
        d.[rrp42],
        d.[rrp43],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [regionid] varchar(20),
        [interval_datetime] datetime2,
        [intervention] decimal(1,0),
        [intervention_active] decimal(1,0),
        [rrp1] decimal(15,5),
        [rrp2] decimal(15,5),
        [rrp3] decimal(15,5),
        [rrp4] decimal(15,5),
        [rrp5] decimal(15,5),
        [rrp6] decimal(15,5),
        [rrp7] decimal(15,5),
        [rrp8] decimal(15,5),
        [rrp9] decimal(15,5),
        [rrp10] decimal(15,5),
        [rrp11] decimal(15,5),
        [rrp12] decimal(15,5),
        [rrp13] decimal(15,5),
        [rrp14] decimal(15,5),
        [rrp15] decimal(15,5),
        [rrp16] decimal(15,5),
        [rrp17] decimal(15,5),
        [rrp18] decimal(15,5),
        [rrp19] decimal(15,5),
        [rrp20] decimal(15,5),
        [rrp21] decimal(15,5),
        [rrp22] decimal(15,5),
        [rrp23] decimal(15,5),
        [rrp24] decimal(15,5),
        [rrp25] decimal(15,5),
        [rrp26] decimal(15,5),
        [rrp27] decimal(15,5),
        [rrp28] decimal(15,5),
        [rrp29] decimal(15,5),
        [rrp30] decimal(15,5),
        [rrp31] decimal(15,5),
        [rrp32] decimal(15,5),
        [rrp33] decimal(15,5),
        [rrp34] decimal(15,5),
        [rrp35] decimal(15,5),
        [rrp36] decimal(15,5),
        [rrp37] decimal(15,5),
        [rrp38] decimal(15,5),
        [rrp39] decimal(15,5),
        [rrp40] decimal(15,5),
        [rrp41] decimal(15,5),
        [rrp42] decimal(15,5),
        [rrp43] decimal(15,5),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[intervention] = src.[intervention],
        tgt.[intervention_active] = src.[intervention_active],
        tgt.[rrp1] = src.[rrp1],
        tgt.[rrp2] = src.[rrp2],
        tgt.[rrp3] = src.[rrp3],
        tgt.[rrp4] = src.[rrp4],
        tgt.[rrp5] = src.[rrp5],
        tgt.[rrp6] = src.[rrp6],
        tgt.[rrp7] = src.[rrp7],
        tgt.[rrp8] = src.[rrp8],
        tgt.[rrp9] = src.[rrp9],
        tgt.[rrp10] = src.[rrp10],
        tgt.[rrp11] = src.[rrp11],
        tgt.[rrp12] = src.[rrp12],
        tgt.[rrp13] = src.[rrp13],
        tgt.[rrp14] = src.[rrp14],
        tgt.[rrp15] = src.[rrp15],
        tgt.[rrp16] = src.[rrp16],
        tgt.[rrp17] = src.[rrp17],
        tgt.[rrp18] = src.[rrp18],
        tgt.[rrp19] = src.[rrp19],
        tgt.[rrp20] = src.[rrp20],
        tgt.[rrp21] = src.[rrp21],
        tgt.[rrp22] = src.[rrp22],
        tgt.[rrp23] = src.[rrp23],
        tgt.[rrp24] = src.[rrp24],
        tgt.[rrp25] = src.[rrp25],
        tgt.[rrp26] = src.[rrp26],
        tgt.[rrp27] = src.[rrp27],
        tgt.[rrp28] = src.[rrp28],
        tgt.[rrp29] = src.[rrp29],
        tgt.[rrp30] = src.[rrp30],
        tgt.[rrp31] = src.[rrp31],
        tgt.[rrp32] = src.[rrp32],
        tgt.[rrp33] = src.[rrp33],
        tgt.[rrp34] = src.[rrp34],
        tgt.[rrp35] = src.[rrp35],
        tgt.[rrp36] = src.[rrp36],
        tgt.[rrp37] = src.[rrp37],
        tgt.[rrp38] = src.[rrp38],
        tgt.[rrp39] = src.[rrp39],
        tgt.[rrp40] = src.[rrp40],
        tgt.[rrp41] = src.[rrp41],
        tgt.[rrp42] = src.[rrp42],
        tgt.[rrp43] = src.[rrp43],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [regionid],
        [interval_datetime],
        [intervention],
        [intervention_active],
        [rrp1],
        [rrp2],
        [rrp3],
        [rrp4],
        [rrp5],
        [rrp6],
        [rrp7],
        [rrp8],
        [rrp9],
        [rrp10],
        [rrp11],
        [rrp12],
        [rrp13],
        [rrp14],
        [rrp15],
        [rrp16],
        [rrp17],
        [rrp18],
        [rrp19],
        [rrp20],
        [rrp21],
        [rrp22],
        [rrp23],
        [rrp24],
        [rrp25],
        [rrp26],
        [rrp27],
        [rrp28],
        [rrp29],
        [rrp30],
        [rrp31],
        [rrp32],
        [rrp33],
        [rrp34],
        [rrp35],
        [rrp36],
        [rrp37],
        [rrp38],
        [rrp39],
        [rrp40],
        [rrp41],
        [rrp42],
        [rrp43],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[regionid],
        src.[interval_datetime],
        src.[intervention],
        src.[intervention_active],
        src.[rrp1],
        src.[rrp2],
        src.[rrp3],
        src.[rrp4],
        src.[rrp5],
        src.[rrp6],
        src.[rrp7],
        src.[rrp8],
        src.[rrp9],
        src.[rrp10],
        src.[rrp11],
        src.[rrp12],
        src.[rrp13],
        src.[rrp14],
        src.[rrp15],
        src.[rrp16],
        src.[rrp17],
        src.[rrp18],
        src.[rrp19],
        src.[rrp20],
        src.[rrp21],
        src.[rrp22],
        src.[rrp23],
        src.[rrp24],
        src.[rrp25],
        src.[rrp26],
        src.[rrp27],
        src.[rrp28],
        src.[rrp29],
        src.[rrp30],
        src.[rrp31],
        src.[rrp32],
        src.[rrp33],
        src.[rrp34],
        src.[rrp35],
        src.[rrp36],
        src.[rrp37],
        src.[rrp38],
        src.[rrp39],
        src.[rrp40],
        src.[rrp41],
        src.[rrp42],
        src.[rrp43],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minRegionsolution9
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minRegionsolution9 as tgt 
using (
    select 
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
        d.[ss_wind_compliancemw],
        d.[wdr_initialmw],
        d.[wdr_available],
        d.[wdr_dispatched],
        d.[ss_solar_availability],
        d.[ss_wind_availability],
        d.[raise1secrrp],
        d.[raise1secrop],
        d.[lower1secrrp],
        d.[lower1secrop],
        d.[raise1seclocaldispatch],
        d.[lower1seclocaldispatch],
        d.[bdu_energy_storage],
        d.[bdu_min_avail],
        d.[bdu_max_avail],
        d.[bdu_clearedmw_gen],
        d.[bdu_clearedmw_load]
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
        [ss_wind_compliancemw] decimal(15,5),
        [wdr_initialmw] decimal(15,5),
        [wdr_available] decimal(15,5),
        [wdr_dispatched] decimal(15,5),
        [ss_solar_availability] decimal(15,5),
        [ss_wind_availability] decimal(15,5),
        [raise1secrrp] decimal(15,5),
        [raise1secrop] decimal(15,5),
        [lower1secrrp] decimal(15,5),
        [lower1secrop] decimal(15,5),
        [raise1seclocaldispatch] decimal(15,5),
        [lower1seclocaldispatch] decimal(15,5),
        [bdu_energy_storage] decimal(15,5),
        [bdu_min_avail] decimal(15,5),
        [bdu_max_avail] decimal(15,5),
        [bdu_clearedmw_gen] decimal(15,5),
        [bdu_clearedmw_load] decimal(15,5)
    ) d
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[rrp] = src.[rrp],
        tgt.[rop] = src.[rop],
        tgt.[excessgeneration] = src.[excessgeneration],
        tgt.[raise6secrrp] = src.[raise6secrrp],
        tgt.[raise6secrop] = src.[raise6secrop],
        tgt.[raise60secrrp] = src.[raise60secrrp],
        tgt.[raise60secrop] = src.[raise60secrop],
        tgt.[raise5minrrp] = src.[raise5minrrp],
        tgt.[raise5minrop] = src.[raise5minrop],
        tgt.[raiseregrrp] = src.[raiseregrrp],
        tgt.[raiseregrop] = src.[raiseregrop],
        tgt.[lower6secrrp] = src.[lower6secrrp],
        tgt.[lower6secrop] = src.[lower6secrop],
        tgt.[lower60secrrp] = src.[lower60secrrp],
        tgt.[lower60secrop] = src.[lower60secrop],
        tgt.[lower5minrrp] = src.[lower5minrrp],
        tgt.[lower5minrop] = src.[lower5minrop],
        tgt.[lowerregrrp] = src.[lowerregrrp],
        tgt.[lowerregrop] = src.[lowerregrop],
        tgt.[totaldemand] = src.[totaldemand],
        tgt.[availablegeneration] = src.[availablegeneration],
        tgt.[availableload] = src.[availableload],
        tgt.[demandforecast] = src.[demandforecast],
        tgt.[dispatchablegeneration] = src.[dispatchablegeneration],
        tgt.[dispatchableload] = src.[dispatchableload],
        tgt.[netinterchange] = src.[netinterchange],
        tgt.[lower5mindispatch] = src.[lower5mindispatch],
        tgt.[lower5minimport] = src.[lower5minimport],
        tgt.[lower5minlocaldispatch] = src.[lower5minlocaldispatch],
        tgt.[lower5minlocalreq] = src.[lower5minlocalreq],
        tgt.[lower5minreq] = src.[lower5minreq],
        tgt.[lower60secdispatch] = src.[lower60secdispatch],
        tgt.[lower60secimport] = src.[lower60secimport],
        tgt.[lower60seclocaldispatch] = src.[lower60seclocaldispatch],
        tgt.[lower60seclocalreq] = src.[lower60seclocalreq],
        tgt.[lower60secreq] = src.[lower60secreq],
        tgt.[lower6secdispatch] = src.[lower6secdispatch],
        tgt.[lower6secimport] = src.[lower6secimport],
        tgt.[lower6seclocaldispatch] = src.[lower6seclocaldispatch],
        tgt.[lower6seclocalreq] = src.[lower6seclocalreq],
        tgt.[lower6secreq] = src.[lower6secreq],
        tgt.[raise5mindispatch] = src.[raise5mindispatch],
        tgt.[raise5minimport] = src.[raise5minimport],
        tgt.[raise5minlocaldispatch] = src.[raise5minlocaldispatch],
        tgt.[raise5minlocalreq] = src.[raise5minlocalreq],
        tgt.[raise5minreq] = src.[raise5minreq],
        tgt.[raise60secdispatch] = src.[raise60secdispatch],
        tgt.[raise60secimport] = src.[raise60secimport],
        tgt.[raise60seclocaldispatch] = src.[raise60seclocaldispatch],
        tgt.[raise60seclocalreq] = src.[raise60seclocalreq],
        tgt.[raise60secreq] = src.[raise60secreq],
        tgt.[raise6secdispatch] = src.[raise6secdispatch],
        tgt.[raise6secimport] = src.[raise6secimport],
        tgt.[raise6seclocaldispatch] = src.[raise6seclocaldispatch],
        tgt.[raise6seclocalreq] = src.[raise6seclocalreq],
        tgt.[raise6secreq] = src.[raise6secreq],
        tgt.[aggregatedispatcherror] = src.[aggregatedispatcherror],
        tgt.[initialsupply] = src.[initialsupply],
        tgt.[clearedsupply] = src.[clearedsupply],
        tgt.[lowerregimport] = src.[lowerregimport],
        tgt.[lowerregdispatch] = src.[lowerregdispatch],
        tgt.[lowerreglocaldispatch] = src.[lowerreglocaldispatch],
        tgt.[lowerreglocalreq] = src.[lowerreglocalreq],
        tgt.[lowerregreq] = src.[lowerregreq],
        tgt.[raiseregimport] = src.[raiseregimport],
        tgt.[raiseregdispatch] = src.[raiseregdispatch],
        tgt.[raisereglocaldispatch] = src.[raisereglocaldispatch],
        tgt.[raisereglocalreq] = src.[raisereglocalreq],
        tgt.[raiseregreq] = src.[raiseregreq],
        tgt.[raise5minlocalviolation] = src.[raise5minlocalviolation],
        tgt.[raisereglocalviolation] = src.[raisereglocalviolation],
        tgt.[raise60seclocalviolation] = src.[raise60seclocalviolation],
        tgt.[raise6seclocalviolation] = src.[raise6seclocalviolation],
        tgt.[lower5minlocalviolation] = src.[lower5minlocalviolation],
        tgt.[lowerreglocalviolation] = src.[lowerreglocalviolation],
        tgt.[lower60seclocalviolation] = src.[lower60seclocalviolation],
        tgt.[lower6seclocalviolation] = src.[lower6seclocalviolation],
        tgt.[raise5minviolation] = src.[raise5minviolation],
        tgt.[raiseregviolation] = src.[raiseregviolation],
        tgt.[raise60secviolation] = src.[raise60secviolation],
        tgt.[raise6secviolation] = src.[raise6secviolation],
        tgt.[lower5minviolation] = src.[lower5minviolation],
        tgt.[lowerregviolation] = src.[lowerregviolation],
        tgt.[lower60secviolation] = src.[lower60secviolation],
        tgt.[lower6secviolation] = src.[lower6secviolation],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[totalintermittentgeneration] = src.[totalintermittentgeneration],
        tgt.[demand_and_nonschedgen] = src.[demand_and_nonschedgen],
        tgt.[uigf] = src.[uigf],
        tgt.[semischedule_clearedmw] = src.[semischedule_clearedmw],
        tgt.[semischedule_compliancemw] = src.[semischedule_compliancemw],
        tgt.[intervention] = src.[intervention],
        tgt.[ss_solar_uigf] = src.[ss_solar_uigf],
        tgt.[ss_wind_uigf] = src.[ss_wind_uigf],
        tgt.[ss_solar_clearedmw] = src.[ss_solar_clearedmw],
        tgt.[ss_wind_clearedmw] = src.[ss_wind_clearedmw],
        tgt.[ss_solar_compliancemw] = src.[ss_solar_compliancemw],
        tgt.[ss_wind_compliancemw] = src.[ss_wind_compliancemw],
        tgt.[wdr_initialmw] = src.[wdr_initialmw],
        tgt.[wdr_available] = src.[wdr_available],
        tgt.[wdr_dispatched] = src.[wdr_dispatched],
        tgt.[ss_solar_availability] = src.[ss_solar_availability],
        tgt.[ss_wind_availability] = src.[ss_wind_availability],
        tgt.[raise1secrrp] = src.[raise1secrrp],
        tgt.[raise1secrop] = src.[raise1secrop],
        tgt.[lower1secrrp] = src.[lower1secrrp],
        tgt.[lower1secrop] = src.[lower1secrop],
        tgt.[raise1seclocaldispatch] = src.[raise1seclocaldispatch],
        tgt.[lower1seclocaldispatch] = src.[lower1seclocaldispatch],
        tgt.[bdu_energy_storage] = src.[bdu_energy_storage],
        tgt.[bdu_min_avail] = src.[bdu_min_avail],
        tgt.[bdu_max_avail] = src.[bdu_max_avail],
        tgt.[bdu_clearedmw_gen] = src.[bdu_clearedmw_gen],
        tgt.[bdu_clearedmw_load] = src.[bdu_clearedmw_load]
when not matched
    then insert (
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
        [ss_wind_compliancemw],
        [wdr_initialmw],
        [wdr_available],
        [wdr_dispatched],
        [ss_solar_availability],
        [ss_wind_availability],
        [raise1secrrp],
        [raise1secrop],
        [lower1secrrp],
        [lower1secrop],
        [raise1seclocaldispatch],
        [lower1seclocaldispatch],
        [bdu_energy_storage],
        [bdu_min_avail],
        [bdu_max_avail],
        [bdu_clearedmw_gen],
        [bdu_clearedmw_load]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[regionid],
        src.[rrp],
        src.[rop],
        src.[excessgeneration],
        src.[raise6secrrp],
        src.[raise6secrop],
        src.[raise60secrrp],
        src.[raise60secrop],
        src.[raise5minrrp],
        src.[raise5minrop],
        src.[raiseregrrp],
        src.[raiseregrop],
        src.[lower6secrrp],
        src.[lower6secrop],
        src.[lower60secrrp],
        src.[lower60secrop],
        src.[lower5minrrp],
        src.[lower5minrop],
        src.[lowerregrrp],
        src.[lowerregrop],
        src.[totaldemand],
        src.[availablegeneration],
        src.[availableload],
        src.[demandforecast],
        src.[dispatchablegeneration],
        src.[dispatchableload],
        src.[netinterchange],
        src.[lower5mindispatch],
        src.[lower5minimport],
        src.[lower5minlocaldispatch],
        src.[lower5minlocalreq],
        src.[lower5minreq],
        src.[lower60secdispatch],
        src.[lower60secimport],
        src.[lower60seclocaldispatch],
        src.[lower60seclocalreq],
        src.[lower60secreq],
        src.[lower6secdispatch],
        src.[lower6secimport],
        src.[lower6seclocaldispatch],
        src.[lower6seclocalreq],
        src.[lower6secreq],
        src.[raise5mindispatch],
        src.[raise5minimport],
        src.[raise5minlocaldispatch],
        src.[raise5minlocalreq],
        src.[raise5minreq],
        src.[raise60secdispatch],
        src.[raise60secimport],
        src.[raise60seclocaldispatch],
        src.[raise60seclocalreq],
        src.[raise60secreq],
        src.[raise6secdispatch],
        src.[raise6secimport],
        src.[raise6seclocaldispatch],
        src.[raise6seclocalreq],
        src.[raise6secreq],
        src.[aggregatedispatcherror],
        src.[initialsupply],
        src.[clearedsupply],
        src.[lowerregimport],
        src.[lowerregdispatch],
        src.[lowerreglocaldispatch],
        src.[lowerreglocalreq],
        src.[lowerregreq],
        src.[raiseregimport],
        src.[raiseregdispatch],
        src.[raisereglocaldispatch],
        src.[raisereglocalreq],
        src.[raiseregreq],
        src.[raise5minlocalviolation],
        src.[raisereglocalviolation],
        src.[raise60seclocalviolation],
        src.[raise6seclocalviolation],
        src.[lower5minlocalviolation],
        src.[lowerreglocalviolation],
        src.[lower60seclocalviolation],
        src.[lower6seclocalviolation],
        src.[raise5minviolation],
        src.[raiseregviolation],
        src.[raise60secviolation],
        src.[raise6secviolation],
        src.[lower5minviolation],
        src.[lowerregviolation],
        src.[lower60secviolation],
        src.[lower6secviolation],
        src.[lastchanged],
        src.[totalintermittentgeneration],
        src.[demand_and_nonschedgen],
        src.[uigf],
        src.[semischedule_clearedmw],
        src.[semischedule_compliancemw],
        src.[intervention],
        src.[ss_solar_uigf],
        src.[ss_wind_uigf],
        src.[ss_solar_clearedmw],
        src.[ss_wind_clearedmw],
        src.[ss_solar_compliancemw],
        src.[ss_wind_compliancemw],
        src.[wdr_initialmw],
        src.[wdr_available],
        src.[wdr_dispatched],
        src.[ss_solar_availability],
        src.[ss_wind_availability],
        src.[raise1secrrp],
        src.[raise1secrop],
        src.[lower1secrrp],
        src.[lower1secrop],
        src.[raise1seclocaldispatch],
        src.[lower1seclocaldispatch],
        src.[bdu_energy_storage],
        src.[bdu_min_avail],
        src.[bdu_max_avail],
        src.[bdu_clearedmw_gen],
        src.[bdu_clearedmw_load]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minScenariodemand1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.P5minScenariodemand1(
file_log_id,
[effectivedate],
        [version_datetime],
        [scenario],
        [regionid],
        [deltamw]
)
select 
@file_log_id,
d.[effectivedate],
        d.[version_datetime],
        d.[scenario],
        d.[regionid],
        d.[deltamw]
from openjson(@data) with (
[effectivedate] datetime2,
        [version_datetime] datetime2,
        [scenario] decimal(2,0),
        [regionid] varchar(20),
        [deltamw] decimal(4,0)
) d
end
go
create or alter procedure mmsdm_proc.InsertP5minScenariodemandtrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minScenariodemandtrk1 as tgt 
using (
    select 
        d.[effectivedate],
        d.[version_datetime],
        d.[authoriseddate],
        d.[lastchanged]
    from openjson(@data) with (
        [effectivedate] datetime2,
        [version_datetime] datetime2,
        [authoriseddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [version_datetime],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[version_datetime],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertP5minUnitsolution6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.P5minUnitsolution6 as tgt 
using (
    select 
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
        d.[intervention],
        d.[dispatchmodetime],
        d.[conformance_mode],
        d.[uigf],
        d.[raise1sec],
        d.[raise1secflags],
        d.[lower1sec],
        d.[lower1secflags],
        d.[initial_energy_storage],
        d.[energy_storage],
        d.[energy_storage_min],
        d.[energy_storage_max],
        d.[min_availability]
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
        [intervention] decimal(2,0),
        [dispatchmodetime] decimal(4,0),
        [conformance_mode] decimal(6,0),
        [uigf] decimal(15,5),
        [raise1sec] decimal(15,5),
        [raise1secflags] decimal(3,0),
        [lower1sec] decimal(15,5),
        [lower1secflags] decimal(3,0),
        [initial_energy_storage] decimal(15,5),
        [energy_storage] decimal(15,5),
        [energy_storage_min] decimal(15,5),
        [energy_storage_max] decimal(15,5),
        [min_availability] decimal(15,5)
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[duid] = src.[duid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[tradetype] = src.[tradetype],
        tgt.[agcstatus] = src.[agcstatus],
        tgt.[initialmw] = src.[initialmw],
        tgt.[totalcleared] = src.[totalcleared],
        tgt.[rampdownrate] = src.[rampdownrate],
        tgt.[rampuprate] = src.[rampuprate],
        tgt.[lower5min] = src.[lower5min],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise5min] = src.[raise5min],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[availability] = src.[availability],
        tgt.[raise6secflags] = src.[raise6secflags],
        tgt.[raise60secflags] = src.[raise60secflags],
        tgt.[raise5minflags] = src.[raise5minflags],
        tgt.[raiseregflags] = src.[raiseregflags],
        tgt.[lower6secflags] = src.[lower6secflags],
        tgt.[lower60secflags] = src.[lower60secflags],
        tgt.[lower5minflags] = src.[lower5minflags],
        tgt.[lowerregflags] = src.[lowerregflags],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[semidispatchcap] = src.[semidispatchcap],
        tgt.[intervention] = src.[intervention],
        tgt.[dispatchmodetime] = src.[dispatchmodetime],
        tgt.[conformance_mode] = src.[conformance_mode],
        tgt.[uigf] = src.[uigf],
        tgt.[raise1sec] = src.[raise1sec],
        tgt.[raise1secflags] = src.[raise1secflags],
        tgt.[lower1sec] = src.[lower1sec],
        tgt.[lower1secflags] = src.[lower1secflags],
        tgt.[initial_energy_storage] = src.[initial_energy_storage],
        tgt.[energy_storage] = src.[energy_storage],
        tgt.[energy_storage_min] = src.[energy_storage_min],
        tgt.[energy_storage_max] = src.[energy_storage_max],
        tgt.[min_availability] = src.[min_availability]
when not matched
    then insert (
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
        [intervention],
        [dispatchmodetime],
        [conformance_mode],
        [uigf],
        [raise1sec],
        [raise1secflags],
        [lower1sec],
        [lower1secflags],
        [initial_energy_storage],
        [energy_storage],
        [energy_storage_min],
        [energy_storage_max],
        [min_availability]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[duid],
        src.[connectionpointid],
        src.[tradetype],
        src.[agcstatus],
        src.[initialmw],
        src.[totalcleared],
        src.[rampdownrate],
        src.[rampuprate],
        src.[lower5min],
        src.[lower60sec],
        src.[lower6sec],
        src.[raise5min],
        src.[raise60sec],
        src.[raise6sec],
        src.[lowerreg],
        src.[raisereg],
        src.[availability],
        src.[raise6secflags],
        src.[raise60secflags],
        src.[raise5minflags],
        src.[raiseregflags],
        src.[lower6secflags],
        src.[lower60secflags],
        src.[lower5minflags],
        src.[lowerregflags],
        src.[lastchanged],
        src.[semidispatchcap],
        src.[intervention],
        src.[dispatchmodetime],
        src.[conformance_mode],
        src.[uigf],
        src.[raise1sec],
        src.[raise1secflags],
        src.[lower1sec],
        src.[lower1secflags],
        src.[initial_energy_storage],
        src.[energy_storage],
        src.[energy_storage_min],
        src.[energy_storage_max],
        src.[min_availability]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationAdgDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationAdgDetail1 as tgt 
using (
    select 
        d.[adg_id],
        d.[effectivedate],
        d.[version_datetime],
        d.[adg_type],
        d.[authoriseddate],
        d.[authorisedby],
        d.[lastchanged]
    from openjson(@data) with (
        [adg_id] varchar(20),
        [effectivedate] datetime2,
        [version_datetime] datetime2,
        [adg_type] varchar(20),
        [authoriseddate] datetime2,
        [authorisedby] varchar(15),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[adg_id] = src.[adg_id]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[adg_id] = src.[adg_id],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[adg_type] = src.[adg_type],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [adg_id],
        [effectivedate],
        [version_datetime],
        [adg_type],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[adg_id],
        src.[effectivedate],
        src.[version_datetime],
        src.[adg_type],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationAggregateDispatchGroup1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationAggregateDispatchGroup1 as tgt 
using (
    select 
        d.[adg_id],
        d.[comments],
        d.[lastchanged]
    from openjson(@data) with (
        [adg_id] varchar(20),
        [comments] varchar(100),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[adg_id] = src.[adg_id]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[adg_id] = src.[adg_id],
        tgt.[comments] = src.[comments],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [adg_id],
        [comments],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[adg_id],
        src.[comments],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationBidduiddetails1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationBidduiddetails1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[duid] = src.[duid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[bidtype] = src.[bidtype],
        tgt.[maxcapacity] = src.[maxcapacity],
        tgt.[minenablementlevel] = src.[minenablementlevel],
        tgt.[maxenablementlevel] = src.[maxenablementlevel],
        tgt.[maxlowerangle] = src.[maxlowerangle],
        tgt.[maxupperangle] = src.[maxupperangle],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[duid],
        src.[effectivedate],
        src.[versionno],
        src.[bidtype],
        src.[maxcapacity],
        src.[minenablementlevel],
        src.[maxenablementlevel],
        src.[maxlowerangle],
        src.[maxupperangle],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationBidduiddetailstrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationBidduiddetailstrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [duid],
        [effectivedate],
        [versionno],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[duid],
        src.[effectivedate],
        src.[versionno],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationDispatchableunit1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationDispatchableunit1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[duname] = src.[duname],
        tgt.[unittype] = src.[unittype],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [duid],
        [duname],
        [unittype],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[duid],
        src.[duname],
        src.[unittype],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationDualloc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationDualloc1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[gensetid] = src.[gensetid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[duid] = src.[duid],
        tgt.[gensetid] = src.[gensetid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [duid],
        [gensetid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[duid],
        src.[gensetid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationDudetail6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationDudetail6 as tgt 
using (
    select 
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
        d.[maxrateofchangedown],
        d.[dispatchsubtype],
        d.[adg_id],
        d.[mincapacity],
        d.[registeredmincapacity],
        d.[maxrateofchangeup_load],
        d.[maxrateofchangedown_load],
        d.[maxstoragecapacity],
        d.[storageimportefficiencyfactor],
        d.[storageexportefficiencyfactor],
        d.[min_ramp_rate_up],
        d.[min_ramp_rate_down],
        d.[load_min_ramp_rate_up],
        d.[load_min_ramp_rate_down]
    from openjson(@data) with (
        [effectivedate] datetime2,
        [duid] varchar(10),
        [versionno] decimal(3,0),
        [connectionpointid] varchar(10),
        [voltlevel] varchar(10),
        [registeredcapacity] decimal(6,0),
        [agccapability] varchar(1),
        [dispatchtype] varchar(20),
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
        [maxrateofchangedown] decimal(6,0),
        [dispatchsubtype] varchar(20),
        [adg_id] varchar(20),
        [mincapacity] decimal(6,0),
        [registeredmincapacity] decimal(6,0),
        [maxrateofchangeup_load] decimal(6,0),
        [maxrateofchangedown_load] decimal(6,0),
        [maxstoragecapacity] decimal(15,5),
        [storageimportefficiencyfactor] decimal(15,5),
        [storageexportefficiencyfactor] decimal(15,5),
        [min_ramp_rate_up] decimal(6,0),
        [min_ramp_rate_down] decimal(6,0),
        [load_min_ramp_rate_up] decimal(6,0),
        [load_min_ramp_rate_down] decimal(6,0)
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[duid] = src.[duid],
        tgt.[versionno] = src.[versionno],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[voltlevel] = src.[voltlevel],
        tgt.[registeredcapacity] = src.[registeredcapacity],
        tgt.[agccapability] = src.[agccapability],
        tgt.[dispatchtype] = src.[dispatchtype],
        tgt.[maxcapacity] = src.[maxcapacity],
        tgt.[starttype] = src.[starttype],
        tgt.[normallyonflag] = src.[normallyonflag],
        tgt.[physicaldetailsflag] = src.[physicaldetailsflag],
        tgt.[spinningreserveflag] = src.[spinningreserveflag],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[intermittentflag] = src.[intermittentflag],
        tgt.[semi_schedule_flag] = src.[semi_schedule_flag],
        tgt.[maxrateofchangeup] = src.[maxrateofchangeup],
        tgt.[maxrateofchangedown] = src.[maxrateofchangedown],
        tgt.[dispatchsubtype] = src.[dispatchsubtype],
        tgt.[adg_id] = src.[adg_id],
        tgt.[mincapacity] = src.[mincapacity],
        tgt.[registeredmincapacity] = src.[registeredmincapacity],
        tgt.[maxrateofchangeup_load] = src.[maxrateofchangeup_load],
        tgt.[maxrateofchangedown_load] = src.[maxrateofchangedown_load],
        tgt.[maxstoragecapacity] = src.[maxstoragecapacity],
        tgt.[storageimportefficiencyfactor] = src.[storageimportefficiencyfactor],
        tgt.[storageexportefficiencyfactor] = src.[storageexportefficiencyfactor],
        tgt.[min_ramp_rate_up] = src.[min_ramp_rate_up],
        tgt.[min_ramp_rate_down] = src.[min_ramp_rate_down],
        tgt.[load_min_ramp_rate_up] = src.[load_min_ramp_rate_up],
        tgt.[load_min_ramp_rate_down] = src.[load_min_ramp_rate_down]
when not matched
    then insert (
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
        [maxrateofchangedown],
        [dispatchsubtype],
        [adg_id],
        [mincapacity],
        [registeredmincapacity],
        [maxrateofchangeup_load],
        [maxrateofchangedown_load],
        [maxstoragecapacity],
        [storageimportefficiencyfactor],
        [storageexportefficiencyfactor],
        [min_ramp_rate_up],
        [min_ramp_rate_down],
        [load_min_ramp_rate_up],
        [load_min_ramp_rate_down]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[duid],
        src.[versionno],
        src.[connectionpointid],
        src.[voltlevel],
        src.[registeredcapacity],
        src.[agccapability],
        src.[dispatchtype],
        src.[maxcapacity],
        src.[starttype],
        src.[normallyonflag],
        src.[physicaldetailsflag],
        src.[spinningreserveflag],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged],
        src.[intermittentflag],
        src.[semi_schedule_flag],
        src.[maxrateofchangeup],
        src.[maxrateofchangedown],
        src.[dispatchsubtype],
        src.[adg_id],
        src.[mincapacity],
        src.[registeredmincapacity],
        src.[maxrateofchangeup_load],
        src.[maxrateofchangedown_load],
        src.[maxstoragecapacity],
        src.[storageimportefficiencyfactor],
        src.[storageexportefficiencyfactor],
        src.[min_ramp_rate_up],
        src.[min_ramp_rate_down],
        src.[load_min_ramp_rate_up],
        src.[load_min_ramp_rate_down]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationDudetailsummary7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationDudetailsummary7 as tgt 
using (
    select 
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
        d.[is_aggregated],
        d.[dispatchsubtype],
        d.[adg_id],
        d.[load_minimum_energy_price],
        d.[load_maximum_energy_price],
        d.[load_min_ramp_rate_up],
        d.[load_min_ramp_rate_down],
        d.[load_max_ramp_rate_up],
        d.[load_max_ramp_rate_down],
        d.[secondary_tlf]
    from openjson(@data) with (
        [duid] varchar(10),
        [start_date] datetime2,
        [end_date] datetime2,
        [dispatchtype] varchar(20),
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
        [is_aggregated] decimal(1,0),
        [dispatchsubtype] varchar(20),
        [adg_id] varchar(20),
        [load_minimum_energy_price] decimal(9,2),
        [load_maximum_energy_price] decimal(9,2),
        [load_min_ramp_rate_up] decimal(6,0),
        [load_min_ramp_rate_down] decimal(6,0),
        [load_max_ramp_rate_up] decimal(6,0),
        [load_max_ramp_rate_down] decimal(6,0),
        [secondary_tlf] decimal(18,8)
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[start_date] = src.[start_date]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[start_date] = src.[start_date],
        tgt.[end_date] = src.[end_date],
        tgt.[dispatchtype] = src.[dispatchtype],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[regionid] = src.[regionid],
        tgt.[stationid] = src.[stationid],
        tgt.[participantid] = src.[participantid],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[transmissionlossfactor] = src.[transmissionlossfactor],
        tgt.[starttype] = src.[starttype],
        tgt.[distributionlossfactor] = src.[distributionlossfactor],
        tgt.[minimum_energy_price] = src.[minimum_energy_price],
        tgt.[maximum_energy_price] = src.[maximum_energy_price],
        tgt.[schedule_type] = src.[schedule_type],
        tgt.[min_ramp_rate_up] = src.[min_ramp_rate_up],
        tgt.[min_ramp_rate_down] = src.[min_ramp_rate_down],
        tgt.[max_ramp_rate_up] = src.[max_ramp_rate_up],
        tgt.[max_ramp_rate_down] = src.[max_ramp_rate_down],
        tgt.[is_aggregated] = src.[is_aggregated],
        tgt.[dispatchsubtype] = src.[dispatchsubtype],
        tgt.[adg_id] = src.[adg_id],
        tgt.[load_minimum_energy_price] = src.[load_minimum_energy_price],
        tgt.[load_maximum_energy_price] = src.[load_maximum_energy_price],
        tgt.[load_min_ramp_rate_up] = src.[load_min_ramp_rate_up],
        tgt.[load_min_ramp_rate_down] = src.[load_min_ramp_rate_down],
        tgt.[load_max_ramp_rate_up] = src.[load_max_ramp_rate_up],
        tgt.[load_max_ramp_rate_down] = src.[load_max_ramp_rate_down],
        tgt.[secondary_tlf] = src.[secondary_tlf]
when not matched
    then insert (
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
        [is_aggregated],
        [dispatchsubtype],
        [adg_id],
        [load_minimum_energy_price],
        [load_maximum_energy_price],
        [load_min_ramp_rate_up],
        [load_min_ramp_rate_down],
        [load_max_ramp_rate_up],
        [load_max_ramp_rate_down],
        [secondary_tlf]
    ) values (
        @file_log_id,
        src.[duid],
        src.[start_date],
        src.[end_date],
        src.[dispatchtype],
        src.[connectionpointid],
        src.[regionid],
        src.[stationid],
        src.[participantid],
        src.[lastchanged],
        src.[transmissionlossfactor],
        src.[starttype],
        src.[distributionlossfactor],
        src.[minimum_energy_price],
        src.[maximum_energy_price],
        src.[schedule_type],
        src.[min_ramp_rate_up],
        src.[min_ramp_rate_down],
        src.[max_ramp_rate_up],
        src.[max_ramp_rate_down],
        src.[is_aggregated],
        src.[dispatchsubtype],
        src.[adg_id],
        src.[load_minimum_energy_price],
        src.[load_maximum_energy_price],
        src.[load_min_ramp_rate_up],
        src.[load_min_ramp_rate_down],
        src.[load_max_ramp_rate_up],
        src.[load_max_ramp_rate_down],
        src.[secondary_tlf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationGenmeter1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationGenmeter1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[applydate] = src.[applydate]
    and tgt.[meterid] = src.[meterid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[meterid] = src.[meterid],
        tgt.[gensetid] = src.[gensetid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[stationid] = src.[stationid],
        tgt.[metertype] = src.[metertype],
        tgt.[meterclass] = src.[meterclass],
        tgt.[voltagelevel] = src.[voltagelevel],
        tgt.[applydate] = src.[applydate],
        tgt.[versionno] = src.[versionno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[comdate] = src.[comdate],
        tgt.[decomdate] = src.[decomdate],
        tgt.[enddate] = src.[enddate],
        tgt.[startdate] = src.[startdate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[meterid],
        src.[gensetid],
        src.[connectionpointid],
        src.[stationid],
        src.[metertype],
        src.[meterclass],
        src.[voltagelevel],
        src.[applydate],
        src.[versionno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[comdate],
        src.[decomdate],
        src.[enddate],
        src.[startdate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationGenunits3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationGenunits3 as tgt 
using (
    select 
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
        d.[co2e_data_source],
        d.[mincapacity],
        d.[registeredmincapacity],
        d.[maxstoragecapacity]
    from openjson(@data) with (
        [gensetid] varchar(20),
        [stationid] varchar(10),
        [setlossfactor] decimal(16,6),
        [cdindicator] varchar(10),
        [agcflag] varchar(2),
        [spinningflag] varchar(2),
        [voltlevel] decimal(6,0),
        [registeredcapacity] decimal(6,0),
        [dispatchtype] varchar(20),
        [starttype] varchar(20),
        [mktgeneratorind] varchar(10),
        [normalstatus] varchar(10),
        [maxcapacity] decimal(6,0),
        [gensettype] varchar(15),
        [gensetname] varchar(40),
        [lastchanged] datetime2,
        [co2e_emissions_factor] decimal(18,8),
        [co2e_energy_source] varchar(100),
        [co2e_data_source] varchar(20),
        [mincapacity] decimal(6,0),
        [registeredmincapacity] decimal(6,0),
        [maxstoragecapacity] decimal(15,5)
    ) d
) as src 
on (
    tgt.[gensetid] = src.[gensetid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[gensetid] = src.[gensetid],
        tgt.[stationid] = src.[stationid],
        tgt.[setlossfactor] = src.[setlossfactor],
        tgt.[cdindicator] = src.[cdindicator],
        tgt.[agcflag] = src.[agcflag],
        tgt.[spinningflag] = src.[spinningflag],
        tgt.[voltlevel] = src.[voltlevel],
        tgt.[registeredcapacity] = src.[registeredcapacity],
        tgt.[dispatchtype] = src.[dispatchtype],
        tgt.[starttype] = src.[starttype],
        tgt.[mktgeneratorind] = src.[mktgeneratorind],
        tgt.[normalstatus] = src.[normalstatus],
        tgt.[maxcapacity] = src.[maxcapacity],
        tgt.[gensettype] = src.[gensettype],
        tgt.[gensetname] = src.[gensetname],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[co2e_emissions_factor] = src.[co2e_emissions_factor],
        tgt.[co2e_energy_source] = src.[co2e_energy_source],
        tgt.[co2e_data_source] = src.[co2e_data_source],
        tgt.[mincapacity] = src.[mincapacity],
        tgt.[registeredmincapacity] = src.[registeredmincapacity],
        tgt.[maxstoragecapacity] = src.[maxstoragecapacity]
when not matched
    then insert (
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
        [co2e_data_source],
        [mincapacity],
        [registeredmincapacity],
        [maxstoragecapacity]
    ) values (
        @file_log_id,
        src.[gensetid],
        src.[stationid],
        src.[setlossfactor],
        src.[cdindicator],
        src.[agcflag],
        src.[spinningflag],
        src.[voltlevel],
        src.[registeredcapacity],
        src.[dispatchtype],
        src.[starttype],
        src.[mktgeneratorind],
        src.[normalstatus],
        src.[maxcapacity],
        src.[gensettype],
        src.[gensetname],
        src.[lastchanged],
        src.[co2e_emissions_factor],
        src.[co2e_energy_source],
        src.[co2e_data_source],
        src.[mincapacity],
        src.[registeredmincapacity],
        src.[maxstoragecapacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationGenunitsUnit2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationGenunitsUnit2 as tgt 
using (
    select 
        d.[gensetid],
        d.[effectivedate],
        d.[versionno],
        d.[unit_grouping_label],
        d.[unit_count],
        d.[unit_size],
        d.[unit_max_size],
        d.[aggregation_flag],
        d.[lastchanged],
        d.[unitminsize],
        d.[maxstoragecapacity],
        d.[registeredcapacity],
        d.[registeredmincapacity]
    from openjson(@data) with (
        [gensetid] varchar(20),
        [effectivedate] datetime2,
        [versionno] decimal(6,0),
        [unit_grouping_label] varchar(20),
        [unit_count] decimal(10,0),
        [unit_size] decimal(8,3),
        [unit_max_size] decimal(8,3),
        [aggregation_flag] decimal(1,0),
        [lastchanged] datetime2,
        [unitminsize] decimal(8,3),
        [maxstoragecapacity] decimal(15,5),
        [registeredcapacity] decimal(8,3),
        [registeredmincapacity] decimal(8,3)
    ) d
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[gensetid] = src.[gensetid]
    and tgt.[unit_grouping_label] = src.[unit_grouping_label]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[gensetid] = src.[gensetid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[unit_grouping_label] = src.[unit_grouping_label],
        tgt.[unit_count] = src.[unit_count],
        tgt.[unit_size] = src.[unit_size],
        tgt.[unit_max_size] = src.[unit_max_size],
        tgt.[aggregation_flag] = src.[aggregation_flag],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[unitminsize] = src.[unitminsize],
        tgt.[maxstoragecapacity] = src.[maxstoragecapacity],
        tgt.[registeredcapacity] = src.[registeredcapacity],
        tgt.[registeredmincapacity] = src.[registeredmincapacity]
when not matched
    then insert (
        file_log_id,
        [gensetid],
        [effectivedate],
        [versionno],
        [unit_grouping_label],
        [unit_count],
        [unit_size],
        [unit_max_size],
        [aggregation_flag],
        [lastchanged],
        [unitminsize],
        [maxstoragecapacity],
        [registeredcapacity],
        [registeredmincapacity]
    ) values (
        @file_log_id,
        src.[gensetid],
        src.[effectivedate],
        src.[versionno],
        src.[unit_grouping_label],
        src.[unit_count],
        src.[unit_size],
        src.[unit_max_size],
        src.[aggregation_flag],
        src.[lastchanged],
        src.[unitminsize],
        src.[maxstoragecapacity],
        src.[registeredcapacity],
        src.[registeredmincapacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationMnspInterconnector2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationMnspInterconnector2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[linkid] = src.[linkid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[linkid] = src.[linkid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregion] = src.[fromregion],
        tgt.[toregion] = src.[toregion],
        tgt.[maxcapacity] = src.[maxcapacity],
        tgt.[tlf] = src.[tlf],
        tgt.[lhsfactor] = src.[lhsfactor],
        tgt.[meterflowconstant] = src.[meterflowconstant],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[from_region_tlf] = src.[from_region_tlf],
        tgt.[to_region_tlf] = src.[to_region_tlf]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[linkid],
        src.[effectivedate],
        src.[versionno],
        src.[interconnectorid],
        src.[fromregion],
        src.[toregion],
        src.[maxcapacity],
        src.[tlf],
        src.[lhsfactor],
        src.[meterflowconstant],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged],
        src.[from_region_tlf],
        src.[to_region_tlf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationMnspParticipant1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationMnspParticipant1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [interconnectorid],
        [effectivedate],
        [versionno],
        [participantid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[interconnectorid],
        src.[effectivedate],
        src.[versionno],
        src.[participantid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationParticipant1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationParticipant1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[participantclassid] = src.[participantclassid],
        tgt.[name] = src.[name],
        tgt.[description] = src.[description],
        tgt.[acn] = src.[acn],
        tgt.[primarybusiness] = src.[primarybusiness],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [participantclassid],
        [name],
        [description],
        [acn],
        [primarybusiness],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[participantclassid],
        src.[name],
        src.[description],
        src.[acn],
        src.[primarybusiness],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationParticipantaccount1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationParticipantaccount1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[accountname] = src.[accountname],
        tgt.[participantid] = src.[participantid],
        tgt.[accountnumber] = src.[accountnumber],
        tgt.[bankname] = src.[bankname],
        tgt.[banknumber] = src.[banknumber],
        tgt.[branchname] = src.[branchname],
        tgt.[branchnumber] = src.[branchnumber],
        tgt.[bsbnumber] = src.[bsbnumber],
        tgt.[nemmcocreditaccountnumber] = src.[nemmcocreditaccountnumber],
        tgt.[nemmcodebitaccountnumber] = src.[nemmcodebitaccountnumber],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[abn] = src.[abn]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[accountname],
        src.[participantid],
        src.[accountnumber],
        src.[bankname],
        src.[banknumber],
        src.[branchname],
        src.[branchnumber],
        src.[bsbnumber],
        src.[nemmcocreditaccountnumber],
        src.[nemmcodebitaccountnumber],
        src.[authorisedby],
        src.[authoriseddate],
        src.[effectivedate],
        src.[lastchanged],
        src.[abn]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationParticipantcategory1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationParticipantcategory1 as tgt 
using (
    select 
        d.[participantcategoryid],
        d.[description],
        d.[lastchanged]
    from openjson(@data) with (
        [participantcategoryid] varchar(10),
        [description] varchar(64),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[participantcategoryid] = src.[participantcategoryid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantcategoryid] = src.[participantcategoryid],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantcategoryid],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantcategoryid],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationParticipantcategoryalloc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationParticipantcategoryalloc1 as tgt 
using (
    select 
        d.[participantcategoryid],
        d.[participantid],
        d.[lastchanged]
    from openjson(@data) with (
        [participantcategoryid] varchar(10),
        [participantid] varchar(10),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[participantcategoryid] = src.[participantcategoryid]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantcategoryid] = src.[participantcategoryid],
        tgt.[participantid] = src.[participantid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantcategoryid],
        [participantid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantcategoryid],
        src.[participantid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationParticipantclass1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationParticipantclass1 as tgt 
using (
    select 
        d.[participantclassid],
        d.[description],
        d.[lastchanged]
    from openjson(@data) with (
        [participantclassid] varchar(20),
        [description] varchar(64),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[participantclassid] = src.[participantclassid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantclassid] = src.[participantclassid],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantclassid],
        [description],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantclassid],
        src.[description],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationParticipantcreditdetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationParticipantcreditdetail1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[participantid] = src.[participantid],
        tgt.[creditlimit] = src.[creditlimit],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [participantid],
        [creditlimit],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[participantid],
        src.[creditlimit],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationPmsGroup1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationPmsGroup1 as tgt 
using (
    select 
        d.[groupid],
        d.[createddate],
        d.[lastchanged]
    from openjson(@data) with (
        [groupid] decimal(20,0),
        [createddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[groupid] = src.[groupid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[groupid] = src.[groupid],
        tgt.[createddate] = src.[createddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [groupid],
        [createddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[groupid],
        src.[createddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationPmsGroupnmi1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationPmsGroupnmi1 as tgt 
using (
    select 
        d.[groupnmiid],
        d.[groupid],
        d.[versionfrom],
        d.[versionto],
        d.[startdate],
        d.[enddate],
        d.[nmi],
        d.[sitename],
        d.[nerrgrouppremises],
        d.[baselinemethodologyid],
        d.[mrc],
        d.[mrcreason],
        d.[retailcustomer],
        d.[suspended],
        d.[unavailable],
        d.[approveddate],
        d.[lastchanged]
    from openjson(@data) with (
        [groupnmiid] decimal(20,0),
        [groupid] decimal(20,0),
        [versionfrom] datetime2,
        [versionto] datetime2,
        [startdate] datetime2,
        [enddate] datetime2,
        [nmi] varchar(20),
        [sitename] varchar(50),
        [nerrgrouppremises] decimal(1,0),
        [baselinemethodologyid] varchar(50),
        [mrc] decimal(10,3),
        [mrcreason] varchar(500),
        [retailcustomer] varchar(50),
        [suspended] decimal(1,0),
        [unavailable] decimal(1,0),
        [approveddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[groupnmiid] = src.[groupnmiid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[groupnmiid] = src.[groupnmiid],
        tgt.[groupid] = src.[groupid],
        tgt.[versionfrom] = src.[versionfrom],
        tgt.[versionto] = src.[versionto],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[nmi] = src.[nmi],
        tgt.[sitename] = src.[sitename],
        tgt.[nerrgrouppremises] = src.[nerrgrouppremises],
        tgt.[baselinemethodologyid] = src.[baselinemethodologyid],
        tgt.[mrc] = src.[mrc],
        tgt.[mrcreason] = src.[mrcreason],
        tgt.[retailcustomer] = src.[retailcustomer],
        tgt.[suspended] = src.[suspended],
        tgt.[unavailable] = src.[unavailable],
        tgt.[approveddate] = src.[approveddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [groupnmiid],
        [groupid],
        [versionfrom],
        [versionto],
        [startdate],
        [enddate],
        [nmi],
        [sitename],
        [nerrgrouppremises],
        [baselinemethodologyid],
        [mrc],
        [mrcreason],
        [retailcustomer],
        [suspended],
        [unavailable],
        [approveddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[groupnmiid],
        src.[groupid],
        src.[versionfrom],
        src.[versionto],
        src.[startdate],
        src.[enddate],
        src.[nmi],
        src.[sitename],
        src.[nerrgrouppremises],
        src.[baselinemethodologyid],
        src.[mrc],
        src.[mrcreason],
        src.[retailcustomer],
        src.[suspended],
        src.[unavailable],
        src.[approveddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationPmsGroupservice1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationPmsGroupservice1 as tgt 
using (
    select 
        d.[groupserviceid],
        d.[groupid],
        d.[versionfrom],
        d.[versionto],
        d.[startdate],
        d.[enddate],
        d.[market],
        d.[servicetype],
        d.[entitytype],
        d.[entityid],
        d.[mrc],
        d.[mrcreason],
        d.[maximumrampratepermin],
        d.[region],
        d.[approveddate],
        d.[lastchanged]
    from openjson(@data) with (
        [groupserviceid] decimal(20,0),
        [groupid] decimal(20,0),
        [versionfrom] datetime2,
        [versionto] datetime2,
        [startdate] datetime2,
        [enddate] datetime2,
        [market] varchar(50),
        [servicetype] varchar(50),
        [entitytype] varchar(50),
        [entityid] varchar(50),
        [mrc] decimal(10,3),
        [mrcreason] varchar(500),
        [maximumrampratepermin] decimal(10,0),
        [region] varchar(20),
        [approveddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[groupserviceid] = src.[groupserviceid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[groupserviceid] = src.[groupserviceid],
        tgt.[groupid] = src.[groupid],
        tgt.[versionfrom] = src.[versionfrom],
        tgt.[versionto] = src.[versionto],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[market] = src.[market],
        tgt.[servicetype] = src.[servicetype],
        tgt.[entitytype] = src.[entitytype],
        tgt.[entityid] = src.[entityid],
        tgt.[mrc] = src.[mrc],
        tgt.[mrcreason] = src.[mrcreason],
        tgt.[maximumrampratepermin] = src.[maximumrampratepermin],
        tgt.[region] = src.[region],
        tgt.[approveddate] = src.[approveddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [groupserviceid],
        [groupid],
        [versionfrom],
        [versionto],
        [startdate],
        [enddate],
        [market],
        [servicetype],
        [entitytype],
        [entityid],
        [mrc],
        [mrcreason],
        [maximumrampratepermin],
        [region],
        [approveddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[groupserviceid],
        src.[groupid],
        src.[versionfrom],
        src.[versionto],
        src.[startdate],
        src.[enddate],
        src.[market],
        src.[servicetype],
        src.[entitytype],
        src.[entityid],
        src.[mrc],
        src.[mrcreason],
        src.[maximumrampratepermin],
        src.[region],
        src.[approveddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationStadualloc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationStadualloc1 as tgt 
using (
    select 
        d.[duid],
        d.[effectivedate],
        d.[stationid],
        d.[versionno],
        d.[lastchanged]
    from openjson(@data) with (
        [duid] varchar(10),
        [effectivedate] datetime2,
        [stationid] varchar(10),
        [versionno] decimal(3,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[stationid] = src.[stationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[duid] = src.[duid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[stationid] = src.[stationid],
        tgt.[versionno] = src.[versionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [duid],
        [effectivedate],
        [stationid],
        [versionno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[duid],
        src.[effectivedate],
        src.[stationid],
        src.[versionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationStation1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationStation1 as tgt 
using (
    select 
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
        d.[connectionpointid]
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
        [connectionpointid] varchar(10)
    ) d
) as src 
on (
    tgt.[stationid] = src.[stationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[stationid] = src.[stationid],
        tgt.[stationname] = src.[stationname],
        tgt.[address1] = src.[address1],
        tgt.[address2] = src.[address2],
        tgt.[address3] = src.[address3],
        tgt.[address4] = src.[address4],
        tgt.[city] = src.[city],
        tgt.[state] = src.[state],
        tgt.[postcode] = src.[postcode],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[connectionpointid] = src.[connectionpointid]
when not matched
    then insert (
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
        [connectionpointid]
    ) values (
        @file_log_id,
        src.[stationid],
        src.[stationname],
        src.[address1],
        src.[address2],
        src.[address3],
        src.[address4],
        src.[city],
        src.[state],
        src.[postcode],
        src.[lastchanged],
        src.[connectionpointid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationStationoperatingstatus1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationStationoperatingstatus1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[stationid] = src.[stationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[stationid] = src.[stationid],
        tgt.[versionno] = src.[versionno],
        tgt.[status] = src.[status],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [stationid],
        [versionno],
        [status],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[stationid],
        src.[versionno],
        src.[status],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationStationowner1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationStationowner1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[stationid] = src.[stationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[participantid] = src.[participantid],
        tgt.[stationid] = src.[stationid],
        tgt.[versionno] = src.[versionno],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [participantid],
        [stationid],
        [versionno],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[participantid],
        src.[stationid],
        src.[versionno],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertParticipantRegistrationStationownertrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ParticipantRegistrationStationownertrk1 as tgt 
using (
    select 
        d.[effectivedate],
        d.[participantid],
        d.[versionno],
        d.[authorisedby],
        d.[authoriseddate],
        d.[lastchanged]
    from openjson(@data) with (
        [effectivedate] datetime2,
        [participantid] varchar(10),
        [versionno] decimal(3,0),
        [authorisedby] varchar(15),
        [authoriseddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[participantid] = src.[participantid],
        tgt.[versionno] = src.[versionno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [participantid],
        [versionno],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[participantid],
        src.[versionno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPd7dayCasesolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.Pd7dayCasesolution1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[intervention],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [intervention] decimal(2,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[run_datetime] = src.[run_datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[intervention] = src.[intervention],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [intervention],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[intervention],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPd7dayConstraintsolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.Pd7dayConstraintsolution1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[intervention],
        d.[interval_datetime],
        d.[constraintid],
        d.[rhs],
        d.[marginalvalue],
        d.[violationdegree],
        d.[lhs],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [intervention] decimal(2,0),
        [interval_datetime] datetime2,
        [constraintid] varchar(20),
        [rhs] decimal(15,5),
        [marginalvalue] decimal(15,5),
        [violationdegree] decimal(15,5),
        [lhs] decimal(15,5),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[intervention] = src.[intervention]
    and tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[intervention] = src.[intervention],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[constraintid] = src.[constraintid],
        tgt.[rhs] = src.[rhs],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lhs] = src.[lhs],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [intervention],
        [interval_datetime],
        [constraintid],
        [rhs],
        [marginalvalue],
        [violationdegree],
        [lhs],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[intervention],
        src.[interval_datetime],
        src.[constraintid],
        src.[rhs],
        src.[marginalvalue],
        src.[violationdegree],
        src.[lhs],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPd7dayInterconnectorsolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.Pd7dayInterconnectorsolution1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[intervention],
        d.[interval_datetime],
        d.[interconnectorid],
        d.[meteredmwflow],
        d.[mwflow],
        d.[mwlosses],
        d.[marginalvalue],
        d.[violationdegree],
        d.[exportlimit],
        d.[importlimit],
        d.[marginalloss],
        d.[exportconstraintid],
        d.[importconstraintid],
        d.[fcasexportlimit],
        d.[fcasimportlimit],
        d.[local_price_adjustment_export],
        d.[locally_constrained_export],
        d.[local_price_adjustment_import],
        d.[locally_constrained_import],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [intervention] decimal(2,0),
        [interval_datetime] datetime2,
        [interconnectorid] varchar(20),
        [meteredmwflow] decimal(15,5),
        [mwflow] decimal(15,5),
        [mwlosses] decimal(15,5),
        [marginalvalue] decimal(15,5),
        [violationdegree] decimal(15,5),
        [exportlimit] decimal(15,5),
        [importlimit] decimal(15,5),
        [marginalloss] decimal(15,5),
        [exportconstraintid] varchar(20),
        [importconstraintid] varchar(20),
        [fcasexportlimit] decimal(15,5),
        [fcasimportlimit] decimal(15,5),
        [local_price_adjustment_export] decimal(10,2),
        [locally_constrained_export] decimal(1,0),
        [local_price_adjustment_import] decimal(10,2),
        [locally_constrained_import] decimal(1,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[intervention] = src.[intervention]
    and tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[intervention] = src.[intervention],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[meteredmwflow] = src.[meteredmwflow],
        tgt.[mwflow] = src.[mwflow],
        tgt.[mwlosses] = src.[mwlosses],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[exportlimit] = src.[exportlimit],
        tgt.[importlimit] = src.[importlimit],
        tgt.[marginalloss] = src.[marginalloss],
        tgt.[exportconstraintid] = src.[exportconstraintid],
        tgt.[importconstraintid] = src.[importconstraintid],
        tgt.[fcasexportlimit] = src.[fcasexportlimit],
        tgt.[fcasimportlimit] = src.[fcasimportlimit],
        tgt.[local_price_adjustment_export] = src.[local_price_adjustment_export],
        tgt.[locally_constrained_export] = src.[locally_constrained_export],
        tgt.[local_price_adjustment_import] = src.[local_price_adjustment_import],
        tgt.[locally_constrained_import] = src.[locally_constrained_import],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [intervention],
        [interval_datetime],
        [interconnectorid],
        [meteredmwflow],
        [mwflow],
        [mwlosses],
        [marginalvalue],
        [violationdegree],
        [exportlimit],
        [importlimit],
        [marginalloss],
        [exportconstraintid],
        [importconstraintid],
        [fcasexportlimit],
        [fcasimportlimit],
        [local_price_adjustment_export],
        [locally_constrained_export],
        [local_price_adjustment_import],
        [locally_constrained_import],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[intervention],
        src.[interval_datetime],
        src.[interconnectorid],
        src.[meteredmwflow],
        src.[mwflow],
        src.[mwlosses],
        src.[marginalvalue],
        src.[violationdegree],
        src.[exportlimit],
        src.[importlimit],
        src.[marginalloss],
        src.[exportconstraintid],
        src.[importconstraintid],
        src.[fcasexportlimit],
        src.[fcasimportlimit],
        src.[local_price_adjustment_export],
        src.[locally_constrained_export],
        src.[local_price_adjustment_import],
        src.[locally_constrained_import],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPd7dayMarketSummary2
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.Pd7dayMarketSummary2(
file_log_id,
[run_datetime],
        [interval_datetime],
        [gpg_fuel_forecast_tj]
)
select 
@file_log_id,
d.[run_datetime],
        d.[interval_datetime],
        d.[gpg_fuel_forecast_tj]
from openjson(@data) with (
[run_datetime] datetime2,
        [interval_datetime] datetime2,
        [gpg_fuel_forecast_tj] decimal(15,5)
) d
end
go
create or alter procedure mmsdm_proc.InsertPd7dayPricesolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.Pd7dayPricesolution1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[intervention],
        d.[interval_datetime],
        d.[regionid],
        d.[rrp],
        d.[lower1secrrp],
        d.[lower6secrrp],
        d.[lower60secrrp],
        d.[lower5minrrp],
        d.[lowerregrrp],
        d.[raise1secrrp],
        d.[raise6secrrp],
        d.[raise60secrrp],
        d.[raise5minrrp],
        d.[raiseregrrp],
        d.[lastchanged]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [intervention] decimal(2,0),
        [interval_datetime] datetime2,
        [regionid] varchar(20),
        [rrp] decimal(15,5),
        [lower1secrrp] decimal(15,5),
        [lower6secrrp] decimal(15,5),
        [lower60secrrp] decimal(15,5),
        [lower5minrrp] decimal(15,5),
        [lowerregrrp] decimal(15,5),
        [raise1secrrp] decimal(15,5),
        [raise6secrrp] decimal(15,5),
        [raise60secrrp] decimal(15,5),
        [raise5minrrp] decimal(15,5),
        [raiseregrrp] decimal(15,5),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[intervention] = src.[intervention]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[intervention] = src.[intervention],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[rrp] = src.[rrp],
        tgt.[lower1secrrp] = src.[lower1secrrp],
        tgt.[lower6secrrp] = src.[lower6secrrp],
        tgt.[lower60secrrp] = src.[lower60secrrp],
        tgt.[lower5minrrp] = src.[lower5minrrp],
        tgt.[lowerregrrp] = src.[lowerregrrp],
        tgt.[raise1secrrp] = src.[raise1secrrp],
        tgt.[raise6secrrp] = src.[raise6secrrp],
        tgt.[raise60secrrp] = src.[raise60secrrp],
        tgt.[raise5minrrp] = src.[raise5minrrp],
        tgt.[raiseregrrp] = src.[raiseregrrp],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [intervention],
        [interval_datetime],
        [regionid],
        [rrp],
        [lower1secrrp],
        [lower6secrrp],
        [lower60secrrp],
        [lower5minrrp],
        [lowerregrrp],
        [raise1secrrp],
        [raise6secrrp],
        [raise60secrrp],
        [raise5minrrp],
        [raiseregrrp],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[intervention],
        src.[interval_datetime],
        src.[regionid],
        src.[rrp],
        src.[lower1secrrp],
        src.[lower6secrrp],
        src.[lower60secrrp],
        src.[lower5minrrp],
        src.[lowerregrrp],
        src.[raise1secrrp],
        src.[raise6secrrp],
        src.[raise60secrrp],
        src.[raise5minrrp],
        src.[raiseregrrp],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPdpasaCasesolution3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PdpasaCasesolution3 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[pasaversion] = src.[pasaversion],
        tgt.[reservecondition] = src.[reservecondition],
        tgt.[lorcondition] = src.[lorcondition],
        tgt.[capacityobjfunction] = src.[capacityobjfunction],
        tgt.[capacityoption] = src.[capacityoption],
        tgt.[maxsurplusreserveoption] = src.[maxsurplusreserveoption],
        tgt.[maxsparecapacityoption] = src.[maxsparecapacityoption],
        tgt.[interconnectorflowpenalty] = src.[interconnectorflowpenalty],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[reliabilitylrcdemandoption] = src.[reliabilitylrcdemandoption],
        tgt.[outagelrcdemandoption] = src.[outagelrcdemandoption],
        tgt.[lordemandoption] = src.[lordemandoption],
        tgt.[reliabilitylrccapacityoption] = src.[reliabilitylrccapacityoption],
        tgt.[outagelrccapacityoption] = src.[outagelrccapacityoption],
        tgt.[lorcapacityoption] = src.[lorcapacityoption],
        tgt.[loruigf_option] = src.[loruigf_option],
        tgt.[reliability_lrcuigf_option] = src.[reliability_lrcuigf_option],
        tgt.[outage_lrcuigf_option] = src.[outage_lrcuigf_option]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[pasaversion],
        src.[reservecondition],
        src.[lorcondition],
        src.[capacityobjfunction],
        src.[capacityoption],
        src.[maxsurplusreserveoption],
        src.[maxsparecapacityoption],
        src.[interconnectorflowpenalty],
        src.[lastchanged],
        src.[reliabilitylrcdemandoption],
        src.[outagelrcdemandoption],
        src.[lordemandoption],
        src.[reliabilitylrccapacityoption],
        src.[outagelrccapacityoption],
        src.[lorcapacityoption],
        src.[loruigf_option],
        src.[reliability_lrcuigf_option],
        src.[outage_lrcuigf_option]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPdpasaConstraintsolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PdpasaConstraintsolution1 as tgt 
using (
    select 
        d.[run_datetime],
        d.[interval_datetime],
        d.[constraintid],
        d.[capacityrhs],
        d.[capacitymarginalvalue],
        d.[capacityviolationdegree],
        d.[lastchanged],
        d.[runtype],
        d.[studyregionid]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [interval_datetime] datetime2,
        [constraintid] varchar(20),
        [capacityrhs] decimal(12,2),
        [capacitymarginalvalue] decimal(12,2),
        [capacityviolationdegree] decimal(12,2),
        [lastchanged] datetime2,
        [runtype] varchar(20),
        [studyregionid] varchar(20)
    ) d
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[runtype] = src.[runtype]
    and tgt.[studyregionid] = src.[studyregionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[constraintid] = src.[constraintid],
        tgt.[capacityrhs] = src.[capacityrhs],
        tgt.[capacitymarginalvalue] = src.[capacitymarginalvalue],
        tgt.[capacityviolationdegree] = src.[capacityviolationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[runtype] = src.[runtype],
        tgt.[studyregionid] = src.[studyregionid]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [interval_datetime],
        [constraintid],
        [capacityrhs],
        [capacitymarginalvalue],
        [capacityviolationdegree],
        [lastchanged],
        [runtype],
        [studyregionid]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[constraintid],
        src.[capacityrhs],
        src.[capacitymarginalvalue],
        src.[capacityviolationdegree],
        src.[lastchanged],
        src.[runtype],
        src.[studyregionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPdpasaInterconnectorsoln1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PdpasaInterconnectorsoln1 as tgt 
using (
    select 
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
        d.[importlimitconstraintid],
        d.[studyregionid]
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
        [importlimitconstraintid] varchar(20),
        [studyregionid] varchar(20)
    ) d
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[runtype] = src.[runtype]
    and tgt.[studyregionid] = src.[studyregionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[capacitymwflow] = src.[capacitymwflow],
        tgt.[capacitymarginalvalue] = src.[capacitymarginalvalue],
        tgt.[capacityviolationdegree] = src.[capacityviolationdegree],
        tgt.[calculatedexportlimit] = src.[calculatedexportlimit],
        tgt.[calculatedimportlimit] = src.[calculatedimportlimit],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[runtype] = src.[runtype],
        tgt.[exportlimitconstraintid] = src.[exportlimitconstraintid],
        tgt.[importlimitconstraintid] = src.[importlimitconstraintid],
        tgt.[studyregionid] = src.[studyregionid]
when not matched
    then insert (
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
        [importlimitconstraintid],
        [studyregionid]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[interconnectorid],
        src.[capacitymwflow],
        src.[capacitymarginalvalue],
        src.[capacityviolationdegree],
        src.[calculatedexportlimit],
        src.[calculatedimportlimit],
        src.[lastchanged],
        src.[runtype],
        src.[exportlimitconstraintid],
        src.[importlimitconstraintid],
        src.[studyregionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPdpasaRegionsolution7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PdpasaRegionsolution7 as tgt 
using (
    select 
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
        d.[ss_wind_cleared],
        d.[wdr_available],
        d.[wdr_pasaavailable],
        d.[wdr_capacity]
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
        [ss_wind_cleared] decimal(12,2),
        [wdr_available] decimal(12,2),
        [wdr_pasaavailable] decimal(12,2),
        [wdr_capacity] decimal(12,2)
    ) d
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[demand10] = src.[demand10],
        tgt.[demand50] = src.[demand50],
        tgt.[demand90] = src.[demand90],
        tgt.[reservereq] = src.[reservereq],
        tgt.[capacityreq] = src.[capacityreq],
        tgt.[energyreqdemand50] = src.[energyreqdemand50],
        tgt.[unconstrainedcapacity] = src.[unconstrainedcapacity],
        tgt.[constrainedcapacity] = src.[constrainedcapacity],
        tgt.[netinterchangeunderscarcity] = src.[netinterchangeunderscarcity],
        tgt.[surpluscapacity] = src.[surpluscapacity],
        tgt.[surplusreserve] = src.[surplusreserve],
        tgt.[reservecondition] = src.[reservecondition],
        tgt.[maxsurplusreserve] = src.[maxsurplusreserve],
        tgt.[maxsparecapacity] = src.[maxsparecapacity],
        tgt.[lorcondition] = src.[lorcondition],
        tgt.[aggregatecapacityavailable] = src.[aggregatecapacityavailable],
        tgt.[aggregatescheduledload] = src.[aggregatescheduledload],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[aggregatepasaavailability] = src.[aggregatepasaavailability],
        tgt.[runtype] = src.[runtype],
        tgt.[energyreqdemand10] = src.[energyreqdemand10],
        tgt.[calculatedlor1level] = src.[calculatedlor1level],
        tgt.[calculatedlor2level] = src.[calculatedlor2level],
        tgt.[msrnetinterchangeunderscarcity] = src.[msrnetinterchangeunderscarcity],
        tgt.[lornetinterchangeunderscarcity] = src.[lornetinterchangeunderscarcity],
        tgt.[totalintermittentgeneration] = src.[totalintermittentgeneration],
        tgt.[demand_and_nonschedgen] = src.[demand_and_nonschedgen],
        tgt.[uigf] = src.[uigf],
        tgt.[semi_scheduled_capacity] = src.[semi_scheduled_capacity],
        tgt.[lor_semi_scheduled_capacity] = src.[lor_semi_scheduled_capacity],
        tgt.[lcr] = src.[lcr],
        tgt.[lcr2] = src.[lcr2],
        tgt.[fum] = src.[fum],
        tgt.[ss_solar_uigf] = src.[ss_solar_uigf],
        tgt.[ss_wind_uigf] = src.[ss_wind_uigf],
        tgt.[ss_solar_capacity] = src.[ss_solar_capacity],
        tgt.[ss_wind_capacity] = src.[ss_wind_capacity],
        tgt.[ss_solar_cleared] = src.[ss_solar_cleared],
        tgt.[ss_wind_cleared] = src.[ss_wind_cleared],
        tgt.[wdr_available] = src.[wdr_available],
        tgt.[wdr_pasaavailable] = src.[wdr_pasaavailable],
        tgt.[wdr_capacity] = src.[wdr_capacity]
when not matched
    then insert (
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
        [ss_wind_cleared],
        [wdr_available],
        [wdr_pasaavailable],
        [wdr_capacity]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[regionid],
        src.[demand10],
        src.[demand50],
        src.[demand90],
        src.[reservereq],
        src.[capacityreq],
        src.[energyreqdemand50],
        src.[unconstrainedcapacity],
        src.[constrainedcapacity],
        src.[netinterchangeunderscarcity],
        src.[surpluscapacity],
        src.[surplusreserve],
        src.[reservecondition],
        src.[maxsurplusreserve],
        src.[maxsparecapacity],
        src.[lorcondition],
        src.[aggregatecapacityavailable],
        src.[aggregatescheduledload],
        src.[lastchanged],
        src.[aggregatepasaavailability],
        src.[runtype],
        src.[energyreqdemand10],
        src.[calculatedlor1level],
        src.[calculatedlor2level],
        src.[msrnetinterchangeunderscarcity],
        src.[lornetinterchangeunderscarcity],
        src.[totalintermittentgeneration],
        src.[demand_and_nonschedgen],
        src.[uigf],
        src.[semi_scheduled_capacity],
        src.[lor_semi_scheduled_capacity],
        src.[lcr],
        src.[lcr2],
        src.[fum],
        src.[ss_solar_uigf],
        src.[ss_wind_uigf],
        src.[ss_solar_capacity],
        src.[ss_wind_capacity],
        src.[ss_solar_cleared],
        src.[ss_wind_cleared],
        src.[wdr_available],
        src.[wdr_pasaavailable],
        src.[wdr_capacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchBlockedConstraints1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.PredispatchBlockedConstraints1(
file_log_id,
[predispatchseqno],
        [constraintid]
)
select 
@file_log_id,
d.[predispatchseqno],
        d.[constraintid]
from openjson(@data) with (
[predispatchseqno] varchar(20),
        [constraintid] varchar(20)
) d
end
go
create or alter procedure mmsdm_proc.InsertPredispatchCaseSolution1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchCaseSolution1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[predispatchseqno] = src.[predispatchseqno]
    and tgt.[runno] = src.[runno]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[solutionstatus] = src.[solutionstatus],
        tgt.[spdversion] = src.[spdversion],
        tgt.[nonphysicallosses] = src.[nonphysicallosses],
        tgt.[totalobjective] = src.[totalobjective],
        tgt.[totalareagenviolation] = src.[totalareagenviolation],
        tgt.[totalinterconnectorviolation] = src.[totalinterconnectorviolation],
        tgt.[totalgenericviolation] = src.[totalgenericviolation],
        tgt.[totalramprateviolation] = src.[totalramprateviolation],
        tgt.[totalunitmwcapacityviolation] = src.[totalunitmwcapacityviolation],
        tgt.[total5minviolation] = src.[total5minviolation],
        tgt.[totalregviolation] = src.[totalregviolation],
        tgt.[total6secviolation] = src.[total6secviolation],
        tgt.[total60secviolation] = src.[total60secviolation],
        tgt.[totalasprofileviolation] = src.[totalasprofileviolation],
        tgt.[totalenergyconstrviolation] = src.[totalenergyconstrviolation],
        tgt.[totalenergyofferviolation] = src.[totalenergyofferviolation],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[intervention] = src.[intervention]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[solutionstatus],
        src.[spdversion],
        src.[nonphysicallosses],
        src.[totalobjective],
        src.[totalareagenviolation],
        src.[totalinterconnectorviolation],
        src.[totalgenericviolation],
        src.[totalramprateviolation],
        src.[totalunitmwcapacityviolation],
        src.[total5minviolation],
        src.[totalregviolation],
        src.[total6secviolation],
        src.[total60secviolation],
        src.[totalasprofileviolation],
        src.[totalenergyconstrviolation],
        src.[totalenergyofferviolation],
        src.[lastchanged],
        src.[intervention]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchConstraintSolution5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchConstraintSolution5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[datetime] = src.[datetime]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[constraintid] = src.[constraintid],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[rhs] = src.[rhs],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[datetime] = src.[datetime],
        tgt.[duid] = src.[duid],
        tgt.[genconid_effectivedate] = src.[genconid_effectivedate],
        tgt.[genconid_versionno] = src.[genconid_versionno],
        tgt.[lhs] = src.[lhs]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[constraintid],
        src.[periodid],
        src.[intervention],
        src.[rhs],
        src.[marginalvalue],
        src.[violationdegree],
        src.[lastchanged],
        src.[datetime],
        src.[duid],
        src.[genconid_effectivedate],
        src.[genconid_versionno],
        src.[lhs]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchInterconnectorSoln3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchInterconnectorSoln3 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[meteredmwflow] = src.[meteredmwflow],
        tgt.[mwflow] = src.[mwflow],
        tgt.[mwlosses] = src.[mwlosses],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[datetime] = src.[datetime],
        tgt.[exportlimit] = src.[exportlimit],
        tgt.[importlimit] = src.[importlimit],
        tgt.[marginalloss] = src.[marginalloss],
        tgt.[exportgenconid] = src.[exportgenconid],
        tgt.[importgenconid] = src.[importgenconid],
        tgt.[fcasexportlimit] = src.[fcasexportlimit],
        tgt.[fcasimportlimit] = src.[fcasimportlimit],
        tgt.[local_price_adjustment_export] = src.[local_price_adjustment_export],
        tgt.[locally_constrained_export] = src.[locally_constrained_export],
        tgt.[local_price_adjustment_import] = src.[local_price_adjustment_import],
        tgt.[locally_constrained_import] = src.[locally_constrained_import]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[interconnectorid],
        src.[periodid],
        src.[intervention],
        src.[meteredmwflow],
        src.[mwflow],
        src.[mwlosses],
        src.[marginalvalue],
        src.[violationdegree],
        src.[lastchanged],
        src.[datetime],
        src.[exportlimit],
        src.[importlimit],
        src.[marginalloss],
        src.[exportgenconid],
        src.[importgenconid],
        src.[fcasexportlimit],
        src.[fcasimportlimit],
        src.[local_price_adjustment_export],
        src.[locally_constrained_export],
        src.[local_price_adjustment_import],
        src.[locally_constrained_import]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchInterconnectrSens1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchInterconnectrSens1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[datetime] = src.[datetime],
        tgt.[intervention_active] = src.[intervention_active],
        tgt.[mwflow1] = src.[mwflow1],
        tgt.[mwflow2] = src.[mwflow2],
        tgt.[mwflow3] = src.[mwflow3],
        tgt.[mwflow4] = src.[mwflow4],
        tgt.[mwflow5] = src.[mwflow5],
        tgt.[mwflow6] = src.[mwflow6],
        tgt.[mwflow7] = src.[mwflow7],
        tgt.[mwflow8] = src.[mwflow8],
        tgt.[mwflow9] = src.[mwflow9],
        tgt.[mwflow10] = src.[mwflow10],
        tgt.[mwflow11] = src.[mwflow11],
        tgt.[mwflow12] = src.[mwflow12],
        tgt.[mwflow13] = src.[mwflow13],
        tgt.[mwflow14] = src.[mwflow14],
        tgt.[mwflow15] = src.[mwflow15],
        tgt.[mwflow16] = src.[mwflow16],
        tgt.[mwflow17] = src.[mwflow17],
        tgt.[mwflow18] = src.[mwflow18],
        tgt.[mwflow19] = src.[mwflow19],
        tgt.[mwflow20] = src.[mwflow20],
        tgt.[mwflow21] = src.[mwflow21],
        tgt.[mwflow22] = src.[mwflow22],
        tgt.[mwflow23] = src.[mwflow23],
        tgt.[mwflow24] = src.[mwflow24],
        tgt.[mwflow25] = src.[mwflow25],
        tgt.[mwflow26] = src.[mwflow26],
        tgt.[mwflow27] = src.[mwflow27],
        tgt.[mwflow28] = src.[mwflow28],
        tgt.[mwflow29] = src.[mwflow29],
        tgt.[mwflow30] = src.[mwflow30],
        tgt.[mwflow31] = src.[mwflow31],
        tgt.[mwflow32] = src.[mwflow32],
        tgt.[mwflow33] = src.[mwflow33],
        tgt.[mwflow34] = src.[mwflow34],
        tgt.[mwflow35] = src.[mwflow35],
        tgt.[mwflow36] = src.[mwflow36],
        tgt.[mwflow37] = src.[mwflow37],
        tgt.[mwflow38] = src.[mwflow38],
        tgt.[mwflow39] = src.[mwflow39],
        tgt.[mwflow40] = src.[mwflow40],
        tgt.[mwflow41] = src.[mwflow41],
        tgt.[mwflow42] = src.[mwflow42],
        tgt.[mwflow43] = src.[mwflow43],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[interconnectorid],
        src.[periodid],
        src.[intervention],
        src.[datetime],
        src.[intervention_active],
        src.[mwflow1],
        src.[mwflow2],
        src.[mwflow3],
        src.[mwflow4],
        src.[mwflow5],
        src.[mwflow6],
        src.[mwflow7],
        src.[mwflow8],
        src.[mwflow9],
        src.[mwflow10],
        src.[mwflow11],
        src.[mwflow12],
        src.[mwflow13],
        src.[mwflow14],
        src.[mwflow15],
        src.[mwflow16],
        src.[mwflow17],
        src.[mwflow18],
        src.[mwflow19],
        src.[mwflow20],
        src.[mwflow21],
        src.[mwflow22],
        src.[mwflow23],
        src.[mwflow24],
        src.[mwflow25],
        src.[mwflow26],
        src.[mwflow27],
        src.[mwflow28],
        src.[mwflow29],
        src.[mwflow30],
        src.[mwflow31],
        src.[mwflow32],
        src.[mwflow33],
        src.[mwflow34],
        src.[mwflow35],
        src.[mwflow36],
        src.[mwflow37],
        src.[mwflow38],
        src.[mwflow39],
        src.[mwflow40],
        src.[mwflow41],
        src.[mwflow42],
        src.[mwflow43],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchUnitSolution4
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchUnitSolution4 as tgt 
using (
    select 
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
        d.[semidispatchcap],
        d.[conformance_mode],
        d.[uigf],
        d.[raise1sec],
        d.[raise1secflags],
        d.[lower1sec],
        d.[lower1secflags],
        d.[raise1secactualavailability],
        d.[lower1secactualavailability],
        d.[initial_energy_storage],
        d.[energy_storage],
        d.[energy_storage_min],
        d.[energy_storage_max],
        d.[min_availability]
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
        [semidispatchcap] decimal(3,0),
        [conformance_mode] decimal(6,0),
        [uigf] decimal(15,5),
        [raise1sec] decimal(15,5),
        [raise1secflags] decimal(3,0),
        [lower1sec] decimal(15,5),
        [lower1secflags] decimal(3,0),
        [raise1secactualavailability] decimal(16,6),
        [lower1secactualavailability] decimal(16,6),
        [initial_energy_storage] decimal(15,5),
        [energy_storage] decimal(15,5),
        [energy_storage_min] decimal(15,5),
        [energy_storage_max] decimal(15,5),
        [min_availability] decimal(15,5)
    ) d
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[duid] = src.[duid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[duid] = src.[duid],
        tgt.[tradetype] = src.[tradetype],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[agcstatus] = src.[agcstatus],
        tgt.[dispatchmode] = src.[dispatchmode],
        tgt.[initialmw] = src.[initialmw],
        tgt.[totalcleared] = src.[totalcleared],
        tgt.[lower5min] = src.[lower5min],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise5min] = src.[raise5min],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[rampdownrate] = src.[rampdownrate],
        tgt.[rampuprate] = src.[rampuprate],
        tgt.[downepf] = src.[downepf],
        tgt.[upepf] = src.[upepf],
        tgt.[marginal5minvalue] = src.[marginal5minvalue],
        tgt.[marginal60secvalue] = src.[marginal60secvalue],
        tgt.[marginal6secvalue] = src.[marginal6secvalue],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[violation5mindegree] = src.[violation5mindegree],
        tgt.[violation60secdegree] = src.[violation60secdegree],
        tgt.[violation6secdegree] = src.[violation6secdegree],
        tgt.[violationdegree] = src.[violationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[datetime] = src.[datetime],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[raisereg] = src.[raisereg],
        tgt.[availability] = src.[availability],
        tgt.[raise6secflags] = src.[raise6secflags],
        tgt.[raise60secflags] = src.[raise60secflags],
        tgt.[raise5minflags] = src.[raise5minflags],
        tgt.[raiseregflags] = src.[raiseregflags],
        tgt.[lower6secflags] = src.[lower6secflags],
        tgt.[lower60secflags] = src.[lower60secflags],
        tgt.[lower5minflags] = src.[lower5minflags],
        tgt.[lowerregflags] = src.[lowerregflags],
        tgt.[raise6secactualavailability] = src.[raise6secactualavailability],
        tgt.[raise60secactualavailability] = src.[raise60secactualavailability],
        tgt.[raise5minactualavailability] = src.[raise5minactualavailability],
        tgt.[raiseregactualavailability] = src.[raiseregactualavailability],
        tgt.[lower6secactualavailability] = src.[lower6secactualavailability],
        tgt.[lower60secactualavailability] = src.[lower60secactualavailability],
        tgt.[lower5minactualavailability] = src.[lower5minactualavailability],
        tgt.[lowerregactualavailability] = src.[lowerregactualavailability],
        tgt.[semidispatchcap] = src.[semidispatchcap],
        tgt.[conformance_mode] = src.[conformance_mode],
        tgt.[uigf] = src.[uigf],
        tgt.[raise1sec] = src.[raise1sec],
        tgt.[raise1secflags] = src.[raise1secflags],
        tgt.[lower1sec] = src.[lower1sec],
        tgt.[lower1secflags] = src.[lower1secflags],
        tgt.[raise1secactualavailability] = src.[raise1secactualavailability],
        tgt.[lower1secactualavailability] = src.[lower1secactualavailability],
        tgt.[initial_energy_storage] = src.[initial_energy_storage],
        tgt.[energy_storage] = src.[energy_storage],
        tgt.[energy_storage_min] = src.[energy_storage_min],
        tgt.[energy_storage_max] = src.[energy_storage_max],
        tgt.[min_availability] = src.[min_availability]
when not matched
    then insert (
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
        [semidispatchcap],
        [conformance_mode],
        [uigf],
        [raise1sec],
        [raise1secflags],
        [lower1sec],
        [lower1secflags],
        [raise1secactualavailability],
        [lower1secactualavailability],
        [initial_energy_storage],
        [energy_storage],
        [energy_storage_min],
        [energy_storage_max],
        [min_availability]
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[duid],
        src.[tradetype],
        src.[periodid],
        src.[intervention],
        src.[connectionpointid],
        src.[agcstatus],
        src.[dispatchmode],
        src.[initialmw],
        src.[totalcleared],
        src.[lower5min],
        src.[lower60sec],
        src.[lower6sec],
        src.[raise5min],
        src.[raise60sec],
        src.[raise6sec],
        src.[rampdownrate],
        src.[rampuprate],
        src.[downepf],
        src.[upepf],
        src.[marginal5minvalue],
        src.[marginal60secvalue],
        src.[marginal6secvalue],
        src.[marginalvalue],
        src.[violation5mindegree],
        src.[violation60secdegree],
        src.[violation6secdegree],
        src.[violationdegree],
        src.[lastchanged],
        src.[datetime],
        src.[lowerreg],
        src.[raisereg],
        src.[availability],
        src.[raise6secflags],
        src.[raise60secflags],
        src.[raise5minflags],
        src.[raiseregflags],
        src.[lower6secflags],
        src.[lower60secflags],
        src.[lower5minflags],
        src.[lowerregflags],
        src.[raise6secactualavailability],
        src.[raise60secactualavailability],
        src.[raise5minactualavailability],
        src.[raiseregactualavailability],
        src.[lower6secactualavailability],
        src.[lower60secactualavailability],
        src.[lower5minactualavailability],
        src.[lowerregactualavailability],
        src.[semidispatchcap],
        src.[conformance_mode],
        src.[uigf],
        src.[raise1sec],
        src.[raise1secflags],
        src.[lower1sec],
        src.[lower1secflags],
        src.[raise1secactualavailability],
        src.[lower1secactualavailability],
        src.[initial_energy_storage],
        src.[energy_storage],
        src.[energy_storage_min],
        src.[energy_storage_max],
        src.[min_availability]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchOffertrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchOffertrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[duid] = src.[duid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[predispatchseqno] = src.[predispatchseqno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[duid] = src.[duid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[periodid] = src.[periodid],
        tgt.[bidsettlementdate] = src.[bidsettlementdate],
        tgt.[bidofferdate] = src.[bidofferdate],
        tgt.[datetime] = src.[datetime],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [predispatchseqno],
        [duid],
        [bidtype],
        [periodid],
        [bidsettlementdate],
        [bidofferdate],
        [datetime],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[duid],
        src.[bidtype],
        src.[periodid],
        src.[bidsettlementdate],
        src.[bidofferdate],
        src.[datetime],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchRegionPrices2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchRegionPrices2 as tgt 
using (
    select 
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
        d.[lowerregrrp],
        d.[raise1secrrp],
        d.[lower1secrrp]
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
        [lowerregrrp] decimal(15,5),
        [raise1secrrp] decimal(15,5),
        [lower1secrrp] decimal(15,5)
    ) d
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[rrp] = src.[rrp],
        tgt.[eep] = src.[eep],
        tgt.[rrp1] = src.[rrp1],
        tgt.[eep1] = src.[eep1],
        tgt.[rrp2] = src.[rrp2],
        tgt.[eep2] = src.[eep2],
        tgt.[rrp3] = src.[rrp3],
        tgt.[eep3] = src.[eep3],
        tgt.[rrp4] = src.[rrp4],
        tgt.[eep4] = src.[eep4],
        tgt.[rrp5] = src.[rrp5],
        tgt.[eep5] = src.[eep5],
        tgt.[rrp6] = src.[rrp6],
        tgt.[eep6] = src.[eep6],
        tgt.[rrp7] = src.[rrp7],
        tgt.[eep7] = src.[eep7],
        tgt.[rrp8] = src.[rrp8],
        tgt.[eep8] = src.[eep8],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[datetime] = src.[datetime],
        tgt.[raise6secrrp] = src.[raise6secrrp],
        tgt.[raise60secrrp] = src.[raise60secrrp],
        tgt.[raise5minrrp] = src.[raise5minrrp],
        tgt.[raiseregrrp] = src.[raiseregrrp],
        tgt.[lower6secrrp] = src.[lower6secrrp],
        tgt.[lower60secrrp] = src.[lower60secrrp],
        tgt.[lower5minrrp] = src.[lower5minrrp],
        tgt.[lowerregrrp] = src.[lowerregrrp],
        tgt.[raise1secrrp] = src.[raise1secrrp],
        tgt.[lower1secrrp] = src.[lower1secrrp]
when not matched
    then insert (
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
        [lowerregrrp],
        [raise1secrrp],
        [lower1secrrp]
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[regionid],
        src.[periodid],
        src.[intervention],
        src.[rrp],
        src.[eep],
        src.[rrp1],
        src.[eep1],
        src.[rrp2],
        src.[eep2],
        src.[rrp3],
        src.[eep3],
        src.[rrp4],
        src.[eep4],
        src.[rrp5],
        src.[eep5],
        src.[rrp6],
        src.[eep6],
        src.[rrp7],
        src.[eep7],
        src.[rrp8],
        src.[eep8],
        src.[lastchanged],
        src.[datetime],
        src.[raise6secrrp],
        src.[raise60secrrp],
        src.[raise5minrrp],
        src.[raiseregrrp],
        src.[lower6secrrp],
        src.[lower60secrrp],
        src.[lower5minrrp],
        src.[lowerregrrp],
        src.[raise1secrrp],
        src.[lower1secrrp]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchPricesensitivities1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchPricesensitivities1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[rrpeep1] = src.[rrpeep1],
        tgt.[rrpeep2] = src.[rrpeep2],
        tgt.[rrpeep3] = src.[rrpeep3],
        tgt.[rrpeep4] = src.[rrpeep4],
        tgt.[rrpeep5] = src.[rrpeep5],
        tgt.[rrpeep6] = src.[rrpeep6],
        tgt.[rrpeep7] = src.[rrpeep7],
        tgt.[rrpeep8] = src.[rrpeep8],
        tgt.[rrpeep9] = src.[rrpeep9],
        tgt.[rrpeep10] = src.[rrpeep10],
        tgt.[rrpeep11] = src.[rrpeep11],
        tgt.[rrpeep12] = src.[rrpeep12],
        tgt.[rrpeep13] = src.[rrpeep13],
        tgt.[rrpeep14] = src.[rrpeep14],
        tgt.[rrpeep15] = src.[rrpeep15],
        tgt.[rrpeep16] = src.[rrpeep16],
        tgt.[rrpeep17] = src.[rrpeep17],
        tgt.[rrpeep18] = src.[rrpeep18],
        tgt.[rrpeep19] = src.[rrpeep19],
        tgt.[rrpeep20] = src.[rrpeep20],
        tgt.[rrpeep21] = src.[rrpeep21],
        tgt.[rrpeep22] = src.[rrpeep22],
        tgt.[rrpeep23] = src.[rrpeep23],
        tgt.[rrpeep24] = src.[rrpeep24],
        tgt.[rrpeep25] = src.[rrpeep25],
        tgt.[rrpeep26] = src.[rrpeep26],
        tgt.[rrpeep27] = src.[rrpeep27],
        tgt.[rrpeep28] = src.[rrpeep28],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[datetime] = src.[datetime],
        tgt.[rrpeep29] = src.[rrpeep29],
        tgt.[rrpeep30] = src.[rrpeep30],
        tgt.[rrpeep31] = src.[rrpeep31],
        tgt.[rrpeep32] = src.[rrpeep32],
        tgt.[rrpeep33] = src.[rrpeep33],
        tgt.[rrpeep34] = src.[rrpeep34],
        tgt.[rrpeep35] = src.[rrpeep35],
        tgt.[intervention_active] = src.[intervention_active],
        tgt.[rrpeep36] = src.[rrpeep36],
        tgt.[rrpeep37] = src.[rrpeep37],
        tgt.[rrpeep38] = src.[rrpeep38],
        tgt.[rrpeep39] = src.[rrpeep39],
        tgt.[rrpeep40] = src.[rrpeep40],
        tgt.[rrpeep41] = src.[rrpeep41],
        tgt.[rrpeep42] = src.[rrpeep42],
        tgt.[rrpeep43] = src.[rrpeep43]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[regionid],
        src.[periodid],
        src.[intervention],
        src.[rrpeep1],
        src.[rrpeep2],
        src.[rrpeep3],
        src.[rrpeep4],
        src.[rrpeep5],
        src.[rrpeep6],
        src.[rrpeep7],
        src.[rrpeep8],
        src.[rrpeep9],
        src.[rrpeep10],
        src.[rrpeep11],
        src.[rrpeep12],
        src.[rrpeep13],
        src.[rrpeep14],
        src.[rrpeep15],
        src.[rrpeep16],
        src.[rrpeep17],
        src.[rrpeep18],
        src.[rrpeep19],
        src.[rrpeep20],
        src.[rrpeep21],
        src.[rrpeep22],
        src.[rrpeep23],
        src.[rrpeep24],
        src.[rrpeep25],
        src.[rrpeep26],
        src.[rrpeep27],
        src.[rrpeep28],
        src.[lastchanged],
        src.[datetime],
        src.[rrpeep29],
        src.[rrpeep30],
        src.[rrpeep31],
        src.[rrpeep32],
        src.[rrpeep33],
        src.[rrpeep34],
        src.[rrpeep35],
        src.[intervention_active],
        src.[rrpeep36],
        src.[rrpeep37],
        src.[rrpeep38],
        src.[rrpeep39],
        src.[rrpeep40],
        src.[rrpeep41],
        src.[rrpeep42],
        src.[rrpeep43]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchRegionSolution8
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchRegionSolution8 as tgt 
using (
    select 
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
        d.[ss_wind_compliancemw],
        d.[wdr_initialmw],
        d.[wdr_available],
        d.[wdr_dispatched],
        d.[ss_solar_availability],
        d.[ss_wind_availability],
        d.[raise1seclocaldispatch],
        d.[lower1seclocaldispatch],
        d.[raise1secactualavailability],
        d.[lower1secactualavailability],
        d.[bdu_energy_storage],
        d.[bdu_min_avail],
        d.[bdu_max_avail],
        d.[bdu_clearedmw_gen],
        d.[bdu_clearedmw_load]
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
        [ss_wind_compliancemw] decimal(15,5),
        [wdr_initialmw] decimal(15,5),
        [wdr_available] decimal(15,5),
        [wdr_dispatched] decimal(15,5),
        [ss_solar_availability] decimal(15,5),
        [ss_wind_availability] decimal(15,5),
        [raise1seclocaldispatch] decimal(15,5),
        [lower1seclocaldispatch] decimal(15,5),
        [raise1secactualavailability] decimal(16,6),
        [lower1secactualavailability] decimal(16,6),
        [bdu_energy_storage] decimal(15,5),
        [bdu_min_avail] decimal(15,5),
        [bdu_max_avail] decimal(15,5),
        [bdu_clearedmw_gen] decimal(15,5),
        [bdu_clearedmw_load] decimal(15,5)
    ) d
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[intervention] = src.[intervention],
        tgt.[totaldemand] = src.[totaldemand],
        tgt.[availablegeneration] = src.[availablegeneration],
        tgt.[availableload] = src.[availableload],
        tgt.[demandforecast] = src.[demandforecast],
        tgt.[dispatchablegeneration] = src.[dispatchablegeneration],
        tgt.[dispatchableload] = src.[dispatchableload],
        tgt.[netinterchange] = src.[netinterchange],
        tgt.[excessgeneration] = src.[excessgeneration],
        tgt.[lower5mindispatch] = src.[lower5mindispatch],
        tgt.[lower5minimport] = src.[lower5minimport],
        tgt.[lower5minlocaldispatch] = src.[lower5minlocaldispatch],
        tgt.[lower5minlocalprice] = src.[lower5minlocalprice],
        tgt.[lower5minlocalreq] = src.[lower5minlocalreq],
        tgt.[lower5minprice] = src.[lower5minprice],
        tgt.[lower5minreq] = src.[lower5minreq],
        tgt.[lower5minsupplyprice] = src.[lower5minsupplyprice],
        tgt.[lower60secdispatch] = src.[lower60secdispatch],
        tgt.[lower60secimport] = src.[lower60secimport],
        tgt.[lower60seclocaldispatch] = src.[lower60seclocaldispatch],
        tgt.[lower60seclocalprice] = src.[lower60seclocalprice],
        tgt.[lower60seclocalreq] = src.[lower60seclocalreq],
        tgt.[lower60secprice] = src.[lower60secprice],
        tgt.[lower60secreq] = src.[lower60secreq],
        tgt.[lower60secsupplyprice] = src.[lower60secsupplyprice],
        tgt.[lower6secdispatch] = src.[lower6secdispatch],
        tgt.[lower6secimport] = src.[lower6secimport],
        tgt.[lower6seclocaldispatch] = src.[lower6seclocaldispatch],
        tgt.[lower6seclocalprice] = src.[lower6seclocalprice],
        tgt.[lower6seclocalreq] = src.[lower6seclocalreq],
        tgt.[lower6secprice] = src.[lower6secprice],
        tgt.[lower6secreq] = src.[lower6secreq],
        tgt.[lower6secsupplyprice] = src.[lower6secsupplyprice],
        tgt.[raise5mindispatch] = src.[raise5mindispatch],
        tgt.[raise5minimport] = src.[raise5minimport],
        tgt.[raise5minlocaldispatch] = src.[raise5minlocaldispatch],
        tgt.[raise5minlocalprice] = src.[raise5minlocalprice],
        tgt.[raise5minlocalreq] = src.[raise5minlocalreq],
        tgt.[raise5minprice] = src.[raise5minprice],
        tgt.[raise5minreq] = src.[raise5minreq],
        tgt.[raise5minsupplyprice] = src.[raise5minsupplyprice],
        tgt.[raise60secdispatch] = src.[raise60secdispatch],
        tgt.[raise60secimport] = src.[raise60secimport],
        tgt.[raise60seclocaldispatch] = src.[raise60seclocaldispatch],
        tgt.[raise60seclocalprice] = src.[raise60seclocalprice],
        tgt.[raise60seclocalreq] = src.[raise60seclocalreq],
        tgt.[raise60secprice] = src.[raise60secprice],
        tgt.[raise60secreq] = src.[raise60secreq],
        tgt.[raise60secsupplyprice] = src.[raise60secsupplyprice],
        tgt.[raise6secdispatch] = src.[raise6secdispatch],
        tgt.[raise6secimport] = src.[raise6secimport],
        tgt.[raise6seclocaldispatch] = src.[raise6seclocaldispatch],
        tgt.[raise6seclocalprice] = src.[raise6seclocalprice],
        tgt.[raise6seclocalreq] = src.[raise6seclocalreq],
        tgt.[raise6secprice] = src.[raise6secprice],
        tgt.[raise6secreq] = src.[raise6secreq],
        tgt.[raise6secsupplyprice] = src.[raise6secsupplyprice],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[datetime] = src.[datetime],
        tgt.[initialsupply] = src.[initialsupply],
        tgt.[clearedsupply] = src.[clearedsupply],
        tgt.[lowerregimport] = src.[lowerregimport],
        tgt.[lowerreglocaldispatch] = src.[lowerreglocaldispatch],
        tgt.[lowerreglocalreq] = src.[lowerreglocalreq],
        tgt.[lowerregreq] = src.[lowerregreq],
        tgt.[raiseregimport] = src.[raiseregimport],
        tgt.[raisereglocaldispatch] = src.[raisereglocaldispatch],
        tgt.[raisereglocalreq] = src.[raisereglocalreq],
        tgt.[raiseregreq] = src.[raiseregreq],
        tgt.[raise5minlocalviolation] = src.[raise5minlocalviolation],
        tgt.[raisereglocalviolation] = src.[raisereglocalviolation],
        tgt.[raise60seclocalviolation] = src.[raise60seclocalviolation],
        tgt.[raise6seclocalviolation] = src.[raise6seclocalviolation],
        tgt.[lower5minlocalviolation] = src.[lower5minlocalviolation],
        tgt.[lowerreglocalviolation] = src.[lowerreglocalviolation],
        tgt.[lower60seclocalviolation] = src.[lower60seclocalviolation],
        tgt.[lower6seclocalviolation] = src.[lower6seclocalviolation],
        tgt.[raise5minviolation] = src.[raise5minviolation],
        tgt.[raiseregviolation] = src.[raiseregviolation],
        tgt.[raise60secviolation] = src.[raise60secviolation],
        tgt.[raise6secviolation] = src.[raise6secviolation],
        tgt.[lower5minviolation] = src.[lower5minviolation],
        tgt.[lowerregviolation] = src.[lowerregviolation],
        tgt.[lower60secviolation] = src.[lower60secviolation],
        tgt.[lower6secviolation] = src.[lower6secviolation],
        tgt.[raise6secactualavailability] = src.[raise6secactualavailability],
        tgt.[raise60secactualavailability] = src.[raise60secactualavailability],
        tgt.[raise5minactualavailability] = src.[raise5minactualavailability],
        tgt.[raiseregactualavailability] = src.[raiseregactualavailability],
        tgt.[lower6secactualavailability] = src.[lower6secactualavailability],
        tgt.[lower60secactualavailability] = src.[lower60secactualavailability],
        tgt.[lower5minactualavailability] = src.[lower5minactualavailability],
        tgt.[lowerregactualavailability] = src.[lowerregactualavailability],
        tgt.[decavailability] = src.[decavailability],
        tgt.[lorsurplus] = src.[lorsurplus],
        tgt.[lrcsurplus] = src.[lrcsurplus],
        tgt.[totalintermittentgeneration] = src.[totalintermittentgeneration],
        tgt.[demand_and_nonschedgen] = src.[demand_and_nonschedgen],
        tgt.[uigf] = src.[uigf],
        tgt.[semischedule_clearedmw] = src.[semischedule_clearedmw],
        tgt.[semischedule_compliancemw] = src.[semischedule_compliancemw],
        tgt.[ss_solar_uigf] = src.[ss_solar_uigf],
        tgt.[ss_wind_uigf] = src.[ss_wind_uigf],
        tgt.[ss_solar_clearedmw] = src.[ss_solar_clearedmw],
        tgt.[ss_wind_clearedmw] = src.[ss_wind_clearedmw],
        tgt.[ss_solar_compliancemw] = src.[ss_solar_compliancemw],
        tgt.[ss_wind_compliancemw] = src.[ss_wind_compliancemw],
        tgt.[wdr_initialmw] = src.[wdr_initialmw],
        tgt.[wdr_available] = src.[wdr_available],
        tgt.[wdr_dispatched] = src.[wdr_dispatched],
        tgt.[ss_solar_availability] = src.[ss_solar_availability],
        tgt.[ss_wind_availability] = src.[ss_wind_availability],
        tgt.[raise1seclocaldispatch] = src.[raise1seclocaldispatch],
        tgt.[lower1seclocaldispatch] = src.[lower1seclocaldispatch],
        tgt.[raise1secactualavailability] = src.[raise1secactualavailability],
        tgt.[lower1secactualavailability] = src.[lower1secactualavailability],
        tgt.[bdu_energy_storage] = src.[bdu_energy_storage],
        tgt.[bdu_min_avail] = src.[bdu_min_avail],
        tgt.[bdu_max_avail] = src.[bdu_max_avail],
        tgt.[bdu_clearedmw_gen] = src.[bdu_clearedmw_gen],
        tgt.[bdu_clearedmw_load] = src.[bdu_clearedmw_load]
when not matched
    then insert (
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
        [ss_wind_compliancemw],
        [wdr_initialmw],
        [wdr_available],
        [wdr_dispatched],
        [ss_solar_availability],
        [ss_wind_availability],
        [raise1seclocaldispatch],
        [lower1seclocaldispatch],
        [raise1secactualavailability],
        [lower1secactualavailability],
        [bdu_energy_storage],
        [bdu_min_avail],
        [bdu_max_avail],
        [bdu_clearedmw_gen],
        [bdu_clearedmw_load]
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[regionid],
        src.[periodid],
        src.[intervention],
        src.[totaldemand],
        src.[availablegeneration],
        src.[availableload],
        src.[demandforecast],
        src.[dispatchablegeneration],
        src.[dispatchableload],
        src.[netinterchange],
        src.[excessgeneration],
        src.[lower5mindispatch],
        src.[lower5minimport],
        src.[lower5minlocaldispatch],
        src.[lower5minlocalprice],
        src.[lower5minlocalreq],
        src.[lower5minprice],
        src.[lower5minreq],
        src.[lower5minsupplyprice],
        src.[lower60secdispatch],
        src.[lower60secimport],
        src.[lower60seclocaldispatch],
        src.[lower60seclocalprice],
        src.[lower60seclocalreq],
        src.[lower60secprice],
        src.[lower60secreq],
        src.[lower60secsupplyprice],
        src.[lower6secdispatch],
        src.[lower6secimport],
        src.[lower6seclocaldispatch],
        src.[lower6seclocalprice],
        src.[lower6seclocalreq],
        src.[lower6secprice],
        src.[lower6secreq],
        src.[lower6secsupplyprice],
        src.[raise5mindispatch],
        src.[raise5minimport],
        src.[raise5minlocaldispatch],
        src.[raise5minlocalprice],
        src.[raise5minlocalreq],
        src.[raise5minprice],
        src.[raise5minreq],
        src.[raise5minsupplyprice],
        src.[raise60secdispatch],
        src.[raise60secimport],
        src.[raise60seclocaldispatch],
        src.[raise60seclocalprice],
        src.[raise60seclocalreq],
        src.[raise60secprice],
        src.[raise60secreq],
        src.[raise60secsupplyprice],
        src.[raise6secdispatch],
        src.[raise6secimport],
        src.[raise6seclocaldispatch],
        src.[raise6seclocalprice],
        src.[raise6seclocalreq],
        src.[raise6secprice],
        src.[raise6secreq],
        src.[raise6secsupplyprice],
        src.[lastchanged],
        src.[datetime],
        src.[initialsupply],
        src.[clearedsupply],
        src.[lowerregimport],
        src.[lowerreglocaldispatch],
        src.[lowerreglocalreq],
        src.[lowerregreq],
        src.[raiseregimport],
        src.[raisereglocaldispatch],
        src.[raisereglocalreq],
        src.[raiseregreq],
        src.[raise5minlocalviolation],
        src.[raisereglocalviolation],
        src.[raise60seclocalviolation],
        src.[raise6seclocalviolation],
        src.[lower5minlocalviolation],
        src.[lowerreglocalviolation],
        src.[lower60seclocalviolation],
        src.[lower6seclocalviolation],
        src.[raise5minviolation],
        src.[raiseregviolation],
        src.[raise60secviolation],
        src.[raise6secviolation],
        src.[lower5minviolation],
        src.[lowerregviolation],
        src.[lower60secviolation],
        src.[lower6secviolation],
        src.[raise6secactualavailability],
        src.[raise60secactualavailability],
        src.[raise5minactualavailability],
        src.[raiseregactualavailability],
        src.[lower6secactualavailability],
        src.[lower60secactualavailability],
        src.[lower5minactualavailability],
        src.[lowerregactualavailability],
        src.[decavailability],
        src.[lorsurplus],
        src.[lrcsurplus],
        src.[totalintermittentgeneration],
        src.[demand_and_nonschedgen],
        src.[uigf],
        src.[semischedule_clearedmw],
        src.[semischedule_compliancemw],
        src.[ss_solar_uigf],
        src.[ss_wind_uigf],
        src.[ss_solar_clearedmw],
        src.[ss_wind_clearedmw],
        src.[ss_solar_compliancemw],
        src.[ss_wind_compliancemw],
        src.[wdr_initialmw],
        src.[wdr_available],
        src.[wdr_dispatched],
        src.[ss_solar_availability],
        src.[ss_wind_availability],
        src.[raise1seclocaldispatch],
        src.[lower1seclocaldispatch],
        src.[raise1secactualavailability],
        src.[lower1secactualavailability],
        src.[bdu_energy_storage],
        src.[bdu_min_avail],
        src.[bdu_max_avail],
        src.[bdu_clearedmw_gen],
        src.[bdu_clearedmw_load]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchScenarioDemand1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.PredispatchScenarioDemand1(
file_log_id,
[effectivedate],
        [versionno],
        [scenario],
        [regionid],
        [deltamw]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertPredispatchScenarioDemandTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchScenarioDemandTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchRegionfcasrequirement2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchRegionfcasrequirement2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[datetime] = src.[datetime]
    and tgt.[genconid] = src.[genconid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[intervention] = src.[intervention]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[runno] = src.[runno],
        tgt.[intervention] = src.[intervention],
        tgt.[periodid] = src.[periodid],
        tgt.[genconid] = src.[genconid],
        tgt.[regionid] = src.[regionid],
        tgt.[bidtype] = src.[bidtype],
        tgt.[genconeffectivedate] = src.[genconeffectivedate],
        tgt.[genconversionno] = src.[genconversionno],
        tgt.[marginalvalue] = src.[marginalvalue],
        tgt.[datetime] = src.[datetime],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[base_cost] = src.[base_cost],
        tgt.[adjusted_cost] = src.[adjusted_cost],
        tgt.[estimated_cmpf] = src.[estimated_cmpf],
        tgt.[estimated_crmpf] = src.[estimated_crmpf],
        tgt.[recovery_factor_cmpf] = src.[recovery_factor_cmpf],
        tgt.[recovery_factor_crmpf] = src.[recovery_factor_crmpf]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[runno],
        src.[intervention],
        src.[periodid],
        src.[genconid],
        src.[regionid],
        src.[bidtype],
        src.[genconeffectivedate],
        src.[genconversionno],
        src.[marginalvalue],
        src.[datetime],
        src.[lastchanged],
        src.[base_cost],
        src.[adjusted_cost],
        src.[estimated_cmpf],
        src.[estimated_crmpf],
        src.[recovery_factor_cmpf],
        src.[recovery_factor_crmpf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchLocalPrice1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchLocalPrice1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[datetime] = src.[datetime]
    and tgt.[duid] = src.[duid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[datetime] = src.[datetime],
        tgt.[duid] = src.[duid],
        tgt.[periodid] = src.[periodid],
        tgt.[local_price_adjustment] = src.[local_price_adjustment],
        tgt.[locally_constrained] = src.[locally_constrained],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [predispatchseqno],
        [datetime],
        [duid],
        [periodid],
        [local_price_adjustment],
        [locally_constrained],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[datetime],
        src.[duid],
        src.[periodid],
        src.[local_price_adjustment],
        src.[locally_constrained],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPredispatchMnspbidtrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PredispatchMnspbidtrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[linkid] = src.[linkid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[predispatchseqno] = src.[predispatchseqno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[predispatchseqno] = src.[predispatchseqno],
        tgt.[linkid] = src.[linkid],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[offerdate] = src.[offerdate],
        tgt.[versionno] = src.[versionno],
        tgt.[datetime] = src.[datetime],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[predispatchseqno],
        src.[linkid],
        src.[periodid],
        src.[participantid],
        src.[settlementdate],
        src.[offerdate],
        src.[versionno],
        src.[datetime],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPrudentialCompanyPosition1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PrudentialCompanyPosition1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[company_id] = src.[company_id]
    and tgt.[prudential_date] = src.[prudential_date]
    and tgt.[runno] = src.[runno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[prudential_date] = src.[prudential_date],
        tgt.[runno] = src.[runno],
        tgt.[company_id] = src.[company_id],
        tgt.[mcl] = src.[mcl],
        tgt.[credit_support] = src.[credit_support],
        tgt.[trading_limit] = src.[trading_limit],
        tgt.[current_amount_balance] = src.[current_amount_balance],
        tgt.[security_deposit_provision] = src.[security_deposit_provision],
        tgt.[security_deposit_offset] = src.[security_deposit_offset],
        tgt.[security_deposit_balance] = src.[security_deposit_balance],
        tgt.[expost_realloc_balance] = src.[expost_realloc_balance],
        tgt.[default_balance] = src.[default_balance],
        tgt.[outstandings] = src.[outstandings],
        tgt.[trading_margin] = src.[trading_margin],
        tgt.[typical_accrual] = src.[typical_accrual],
        tgt.[prudential_margin] = src.[prudential_margin],
        tgt.[early_payment_amount] = src.[early_payment_amount],
        tgt.[percentage_outstandings] = src.[percentage_outstandings],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[prudential_date],
        src.[runno],
        src.[company_id],
        src.[mcl],
        src.[credit_support],
        src.[trading_limit],
        src.[current_amount_balance],
        src.[security_deposit_provision],
        src.[security_deposit_offset],
        src.[security_deposit_balance],
        src.[expost_realloc_balance],
        src.[default_balance],
        src.[outstandings],
        src.[trading_margin],
        src.[typical_accrual],
        src.[prudential_margin],
        src.[early_payment_amount],
        src.[percentage_outstandings],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertPrudentialRuntrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.PrudentialRuntrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[prudential_date] = src.[prudential_date]
    and tgt.[runno] = src.[runno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[prudential_date] = src.[prudential_date],
        tgt.[runno] = src.[runno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [prudential_date],
        [runno],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[prudential_date],
        src.[runno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaReservelimit1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaReservelimit1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[reservelimitid] = src.[reservelimitid]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[reservelimitid] = src.[reservelimitid],
        tgt.[description] = src.[description],
        tgt.[rhs] = src.[rhs],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [version_datetime],
        [reservelimitid],
        [description],
        [rhs],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[version_datetime],
        src.[reservelimitid],
        src.[description],
        src.[rhs],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaReservelimitRegion1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaReservelimitRegion1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[regionid] = src.[regionid]
    and tgt.[reservelimitid] = src.[reservelimitid]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[reservelimitid] = src.[reservelimitid],
        tgt.[regionid] = src.[regionid],
        tgt.[coef] = src.[coef],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [version_datetime],
        [reservelimitid],
        [regionid],
        [coef],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[version_datetime],
        src.[reservelimitid],
        src.[regionid],
        src.[coef],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertMtpasaReservelimitSet1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.MtpasaReservelimitSet1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[reservelimit_set_id] = src.[reservelimit_set_id],
        tgt.[description] = src.[description],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [version_datetime],
        [reservelimit_set_id],
        [description],
        [authoriseddate],
        [authorisedby],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[version_datetime],
        src.[reservelimit_set_id],
        src.[description],
        src.[authoriseddate],
        src.[authorisedby],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertReserveDataReserve1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.ReserveDataReserve1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[lower5min] = src.[lower5min],
        tgt.[lower60sec] = src.[lower60sec],
        tgt.[lower6sec] = src.[lower6sec],
        tgt.[raise5min] = src.[raise5min],
        tgt.[raise60sec] = src.[raise60sec],
        tgt.[raise6sec] = src.[raise6sec],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[pasareserve] = src.[pasareserve],
        tgt.[loadrejectionreservereq] = src.[loadrejectionreservereq],
        tgt.[raisereg] = src.[raisereg],
        tgt.[lowerreg] = src.[lowerreg],
        tgt.[lor1level] = src.[lor1level],
        tgt.[lor2level] = src.[lor2level]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[regionid],
        src.[periodid],
        src.[lower5min],
        src.[lower60sec],
        src.[lower6sec],
        src.[raise5min],
        src.[raise60sec],
        src.[raise6sec],
        src.[lastchanged],
        src.[pasareserve],
        src.[loadrejectionreservereq],
        src.[raisereg],
        src.[lowerreg],
        src.[lor1level],
        src.[lor2level]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigAncillaryRecoverySplit2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigAncillaryRecoverySplit2 as tgt 
using (
    select 
        d.[effectivedate],
        d.[versionno],
        d.[service],
        d.[paymenttype],
        d.[customer_portion],
        d.[lastchanged],
        d.[ace_portion]
    from openjson(@data) with (
        [effectivedate] datetime2,
        [versionno] decimal(3,0),
        [service] varchar(10),
        [paymenttype] varchar(20),
        [customer_portion] decimal(8,5),
        [lastchanged] datetime2,
        [ace_portion] decimal(18,8)
    ) d
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[paymenttype] = src.[paymenttype]
    and tgt.[service] = src.[service]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[service] = src.[service],
        tgt.[paymenttype] = src.[paymenttype],
        tgt.[customer_portion] = src.[customer_portion],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[ace_portion] = src.[ace_portion]
when not matched
    then insert (
        file_log_id,
        [effectivedate],
        [versionno],
        [service],
        [paymenttype],
        [customer_portion],
        [lastchanged],
        [ace_portion]
    ) values (
        @file_log_id,
        src.[effectivedate],
        src.[versionno],
        src.[service],
        src.[paymenttype],
        src.[customer_portion],
        src.[lastchanged],
        src.[ace_portion]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketfee2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigMarketfee2 as tgt 
using (
    select 
        d.[marketfeeid],
        d.[marketfeeperiod],
        d.[marketfeetype],
        d.[description],
        d.[lastchanged],
        d.[gl_tcode],
        d.[gl_financialcode],
        d.[fee_class],
        d.[meter_type],
        d.[meter_subtype]
    from openjson(@data) with (
        [marketfeeid] varchar(10),
        [marketfeeperiod] varchar(20),
        [marketfeetype] varchar(12),
        [description] varchar(64),
        [lastchanged] datetime2,
        [gl_tcode] varchar(15),
        [gl_financialcode] varchar(10),
        [fee_class] varchar(40),
        [meter_type] varchar(20),
        [meter_subtype] varchar(20)
    ) d
) as src 
on (
    tgt.[marketfeeid] = src.[marketfeeid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[marketfeeperiod] = src.[marketfeeperiod],
        tgt.[marketfeetype] = src.[marketfeetype],
        tgt.[description] = src.[description],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[gl_tcode] = src.[gl_tcode],
        tgt.[gl_financialcode] = src.[gl_financialcode],
        tgt.[fee_class] = src.[fee_class],
        tgt.[meter_type] = src.[meter_type],
        tgt.[meter_subtype] = src.[meter_subtype]
when not matched
    then insert (
        file_log_id,
        [marketfeeid],
        [marketfeeperiod],
        [marketfeetype],
        [description],
        [lastchanged],
        [gl_tcode],
        [gl_financialcode],
        [fee_class],
        [meter_type],
        [meter_subtype]
    ) values (
        @file_log_id,
        src.[marketfeeid],
        src.[marketfeeperiod],
        src.[marketfeetype],
        src.[description],
        src.[lastchanged],
        src.[gl_tcode],
        src.[gl_financialcode],
        src.[fee_class],
        src.[meter_type],
        src.[meter_subtype]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketfeedata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigMarketfeedata1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[marketfeeid] = src.[marketfeeid]
    and tgt.[marketfeeversionno] = src.[marketfeeversionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[marketfeeversionno] = src.[marketfeeversionno],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[marketfeevalue] = src.[marketfeevalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [marketfeeid],
        [marketfeeversionno],
        [effectivedate],
        [marketfeevalue],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[marketfeeid],
        src.[marketfeeversionno],
        src.[effectivedate],
        src.[marketfeevalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketfeetrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigMarketfeetrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[marketfeeversionno] = src.[marketfeeversionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[marketfeeversionno] = src.[marketfeeversionno],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [marketfeeversionno],
        [effectivedate],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[marketfeeversionno],
        src.[effectivedate],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketFeeCatExcl1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.SettlementConfigMarketFeeCatExcl1(
file_log_id,
[marketfeeid],
        [effectivedate],
        [version_datetime],
        [participant_categoryid]
)
select 
@file_log_id,
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
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketFeeCatExclTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigMarketFeeCatExclTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[marketfeeid] = src.[marketfeeid]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [marketfeeid],
        [effectivedate],
        [version_datetime],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[marketfeeid],
        src.[effectivedate],
        src.[version_datetime],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketFeeExclusion1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigMarketFeeExclusion1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[marketfeeid] = src.[marketfeeid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [effectivedate],
        [versionno],
        [marketfeeid],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[effectivedate],
        src.[versionno],
        src.[marketfeeid],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigMarketFeeExclusionTrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigMarketFeeExclusionTrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [effectivedate],
        [versionno],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[effectivedate],
        src.[versionno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigParticipantBandfeeAlloc1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigParticipantBandfeeAlloc1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[marketfeeid] = src.[marketfeeid]
    and tgt.[participantcategoryid] = src.[participantcategoryid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantcategoryid] = src.[participantcategoryid],
        tgt.[marketfeevalue] = src.[marketfeevalue],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [marketfeeid],
        [effectivedate],
        [versionno],
        [participantcategoryid],
        [marketfeevalue],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[marketfeeid],
        src.[effectivedate],
        src.[versionno],
        src.[participantcategoryid],
        src.[marketfeevalue],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSetcfgReallocation2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SetcfgReallocation2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[reallocationid] = src.[reallocationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[reallocationid] = src.[reallocationid],
        tgt.[creditparticipantid] = src.[creditparticipantid],
        tgt.[debitparticipantid] = src.[debitparticipantid],
        tgt.[regionid] = src.[regionid],
        tgt.[agreementtype] = src.[agreementtype],
        tgt.[creditreference] = src.[creditreference],
        tgt.[debitreference] = src.[debitreference],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[current_stepid] = src.[current_stepid],
        tgt.[daytype] = src.[daytype],
        tgt.[reallocation_type] = src.[reallocation_type],
        tgt.[calendarid] = src.[calendarid],
        tgt.[intervallength] = src.[intervallength]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[reallocationid],
        src.[creditparticipantid],
        src.[debitparticipantid],
        src.[regionid],
        src.[agreementtype],
        src.[creditreference],
        src.[debitreference],
        src.[lastchanged],
        src.[startdate],
        src.[enddate],
        src.[current_stepid],
        src.[daytype],
        src.[reallocation_type],
        src.[calendarid],
        src.[intervallength]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSetcfgReallocationinterval1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SetcfgReallocationinterval1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[reallocationid] = src.[reallocationid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[reallocationid] = src.[reallocationid],
        tgt.[periodid] = src.[periodid],
        tgt.[value] = src.[value],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[nrp] = src.[nrp]
when not matched
    then insert (
        file_log_id,
        [reallocationid],
        [periodid],
        [value],
        [lastchanged],
        [nrp]
    ) values (
        @file_log_id,
        src.[reallocationid],
        src.[periodid],
        src.[value],
        src.[lastchanged],
        src.[nrp]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpf1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigSetcfgParticipantMpf1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantcategoryid] = src.[participantcategoryid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantcategoryid] = src.[participantcategoryid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[mpf] = src.[mpf],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [effectivedate],
        [versionno],
        [participantcategoryid],
        [connectionpointid],
        [mpf],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[effectivedate],
        src.[versionno],
        src.[participantcategoryid],
        src.[connectionpointid],
        src.[mpf],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpftrk1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementConfigSetcfgParticipantMpftrk1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[effectivedate] = src.[effectivedate]
    and tgt.[participantid] = src.[participantid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[participantid] = src.[participantid],
        tgt.[effectivedate] = src.[effectivedate],
        tgt.[versionno] = src.[versionno],
        tgt.[authorisedby] = src.[authorisedby],
        tgt.[authoriseddate] = src.[authoriseddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [participantid],
        [effectivedate],
        [versionno],
        [authorisedby],
        [authoriseddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[participantid],
        src.[effectivedate],
        src.[versionno],
        src.[authorisedby],
        src.[authoriseddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSetcfgSapsSettPrice1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SetcfgSapsSettPrice1 as tgt 
using (
    select 
        d.[fromdate],
        d.[todate],
        d.[regionid],
        d.[version_datetime],
        d.[saps_rrp],
        d.[isfirm],
        d.[lastchanged]
    from openjson(@data) with (
        [fromdate] datetime2,
        [todate] datetime2,
        [regionid] varchar(20),
        [version_datetime] datetime2,
        [saps_rrp] decimal(18,8),
        [isfirm] decimal(3,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[fromdate] = src.[fromdate]
    and tgt.[regionid] = src.[regionid]
    and tgt.[todate] = src.[todate]
    and tgt.[version_datetime] = src.[version_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[fromdate] = src.[fromdate],
        tgt.[todate] = src.[todate],
        tgt.[regionid] = src.[regionid],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[saps_rrp] = src.[saps_rrp],
        tgt.[isfirm] = src.[isfirm],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [fromdate],
        [todate],
        [regionid],
        [version_datetime],
        [saps_rrp],
        [isfirm],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[fromdate],
        src.[todate],
        src.[regionid],
        src.[version_datetime],
        src.[saps_rrp],
        src.[isfirm],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsConfigWdrrrCalendar1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsConfigWdrrrCalendar1 as tgt 
using (
    select 
        d.[wdrrrperiod],
        d.[regionid],
        d.[version_datetime],
        d.[startdate],
        d.[enddate],
        d.[lastchanged]
    from openjson(@data) with (
        [wdrrrperiod] varchar(20),
        [regionid] varchar(20),
        [version_datetime] datetime2,
        [startdate] datetime2,
        [enddate] datetime2,
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[regionid] = src.[regionid]
    and tgt.[version_datetime] = src.[version_datetime]
    and tgt.[wdrrrperiod] = src.[wdrrrperiod]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[wdrrrperiod] = src.[wdrrrperiod],
        tgt.[regionid] = src.[regionid],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[startdate] = src.[startdate],
        tgt.[enddate] = src.[enddate],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [wdrrrperiod],
        [regionid],
        [version_datetime],
        [startdate],
        [enddate],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[wdrrrperiod],
        src.[regionid],
        src.[version_datetime],
        src.[startdate],
        src.[enddate],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsConfigWdrReimburseRate1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsConfigWdrReimburseRate1 as tgt 
using (
    select 
        d.[wdrrrperiod],
        d.[regionid],
        d.[version_datetime],
        d.[wdrrr],
        d.[isfirm],
        d.[lastchanged]
    from openjson(@data) with (
        [wdrrrperiod] varchar(20),
        [regionid] varchar(20),
        [version_datetime] datetime2,
        [wdrrr] decimal(18,8),
        [isfirm] decimal(3,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[regionid] = src.[regionid]
    and tgt.[version_datetime] = src.[version_datetime]
    and tgt.[wdrrrperiod] = src.[wdrrrperiod]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[wdrrrperiod] = src.[wdrrrperiod],
        tgt.[regionid] = src.[regionid],
        tgt.[version_datetime] = src.[version_datetime],
        tgt.[wdrrr] = src.[wdrrr],
        tgt.[isfirm] = src.[isfirm],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [wdrrrperiod],
        [regionid],
        [version_datetime],
        [wdrrr],
        [isfirm],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[wdrrrperiod],
        src.[regionid],
        src.[version_datetime],
        src.[wdrrr],
        src.[isfirm],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsDaytrack6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsDaytrack6 as tgt 
using (
    select 
        d.[settlementdate],
        d.[regionid],
        d.[exanterunstatus],
        d.[exanterunno],
        d.[expostrunstatus],
        d.[expostrunno],
        d.[lastchanged],
        d.[settlementintervallength]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [regionid] varchar(10),
        [exanterunstatus] varchar(15),
        [exanterunno] decimal(3,0),
        [expostrunstatus] varchar(15),
        [expostrunno] decimal(3,0),
        [lastchanged] datetime2,
        [settlementintervallength] decimal(3,0)
    ) d
) as src 
on (
    tgt.[expostrunno] = src.[expostrunno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[regionid] = src.[regionid],
        tgt.[exanterunstatus] = src.[exanterunstatus],
        tgt.[exanterunno] = src.[exanterunno],
        tgt.[expostrunstatus] = src.[expostrunstatus],
        tgt.[expostrunno] = src.[expostrunno],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[settlementintervallength] = src.[settlementintervallength]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [regionid],
        [exanterunstatus],
        [exanterunno],
        [expostrunstatus],
        [expostrunno],
        [lastchanged],
        [settlementintervallength]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[regionid],
        src.[exanterunstatus],
        src.[exanterunno],
        src.[expostrunstatus],
        src.[expostrunno],
        src.[lastchanged],
        src.[settlementintervallength]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsCpdata7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsCpdata7 as tgt 
using (
    select 
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
        d.[mda],
        d.[afe],
        d.[dme],
        d.[ufea],
        d.[age],
        d.[importenergycost],
        d.[exportenergycost]
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
        [mda] varchar(10),
        [afe] decimal(18,8),
        [dme] decimal(18,8),
        [ufea] decimal(18,8),
        [age] decimal(18,8),
        [importenergycost] decimal(18,8),
        [exportenergycost] decimal(18,8)
    ) d
) as src 
on (
    tgt.[mda] = src.[mda]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[tcpid] = src.[tcpid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[tcpid] = src.[tcpid],
        tgt.[regionid] = src.[regionid],
        tgt.[igenergy] = src.[igenergy],
        tgt.[xgenergy] = src.[xgenergy],
        tgt.[inenergy] = src.[inenergy],
        tgt.[xnenergy] = src.[xnenergy],
        tgt.[ipower] = src.[ipower],
        tgt.[xpower] = src.[xpower],
        tgt.[rrp] = src.[rrp],
        tgt.[eep] = src.[eep],
        tgt.[tlf] = src.[tlf],
        tgt.[cprrp] = src.[cprrp],
        tgt.[cpeep] = src.[cpeep],
        tgt.[ta] = src.[ta],
        tgt.[ep] = src.[ep],
        tgt.[apc] = src.[apc],
        tgt.[resc] = src.[resc],
        tgt.[resp] = src.[resp],
        tgt.[meterrunno] = src.[meterrunno],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[hostdistributor] = src.[hostdistributor],
        tgt.[mda] = src.[mda],
        tgt.[afe] = src.[afe],
        tgt.[dme] = src.[dme],
        tgt.[ufea] = src.[ufea],
        tgt.[age] = src.[age],
        tgt.[importenergycost] = src.[importenergycost],
        tgt.[exportenergycost] = src.[exportenergycost]
when not matched
    then insert (
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
        [mda],
        [afe],
        [dme],
        [ufea],
        [age],
        [importenergycost],
        [exportenergycost]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[participantid],
        src.[tcpid],
        src.[regionid],
        src.[igenergy],
        src.[xgenergy],
        src.[inenergy],
        src.[xnenergy],
        src.[ipower],
        src.[xpower],
        src.[rrp],
        src.[eep],
        src.[tlf],
        src.[cprrp],
        src.[cpeep],
        src.[ta],
        src.[ep],
        src.[apc],
        src.[resc],
        src.[resp],
        src.[meterrunno],
        src.[lastchanged],
        src.[hostdistributor],
        src.[mda],
        src.[afe],
        src.[dme],
        src.[ufea],
        src.[age],
        src.[importenergycost],
        src.[exportenergycost]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsCpdataregion5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsCpdataregion5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[regionid] = src.[regionid],
        tgt.[sumigenergy] = src.[sumigenergy],
        tgt.[sumxgenergy] = src.[sumxgenergy],
        tgt.[suminenergy] = src.[suminenergy],
        tgt.[sumxnenergy] = src.[sumxnenergy],
        tgt.[sumipower] = src.[sumipower],
        tgt.[sumxpower] = src.[sumxpower],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[sumep] = src.[sumep]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[regionid],
        src.[sumigenergy],
        src.[sumxgenergy],
        src.[suminenergy],
        src.[sumxnenergy],
        src.[sumipower],
        src.[sumxpower],
        src.[lastchanged],
        src.[sumep]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsFcasregionrecovery6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsFcasregionrecovery6 as tgt 
using (
    select 
        d.[settlementdate],
        d.[versionno],
        d.[bidtype],
        d.[regionid],
        d.[periodid],
        d.[generatorregionenergy],
        d.[customerregionenergy],
        d.[regionrecovery],
        d.[lastchanged],
        d.[region_ace_mwh],
        d.[region_asoe_mwh],
        d.[regionrecoveryamount_ace],
        d.[regionrecoveryamount_asoe],
        d.[regionrecoveryamount]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [versionno] decimal(3,0),
        [bidtype] varchar(10),
        [regionid] varchar(10),
        [periodid] decimal(3,0),
        [generatorregionenergy] decimal(16,6),
        [customerregionenergy] decimal(16,6),
        [regionrecovery] decimal(18,8),
        [lastchanged] datetime2,
        [region_ace_mwh] decimal(18,8),
        [region_asoe_mwh] decimal(18,8),
        [regionrecoveryamount_ace] decimal(18,8),
        [regionrecoveryamount_asoe] decimal(18,8),
        [regionrecoveryamount] decimal(18,8)
    ) d
) as src 
on (
    tgt.[bidtype] = src.[bidtype]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[bidtype] = src.[bidtype],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[generatorregionenergy] = src.[generatorregionenergy],
        tgt.[customerregionenergy] = src.[customerregionenergy],
        tgt.[regionrecovery] = src.[regionrecovery],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[region_ace_mwh] = src.[region_ace_mwh],
        tgt.[region_asoe_mwh] = src.[region_asoe_mwh],
        tgt.[regionrecoveryamount_ace] = src.[regionrecoveryamount_ace],
        tgt.[regionrecoveryamount_asoe] = src.[regionrecoveryamount_asoe],
        tgt.[regionrecoveryamount] = src.[regionrecoveryamount]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [bidtype],
        [regionid],
        [periodid],
        [generatorregionenergy],
        [customerregionenergy],
        [regionrecovery],
        [lastchanged],
        [region_ace_mwh],
        [region_asoe_mwh],
        [regionrecoveryamount_ace],
        [regionrecoveryamount_asoe],
        [regionrecoveryamount]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[bidtype],
        src.[regionid],
        src.[periodid],
        src.[generatorregionenergy],
        src.[customerregionenergy],
        src.[regionrecovery],
        src.[lastchanged],
        src.[region_ace_mwh],
        src.[region_asoe_mwh],
        src.[regionrecoveryamount_ace],
        src.[regionrecoveryamount_asoe],
        src.[regionrecoveryamount]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsGendata6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsGendata6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[gensetid] = src.[gensetid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[stationid] = src.[stationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[stationid] = src.[stationid],
        tgt.[duid] = src.[duid],
        tgt.[gensetid] = src.[gensetid],
        tgt.[regionid] = src.[regionid],
        tgt.[genergy] = src.[genergy],
        tgt.[aenergy] = src.[aenergy],
        tgt.[gpower] = src.[gpower],
        tgt.[apower] = src.[apower],
        tgt.[rrp] = src.[rrp],
        tgt.[eep] = src.[eep],
        tgt.[tlf] = src.[tlf],
        tgt.[cprrp] = src.[cprrp],
        tgt.[cpeep] = src.[cpeep],
        tgt.[netenergy] = src.[netenergy],
        tgt.[energycost] = src.[energycost],
        tgt.[excessenergycost] = src.[excessenergycost],
        tgt.[apc] = src.[apc],
        tgt.[resc] = src.[resc],
        tgt.[resp] = src.[resp],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[expenergy] = src.[expenergy],
        tgt.[expenergycost] = src.[expenergycost],
        tgt.[meterrunno] = src.[meterrunno],
        tgt.[mda] = src.[mda],
        tgt.[secondary_tlf] = src.[secondary_tlf]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[participantid],
        src.[stationid],
        src.[duid],
        src.[gensetid],
        src.[regionid],
        src.[genergy],
        src.[aenergy],
        src.[gpower],
        src.[apower],
        src.[rrp],
        src.[eep],
        src.[tlf],
        src.[cprrp],
        src.[cpeep],
        src.[netenergy],
        src.[energycost],
        src.[excessenergycost],
        src.[apc],
        src.[resc],
        src.[resp],
        src.[lastchanged],
        src.[expenergy],
        src.[expenergycost],
        src.[meterrunno],
        src.[mda],
        src.[secondary_tlf]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsGendataregion5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsGendataregion5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[regionid] = src.[regionid],
        tgt.[genergy] = src.[genergy],
        tgt.[aenergy] = src.[aenergy],
        tgt.[gpower] = src.[gpower],
        tgt.[apower] = src.[apower],
        tgt.[netenergy] = src.[netenergy],
        tgt.[energycost] = src.[energycost],
        tgt.[excessenergycost] = src.[excessenergycost],
        tgt.[expenergy] = src.[expenergy],
        tgt.[expenergycost] = src.[expenergycost],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[regionid],
        src.[genergy],
        src.[aenergy],
        src.[gpower],
        src.[apower],
        src.[netenergy],
        src.[energycost],
        src.[excessenergycost],
        src.[expenergy],
        src.[expenergycost],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsIntraregionresidues6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsIntraregionresidues6 as tgt 
using (
    select 
        d.[settlementdate],
        d.[runno],
        d.[periodid],
        d.[regionid],
        d.[ep],
        d.[ec],
        d.[rrp],
        d.[exp],
        d.[irss],
        d.[lastchanged],
        d.[ace_amount],
        d.[asoe_amount]
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
        [lastchanged] datetime2,
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8)
    ) d
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[periodid] = src.[periodid],
        tgt.[regionid] = src.[regionid],
        tgt.[ep] = src.[ep],
        tgt.[ec] = src.[ec],
        tgt.[rrp] = src.[rrp],
        tgt.[exp] = src.[exp],
        tgt.[irss] = src.[irss],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[ace_amount] = src.[ace_amount],
        tgt.[asoe_amount] = src.[asoe_amount]
when not matched
    then insert (
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
        [lastchanged],
        [ace_amount],
        [asoe_amount]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[periodid],
        src.[regionid],
        src.[ep],
        src.[ec],
        src.[rrp],
        src.[exp],
        src.[irss],
        src.[lastchanged],
        src.[ace_amount],
        src.[asoe_amount]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsIraucsurplus6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsIraucsurplus6 as tgt 
using (
    select 
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
        [periodid] decimal(3,0),
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[settlementrunno] = src.[settlementrunno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[settlementrunno] = src.[settlementrunno],
        tgt.[contractid] = src.[contractid],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[contractallocation] = src.[contractallocation],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[settlementrunno],
        src.[contractid],
        src.[periodid],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[totalsurplus],
        src.[contractallocation],
        src.[surplusvalue],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsIrnspsurplus6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsIrnspsurplus6 as tgt 
using (
    select 
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
        [periodid] decimal(3,0),
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[settlementrunno] = src.[settlementrunno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[settlementrunno] = src.[settlementrunno],
        tgt.[contractid] = src.[contractid],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[contractallocation] = src.[contractallocation],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[settlementrunno],
        src.[contractid],
        src.[periodid],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[totalsurplus],
        src.[contractallocation],
        src.[surplusvalue],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsIrpartsurplus6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsIrpartsurplus6 as tgt 
using (
    select 
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
        [periodid] decimal(3,0),
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[fromregionid] = src.[fromregionid]
    and tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[settlementrunno] = src.[settlementrunno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[settlementrunno] = src.[settlementrunno],
        tgt.[contractid] = src.[contractid],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[fromregionid] = src.[fromregionid],
        tgt.[totalsurplus] = src.[totalsurplus],
        tgt.[contractallocation] = src.[contractallocation],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[settlementrunno],
        src.[contractid],
        src.[periodid],
        src.[participantid],
        src.[interconnectorid],
        src.[fromregionid],
        src.[totalsurplus],
        src.[contractallocation],
        src.[surplusvalue],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsIrsurplus6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsIrsurplus6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[settlementrunno] = src.[settlementrunno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[settlementrunno] = src.[settlementrunno],
        tgt.[periodid] = src.[periodid],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[regionid] = src.[regionid],
        tgt.[mwflow] = src.[mwflow],
        tgt.[lossfactor] = src.[lossfactor],
        tgt.[surplusvalue] = src.[surplusvalue],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[csp_derogation_amount] = src.[csp_derogation_amount],
        tgt.[unadjusted_irsr] = src.[unadjusted_irsr]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[settlementrunno],
        src.[periodid],
        src.[interconnectorid],
        src.[regionid],
        src.[mwflow],
        src.[lossfactor],
        src.[surplusvalue],
        src.[lastchanged],
        src.[csp_derogation_amount],
        src.[unadjusted_irsr]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsLocalareaenergy1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsLocalareaenergy1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[settlementrunno],
        d.[localareaid],
        d.[periodid],
        d.[ufe],
        d.[ddme],
        d.[tme],
        d.[adme],
        d.[admela],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [localareaid] varchar(30),
        [periodid] decimal(3,0),
        [ufe] decimal(18,8),
        [ddme] decimal(18,8),
        [tme] decimal(18,8),
        [adme] decimal(18,8),
        [admela] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[localareaid] = src.[localareaid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[settlementrunno] = src.[settlementrunno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[settlementrunno] = src.[settlementrunno],
        tgt.[localareaid] = src.[localareaid],
        tgt.[periodid] = src.[periodid],
        tgt.[ufe] = src.[ufe],
        tgt.[ddme] = src.[ddme],
        tgt.[tme] = src.[tme],
        tgt.[adme] = src.[adme],
        tgt.[admela] = src.[admela],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [settlementrunno],
        [localareaid],
        [periodid],
        [ufe],
        [ddme],
        [tme],
        [adme],
        [admela],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[settlementrunno],
        src.[localareaid],
        src.[periodid],
        src.[ufe],
        src.[ddme],
        src.[tme],
        src.[adme],
        src.[admela],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsLocalareatni1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsLocalareatni1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[settlementrunno],
        d.[localareaid],
        d.[tni],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [localareaid] varchar(30),
        [tni] varchar(30),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[localareaid] = src.[localareaid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[settlementrunno] = src.[settlementrunno]
    and tgt.[tni] = src.[tni]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[settlementrunno] = src.[settlementrunno],
        tgt.[localareaid] = src.[localareaid],
        tgt.[tni] = src.[tni],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [settlementrunno],
        [localareaid],
        [tni],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[settlementrunno],
        src.[localareaid],
        src.[tni],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsLshedpayment5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsLshedpayment5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[contractid] = src.[contractid],
        tgt.[periodid] = src.[periodid],
        tgt.[duid] = src.[duid],
        tgt.[regionid] = src.[regionid],
        tgt.[tlf] = src.[tlf],
        tgt.[rrp] = src.[rrp],
        tgt.[lseprice] = src.[lseprice],
        tgt.[mcpprice] = src.[mcpprice],
        tgt.[lscr] = src.[lscr],
        tgt.[lsepayment] = src.[lsepayment],
        tgt.[ccpayment] = src.[ccpayment],
        tgt.[constrainedmw] = src.[constrainedmw],
        tgt.[unconstrainedmw] = src.[unconstrainedmw],
        tgt.[als] = src.[als],
        tgt.[initialdemand] = src.[initialdemand],
        tgt.[finaldemand] = src.[finaldemand],
        tgt.[contractversionno] = src.[contractversionno],
        tgt.[offerdate] = src.[offerdate],
        tgt.[offerversionno] = src.[offerversionno],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[availabilitypayment] = src.[availabilitypayment]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[participantid],
        src.[contractid],
        src.[periodid],
        src.[duid],
        src.[regionid],
        src.[tlf],
        src.[rrp],
        src.[lseprice],
        src.[mcpprice],
        src.[lscr],
        src.[lsepayment],
        src.[ccpayment],
        src.[constrainedmw],
        src.[unconstrainedmw],
        src.[als],
        src.[initialdemand],
        src.[finaldemand],
        src.[contractversionno],
        src.[offerdate],
        src.[offerversionno],
        src.[lastchanged],
        src.[availabilitypayment]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsMarketfees7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsMarketfees7 as tgt 
using (
    select 
        d.[settlementdate],
        d.[runno],
        d.[participantid],
        d.[periodid],
        d.[marketfeeid],
        d.[marketfeevalue],
        d.[energy],
        d.[lastchanged],
        d.[participantcategoryid],
        d.[feerate],
        d.[feeunits],
        d.[meter_type],
        d.[meter_subtype]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [runno] decimal(3,0),
        [participantid] varchar(10),
        [periodid] decimal(3,0),
        [marketfeeid] varchar(10),
        [marketfeevalue] decimal(15,5),
        [energy] decimal(16,6),
        [lastchanged] datetime2,
        [participantcategoryid] varchar(10),
        [feerate] decimal(18,8),
        [feeunits] decimal(18,8),
        [meter_type] varchar(20),
        [meter_subtype] varchar(20)
    ) d
) as src 
on (
    tgt.[marketfeeid] = src.[marketfeeid]
    and tgt.[participantcategoryid] = src.[participantcategoryid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[participantid] = src.[participantid],
        tgt.[periodid] = src.[periodid],
        tgt.[marketfeeid] = src.[marketfeeid],
        tgt.[marketfeevalue] = src.[marketfeevalue],
        tgt.[energy] = src.[energy],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[participantcategoryid] = src.[participantcategoryid],
        tgt.[feerate] = src.[feerate],
        tgt.[feeunits] = src.[feeunits],
        tgt.[meter_type] = src.[meter_type],
        tgt.[meter_subtype] = src.[meter_subtype]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [participantid],
        [periodid],
        [marketfeeid],
        [marketfeevalue],
        [energy],
        [lastchanged],
        [participantcategoryid],
        [feerate],
        [feeunits],
        [meter_type],
        [meter_subtype]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[participantid],
        src.[periodid],
        src.[marketfeeid],
        src.[marketfeevalue],
        src.[energy],
        src.[lastchanged],
        src.[participantcategoryid],
        src.[feerate],
        src.[feeunits],
        src.[meter_type],
        src.[meter_subtype]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsReallocations5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsReallocations5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[reallocationid] = src.[reallocationid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[reallocationid] = src.[reallocationid],
        tgt.[reallocationvalue] = src.[reallocationvalue],
        tgt.[energy] = src.[energy],
        tgt.[rrp] = src.[rrp],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[periodid],
        src.[participantid],
        src.[reallocationid],
        src.[reallocationvalue],
        src.[energy],
        src.[rrp],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsRestartpayment6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsRestartpayment6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[contractid] = src.[contractid],
        tgt.[periodid] = src.[periodid],
        tgt.[regionid] = src.[regionid],
        tgt.[restarttype] = src.[restarttype],
        tgt.[avaflag] = src.[avaflag],
        tgt.[availabilityprice] = src.[availabilityprice],
        tgt.[tcf] = src.[tcf],
        tgt.[availabilitypayment] = src.[availabilitypayment],
        tgt.[contractversionno] = src.[contractversionno],
        tgt.[offerdate] = src.[offerdate],
        tgt.[offerversionno] = src.[offerversionno],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[enablingpayment] = src.[enablingpayment]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[participantid],
        src.[contractid],
        src.[periodid],
        src.[regionid],
        src.[restarttype],
        src.[avaflag],
        src.[availabilityprice],
        src.[tcf],
        src.[availabilitypayment],
        src.[contractversionno],
        src.[offerdate],
        src.[offerversionno],
        src.[lastchanged],
        src.[enablingpayment]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsRpowerpayment6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsRpowerpayment6 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[contractid] = src.[contractid],
        tgt.[periodid] = src.[periodid],
        tgt.[duid] = src.[duid],
        tgt.[regionid] = src.[regionid],
        tgt.[tlf] = src.[tlf],
        tgt.[ebp] = src.[ebp],
        tgt.[rrp] = src.[rrp],
        tgt.[mvaraprice] = src.[mvaraprice],
        tgt.[mvareprice] = src.[mvareprice],
        tgt.[mvargprice] = src.[mvargprice],
        tgt.[ccprice] = src.[ccprice],
        tgt.[synccompensation] = src.[synccompensation],
        tgt.[mta] = src.[mta],
        tgt.[mtg] = src.[mtg],
        tgt.[blocksize] = src.[blocksize],
        tgt.[avaflag] = src.[avaflag],
        tgt.[clearedmw] = src.[clearedmw],
        tgt.[unconstrainedmw] = src.[unconstrainedmw],
        tgt.[availabilitypayment] = src.[availabilitypayment],
        tgt.[enablingpayment] = src.[enablingpayment],
        tgt.[ccpayment] = src.[ccpayment],
        tgt.[contractversionno] = src.[contractversionno],
        tgt.[offerdate] = src.[offerdate],
        tgt.[offerversionno] = src.[offerversionno],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[availabilitypayment_rebate] = src.[availabilitypayment_rebate]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[participantid],
        src.[contractid],
        src.[periodid],
        src.[duid],
        src.[regionid],
        src.[tlf],
        src.[ebp],
        src.[rrp],
        src.[mvaraprice],
        src.[mvareprice],
        src.[mvargprice],
        src.[ccprice],
        src.[synccompensation],
        src.[mta],
        src.[mtg],
        src.[blocksize],
        src.[avaflag],
        src.[clearedmw],
        src.[unconstrainedmw],
        src.[availabilitypayment],
        src.[enablingpayment],
        src.[ccpayment],
        src.[contractversionno],
        src.[offerdate],
        src.[offerversionno],
        src.[lastchanged],
        src.[availabilitypayment_rebate]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsSmallgendata1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsSmallgendata1 as tgt 
using (
    select 
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
        d.[lastchanged]
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
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[regionid] = src.[regionid],
        tgt.[importenergy] = src.[importenergy],
        tgt.[exportenergy] = src.[exportenergy],
        tgt.[rrp] = src.[rrp],
        tgt.[tlf] = src.[tlf],
        tgt.[impenergycost] = src.[impenergycost],
        tgt.[expenergycost] = src.[expenergycost],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[connectionpointid],
        src.[periodid],
        src.[participantid],
        src.[regionid],
        src.[importenergy],
        src.[exportenergy],
        src.[rrp],
        src.[tlf],
        src.[impenergycost],
        src.[expenergycost],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsAncillarySummary5
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsAncillarySummary5 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[paymenttype] = src.[paymenttype]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[service] = src.[service]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[service] = src.[service],
        tgt.[paymenttype] = src.[paymenttype],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[paymentamount] = src.[paymentamount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [service],
        [paymenttype],
        [regionid],
        [periodid],
        [paymentamount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[service],
        src.[paymenttype],
        src.[regionid],
        src.[periodid],
        src.[paymentamount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsEnergyGensetDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsEnergyGensetDetail1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[versionno],
        d.[periodid],
        d.[participantid],
        d.[stationid],
        d.[duid],
        d.[gensetid],
        d.[regionid],
        d.[connectionpointid],
        d.[rrp],
        d.[tlf],
        d.[meterid],
        d.[ce_mwh],
        d.[ufea_mwh],
        d.[ace_mwh],
        d.[asoe_mwh],
        d.[total_mwh],
        d.[dme_mwh],
        d.[ace_amount],
        d.[asoe_amount],
        d.[total_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [versionno] decimal(3,0),
        [periodid] decimal(3,0),
        [participantid] varchar(20),
        [stationid] varchar(20),
        [duid] varchar(20),
        [gensetid] varchar(20),
        [regionid] varchar(20),
        [connectionpointid] varchar(20),
        [rrp] decimal(18,8),
        [tlf] decimal(18,8),
        [meterid] varchar(20),
        [ce_mwh] decimal(18,8),
        [ufea_mwh] decimal(18,8),
        [ace_mwh] decimal(18,8),
        [asoe_mwh] decimal(18,8),
        [total_mwh] decimal(18,8),
        [dme_mwh] decimal(18,8),
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8),
        [total_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[gensetid] = src.[gensetid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[stationid] = src.[stationid]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[stationid] = src.[stationid],
        tgt.[duid] = src.[duid],
        tgt.[gensetid] = src.[gensetid],
        tgt.[regionid] = src.[regionid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[rrp] = src.[rrp],
        tgt.[tlf] = src.[tlf],
        tgt.[meterid] = src.[meterid],
        tgt.[ce_mwh] = src.[ce_mwh],
        tgt.[ufea_mwh] = src.[ufea_mwh],
        tgt.[ace_mwh] = src.[ace_mwh],
        tgt.[asoe_mwh] = src.[asoe_mwh],
        tgt.[total_mwh] = src.[total_mwh],
        tgt.[dme_mwh] = src.[dme_mwh],
        tgt.[ace_amount] = src.[ace_amount],
        tgt.[asoe_amount] = src.[asoe_amount],
        tgt.[total_amount] = src.[total_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [periodid],
        [participantid],
        [stationid],
        [duid],
        [gensetid],
        [regionid],
        [connectionpointid],
        [rrp],
        [tlf],
        [meterid],
        [ce_mwh],
        [ufea_mwh],
        [ace_mwh],
        [asoe_mwh],
        [total_mwh],
        [dme_mwh],
        [ace_amount],
        [asoe_amount],
        [total_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[participantid],
        src.[stationid],
        src.[duid],
        src.[gensetid],
        src.[regionid],
        src.[connectionpointid],
        src.[rrp],
        src.[tlf],
        src.[meterid],
        src.[ce_mwh],
        src.[ufea_mwh],
        src.[ace_mwh],
        src.[asoe_mwh],
        src.[total_mwh],
        src.[dme_mwh],
        src.[ace_amount],
        src.[asoe_amount],
        src.[total_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsEnergyRegionSummary1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsEnergyRegionSummary1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[versionno],
        d.[periodid],
        d.[regionid],
        d.[ce_mwh],
        d.[ufea_mwh],
        d.[ace_mwh],
        d.[asoe_mwh],
        d.[ace_amount],
        d.[asoe_amount],
        d.[total_mwh],
        d.[total_amount],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [versionno] decimal(3,0),
        [periodid] decimal(3,0),
        [regionid] varchar(20),
        [ce_mwh] decimal(18,8),
        [ufea_mwh] decimal(18,8),
        [ace_mwh] decimal(18,8),
        [asoe_mwh] decimal(18,8),
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8),
        [total_mwh] decimal(18,8),
        [total_amount] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[regionid] = src.[regionid],
        tgt.[ce_mwh] = src.[ce_mwh],
        tgt.[ufea_mwh] = src.[ufea_mwh],
        tgt.[ace_mwh] = src.[ace_mwh],
        tgt.[asoe_mwh] = src.[asoe_mwh],
        tgt.[ace_amount] = src.[ace_amount],
        tgt.[asoe_amount] = src.[asoe_amount],
        tgt.[total_mwh] = src.[total_mwh],
        tgt.[total_amount] = src.[total_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [periodid],
        [regionid],
        [ce_mwh],
        [ufea_mwh],
        [ace_mwh],
        [asoe_mwh],
        [ace_amount],
        [asoe_amount],
        [total_mwh],
        [total_amount],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[regionid],
        src.[ce_mwh],
        src.[ufea_mwh],
        src.[ace_mwh],
        src.[asoe_mwh],
        src.[ace_amount],
        src.[asoe_amount],
        src.[total_mwh],
        src.[total_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsEnergyTransaction1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsEnergyTransaction1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[versionno],
        d.[periodid],
        d.[participantid],
        d.[connectionpointid],
        d.[meter_type],
        d.[regionid],
        d.[rrp],
        d.[tlf],
        d.[ce_mwh],
        d.[ufea_mwh],
        d.[ace_mwh],
        d.[asoe_mwh],
        d.[total_mwh],
        d.[ace_amount],
        d.[asoe_amount],
        d.[total_amount],
        d.[case_id],
        d.[dme_mwh],
        d.[aggregate_read_flag],
        d.[individual_read_flag],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [versionno] decimal(3,0),
        [periodid] decimal(3,0),
        [participantid] varchar(20),
        [connectionpointid] varchar(20),
        [meter_type] varchar(20),
        [regionid] varchar(20),
        [rrp] decimal(18,8),
        [tlf] decimal(18,8),
        [ce_mwh] decimal(18,8),
        [ufea_mwh] decimal(18,8),
        [ace_mwh] decimal(18,8),
        [asoe_mwh] decimal(18,8),
        [total_mwh] decimal(18,8),
        [ace_amount] decimal(18,8),
        [asoe_amount] decimal(18,8),
        [total_amount] decimal(18,8),
        [case_id] decimal(10,0),
        [dme_mwh] decimal(18,8),
        [aggregate_read_flag] decimal(3,0),
        [individual_read_flag] decimal(3,0),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[connectionpointid] = src.[connectionpointid]
    and tgt.[meter_type] = src.[meter_type]
    and tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[connectionpointid] = src.[connectionpointid],
        tgt.[meter_type] = src.[meter_type],
        tgt.[regionid] = src.[regionid],
        tgt.[rrp] = src.[rrp],
        tgt.[tlf] = src.[tlf],
        tgt.[ce_mwh] = src.[ce_mwh],
        tgt.[ufea_mwh] = src.[ufea_mwh],
        tgt.[ace_mwh] = src.[ace_mwh],
        tgt.[asoe_mwh] = src.[asoe_mwh],
        tgt.[total_mwh] = src.[total_mwh],
        tgt.[ace_amount] = src.[ace_amount],
        tgt.[asoe_amount] = src.[asoe_amount],
        tgt.[total_amount] = src.[total_amount],
        tgt.[case_id] = src.[case_id],
        tgt.[dme_mwh] = src.[dme_mwh],
        tgt.[aggregate_read_flag] = src.[aggregate_read_flag],
        tgt.[individual_read_flag] = src.[individual_read_flag],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [periodid],
        [participantid],
        [connectionpointid],
        [meter_type],
        [regionid],
        [rrp],
        [tlf],
        [ce_mwh],
        [ufea_mwh],
        [ace_mwh],
        [asoe_mwh],
        [total_mwh],
        [ace_amount],
        [asoe_amount],
        [total_amount],
        [case_id],
        [dme_mwh],
        [aggregate_read_flag],
        [individual_read_flag],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[participantid],
        src.[connectionpointid],
        src.[meter_type],
        src.[regionid],
        src.[rrp],
        src.[tlf],
        src.[ce_mwh],
        src.[ufea_mwh],
        src.[ace_mwh],
        src.[asoe_mwh],
        src.[total_mwh],
        src.[ace_amount],
        src.[asoe_amount],
        src.[total_amount],
        src.[case_id],
        src.[dme_mwh],
        src.[aggregate_read_flag],
        src.[individual_read_flag],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsEnergyTranSaps1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsEnergyTranSaps1 as tgt 
using (
    select 
        d.[settlementdate],
        d.[versionno],
        d.[periodid],
        d.[participantid],
        d.[tni],
        d.[regionid],
        d.[saps_rrp],
        d.[consumed_energy_mwh],
        d.[sentout_energy_mwh],
        d.[consumed_energy_cost],
        d.[sentout_energy_cost],
        d.[lastchanged]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [versionno] decimal(3,0),
        [periodid] decimal(3,0),
        [participantid] varchar(20),
        [tni] varchar(20),
        [regionid] varchar(20),
        [saps_rrp] decimal(18,8),
        [consumed_energy_mwh] decimal(18,8),
        [sentout_energy_mwh] decimal(18,8),
        [consumed_energy_cost] decimal(18,8),
        [sentout_energy_cost] decimal(18,8),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[tni] = src.[tni]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[tni] = src.[tni],
        tgt.[regionid] = src.[regionid],
        tgt.[saps_rrp] = src.[saps_rrp],
        tgt.[consumed_energy_mwh] = src.[consumed_energy_mwh],
        tgt.[sentout_energy_mwh] = src.[sentout_energy_mwh],
        tgt.[consumed_energy_cost] = src.[consumed_energy_cost],
        tgt.[sentout_energy_cost] = src.[sentout_energy_cost],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [periodid],
        [participantid],
        [tni],
        [regionid],
        [saps_rrp],
        [consumed_energy_mwh],
        [sentout_energy_mwh],
        [consumed_energy_cost],
        [sentout_energy_cost],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[participantid],
        src.[tni],
        src.[regionid],
        src.[saps_rrp],
        src.[consumed_energy_mwh],
        src.[sentout_energy_mwh],
        src.[consumed_energy_cost],
        src.[sentout_energy_cost],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsFcasPayment6
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsFcasPayment6 as tgt 
using (
    select 
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
        d.[lastchanged],
        d.[raise1sec_payment],
        d.[lower1sec_payment]
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
        [lastchanged] datetime2,
        [raise1sec_payment] decimal(18,8),
        [lower1sec_payment] decimal(18,8)
    ) d
) as src 
on (
    tgt.[duid] = src.[duid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[duid] = src.[duid],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[lower6sec_payment] = src.[lower6sec_payment],
        tgt.[raise6sec_payment] = src.[raise6sec_payment],
        tgt.[lower60sec_payment] = src.[lower60sec_payment],
        tgt.[raise60sec_payment] = src.[raise60sec_payment],
        tgt.[lower5min_payment] = src.[lower5min_payment],
        tgt.[raise5min_payment] = src.[raise5min_payment],
        tgt.[lowerreg_payment] = src.[lowerreg_payment],
        tgt.[raisereg_payment] = src.[raisereg_payment],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[raise1sec_payment] = src.[raise1sec_payment],
        tgt.[lower1sec_payment] = src.[lower1sec_payment]
when not matched
    then insert (
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
        [lastchanged],
        [raise1sec_payment],
        [lower1sec_payment]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[participantid],
        src.[duid],
        src.[regionid],
        src.[periodid],
        src.[lower6sec_payment],
        src.[raise6sec_payment],
        src.[lower60sec_payment],
        src.[raise60sec_payment],
        src.[lower5min_payment],
        src.[raise5min_payment],
        src.[lowerreg_payment],
        src.[raisereg_payment],
        src.[lastchanged],
        src.[raise1sec_payment],
        src.[lower1sec_payment]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsFcasRecovery8
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsFcasRecovery8 as tgt 
using (
    select 
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
        d.[raisereg_recovery_gen],
        d.[raise1sec_recovery],
        d.[lower1sec_recovery],
        d.[raise1sec_recovery_gen],
        d.[lower1sec_recovery_gen],
        d.[lowerreg_ace],
        d.[raisereg_ace],
        d.[raise1sec_ace],
        d.[raise1sec_asoe],
        d.[lower1sec_ace],
        d.[lower1sec_asoe],
        d.[raise6sec_ace],
        d.[raise6sec_asoe],
        d.[lower6sec_ace],
        d.[lower6sec_asoe],
        d.[raise60sec_ace],
        d.[raise60sec_asoe],
        d.[lower60sec_ace],
        d.[lower60sec_asoe],
        d.[raise5min_ace],
        d.[raise5min_asoe],
        d.[lower5min_ace],
        d.[lower5min_asoe]
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
        [raisereg_recovery_gen] decimal(18,8),
        [raise1sec_recovery] decimal(18,8),
        [lower1sec_recovery] decimal(18,8),
        [raise1sec_recovery_gen] decimal(18,8),
        [lower1sec_recovery_gen] decimal(18,8),
        [lowerreg_ace] decimal(18,8),
        [raisereg_ace] decimal(18,8),
        [raise1sec_ace] decimal(18,8),
        [raise1sec_asoe] decimal(18,8),
        [lower1sec_ace] decimal(18,8),
        [lower1sec_asoe] decimal(18,8),
        [raise6sec_ace] decimal(18,8),
        [raise6sec_asoe] decimal(18,8),
        [lower6sec_ace] decimal(18,8),
        [lower6sec_asoe] decimal(18,8),
        [raise60sec_ace] decimal(18,8),
        [raise60sec_asoe] decimal(18,8),
        [lower60sec_ace] decimal(18,8),
        [lower60sec_asoe] decimal(18,8),
        [raise5min_ace] decimal(18,8),
        [raise5min_asoe] decimal(18,8),
        [lower5min_ace] decimal(18,8),
        [lower5min_asoe] decimal(18,8)
    ) d
) as src 
on (
    tgt.[participantid] = src.[participantid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[participantid] = src.[participantid],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[lower6sec_recovery] = src.[lower6sec_recovery],
        tgt.[raise6sec_recovery] = src.[raise6sec_recovery],
        tgt.[lower60sec_recovery] = src.[lower60sec_recovery],
        tgt.[raise60sec_recovery] = src.[raise60sec_recovery],
        tgt.[lower5min_recovery] = src.[lower5min_recovery],
        tgt.[raise5min_recovery] = src.[raise5min_recovery],
        tgt.[lowerreg_recovery] = src.[lowerreg_recovery],
        tgt.[raisereg_recovery] = src.[raisereg_recovery],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[lower6sec_recovery_gen] = src.[lower6sec_recovery_gen],
        tgt.[raise6sec_recovery_gen] = src.[raise6sec_recovery_gen],
        tgt.[lower60sec_recovery_gen] = src.[lower60sec_recovery_gen],
        tgt.[raise60sec_recovery_gen] = src.[raise60sec_recovery_gen],
        tgt.[lower5min_recovery_gen] = src.[lower5min_recovery_gen],
        tgt.[raise5min_recovery_gen] = src.[raise5min_recovery_gen],
        tgt.[lowerreg_recovery_gen] = src.[lowerreg_recovery_gen],
        tgt.[raisereg_recovery_gen] = src.[raisereg_recovery_gen],
        tgt.[raise1sec_recovery] = src.[raise1sec_recovery],
        tgt.[lower1sec_recovery] = src.[lower1sec_recovery],
        tgt.[raise1sec_recovery_gen] = src.[raise1sec_recovery_gen],
        tgt.[lower1sec_recovery_gen] = src.[lower1sec_recovery_gen],
        tgt.[lowerreg_ace] = src.[lowerreg_ace],
        tgt.[raisereg_ace] = src.[raisereg_ace],
        tgt.[raise1sec_ace] = src.[raise1sec_ace],
        tgt.[raise1sec_asoe] = src.[raise1sec_asoe],
        tgt.[lower1sec_ace] = src.[lower1sec_ace],
        tgt.[lower1sec_asoe] = src.[lower1sec_asoe],
        tgt.[raise6sec_ace] = src.[raise6sec_ace],
        tgt.[raise6sec_asoe] = src.[raise6sec_asoe],
        tgt.[lower6sec_ace] = src.[lower6sec_ace],
        tgt.[lower6sec_asoe] = src.[lower6sec_asoe],
        tgt.[raise60sec_ace] = src.[raise60sec_ace],
        tgt.[raise60sec_asoe] = src.[raise60sec_asoe],
        tgt.[lower60sec_ace] = src.[lower60sec_ace],
        tgt.[lower60sec_asoe] = src.[lower60sec_asoe],
        tgt.[raise5min_ace] = src.[raise5min_ace],
        tgt.[raise5min_asoe] = src.[raise5min_asoe],
        tgt.[lower5min_ace] = src.[lower5min_ace],
        tgt.[lower5min_asoe] = src.[lower5min_asoe]
when not matched
    then insert (
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
        [raisereg_recovery_gen],
        [raise1sec_recovery],
        [lower1sec_recovery],
        [raise1sec_recovery_gen],
        [lower1sec_recovery_gen],
        [lowerreg_ace],
        [raisereg_ace],
        [raise1sec_ace],
        [raise1sec_asoe],
        [lower1sec_ace],
        [lower1sec_asoe],
        [raise6sec_ace],
        [raise6sec_asoe],
        [lower6sec_ace],
        [lower6sec_asoe],
        [raise60sec_ace],
        [raise60sec_asoe],
        [lower60sec_ace],
        [lower60sec_asoe],
        [raise5min_ace],
        [raise5min_asoe],
        [lower5min_ace],
        [lower5min_asoe]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[participantid],
        src.[regionid],
        src.[periodid],
        src.[lower6sec_recovery],
        src.[raise6sec_recovery],
        src.[lower60sec_recovery],
        src.[raise60sec_recovery],
        src.[lower5min_recovery],
        src.[raise5min_recovery],
        src.[lowerreg_recovery],
        src.[raisereg_recovery],
        src.[lastchanged],
        src.[lower6sec_recovery_gen],
        src.[raise6sec_recovery_gen],
        src.[lower60sec_recovery_gen],
        src.[raise60sec_recovery_gen],
        src.[lower5min_recovery_gen],
        src.[raise5min_recovery_gen],
        src.[lowerreg_recovery_gen],
        src.[raisereg_recovery_gen],
        src.[raise1sec_recovery],
        src.[lower1sec_recovery],
        src.[raise1sec_recovery_gen],
        src.[lower1sec_recovery_gen],
        src.[lowerreg_ace],
        src.[raisereg_ace],
        src.[raise1sec_ace],
        src.[raise1sec_asoe],
        src.[lower1sec_ace],
        src.[lower1sec_asoe],
        src.[raise6sec_ace],
        src.[raise6sec_asoe],
        src.[lower6sec_ace],
        src.[lower6sec_asoe],
        src.[raise60sec_ace],
        src.[raise60sec_asoe],
        src.[lower60sec_ace],
        src.[lower60sec_asoe],
        src.[raise5min_ace],
        src.[raise5min_asoe],
        src.[lower5min_ace],
        src.[lower5min_asoe]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsSetFcasRegulationTrk2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsSetFcasRegulationTrk2 as tgt 
using (
    select 
        d.[settlementdate],
        d.[versionno],
        d.[interval_datetime],
        d.[constraintid],
        d.[cmpf],
        d.[crmpf],
        d.[recovery_factor_cmpf],
        d.[recovery_factor_crmpf],
        d.[lastchanged],
        d.[usesubstitutedemand],
        d.[requirementdemand]
    from openjson(@data) with (
        [settlementdate] datetime2,
        [versionno] decimal(3,0),
        [interval_datetime] datetime2,
        [constraintid] varchar(20),
        [cmpf] decimal(18,8),
        [crmpf] decimal(18,8),
        [recovery_factor_cmpf] decimal(18,8),
        [recovery_factor_crmpf] decimal(18,8),
        [lastchanged] datetime2,
        [usesubstitutedemand] decimal(1,0),
        [requirementdemand] decimal(18,8)
    ) d
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[constraintid] = src.[constraintid],
        tgt.[cmpf] = src.[cmpf],
        tgt.[crmpf] = src.[crmpf],
        tgt.[recovery_factor_cmpf] = src.[recovery_factor_cmpf],
        tgt.[recovery_factor_crmpf] = src.[recovery_factor_crmpf],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[usesubstitutedemand] = src.[usesubstitutedemand],
        tgt.[requirementdemand] = src.[requirementdemand]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [versionno],
        [interval_datetime],
        [constraintid],
        [cmpf],
        [crmpf],
        [recovery_factor_cmpf],
        [recovery_factor_crmpf],
        [lastchanged],
        [usesubstitutedemand],
        [requirementdemand]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[interval_datetime],
        src.[constraintid],
        src.[cmpf],
        src.[crmpf],
        src.[recovery_factor_cmpf],
        src.[recovery_factor_crmpf],
        src.[lastchanged],
        src.[usesubstitutedemand],
        src.[requirementdemand]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsNmasRecovery3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsNmasRecovery3 as tgt 
using (
    select 
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
        d.[recovery_amount_generator],
        d.[participant_ace_mwh],
        d.[region_ace_mwh],
        d.[participant_asoe_mwh],
        d.[region_asoe_mwh],
        d.[recoveryamount_ace],
        d.[recoveryamount_asoe]
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
        [recovery_amount_generator] decimal(18,8),
        [participant_ace_mwh] decimal(18,8),
        [region_ace_mwh] decimal(18,8),
        [participant_asoe_mwh] decimal(18,8),
        [region_asoe_mwh] decimal(18,8),
        [recoveryamount_ace] decimal(18,8),
        [recoveryamount_asoe] decimal(18,8)
    ) d
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[participantid] = src.[participantid]
    and tgt.[paymenttype] = src.[paymenttype]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[service] = src.[service]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[participantid] = src.[participantid],
        tgt.[service] = src.[service],
        tgt.[contractid] = src.[contractid],
        tgt.[paymenttype] = src.[paymenttype],
        tgt.[regionid] = src.[regionid],
        tgt.[rbf] = src.[rbf],
        tgt.[payment_amount] = src.[payment_amount],
        tgt.[participant_energy] = src.[participant_energy],
        tgt.[region_energy] = src.[region_energy],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[participant_generation] = src.[participant_generation],
        tgt.[region_generation] = src.[region_generation],
        tgt.[recovery_amount_customer] = src.[recovery_amount_customer],
        tgt.[recovery_amount_generator] = src.[recovery_amount_generator],
        tgt.[participant_ace_mwh] = src.[participant_ace_mwh],
        tgt.[region_ace_mwh] = src.[region_ace_mwh],
        tgt.[participant_asoe_mwh] = src.[participant_asoe_mwh],
        tgt.[region_asoe_mwh] = src.[region_asoe_mwh],
        tgt.[recoveryamount_ace] = src.[recoveryamount_ace],
        tgt.[recoveryamount_asoe] = src.[recoveryamount_asoe]
when not matched
    then insert (
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
        [recovery_amount_generator],
        [participant_ace_mwh],
        [region_ace_mwh],
        [participant_asoe_mwh],
        [region_asoe_mwh],
        [recoveryamount_ace],
        [recoveryamount_asoe]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[participantid],
        src.[service],
        src.[contractid],
        src.[paymenttype],
        src.[regionid],
        src.[rbf],
        src.[payment_amount],
        src.[participant_energy],
        src.[region_energy],
        src.[recovery_amount],
        src.[lastchanged],
        src.[participant_generation],
        src.[region_generation],
        src.[recovery_amount_customer],
        src.[recovery_amount_generator],
        src.[participant_ace_mwh],
        src.[region_ace_mwh],
        src.[participant_asoe_mwh],
        src.[region_asoe_mwh],
        src.[recoveryamount_ace],
        src.[recoveryamount_asoe]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsNmasRecoveryRbf1
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.SettlementsNmasRecoveryRbf1 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[contractid] = src.[contractid]
    and tgt.[paymenttype] = src.[paymenttype]
    and tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[service] = src.[service]
    and tgt.[settlementdate] = src.[settlementdate]
    and tgt.[versionno] = src.[versionno]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[versionno] = src.[versionno],
        tgt.[periodid] = src.[periodid],
        tgt.[service] = src.[service],
        tgt.[contractid] = src.[contractid],
        tgt.[paymenttype] = src.[paymenttype],
        tgt.[regionid] = src.[regionid],
        tgt.[rbf] = src.[rbf],
        tgt.[payment_amount] = src.[payment_amount],
        tgt.[recovery_amount] = src.[recovery_amount],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[versionno],
        src.[periodid],
        src.[service],
        src.[contractid],
        src.[paymenttype],
        src.[regionid],
        src.[rbf],
        src.[payment_amount],
        src.[recovery_amount],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertSettlementsRecoveryEnergy2
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.SettlementsRecoveryEnergy2(
file_log_id,
[settlementdate],
        [settlementrunno],
        [participantid],
        [regionid],
        [periodid],
        [customerenergyactual],
        [customerenergympfexactual],
        [customerenergysubstitute],
        [customerenergympfexsubstitute],
        [generatorenergyactual],
        [regioncustenergyactual],
        [regioncustenergympfexactual],
        [regioncustenergysubst],
        [regioncustenergympfexsubst],
        [regiongenenergyactual],
        [ace_mwh_actual],
        [ace_mwh_mpfex_actual],
        [ace_mwh_substitute],
        [ace_mwh_mpfex_substitute],
        [asoe_mwh_actual],
        [region_ace_mwh_actual],
        [region_ace_mwh_mpfex_actual],
        [region_ace_mwh_subst],
        [region_ace_mwh_mpfex_subst],
        [region_asoe_mwh_actual]
)
select 
@file_log_id,
d.[settlementdate],
        d.[settlementrunno],
        d.[participantid],
        d.[regionid],
        d.[periodid],
        d.[customerenergyactual],
        d.[customerenergympfexactual],
        d.[customerenergysubstitute],
        d.[customerenergympfexsubstitute],
        d.[generatorenergyactual],
        d.[regioncustenergyactual],
        d.[regioncustenergympfexactual],
        d.[regioncustenergysubst],
        d.[regioncustenergympfexsubst],
        d.[regiongenenergyactual],
        d.[ace_mwh_actual],
        d.[ace_mwh_mpfex_actual],
        d.[ace_mwh_substitute],
        d.[ace_mwh_mpfex_substitute],
        d.[asoe_mwh_actual],
        d.[region_ace_mwh_actual],
        d.[region_ace_mwh_mpfex_actual],
        d.[region_ace_mwh_subst],
        d.[region_ace_mwh_mpfex_subst],
        d.[region_asoe_mwh_actual]
from openjson(@data) with (
[settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [participantid] varchar(20),
        [regionid] varchar(20),
        [periodid] decimal(3,0),
        [customerenergyactual] decimal(18,8),
        [customerenergympfexactual] decimal(18,8),
        [customerenergysubstitute] decimal(18,8),
        [customerenergympfexsubstitute] decimal(18,8),
        [generatorenergyactual] decimal(18,8),
        [regioncustenergyactual] decimal(18,8),
        [regioncustenergympfexactual] decimal(18,8),
        [regioncustenergysubst] decimal(18,8),
        [regioncustenergympfexsubst] decimal(18,8),
        [regiongenenergyactual] decimal(18,8),
        [ace_mwh_actual] decimal(18,8),
        [ace_mwh_mpfex_actual] decimal(18,8),
        [ace_mwh_substitute] decimal(18,8),
        [ace_mwh_mpfex_substitute] decimal(18,8),
        [asoe_mwh_actual] decimal(18,8),
        [region_ace_mwh_actual] decimal(18,8),
        [region_ace_mwh_mpfex_actual] decimal(18,8),
        [region_ace_mwh_subst] decimal(18,8),
        [region_ace_mwh_mpfex_subst] decimal(18,8),
        [region_asoe_mwh_actual] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertSettlementsSubstDemand1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.SettlementsSubstDemand1(
file_log_id,
[settlementdate],
        [settlementrunno],
        [tni],
        [participantid],
        [regionid],
        [substitutedemand]
)
select 
@file_log_id,
d.[settlementdate],
        d.[settlementrunno],
        d.[tni],
        d.[participantid],
        d.[regionid],
        d.[substitutedemand]
from openjson(@data) with (
[settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [tni] varchar(20),
        [participantid] varchar(20),
        [regionid] varchar(20),
        [substitutedemand] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertSettlementsSubstRunVersion1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.SettlementsSubstRunVersion1(
file_log_id,
[settlementdate],
        [settlementrunno],
        [referencesettlementdate],
        [referencesettlementrunno]
)
select 
@file_log_id,
d.[settlementdate],
        d.[settlementrunno],
        d.[referencesettlementdate],
        d.[referencesettlementrunno]
from openjson(@data) with (
[settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [referencesettlementdate] datetime2,
        [referencesettlementrunno] decimal(3,0)
) d
end
go
create or alter procedure mmsdm_proc.InsertSettlementsWdrReconDetail1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.SettlementsWdrReconDetail1(
file_log_id,
[settlementdate],
        [settlementrunno],
        [nmi],
        [tni],
        [regionid],
        [frmp],
        [drsp],
        [periodid],
        [wdrsq_uncapped],
        [wdrsq_capped],
        [mrc],
        [mrcsq],
        [wdrrr],
        [rrp],
        [tlf],
        [me_dlfadjusted],
        [bq_dlfadjusted],
        [isnoncompliant],
        [qualityflag],
        [transactionamount],
        [baselinecalculationid]
)
select 
@file_log_id,
d.[settlementdate],
        d.[settlementrunno],
        d.[nmi],
        d.[tni],
        d.[regionid],
        d.[frmp],
        d.[drsp],
        d.[periodid],
        d.[wdrsq_uncapped],
        d.[wdrsq_capped],
        d.[mrc],
        d.[mrcsq],
        d.[wdrrr],
        d.[rrp],
        d.[tlf],
        d.[me_dlfadjusted],
        d.[bq_dlfadjusted],
        d.[isnoncompliant],
        d.[qualityflag],
        d.[transactionamount],
        d.[baselinecalculationid]
from openjson(@data) with (
[settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [nmi] varchar(20),
        [tni] varchar(20),
        [regionid] varchar(20),
        [frmp] varchar(20),
        [drsp] varchar(20),
        [periodid] decimal(3,0),
        [wdrsq_uncapped] decimal(18,8),
        [wdrsq_capped] decimal(18,8),
        [mrc] decimal(18,8),
        [mrcsq] decimal(18,8),
        [wdrrr] decimal(18,8),
        [rrp] decimal(18,8),
        [tlf] decimal(18,8),
        [me_dlfadjusted] decimal(18,8),
        [bq_dlfadjusted] decimal(18,8),
        [isnoncompliant] decimal(1,0),
        [qualityflag] varchar(20),
        [transactionamount] decimal(18,8),
        [baselinecalculationid] varchar(100)
) d
end
go
create or alter procedure mmsdm_proc.InsertSettlementsWdrTransact1
    @file_log_id bigint,
    @data nvarchar(max)
as begin
insert into mmsdm.SettlementsWdrTransact1(
file_log_id,
[settlementdate],
        [settlementrunno],
        [periodid],
        [regionid],
        [participantid],
        [participantroleid],
        [counterpartyparticipantid],
        [transactionamount]
)
select 
@file_log_id,
d.[settlementdate],
        d.[settlementrunno],
        d.[periodid],
        d.[regionid],
        d.[participantid],
        d.[participantroleid],
        d.[counterpartyparticipantid],
        d.[transactionamount]
from openjson(@data) with (
[settlementdate] datetime2,
        [settlementrunno] decimal(3,0),
        [periodid] decimal(3,0),
        [regionid] varchar(20),
        [participantid] varchar(20),
        [participantroleid] varchar(20),
        [counterpartyparticipantid] varchar(20),
        [transactionamount] decimal(18,8)
) d
end
go
create or alter procedure mmsdm_proc.InsertStpasaCasesolution3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.StpasaCasesolution3 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[run_datetime] = src.[run_datetime]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[pasaversion] = src.[pasaversion],
        tgt.[reservecondition] = src.[reservecondition],
        tgt.[lorcondition] = src.[lorcondition],
        tgt.[capacityobjfunction] = src.[capacityobjfunction],
        tgt.[capacityoption] = src.[capacityoption],
        tgt.[maxsurplusreserveoption] = src.[maxsurplusreserveoption],
        tgt.[maxsparecapacityoption] = src.[maxsparecapacityoption],
        tgt.[interconnectorflowpenalty] = src.[interconnectorflowpenalty],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[reliabilitylrcdemandoption] = src.[reliabilitylrcdemandoption],
        tgt.[outagelrcdemandoption] = src.[outagelrcdemandoption],
        tgt.[lordemandoption] = src.[lordemandoption],
        tgt.[reliabilitylrccapacityoption] = src.[reliabilitylrccapacityoption],
        tgt.[outagelrccapacityoption] = src.[outagelrccapacityoption],
        tgt.[lorcapacityoption] = src.[lorcapacityoption],
        tgt.[loruigf_option] = src.[loruigf_option],
        tgt.[reliability_lrcuigf_option] = src.[reliability_lrcuigf_option],
        tgt.[outage_lrcuigf_option] = src.[outage_lrcuigf_option]
when not matched
    then insert (
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
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[pasaversion],
        src.[reservecondition],
        src.[lorcondition],
        src.[capacityobjfunction],
        src.[capacityoption],
        src.[maxsurplusreserveoption],
        src.[maxsparecapacityoption],
        src.[interconnectorflowpenalty],
        src.[lastchanged],
        src.[reliabilitylrcdemandoption],
        src.[outagelrcdemandoption],
        src.[lordemandoption],
        src.[reliabilitylrccapacityoption],
        src.[outagelrccapacityoption],
        src.[lorcapacityoption],
        src.[loruigf_option],
        src.[reliability_lrcuigf_option],
        src.[outage_lrcuigf_option]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertStpasaConstraintsolution3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.StpasaConstraintsolution3 as tgt 
using (
    select 
        d.[run_datetime],
        d.[interval_datetime],
        d.[constraintid],
        d.[capacityrhs],
        d.[capacitymarginalvalue],
        d.[capacityviolationdegree],
        d.[lastchanged],
        d.[runtype],
        d.[studyregionid]
    from openjson(@data) with (
        [run_datetime] datetime2,
        [interval_datetime] datetime2,
        [constraintid] varchar(20),
        [capacityrhs] decimal(12,2),
        [capacitymarginalvalue] decimal(12,2),
        [capacityviolationdegree] decimal(12,2),
        [lastchanged] datetime2,
        [runtype] varchar(20),
        [studyregionid] varchar(20)
    ) d
) as src 
on (
    tgt.[constraintid] = src.[constraintid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[runtype] = src.[runtype]
    and tgt.[studyregionid] = src.[studyregionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[constraintid] = src.[constraintid],
        tgt.[capacityrhs] = src.[capacityrhs],
        tgt.[capacitymarginalvalue] = src.[capacitymarginalvalue],
        tgt.[capacityviolationdegree] = src.[capacityviolationdegree],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[runtype] = src.[runtype],
        tgt.[studyregionid] = src.[studyregionid]
when not matched
    then insert (
        file_log_id,
        [run_datetime],
        [interval_datetime],
        [constraintid],
        [capacityrhs],
        [capacitymarginalvalue],
        [capacityviolationdegree],
        [lastchanged],
        [runtype],
        [studyregionid]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[constraintid],
        src.[capacityrhs],
        src.[capacitymarginalvalue],
        src.[capacityviolationdegree],
        src.[lastchanged],
        src.[runtype],
        src.[studyregionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertStpasaInterconnectorsoln3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.StpasaInterconnectorsoln3 as tgt 
using (
    select 
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
        d.[importlimitconstraintid],
        d.[studyregionid]
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
        [importlimitconstraintid] varchar(20),
        [studyregionid] varchar(20)
    ) d
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[runtype] = src.[runtype]
    and tgt.[studyregionid] = src.[studyregionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[capacitymwflow] = src.[capacitymwflow],
        tgt.[capacitymarginalvalue] = src.[capacitymarginalvalue],
        tgt.[capacityviolationdegree] = src.[capacityviolationdegree],
        tgt.[calculatedexportlimit] = src.[calculatedexportlimit],
        tgt.[calculatedimportlimit] = src.[calculatedimportlimit],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[runtype] = src.[runtype],
        tgt.[exportlimitconstraintid] = src.[exportlimitconstraintid],
        tgt.[importlimitconstraintid] = src.[importlimitconstraintid],
        tgt.[studyregionid] = src.[studyregionid]
when not matched
    then insert (
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
        [importlimitconstraintid],
        [studyregionid]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[interconnectorid],
        src.[capacitymwflow],
        src.[capacitymarginalvalue],
        src.[capacityviolationdegree],
        src.[calculatedexportlimit],
        src.[calculatedimportlimit],
        src.[lastchanged],
        src.[runtype],
        src.[exportlimitconstraintid],
        src.[importlimitconstraintid],
        src.[studyregionid]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertStpasaRegionsolution7
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.StpasaRegionsolution7 as tgt 
using (
    select 
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
        d.[semischeduledcapacity],
        d.[lor_semischeduledcapacity],
        d.[lcr],
        d.[lcr2],
        d.[fum],
        d.[ss_solar_uigf],
        d.[ss_wind_uigf],
        d.[ss_solar_capacity],
        d.[ss_wind_capacity],
        d.[ss_solar_cleared],
        d.[ss_wind_cleared],
        d.[wdr_available],
        d.[wdr_pasaavailable],
        d.[wdr_capacity]
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
        [semischeduledcapacity] decimal(12,2),
        [lor_semischeduledcapacity] decimal(12,2),
        [lcr] decimal(16,6),
        [lcr2] decimal(16,6),
        [fum] decimal(16,6),
        [ss_solar_uigf] decimal(12,2),
        [ss_wind_uigf] decimal(12,2),
        [ss_solar_capacity] decimal(12,2),
        [ss_wind_capacity] decimal(12,2),
        [ss_solar_cleared] decimal(12,2),
        [ss_wind_cleared] decimal(12,2),
        [wdr_available] decimal(12,2),
        [wdr_pasaavailable] decimal(12,2),
        [wdr_capacity] decimal(12,2)
    ) d
) as src 
on (
    tgt.[interval_datetime] = src.[interval_datetime]
    and tgt.[regionid] = src.[regionid]
    and tgt.[run_datetime] = src.[run_datetime]
    and tgt.[runtype] = src.[runtype]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[run_datetime] = src.[run_datetime],
        tgt.[interval_datetime] = src.[interval_datetime],
        tgt.[regionid] = src.[regionid],
        tgt.[demand10] = src.[demand10],
        tgt.[demand50] = src.[demand50],
        tgt.[demand90] = src.[demand90],
        tgt.[reservereq] = src.[reservereq],
        tgt.[capacityreq] = src.[capacityreq],
        tgt.[energyreqdemand50] = src.[energyreqdemand50],
        tgt.[unconstrainedcapacity] = src.[unconstrainedcapacity],
        tgt.[constrainedcapacity] = src.[constrainedcapacity],
        tgt.[netinterchangeunderscarcity] = src.[netinterchangeunderscarcity],
        tgt.[surpluscapacity] = src.[surpluscapacity],
        tgt.[surplusreserve] = src.[surplusreserve],
        tgt.[reservecondition] = src.[reservecondition],
        tgt.[maxsurplusreserve] = src.[maxsurplusreserve],
        tgt.[maxsparecapacity] = src.[maxsparecapacity],
        tgt.[lorcondition] = src.[lorcondition],
        tgt.[aggregatecapacityavailable] = src.[aggregatecapacityavailable],
        tgt.[aggregatescheduledload] = src.[aggregatescheduledload],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[aggregatepasaavailability] = src.[aggregatepasaavailability],
        tgt.[runtype] = src.[runtype],
        tgt.[energyreqdemand10] = src.[energyreqdemand10],
        tgt.[calculatedlor1level] = src.[calculatedlor1level],
        tgt.[calculatedlor2level] = src.[calculatedlor2level],
        tgt.[msrnetinterchangeunderscarcity] = src.[msrnetinterchangeunderscarcity],
        tgt.[lornetinterchangeunderscarcity] = src.[lornetinterchangeunderscarcity],
        tgt.[totalintermittentgeneration] = src.[totalintermittentgeneration],
        tgt.[demand_and_nonschedgen] = src.[demand_and_nonschedgen],
        tgt.[uigf] = src.[uigf],
        tgt.[semischeduledcapacity] = src.[semischeduledcapacity],
        tgt.[lor_semischeduledcapacity] = src.[lor_semischeduledcapacity],
        tgt.[lcr] = src.[lcr],
        tgt.[lcr2] = src.[lcr2],
        tgt.[fum] = src.[fum],
        tgt.[ss_solar_uigf] = src.[ss_solar_uigf],
        tgt.[ss_wind_uigf] = src.[ss_wind_uigf],
        tgt.[ss_solar_capacity] = src.[ss_solar_capacity],
        tgt.[ss_wind_capacity] = src.[ss_wind_capacity],
        tgt.[ss_solar_cleared] = src.[ss_solar_cleared],
        tgt.[ss_wind_cleared] = src.[ss_wind_cleared],
        tgt.[wdr_available] = src.[wdr_available],
        tgt.[wdr_pasaavailable] = src.[wdr_pasaavailable],
        tgt.[wdr_capacity] = src.[wdr_capacity]
when not matched
    then insert (
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
        [semischeduledcapacity],
        [lor_semischeduledcapacity],
        [lcr],
        [lcr2],
        [fum],
        [ss_solar_uigf],
        [ss_wind_uigf],
        [ss_solar_capacity],
        [ss_wind_capacity],
        [ss_solar_cleared],
        [ss_wind_cleared],
        [wdr_available],
        [wdr_pasaavailable],
        [wdr_capacity]
    ) values (
        @file_log_id,
        src.[run_datetime],
        src.[interval_datetime],
        src.[regionid],
        src.[demand10],
        src.[demand50],
        src.[demand90],
        src.[reservereq],
        src.[capacityreq],
        src.[energyreqdemand50],
        src.[unconstrainedcapacity],
        src.[constrainedcapacity],
        src.[netinterchangeunderscarcity],
        src.[surpluscapacity],
        src.[surplusreserve],
        src.[reservecondition],
        src.[maxsurplusreserve],
        src.[maxsparecapacity],
        src.[lorcondition],
        src.[aggregatecapacityavailable],
        src.[aggregatescheduledload],
        src.[lastchanged],
        src.[aggregatepasaavailability],
        src.[runtype],
        src.[energyreqdemand10],
        src.[calculatedlor1level],
        src.[calculatedlor2level],
        src.[msrnetinterchangeunderscarcity],
        src.[lornetinterchangeunderscarcity],
        src.[totalintermittentgeneration],
        src.[demand_and_nonschedgen],
        src.[uigf],
        src.[semischeduledcapacity],
        src.[lor_semischeduledcapacity],
        src.[lcr],
        src.[lcr2],
        src.[fum],
        src.[ss_solar_uigf],
        src.[ss_wind_uigf],
        src.[ss_solar_capacity],
        src.[ss_wind_capacity],
        src.[ss_solar_cleared],
        src.[ss_wind_cleared],
        src.[wdr_available],
        src.[wdr_pasaavailable],
        src.[wdr_capacity]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertTradingAverageprice301
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.TradingAverageprice301 as tgt 
using (
    select 
        d.[perioddate],
        d.[regionid],
        d.[periodid],
        d.[rrp],
        d.[price_confidence],
        d.[lastchanged]
    from openjson(@data) with (
        [perioddate] datetime2,
        [regionid] varchar(10),
        [periodid] decimal(3,0),
        [rrp] decimal(15,5),
        [price_confidence] varchar(20),
        [lastchanged] datetime2
    ) d
) as src 
on (
    tgt.[perioddate] = src.[perioddate]
    and tgt.[regionid] = src.[regionid]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[perioddate] = src.[perioddate],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[rrp] = src.[rrp],
        tgt.[price_confidence] = src.[price_confidence],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [perioddate],
        [regionid],
        [periodid],
        [rrp],
        [price_confidence],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[perioddate],
        src.[regionid],
        src.[periodid],
        src.[rrp],
        src.[price_confidence],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertTradingInterconnectorres2
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.TradingInterconnectorres2 as tgt 
using (
    select 
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
) as src 
on (
    tgt.[interconnectorid] = src.[interconnectorid]
    and tgt.[periodid] = src.[periodid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[interconnectorid] = src.[interconnectorid],
        tgt.[periodid] = src.[periodid],
        tgt.[meteredmwflow] = src.[meteredmwflow],
        tgt.[mwflow] = src.[mwflow],
        tgt.[mwlosses] = src.[mwlosses],
        tgt.[lastchanged] = src.[lastchanged]
when not matched
    then insert (
        file_log_id,
        [settlementdate],
        [runno],
        [interconnectorid],
        [periodid],
        [meteredmwflow],
        [mwflow],
        [mwlosses],
        [lastchanged]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[interconnectorid],
        src.[periodid],
        src.[meteredmwflow],
        src.[mwflow],
        src.[mwlosses],
        src.[lastchanged]
    );
    
end
go
create or alter procedure mmsdm_proc.InsertTradingPrice3
    @file_log_id bigint,
    @data nvarchar(max)
as begin

merge mmsdm.TradingPrice3 as tgt 
using (
    select 
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
        d.[price_status],
        d.[raise1secrrp],
        d.[raise1secrop],
        d.[lower1secrrp],
        d.[lower1secrop]
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
        [price_status] varchar(20),
        [raise1secrrp] decimal(15,5),
        [raise1secrop] decimal(15,5),
        [lower1secrrp] decimal(15,5),
        [lower1secrop] decimal(15,5)
    ) d
) as src 
on (
    tgt.[periodid] = src.[periodid]
    and tgt.[regionid] = src.[regionid]
    and tgt.[runno] = src.[runno]
    and tgt.[settlementdate] = src.[settlementdate]

)
when matched and src.[lastchanged] > tgt.[lastchanged]
    then update set 
        tgt.file_log_id = @file_log_id,
        tgt.[settlementdate] = src.[settlementdate],
        tgt.[runno] = src.[runno],
        tgt.[regionid] = src.[regionid],
        tgt.[periodid] = src.[periodid],
        tgt.[rrp] = src.[rrp],
        tgt.[eep] = src.[eep],
        tgt.[invalidflag] = src.[invalidflag],
        tgt.[lastchanged] = src.[lastchanged],
        tgt.[rop] = src.[rop],
        tgt.[raise6secrrp] = src.[raise6secrrp],
        tgt.[raise6secrop] = src.[raise6secrop],
        tgt.[raise60secrrp] = src.[raise60secrrp],
        tgt.[raise60secrop] = src.[raise60secrop],
        tgt.[raise5minrrp] = src.[raise5minrrp],
        tgt.[raise5minrop] = src.[raise5minrop],
        tgt.[raiseregrrp] = src.[raiseregrrp],
        tgt.[raiseregrop] = src.[raiseregrop],
        tgt.[lower6secrrp] = src.[lower6secrrp],
        tgt.[lower6secrop] = src.[lower6secrop],
        tgt.[lower60secrrp] = src.[lower60secrrp],
        tgt.[lower60secrop] = src.[lower60secrop],
        tgt.[lower5minrrp] = src.[lower5minrrp],
        tgt.[lower5minrop] = src.[lower5minrop],
        tgt.[lowerregrrp] = src.[lowerregrrp],
        tgt.[lowerregrop] = src.[lowerregrop],
        tgt.[price_status] = src.[price_status],
        tgt.[raise1secrrp] = src.[raise1secrrp],
        tgt.[raise1secrop] = src.[raise1secrop],
        tgt.[lower1secrrp] = src.[lower1secrrp],
        tgt.[lower1secrop] = src.[lower1secrop]
when not matched
    then insert (
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
        [price_status],
        [raise1secrrp],
        [raise1secrop],
        [lower1secrrp],
        [lower1secrop]
    ) values (
        @file_log_id,
        src.[settlementdate],
        src.[runno],
        src.[regionid],
        src.[periodid],
        src.[rrp],
        src.[eep],
        src.[invalidflag],
        src.[lastchanged],
        src.[rop],
        src.[raise6secrrp],
        src.[raise6secrop],
        src.[raise60secrrp],
        src.[raise60secrop],
        src.[raise5minrrp],
        src.[raise5minrop],
        src.[raiseregrrp],
        src.[raiseregrop],
        src.[lower6secrrp],
        src.[lower6secrop],
        src.[lower60secrrp],
        src.[lower60secrop],
        src.[lower5minrrp],
        src.[lower5minrop],
        src.[lowerregrrp],
        src.[lowerregrop],
        src.[price_status],
        src.[raise1secrrp],
        src.[raise1secrop],
        src.[lower1secrrp],
        src.[lower1secrop]
    );
    
end
go