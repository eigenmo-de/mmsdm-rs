
create schema mmsdm
go
create schema mmsdm_proc
go
create table mmsdm.FileLog (
    file_log_id bigint identity(1,1) not null primary key,
    data_source varchar(255) not null,
    file_name varchar(255) not null,
    from_participant varchar(255) not null,
    to_participant varchar(255) not null,
    effective_date date,
    effective_time time,
    serial_number bigint not null,
    file_name_2 varchar(255) null,
    serial_number_2 bigint null,
    data_set varchar(255) not null,
    sub_type varchar(255) not null,
    version tinyint not null,
    [status] char(1) not null default 'P' check ([status] in ('P','E','C')),
    message varchar(max) null,
    check ((status != 'E' and message is null) or (status = 'E' and message is not null)),
    unique (to_participant, serial_number, data_set, sub_type, version)
)
go
            
create table mmsdm.AncilliaryServicesContractagc1 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[participantid] varchar(10) null,
[duid] varchar(10) null,
[crr] decimal(4,0) null,
[crl] decimal(4,0) null,
[rlprice] decimal(10,2) null,
[ccprice] decimal(10,2) null,
[bs] decimal(10,2) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([contractid],[versionno])
)
go
                        
create table mmsdm.AncilliaryServicesContractloadshed2 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[participantid] varchar(10) null,
[duid] varchar(10) null,
[lseprice] decimal(6,2) null,
[mcpprice] decimal(12,2) null,
[tenderedprice] decimal(6,2) null,
[lscr] decimal(6,2) null,
[ilscalingfactor] decimal(15,5) null,
[lower60secbreakpoint] decimal(9,6) null,
[lower60secmax] decimal(9,6) null,
[lower6secbreakpoint] decimal(9,6) null,
[lower6secmax] decimal(9,6) null,
[raise60secbreakpoint] decimal(9,6) null,
[raise60seccapacity] decimal(9,6) null,
[raise60secmax] decimal(9,6) null,
[raise6secbreakpoint] decimal(9,6) null,
[raise6seccapacity] decimal(9,6) null,
[raise6secmax] decimal(9,6) null,
[price6secraisemandatory] decimal(16,6) null,
[quant6secraisemandatory] decimal(9,6) null,
[price6secraisecontract] decimal(16,6) null,
[quant6secraisecontract] decimal(9,6) null,
[price60secraisemandatory] decimal(16,6) null,
[quant60secraisemandatory] decimal(9,6) null,
[price60secraisecontract] decimal(16,6) null,
[quant60secraisecontract] decimal(9,6) null,
[price6seclowermandatory] decimal(16,6) null,
[quant6seclowermandatory] decimal(9,6) null,
[price6seclowercontract] decimal(16,6) null,
[quant6seclowercontract] decimal(9,6) null,
[price60seclowermandatory] decimal(16,6) null,
[quant60seclowermandatory] decimal(9,6) null,
[price60seclowercontract] decimal(16,6) null,
[quant60seclowercontract] decimal(9,6) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
[default_testingpayment_amount] decimal(18,8) null,
[service_start_date] datetime2 null,
    primary key ([contractid],[versionno])
)
go
                        
create table mmsdm.AncilliaryServicesContractreactivepower4 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[participantid] varchar(10) null,
[duid] varchar(10) null,
[synccompensation] varchar(1) null,
[mvaraprice] decimal(10,2) null,
[mvareprice] decimal(10,2) null,
[mvargprice] decimal(10,2) null,
[ccprice] decimal(10,2) null,
[mta] decimal(10,2) null,
[mtg] decimal(10,2) null,
[mmca] decimal(10,2) null,
[mmcg] decimal(10,2) null,
[eu] decimal(10,2) null,
[pp] decimal(10,2) null,
[bs] decimal(10,2) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
[default_testingpayment_amount] decimal(18,8) null,
[service_start_date] datetime2 null,
[availability_mwh_threshold] decimal(18,8) null,
[mvar_threshold] decimal(18,8) null,
[rebate_cap] decimal(18,8) null,
[rebate_amount_per_mvar] decimal(18,8) null,
[isrebateapplicable] decimal(1,0) null,
    primary key ([contractid],[versionno])
)
go
                        
create table mmsdm.AncilliaryServicesContractrestartservices2 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[participantid] varchar(10) null,
[restarttype] decimal(1,0) null,
[rcprice] decimal(6,2) null,
[triptohouselevel] decimal(5,0) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
[default_testingpayment_amount] decimal(18,8) null,
[service_start_date] datetime2 null,
    primary key ([contractid],[versionno])
)
go
                        
create table mmsdm.AncilliaryServicesContractrestartunits1 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[duid] varchar(10) not null,
[lastchanged] datetime2 null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
    primary key ([contractid],[duid],[versionno])
)
go
                        
create table mmsdm.AsofferOfferagcdata1 (
file_log_id bigint not null,
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
    primary key ([contractid],[effectivedate],[periodid],[versionno])
)
go
                        
create table mmsdm.AsofferOfferastrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid],[versionno])
)
go
                        
create table mmsdm.AsofferOfferlsheddata1 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[availableload] decimal(4,0) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,
[periodid] decimal(3,0) not null,
    primary key ([contractid],[effectivedate],[periodid],[versionno])
)
go
                        
create table mmsdm.AsofferOfferrestartdata1 (
file_log_id bigint not null,
    [contractid] varchar(10) not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[availability] varchar(3) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,
[periodid] decimal(3,0) not null,
    primary key ([contractid],[offerdate],[periodid],[versionno])
)
go
                        
create table mmsdm.AsofferOfferrpowerdata1 (
file_log_id bigint not null,
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
    primary key ([contractid],[effectivedate],[periodid],[versionno])
)
go
                        
create table mmsdm.BidsBiddayoffer1 (
file_log_id bigint not null,
    [duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[versionno] decimal(22,0) null,
[participantid] varchar(10) not null,
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
[rebid_event_time] varchar(20) null,
[rebid_aware_time] varchar(20) null,
[rebid_decision_time] varchar(20) null,
[rebid_category] varchar(1) null,
[reference_id] varchar(100) null,
    primary key ([bidtype],[duid],[offerdate],[settlementdate])
)
go
                        
create table mmsdm.BidsBidofferfiletrk1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[offerdate] datetime2 not null,
[filename] varchar(80) not null,
[status] varchar(10) null,
[lastchanged] datetime2 null,
[authorisedby] varchar(20) null,
[authoriseddate] datetime2 null,
[transaction_id] varchar(100) null,
[reference_id] varchar(100) null,
[submission_timestamp] datetime2 null,
[comments] varchar(1000) null,
[submission_method] varchar(20) null,
    primary key ([offerdate],[participantid])
)
go
                        
create table mmsdm.BidsBidofferperiod1 (
file_log_id bigint not null,
    [duid] varchar(20) not null,
[bidtype] varchar(10) not null,
[tradingdate] datetime2 not null,
[offerdatetime] datetime2 not null,
[periodid] decimal(3,0) not null,
[maxavail] decimal(8,3) null,
[fixedload] decimal(8,3) null,
[rampuprate] decimal(6,0) null,
[rampdownrate] decimal(6,0) null,
[enablementmin] decimal(8,3) null,
[enablementmax] decimal(8,3) null,
[lowbreakpoint] decimal(8,3) null,
[highbreakpoint] decimal(8,3) null,
[bandavail1] decimal(8,3) null,
[bandavail2] decimal(8,3) null,
[bandavail3] decimal(8,3) null,
[bandavail4] decimal(8,3) null,
[bandavail5] decimal(8,3) null,
[bandavail6] decimal(8,3) null,
[bandavail7] decimal(8,3) null,
[bandavail8] decimal(8,3) null,
[bandavail9] decimal(8,3) null,
[bandavail10] decimal(8,3) null,
[pasaavailability] decimal(8,3) null,
    primary key ([bidtype],[duid],[offerdatetime],[periodid],[tradingdate])
)
go
                        
create table mmsdm.BidsMnspBidofferperiod1 (
file_log_id bigint not null,
    [linkid] varchar(20) not null,
[tradingdate] datetime2 not null,
[offerdatetime] datetime2 not null,
[periodid] decimal(3,0) not null,
[maxavail] decimal(8,3) null,
[fixedload] decimal(8,3) null,
[rampuprate] decimal(6,0) null,
[bandavail1] decimal(8,3) null,
[bandavail2] decimal(8,3) null,
[bandavail3] decimal(8,3) null,
[bandavail4] decimal(8,3) null,
[bandavail5] decimal(8,3) null,
[bandavail6] decimal(8,3) null,
[bandavail7] decimal(8,3) null,
[bandavail8] decimal(8,3) null,
[bandavail9] decimal(8,3) null,
[bandavail10] decimal(8,3) null,
[pasaavailability] decimal(8,3) null,
    primary key ([linkid],[offerdatetime],[periodid],[tradingdate])
)
go
                        
create table mmsdm.BidsMnspDayoffer1 (
file_log_id bigint not null,
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
[rebid_event_time] varchar(20) null,
[rebid_aware_time] varchar(20) null,
[rebid_decision_time] varchar(20) null,
[rebid_category] varchar(1) null,
[reference_id] varchar(100) null,
    primary key ([linkid],[offerdate],[participantid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.OfferMtpasaOfferdata1 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[offerdatetime],[participantid],[unitid])
)
go
                        
create table mmsdm.OfferMtpasaOfferfiletrk1 (
file_log_id bigint not null,
    [participantid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[filename] varchar(200) null,
    primary key ([offerdatetime],[participantid])
)
go
                        
create table mmsdm.BillingConfigBillingcalendar2 (
file_log_id bigint not null,
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
    primary key ([contractyear],[weekno])
)
go
                        
create table mmsdm.BillingConfigGstBasClass1 (
file_log_id bigint not null,
    [bas_class] varchar(30) not null,
[description] varchar(100) null,
[lastchanged] datetime2 null,
    primary key ([bas_class])
)
go
                        
create table mmsdm.BillingConfigGstRate1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[bas_class] varchar(30) not null,
[gst_rate] decimal(8,5) null,
[lastchanged] datetime2 null,
    primary key ([bas_class],[effectivedate],[versionno])
)
go
                        
create table mmsdm.BillingConfigGstTransactionClass1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[transaction_type] varchar(30) not null,
[bas_class] varchar(30) not null,
[lastchanged] datetime2 null,
    primary key ([bas_class],[effectivedate],[transaction_type],[versionno])
)
go
                        
create table mmsdm.BillingConfigGstTransactionType1 (
file_log_id bigint not null,
    [transaction_type] varchar(30) not null,
[description] varchar(100) null,
[gl_financialcode] varchar(10) null,
[gl_tcode] varchar(15) null,
[lastchanged] datetime2 null,
    primary key ([transaction_type])
)
go
                        
create table mmsdm.BillingConfigSecdepositInterestRate1 (
file_log_id bigint not null,
    [interest_acct_id] varchar(20) not null,
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[interest_rate] decimal(18,8) null,
    primary key ([effectivedate],[interest_acct_id],[version_datetime])
)
go
                        
create table mmsdm.BillingConfigSecdepositProvision1 (
file_log_id bigint not null,
    [security_deposit_id] varchar(20) not null,
[participantid] varchar(20) not null,
[transaction_date] datetime2 null,
[maturity_contractyear] decimal(4,0) null,
[maturity_weekno] decimal(3,0) null,
[amount] decimal(18,8) null,
[interest_rate] decimal(18,8) null,
[interest_calc_type] varchar(20) null,
[interest_acct_id] varchar(20) null,
    primary key ([participantid],[security_deposit_id])
)
go
                        
create table mmsdm.BillingAspayments6 (
file_log_id bigint not null,
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
    primary key ([billrunno],[connectionpointid],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingAsrecovery7 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[participantid],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingCpdata6 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[connectionpointid] varchar(10) not null,
[aggregateenergy] decimal(16,6) null,
[purchases] decimal(16,6) null,
[lastchanged] datetime2 null,
[mda] varchar(10) not null,
[afe] decimal(18,8) null,
[dme] decimal(18,8) null,
[ufea] decimal(18,8) null,
[age] decimal(18,8) null,
    primary key ([billrunno],[connectionpointid],[contractyear],[mda],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingDaytrk5 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[settlementdate] datetime2 not null,
[runno] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([billrunno],[contractyear],[settlementdate],[weekno])
)
go
                        
create table mmsdm.BillingFees5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[marketfeeid],[participantcategoryid],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingFinancialadjustments5 (
file_log_id bigint not null,
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
    primary key ([adjustmentitem],[billrunno],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingGendata5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[connectionpointid],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingInterresidues5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[interconnectorid],[participantid],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingIntraresidues5 (
file_log_id bigint not null,
    [allocation] decimal(6,3) null,
[totalsurplus] decimal(15,5) null,
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[surplusvalue] decimal(15,6) null,
[lastchanged] datetime2 null,
[regionid] varchar(10) not null,
    primary key ([billrunno],[contractyear],[participantid],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingIraucsurplus5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractid],[contractyear],[fromregionid],[interconnectorid],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingIraucsurplussum7 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[fromregionid],[interconnectorid],[participantid],[quarter],[residueyear],[weekno])
)
go
                        
create table mmsdm.BillingIrfm5 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[irfmpayment] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([billrunno],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingIrnspsurplus5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractid],[contractyear],[fromregionid],[interconnectorid],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingIrnspsurplussum6 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[fromregionid],[interconnectorid],[participantid],[quarter],[residueyear],[weekno])
)
go
                        
create table mmsdm.BillingIrpartsurplus5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractid],[contractyear],[fromregionid],[interconnectorid],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingIrpartsurplussum7 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[fromregionid],[interconnectorid],[participantid],[quarter],[residueyear],[weekno])
)
go
                        
create table mmsdm.BillingPrioradjustments5 (
file_log_id bigint not null,
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
    primary key ([adjbillrunno],[adjcontractyear],[adjweekno],[billrunno],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingRealloc5 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[counterparty] varchar(10) not null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([billrunno],[contractyear],[counterparty],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingReallocDetail5 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[counterparty] varchar(10) not null,
[reallocationid] varchar(20) not null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([billrunno],[contractyear],[counterparty],[participantid],[reallocationid],[weekno])
)
go
                        
create table mmsdm.BillingRegionexports5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[exportto],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingRegionfigures5 (
file_log_id bigint not null,
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
[wdrsq] decimal(18,8) null,
[wdrta] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingRegionimports5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[importfrom],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingRuntrk5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[weekno])
)
go
                        
create table mmsdm.BillingApcCompensation2 (
file_log_id bigint not null,
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
    primary key ([apeventid],[billrunno],[claimid],[contractyear],[weekno])
)
go
                        
create table mmsdm.BillingApcRecovery2 (
file_log_id bigint not null,
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
    primary key ([apeventid],[billrunno],[claimid],[contractyear],[participantid],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingBillingCo2ePublication1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[settlementdate] datetime2 not null,
[regionid] varchar(20) not null,
[sentoutenergy] decimal(18,8) null,
[generatoremissions] decimal(18,8) null,
[intensityindex] decimal(18,8) null,
    primary key ([contractyear],[regionid],[settlementdate],[weekno])
)
go
                        
create table mmsdm.BillingBillingCo2ePublicationTrk1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([contractyear],[weekno])
)
go
                        
create table mmsdm.BillingDailyEnergySummary1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[settlementdate] datetime2 not null,
[participantid] varchar(20) not null,
[regionid] varchar(20) not null,
[customer_energy_purchased] decimal(18,8) null,
[generator_energy_sold] decimal(18,8) null,
[generator_energy_purchased] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[participantid],[regionid],[settlementdate],[weekno])
)
go
                        
create table mmsdm.BillingDirectionReconciliatn1 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[direction_id],[weekno])
)
go
                        
create table mmsdm.BillingBillingDirectionReconOther1 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[direction_id],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingEftshortfallAmount1 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingEftshortfallDetail1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[transaction_type] varchar(40) not null,
[amount] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[participantid],[transaction_type],[weekno])
)
go
                        
create table mmsdm.BillingGstDetail5 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[bas_class] varchar(30) not null,
[transaction_type] varchar(30) not null,
[gst_exclusive_amount] decimal(15,5) null,
[gst_amount] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([bas_class],[billrunno],[contractyear],[participantid],[transaction_type],[weekno])
)
go
                        
create table mmsdm.BillingGstSummary5 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[bas_class] varchar(30) not null,
[gst_exclusive_amount] decimal(15,5) null,
[gst_amount] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([bas_class],[billrunno],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingNmasTstPayments1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[service] varchar(10) not null,
[contractid] varchar(10) not null,
[payment_amount] decimal(18,8) null,
    primary key ([billrunno],[contractid],[contractyear],[participantid],[service],[weekno])
)
go
                        
create table mmsdm.BillingNmasTstRecovery1 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractid],[contractyear],[participantid],[regionid],[service],[weekno])
)
go
                        
create table mmsdm.BillingNmasTstRecvryRbf1 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractid],[contractyear],[regionid],[service],[weekno])
)
go
                        
create table mmsdm.BillingNmasTstRecvryTrk1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[recovery_contractyear] decimal(4,0) not null,
[recovery_weekno] decimal(3,0) not null,
[recovery_billrunno] decimal(3,0) not null,
    primary key ([billrunno],[contractyear],[recovery_billrunno],[recovery_contractyear],[recovery_weekno],[weekno])
)
go
                        
create table mmsdm.BillingSecdepositApplication1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[application_amount] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[participantid],[weekno])
)
go
                        
create table mmsdm.BillingSecdepInterestPay1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[security_deposit_id] varchar(20) not null,
[participantid] varchar(20) not null,
[interest_amount] decimal(18,8) null,
[interest_calc_type] varchar(20) null,
[interest_acct_id] varchar(20) null,
[interest_rate] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[participantid],[security_deposit_id],[weekno])
)
go
                        
create table mmsdm.BillingSecdepInterestRate1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[interest_acct_id] varchar(20) not null,
[effectivedate] datetime2 not null,
[interest_rate] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[effectivedate],[interest_acct_id],[weekno])
)
go
                        
create table mmsdm.BillingReservetraderpayment1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) null,
[contractid] varchar(20) not null,
[payment_id] decimal(3,0) not null,
[payment_type] varchar(40) null,
[payment_amount] decimal(18,8) null,
    primary key ([billrunno],[contractid],[contractyear],[payment_id],[weekno])
)
go
                        
create table mmsdm.BillingReservetraderrecovery1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[publication_id] varchar(40) not null,
[payment_id] decimal(3,0) not null,
[payment_amount] decimal(18,8) null,
[participantid] varchar(20) not null,
[regionid] varchar(20) not null,
[participant_demand] decimal(18,8) null,
[region_demand] decimal(18,8) null,
[eligibility_start_interval] datetime2 null,
[eligibility_end_interval] datetime2 null,
[recovery_amount] decimal(18,8) null,
    primary key ([billrunno],[contractyear],[participantid],[payment_id],[publication_id],[regionid],[weekno])
)
go
                        
create table mmsdm.BillingWhitehole5 (
file_log_id bigint not null,
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
    primary key ([billrunno],[contractyear],[interconnectorid],[participantid],[weekno])
)
go
                        
create table mmsdm.OperationalDemandActual2 (
file_log_id bigint not null,
    [interval_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[operational_demand] decimal(10,0) null,
[lastchanged] datetime2 null,
[operational_demand_adjustment] decimal(10,0) null,
[wdr_estimate] decimal(10,0) null,
    primary key ([interval_datetime],[regionid])
)
go
                        
create table mmsdm.OperationalDemandForecast1 (
file_log_id bigint not null,
    [interval_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[load_date] datetime2 null,
[operational_demand_poe10] decimal(15,2) null,
[operational_demand_poe50] decimal(15,2) null,
[operational_demand_poe90] decimal(15,2) null,
[lastchanged] datetime2 null,
    primary key ([interval_datetime],[regionid])
)
go
                        
create table mmsdm.DemandIntermittentClusterAvail1 (
file_log_id bigint not null,
    [tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[clusterid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[elements_unavailable] decimal(5,0) null,
[elements_available] decimal(5,0) null,
    primary key ([clusterid],[duid],[offerdatetime],[periodid],[tradingdate])
)
go
                        
create table mmsdm.DemandIntermittentClusterAvailDay1 (
file_log_id bigint not null,
    [tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[clusterid] varchar(20) not null,
    primary key ([clusterid],[duid],[offerdatetime],[tradingdate])
)
go
                        
create table mmsdm.DemandIntermittentDsPred1 (
file_log_id bigint not null,
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
    primary key ([duid],[forecast_priority],[interval_datetime],[offerdatetime],[origin],[run_datetime])
)
go
                        
create table mmsdm.DemandIntermittentDsRun1 (
file_log_id bigint not null,
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
    primary key ([duid],[forecast_priority],[offerdatetime],[origin],[run_datetime])
)
go
                        
create table mmsdm.ForecastIntermittentGen1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[duid] varchar(20) not null,
[start_interval_datetime] datetime2 not null,
[end_interval_datetime] datetime2 not null,
[versionno] decimal(10,0) null,
[lastchanged] datetime2 null,
    primary key ([duid],[run_datetime])
)
go
                        
create table mmsdm.ForecastIntermittentGenData1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[duid] varchar(20) not null,
[interval_datetime] datetime2 not null,
[powermean] decimal(9,3) null,
[powerpoe50] decimal(9,3) null,
[powerpoelow] decimal(9,3) null,
[powerpoehigh] decimal(9,3) null,
[lastchanged] datetime2 null,
    primary key ([duid],[interval_datetime],[run_datetime])
)
go
                        
create table mmsdm.DemandIntermittentGenLimit1 (
file_log_id bigint not null,
    [tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[periodid] decimal(3,0) not null,
[uppermwlimit] decimal(6,0) null,
    primary key ([duid],[offerdatetime],[periodid],[tradingdate])
)
go
                        
create table mmsdm.DemandIntermittentGenLimitDay1 (
file_log_id bigint not null,
    [tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[participantid] varchar(20) null,
[lastchanged] datetime2 null,
[authorisedbyuser] varchar(20) null,
[authorisedbyparticipantid] varchar(20) null,
    primary key ([duid],[offerdatetime],[tradingdate])
)
go
                        
create table mmsdm.DemandMtpasaIntermittentAvail1 (
file_log_id bigint not null,
    [tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[clusterid] varchar(20) not null,
[lastchanged] datetime2 null,
[elements_unavailable] decimal(5,0) null,
[elements_available] decimal(5,0) null,
    primary key ([clusterid],[duid],[offerdatetime],[tradingdate])
)
go
                        
create table mmsdm.DemandMtpasaIntermittentLimit1 (
file_log_id bigint not null,
    [tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[lastchanged] datetime2 null,
[uppermwlimit] decimal(6,0) null,
[authorisedbyuser] varchar(20) null,
[authorisedbyparticipantid] varchar(20) null,
    primary key ([duid],[offerdatetime],[tradingdate])
)
go
                        
create table mmsdm.DemandPeriod1 (
file_log_id bigint not null,
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
    primary key ([offerdate],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.DemandTrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[regionid] varchar(10) not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[filename] varchar(40) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(10) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[offerdate],[regionid],[versionno])
)
go
                        
create table mmsdm.RooftopActual2 (
file_log_id bigint not null,
    [interval_datetime] datetime2 not null,
[type] varchar(20) not null,
[regionid] varchar(20) not null,
[power] decimal(12,3) null,
[qi] decimal(2,1) null,
[lastchanged] datetime2 null,
    primary key ([interval_datetime],[regionid],[type])
)
go
                        
create table mmsdm.RooftopForecast1 (
file_log_id bigint not null,
    [version_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[interval_datetime] datetime2 not null,
[powermean] decimal(12,3) null,
[powerpoe50] decimal(12,3) null,
[powerpoelow] decimal(12,3) null,
[powerpoehigh] decimal(12,3) null,
[lastchanged] datetime2 null,
    primary key ([interval_datetime],[regionid],[version_datetime])
)
go
                        
create table mmsdm.PriceloadConstraintrelaxation1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[constraintid] varchar(20) not null,
[rhs] decimal(16,6) null,
[lastchanged] datetime2 null,
[versionno] decimal(3,0) not null,
    primary key ([constraintid],[runno],[settlementdate],[versionno])
)
go
                        
create table mmsdm.DispatchBlockedConstraints1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[constraintid] varchar(20) not null,
    primary key ([constraintid],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchCaseSolution2 (
file_log_id bigint not null,
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
    primary key ([runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchConstraint5 (
file_log_id bigint not null,
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
    primary key ([constraintid],[dispatchinterval],[intervention],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchInterconnectorres3 (
file_log_id bigint not null,
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
    primary key ([dispatchinterval],[interconnectorid],[intervention],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchUnitSolution2 (
file_log_id bigint not null,
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
[dispatchmodetime] decimal(4,0) null,
    primary key ([duid],[intervention],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchOffertrk1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[bidsettlementdate] datetime2 null,
[bidofferdate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([bidtype],[duid],[settlementdate])
)
go
                        
create table mmsdm.DispatchPrice4 (
file_log_id bigint not null,
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
    primary key ([dispatchinterval],[intervention],[regionid],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchRegionsum5 (
file_log_id bigint not null,
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
[wdr_initialmw] decimal(15,5) null,
[wdr_available] decimal(15,5) null,
[wdr_dispatched] decimal(15,5) null,
    primary key ([dispatchinterval],[intervention],[regionid],[runno],[settlementdate])
)
go
                        
create table mmsdm.PriceloadConstraintFcasOcd1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[constraintid] varchar(20) not null,
[versionno] decimal(3,0) not null,
[lastchanged] datetime2 null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
    primary key ([constraintid],[intervention],[runno],[settlementdate],[versionno])
)
go
                        
create table mmsdm.DispatchFcasReq2 (
file_log_id bigint not null,
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
    primary key ([bidtype],[genconid],[intervention],[regionid],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchInterconnection1 (
file_log_id bigint not null,
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
    primary key ([from_regionid],[intervention],[runno],[settlementdate],[to_regionid])
)
go
                        
create table mmsdm.DispatchLocalPrice1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[duid] varchar(20) not null,
[local_price_adjustment] decimal(10,2) null,
[locally_constrained] decimal(1,0) null,
    primary key ([duid],[settlementdate])
)
go
                        
create table mmsdm.DispatchMnspbidtrk1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[linkid] varchar(10) not null,
[offersettlementdate] datetime2 null,
[offereffectivedate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([linkid],[participantid],[runno],[settlementdate])
)
go
                        
create table mmsdm.DispatchMrScheduleTrk1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[regionid] varchar(10) not null,
[mr_date] datetime2 null,
[version_datetime] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([regionid],[settlementdate])
)
go
                        
create table mmsdm.PriceloadPriceRevision1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[regionid] varchar(10) not null,
[bidtype] varchar(10) not null,
[versionno] decimal(3,0) not null,
[rrp_new] decimal(15,5) null,
[rrp_old] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([bidtype],[intervention],[regionid],[runno],[settlementdate],[versionno])
)
go
                        
create table mmsdm.DispatchUnitConformance1 (
file_log_id bigint not null,
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
    primary key ([duid],[interval_datetime])
)
go
                        
create table mmsdm.DispatchUnitScada1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[duid] varchar(20) not null,
[scadavalue] decimal(16,6) null,
    primary key ([duid],[settlementdate])
)
go
                        
create table mmsdm.DispatchIntermittentForecastTrk1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[duid] varchar(20) not null,
[origin] varchar(20) null,
[forecast_priority] decimal(10,0) null,
[offerdatetime] datetime2 null,
    primary key ([duid],[settlementdate])
)
go
                        
create table mmsdm.DispatchNegativeResidue1 (
file_log_id bigint not null,
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
    primary key ([directional_interconnectorid],[nrm_datetime],[settlementdate])
)
go
                        
create table mmsdm.ApApevent1 (
file_log_id bigint not null,
    [apeventid] decimal(22,0) not null,
[effectivefrominterval] datetime2 null,
[effectivetointerval] datetime2 null,
[reason] varchar(2000) null,
[startauthorisedby] varchar(15) null,
[startauthoriseddate] datetime2 null,
[endauthorisedby] varchar(15) null,
[endauthoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([apeventid])
)
go
                        
create table mmsdm.ApApeventregion1 (
file_log_id bigint not null,
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
    primary key ([apeventid],[regionid])
)
go
                        
create table mmsdm.ForceMajeureIrfmamount1 (
file_log_id bigint not null,
    [irfmid] varchar(10) not null,
[effectivedate] datetime2 null,
[versionno] decimal(3,0) not null,
[periodid] decimal(4,0) not null,
[amount] decimal(15,5) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([irfmid],[periodid],[versionno])
)
go
                        
create table mmsdm.ForceMajeureIrfmevents1 (
file_log_id bigint not null,
    [irfmid] varchar(10) not null,
[startdate] datetime2 null,
[startperiod] decimal(3,0) null,
[enddate] datetime2 null,
[endperiod] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([irfmid])
)
go
                        
create table mmsdm.ForceMajeureMarketSuspendRegimeSum1 (
file_log_id bigint not null,
    [suspension_id] varchar(20) not null,
[regionid] varchar(20) not null,
[start_interval] datetime2 not null,
[end_interval] datetime2 null,
[pricing_regime] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([regionid],[start_interval],[suspension_id])
)
go
                        
create table mmsdm.ForceMajeureMarketSuspendRegionSum1 (
file_log_id bigint not null,
    [suspension_id] varchar(20) not null,
[regionid] varchar(20) not null,
[initial_interval] datetime2 null,
[end_region_interval] datetime2 null,
[end_suspension_interval] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([regionid],[suspension_id])
)
go
                        
create table mmsdm.ForceMajeureMarketSuspendSchedule1 (
file_log_id bigint not null,
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
    primary key ([day_type],[effectivedate],[periodid],[regionid])
)
go
                        
create table mmsdm.ForceMajeureMarketSuspendScheduleTrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[source_start_date] datetime2 null,
[source_end_date] datetime2 null,
[comments] varchar(1000) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate])
)
go
                        
create table mmsdm.ForceMajeureOverriderrp1 (
file_log_id bigint not null,
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
    primary key ([regionid],[startdate],[startperiod])
)
go
                        
create table mmsdm.ApRegionapc1 (
file_log_id bigint not null,
    [regionid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(10) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[regionid],[versionno])
)
go
                        
create table mmsdm.ApRegionapcintervals1 (
file_log_id bigint not null,
    [regionid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[apcvalue] decimal(16,6) null,
[lastchanged] datetime2 null,
[apctype] decimal(3,0) null,
[fcasapcvalue] decimal(16,6) null,
[apfvalue] decimal(16,6) null,
    primary key ([effectivedate],[periodid],[regionid],[versionno])
)
go
                        
create table mmsdm.GdInstructGdinstruct1 (
file_log_id bigint not null,
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
    primary key ([id])
)
go
                        
create table mmsdm.GdInstructInstructionsubtype1 (
file_log_id bigint not null,
    [instructiontypeid] varchar(10) not null,
[instructionsubtypeid] varchar(10) not null,
[description] varchar(64) null,
[lastchanged] datetime2 null,
    primary key ([instructionsubtypeid],[instructiontypeid])
)
go
                        
create table mmsdm.GdInstructInstructiontype1 (
file_log_id bigint not null,
    [instructiontypeid] varchar(10) not null,
[description] varchar(64) null,
[regionid] varchar(10) null,
[lastchanged] datetime2 null,
    primary key ([instructiontypeid])
)
go
                        
create table mmsdm.GenericConstraintEmsmaster1 (
file_log_id bigint not null,
    [spd_id] varchar(21) not null,
[spd_type] varchar(1) not null,
[description] varchar(255) null,
[grouping_id] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([spd_id],[spd_type])
)
go
                        
create table mmsdm.GencondataNull6 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[genconid],[versionno])
)
go
                        
create table mmsdm.GenconsetNull1 (
file_log_id bigint not null,
    [genconsetid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[genconeffdate] datetime2 null,
[genconversionno] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[genconid],[genconsetid],[versionno])
)
go
                        
create table mmsdm.GenericConstraintGenconsetinvoke2 (
file_log_id bigint not null,
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
    primary key ([invocation_id])
)
go
                        
create table mmsdm.GenconsettrkNull2 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[genconsetid],[versionno])
)
go
                        
create table mmsdm.GcrhsNull1 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[genconid],[scope],[termid],[versionno])
)
go
                        
create table mmsdm.GeqdescNull2 (
file_log_id bigint not null,
    [equationid] varchar(20) not null,
[description] varchar(256) null,
[lastchanged] datetime2 null,
[impact] varchar(64) null,
[source] varchar(128) null,
[limittype] varchar(64) null,
[reason] varchar(256) null,
[modifications] varchar(256) null,
[additionalnotes] varchar(256) null,
    primary key ([equationid])
)
go
                        
create table mmsdm.GeqrhsNull1 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[equationid],[termid],[versionno])
)
go
                        
create table mmsdm.SpdcpcNull2 (
file_log_id bigint not null,
    [connectionpointid] varchar(12) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[factor] decimal(16,6) null,
[lastchanged] datetime2 null,
[bidtype] varchar(12) not null,
    primary key ([bidtype],[connectionpointid],[effectivedate],[genconid],[versionno])
)
go
                        
create table mmsdm.SpdiccNull1 (
file_log_id bigint not null,
    [interconnectorid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[factor] decimal(16,6) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[genconid],[interconnectorid],[versionno])
)
go
                        
create table mmsdm.SpdrcNull2 (
file_log_id bigint not null,
    [regionid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[factor] decimal(16,6) null,
[lastchanged] datetime2 null,
[bidtype] varchar(10) not null,
    primary key ([bidtype],[effectivedate],[genconid],[regionid],[versionno])
)
go
                        
create table mmsdm.IrauctionConfigAuction1 (
file_log_id bigint not null,
    [auctionid] varchar(30) not null,
[auctiondate] datetime2 null,
[notifydate] datetime2 null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[description] varchar(100) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(30) null,
[lastchanged] datetime2 null,
    primary key ([auctionid])
)
go
                        
create table mmsdm.IrauctionConfigAuctionCalendar2 (
file_log_id bigint not null,
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
    primary key ([contractyear],[quarter])
)
go
                        
create table mmsdm.IrauctionConfigAuctionIcAllocations2 (
file_log_id bigint not null,
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
    primary key ([contractyear],[fromregionid],[interconnectorid],[quarter],[versionno])
)
go
                        
create table mmsdm.IrauctionConfigAuctionRevenueEstimate1 (
file_log_id bigint not null,
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
    primary key ([contractyear],[fromregionid],[interconnectorid],[monthno],[quarter],[valuationid],[versionno])
)
go
                        
create table mmsdm.IrauctionConfigAuctionRevenueTrack1 (
file_log_id bigint not null,
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
    primary key ([contractyear],[quarter],[valuationid],[versionno])
)
go
                        
create table mmsdm.IrauctionConfigAuctionRpEstimate1 (
file_log_id bigint not null,
    [contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[valuationid] varchar(15) not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[rpestimate] decimal(17,5) null,
[lastchanged] datetime2 null,
    primary key ([contractyear],[fromregionid],[interconnectorid],[quarter],[valuationid],[versionno])
)
go
                        
create table mmsdm.IrauctionConfigAuctionTranche1 (
file_log_id bigint not null,
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
    primary key ([contractyear],[quarter],[tranche],[versionno])
)
go
                        
create table mmsdm.SettlementConfigResiduecontractpayments1 (
file_log_id bigint not null,
    [contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[lastchanged] datetime2 null,
    primary key ([contractid],[participantid])
)
go
                        
create table mmsdm.IrauctionBidsFileTrk1 (
file_log_id bigint not null,
    [contractid] varchar(30) null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[filename] varchar(40) null,
[ackfilename] varchar(40) null,
[status] varchar(10) null,
[lastchanged] datetime2 null,
[auctionid] varchar(30) not null,
    primary key ([auctionid],[loaddate],[participantid])
)
go
                        
create table mmsdm.IrauctionResidueBidTrk1 (
file_log_id bigint not null,
    [contractid] varchar(30) null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[bidloaddate] datetime2 null,
[lastchanged] datetime2 null,
[auctionid] varchar(30) not null,
    primary key ([auctionid],[participantid],[versionno])
)
go
                        
create table mmsdm.IrauctionResidueContracts1 (
file_log_id bigint not null,
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
    primary key ([contractyear],[quarter],[tranche])
)
go
                        
create table mmsdm.IrauctionResidueConData2 (
file_log_id bigint not null,
    [contractid] varchar(30) not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[unitspurchased] decimal(17,5) null,
[linkpayment] decimal(17,5) null,
[lastchanged] datetime2 null,
[secondary_units_sold] decimal(18,8) null,
    primary key ([contractid],[fromregionid],[interconnectorid],[participantid],[versionno])
)
go
                        
create table mmsdm.IrauctionResidueConEstimatesTrk1 (
file_log_id bigint not null,
    [contractid] varchar(30) not null,
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[valuationid] varchar(15) not null,
[versionno] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([contractid],[contractyear],[quarter],[valuationid])
)
go
                        
create table mmsdm.IrauctionResidueConFunds1 (
file_log_id bigint not null,
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
    primary key ([contractid],[fromregionid],[interconnectorid])
)
go
                        
create table mmsdm.IrauctionBidsFundsBid1 (
file_log_id bigint not null,
    [contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[optionid] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[units] decimal(5,0) null,
[lastchanged] datetime2 null,
    primary key ([contractid],[fromregionid],[interconnectorid],[loaddate],[optionid],[participantid])
)
go
                        
create table mmsdm.IrauctionBidsPriceBid1 (
file_log_id bigint not null,
    [contractid] varchar(30) null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[optionid] decimal(3,0) not null,
[bidprice] decimal(17,5) null,
[lastchanged] datetime2 null,
[auctionid] varchar(30) not null,
    primary key ([auctionid],[loaddate],[optionid],[participantid])
)
go
                        
create table mmsdm.IrauctionResiduePriceFundsBid1 (
file_log_id bigint not null,
    [contractid] varchar(30) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[units] decimal(5,0) null,
[bidprice] decimal(17,5) null,
[linkedbidflag] decimal(6,0) not null,
[auctionid] varchar(30) not null,
[lastchanged] datetime2 null,
    primary key ([auctionid],[contractid],[fromregionid],[interconnectorid],[linkedbidflag])
)
go
                        
create table mmsdm.IrauctionResiduePublicData1 (
file_log_id bigint not null,
    [contractid] varchar(30) not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[unitsoffered] decimal(5,0) null,
[unitssold] decimal(16,6) null,
[clearingprice] decimal(17,5) null,
[reserveprice] decimal(17,5) null,
[lastchanged] datetime2 null,
    primary key ([contractid],[fromregionid],[interconnectorid],[versionno])
)
go
                        
create table mmsdm.IrauctionResidueTrk1 (
file_log_id bigint not null,
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
    primary key ([auctionid],[versionno])
)
go
                        
create table mmsdm.IrauctionSraCashSecurity1 (
file_log_id bigint not null,
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
    primary key ([cash_security_id])
)
go
                        
create table mmsdm.IrauctionSraFinancialAucpayDetail1 (
file_log_id bigint not null,
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
    primary key ([contractid],[fromregionid],[interconnectorid],[participantid],[sra_quarter],[sra_runno],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraFinancialAucpaySum1 (
file_log_id bigint not null,
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
    primary key ([participantid],[sra_quarter],[sra_runno],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraFinancialAucMardetail1 (
file_log_id bigint not null,
    [sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[cash_security_id] varchar(36) not null,
[returned_amount] decimal(18,8) null,
[returned_interest] decimal(18,8) null,
    primary key ([cash_security_id],[participantid],[sra_quarter],[sra_runno],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraFinancialAucMargin1 (
file_log_id bigint not null,
    [sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[total_cash_security] decimal(18,8) null,
[required_margin] decimal(18,8) null,
[returned_margin] decimal(18,8) null,
[returned_margin_interest] decimal(18,8) null,
    primary key ([participantid],[sra_quarter],[sra_runno],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraFinancialAucReceipts1 (
file_log_id bigint not null,
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
    primary key ([contractid],[fromregionid],[interconnectorid],[participantid],[sra_quarter],[sra_runno],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraFinancialRuntrk1 (
file_log_id bigint not null,
    [sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[runtype] varchar(20) null,
[rundate] datetime2 null,
[posteddate] datetime2 null,
[interest_versionno] decimal(3,0) null,
[makeup_versionno] decimal(3,0) null,
[lastchanged] datetime2 null,
    primary key ([sra_quarter],[sra_runno],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraOfferProduct1 (
file_log_id bigint not null,
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
    primary key ([auctionid],[loaddate],[optionid],[participantid])
)
go
                        
create table mmsdm.IrauctionSraOfferProfile1 (
file_log_id bigint not null,
    [auctionid] varchar(30) not null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[filename] varchar(40) null,
[ackfilename] varchar(40) null,
[transactionid] varchar(100) null,
[lastchanged] datetime2 null,
    primary key ([auctionid],[loaddate],[participantid])
)
go
                        
create table mmsdm.IrauctionSraPrudentialCashSecurity1 (
file_log_id bigint not null,
    [prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,
[participantid] varchar(10) not null,
[cash_security_id] varchar(36) not null,
[cash_security_amount] decimal(18,8) null,
    primary key ([cash_security_id],[participantid],[prudential_date],[prudential_runno])
)
go
                        
create table mmsdm.IrauctionSraPrudentialCompPosition1 (
file_log_id bigint not null,
    [prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,
[participantid] varchar(10) not null,
[trading_limit] decimal(18,8) null,
[prudential_exposure_amount] decimal(18,8) null,
[trading_margin] decimal(18,8) null,
    primary key ([participantid],[prudential_date],[prudential_runno])
)
go
                        
create table mmsdm.IrauctionSraPrudentialExposure1 (
file_log_id bigint not null,
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
    primary key ([fromregionid],[interconnectorid],[participantid],[prudential_date],[prudential_runno],[sra_quarter],[sra_year])
)
go
                        
create table mmsdm.IrauctionSraPrudentialRun1 (
file_log_id bigint not null,
    [prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,
    primary key ([prudential_date],[prudential_runno])
)
go
                        
create table mmsdm.IrauctionValuationid1 (
file_log_id bigint not null,
    [valuationid] varchar(15) not null,
[description] varchar(80) null,
[lastchanged] datetime2 null,
    primary key ([valuationid])
)
go
                        
create table mmsdm.MarketConfigBidtypes1 (
file_log_id bigint not null,
    [bidtype] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[description] varchar(64) null,
[numberofbands] decimal(3,0) null,
[numdaysaheadpricelocked] decimal(2,0) null,
[validationrule] varchar(10) null,
[lastchanged] datetime2 null,
[spdalias] varchar(10) null,
    primary key ([bidtype],[effectivedate],[versionno])
)
go
                        
create table mmsdm.MarketConfigBidtypestrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[versionno])
)
go
                        
create table mmsdm.MarketConfigInterconnector1 (
file_log_id bigint not null,
    [interconnectorid] varchar(10) not null,
[regionfrom] varchar(10) null,
[rsoid] varchar(10) null,
[regionto] varchar(10) null,
[description] varchar(64) null,
[lastchanged] datetime2 null,
    primary key ([interconnectorid])
)
go
                        
create table mmsdm.MarketConfigInterconnectoralloc1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(5,0) not null,
[interconnectorid] varchar(10) not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) not null,
[allocation] decimal(12,5) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[interconnectorid],[participantid],[regionid],[versionno])
)
go
                        
create table mmsdm.MarketConfigInterconnectorconstraint1 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[interconnectorid],[versionno])
)
go
                        
create table mmsdm.MarketConfigIntraregionalloc1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(5,0) not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) not null,
[allocation] decimal(12,5) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid],[regionid],[versionno])
)
go
                        
create table mmsdm.MarketConfigLossfactormodel1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[regionid] varchar(10) not null,
[demandcoefficient] decimal(27,17) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[interconnectorid],[regionid],[versionno])
)
go
                        
create table mmsdm.MarketConfigLossmodel1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[periodid] varchar(20) null,
[losssegment] decimal(6,0) not null,
[mwbreakpoint] decimal(6,0) null,
[lossfactor] decimal(16,6) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[interconnectorid],[losssegment],[versionno])
)
go
                        
create table mmsdm.MarketConfigMarketPriceThresholds1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(4,0) not null,
[voll] decimal(15,5) null,
[marketpricefloor] decimal(15,5) null,
[administered_price_threshold] decimal(15,5) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[versionno])
)
go
                        
create table mmsdm.MarketConfigRegion1 (
file_log_id bigint not null,
    [regionid] varchar(10) not null,
[description] varchar(64) null,
[regionstatus] varchar(8) null,
[lastchanged] datetime2 null,
    primary key ([regionid])
)
go
                        
create table mmsdm.MarketConfigRegionstandingdata1 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[regionid],[versionno])
)
go
                        
create table mmsdm.MarketConfigTransmissionlossfactor2 (
file_log_id bigint not null,
    [transmissionlossfactor] decimal(15,5) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(22,0) not null,
[connectionpointid] varchar(10) not null,
[regionid] varchar(10) null,
[lastchanged] datetime2 null,
[secondary_tlf] decimal(18,8) null,
    primary key ([connectionpointid],[effectivedate],[versionno])
)
go
                        
create table mmsdm.MarketNoticeMarketnoticedata1 (
file_log_id bigint not null,
    [noticeid] decimal(10,0) not null,
[effectivedate] datetime2 null,
[typeid] varchar(25) null,
[noticetype] varchar(25) null,
[lastchanged] datetime2 null,
[reason] varchar(2000) null,
[externalreference] varchar(255) null,
    primary key ([noticeid])
)
go
                        
create table mmsdm.MarketNoticeMarketnoticetype1 (
file_log_id bigint not null,
    [typeid] varchar(25) not null,
[description] varchar(64) null,
[raisedby] varchar(10) null,
[lastchanged] datetime2 null,
    primary key ([typeid])
)
go
                        
create table mmsdm.MarketNoticeParticipantnoticetrk1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[noticeid] decimal(10,0) not null,
[lastchanged] datetime2 null,
    primary key ([noticeid],[participantid])
)
go
                        
create table mmsdm.MccCasesolution1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
    primary key ([run_datetime])
)
go
                        
create table mmsdm.MccConstraintsolution1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
    primary key ([constraintid],[run_datetime])
)
go
                        
create table mmsdm.MeterdataAggregateReads1 (
file_log_id bigint not null,
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
    primary key ([case_id],[connectionpointid],[frmp],[lr],[meter_type],[periodid],[settlementdate])
)
go
                        
create table mmsdm.MeterdataIndividualReads1 (
file_log_id bigint not null,
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
    primary key ([case_id],[meter_id],[meter_id_suffix],[periodid],[settlementdate])
)
go
                        
create table mmsdm.MeterdataInterconnector1 (
file_log_id bigint not null,
    [case_id] decimal(15,0) not null,
[settlementdate] datetime2 not null,
[interconnectorid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[importvalue] decimal(18,8) null,
[exportvalue] decimal(18,8) null,
[lastchanged] datetime2 null,
    primary key ([case_id],[interconnectorid],[periodid],[settlementdate])
)
go
                        
create table mmsdm.MtpasaCaseresult1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[plexos_version] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([run_datetime],[run_no])
)
go
                        
create table mmsdm.MtpasaConstraintresult1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[demand_poe_type] varchar(20) not null,
[day] datetime2 not null,
[constraintid] varchar(20) not null,
[effectivedate] datetime2 null,
[versionno] decimal(3,0) null,
[periodid] decimal(3,0) null,
[probabilityofbinding] decimal(8,5) null,
[probabilityofviolation] decimal(8,5) null,
[constraintviolation90] decimal(12,2) null,
[constraintviolation50] decimal(12,2) null,
[constraintviolation10] decimal(12,2) null,
[lastchanged] datetime2 null,
    primary key ([constraintid],[day],[demand_poe_type],[run_datetime],[run_no],[runtype])
)
go
                        
create table mmsdm.MtpasaConstraintsummary1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[demand_poe_type] varchar(20) not null,
[day] datetime2 not null,
[constraintid] varchar(20) not null,
[effectivedate] datetime2 null,
[versionno] decimal(3,0) null,
[aggregation_period] varchar(20) not null,
[constrainthoursbinding] decimal(12,2) null,
[lastchanged] datetime2 null,
    primary key ([aggregation_period],[constraintid],[day],[demand_poe_type],[run_datetime],[run_no],[runtype])
)
go
                        
create table mmsdm.MtpasaDuidavailability1 (
file_log_id bigint not null,
    [publish_datetime] datetime2 not null,
[day] datetime2 not null,
[regionid] varchar(20) not null,
[duid] varchar(20) not null,
[pasaavailability] decimal(12,0) null,
[latest_offer_datetime] datetime2 null,
[lastchanged] datetime2 null,
[carryoverstatus] decimal(1,0) null,
    primary key ([day],[duid],[publish_datetime],[regionid])
)
go
                        
create table mmsdm.MtpasaInterconnectorresult1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[demand_poe_type] varchar(20) not null,
[day] datetime2 not null,
[interconnectorid] varchar(20) not null,
[periodid] decimal(3,0) null,
[flow90] decimal(12,2) null,
[flow50] decimal(12,2) null,
[flow10] decimal(12,2) null,
[probabilityofbindingexport] decimal(8,5) null,
[probabilityofbindingimport] decimal(8,5) null,
[calculatedexportlimit] decimal(12,2) null,
[calculatedimportlimit] decimal(12,2) null,
[lastchanged] datetime2 null,
    primary key ([day],[demand_poe_type],[interconnectorid],[run_datetime],[run_no],[runtype])
)
go
                        
create table mmsdm.MtpasaLolpresult1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[day] datetime2 not null,
[regionid] varchar(20) not null,
[worst_interval_periodid] decimal(3,0) null,
[worst_interval_demand] decimal(12,2) null,
[worst_interval_intgen] decimal(12,2) null,
[worst_interval_dsp] decimal(12,2) null,
[lossofloadprobability] decimal(8,5) null,
[lossofloadmagnitude] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([day],[regionid],[run_datetime],[run_no],[runtype])
)
go
                        
create table mmsdm.MtpasaRegionavailability3 (
file_log_id bigint not null,
    [publish_datetime] datetime2 not null,
[day] datetime2 not null,
[regionid] varchar(20) not null,
[pasaavailability_scheduled] decimal(12,0) null,
[latest_offer_datetime] datetime2 null,
[energyunconstrainedcapacity] decimal(12,0) null,
[energyconstrainedcapacity] decimal(12,0) null,
[nonscheduledgeneration] decimal(12,2) null,
[demand10] decimal(12,2) null,
[demand50] decimal(12,2) null,
[energyreqdemand10] decimal(12,2) null,
[energyreqdemand50] decimal(12,2) null,
[lastchanged] datetime2 null,
[demand10min] decimal(12,2) null,
[demand10max] decimal(12,2) null,
[demand50min] decimal(12,2) null,
[demand50max] decimal(12,2) null,
[carryovercapacity] decimal(12,0) null,
    primary key ([day],[publish_datetime],[regionid])
)
go
                        
create table mmsdm.MtpasaRegionavailtrk1 (
file_log_id bigint not null,
    [publish_datetime] datetime2 not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[latest_offer_datetime] datetime2 null,
    primary key ([publish_datetime])
)
go
                        
create table mmsdm.MtpasaRegioniteration1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[demand_poe_type] varchar(20) not null,
[aggregation_period] varchar(20) not null,
[period_ending] datetime2 not null,
[regionid] varchar(20) not null,
[use_iteration_id] decimal(5,0) not null,
[use_iteration_event_number] decimal(12,2) null,
[use_iteration_event_average] decimal(12,2) null,
[lastchanged] datetime2 null,
    primary key ([aggregation_period],[demand_poe_type],[period_ending],[regionid],[run_datetime],[run_no],[runtype],[use_iteration_id])
)
go
                        
create table mmsdm.MtpasaRegionresult2 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[demand_poe_type] varchar(20) not null,
[day] datetime2 not null,
[regionid] varchar(20) not null,
[periodid] decimal(3,0) null,
[demand] decimal(12,2) null,
[aggregateinstalledcapacity] decimal(12,2) null,
[numberofiterations] decimal(12,2) null,
[use_numberofiterations] decimal(12,2) null,
[use_max] decimal(12,2) null,
[use_upperquartile] decimal(12,2) null,
[use_median] decimal(12,2) null,
[use_lowerquartile] decimal(12,2) null,
[use_min] decimal(12,2) null,
[use_average] decimal(12,2) null,
[use_event_average] decimal(12,2) null,
[totalscheduledgen90] decimal(12,2) null,
[totalscheduledgen50] decimal(12,2) null,
[totalscheduledgen10] decimal(12,2) null,
[totalintermittentgen90] decimal(12,2) null,
[totalintermittentgen50] decimal(12,2) null,
[totalintermittentgen10] decimal(12,2) null,
[demandsideparticipation90] decimal(12,2) null,
[demandsideparticipation50] decimal(12,2) null,
[demandsideparticipation10] decimal(12,2) null,
[lastchanged] datetime2 null,
[totalsemischedulegen90] decimal(12,2) null,
[totalsemischedulegen50] decimal(12,2) null,
[totalsemischedulegen10] decimal(12,2) null,
[totalavailablegenmin] decimal(12,2) null,
[totalavailablegen10] decimal(12,2) null,
[totalavailablegen50] decimal(12,2) null,
[totalavailablegen90] decimal(12,2) null,
[totalavailablegenmax] decimal(12,2) null,
    primary key ([day],[demand_poe_type],[regionid],[run_datetime],[run_no],[runtype])
)
go
                        
create table mmsdm.MtpasaRegionsummary1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[run_no] decimal(4,0) not null,
[runtype] varchar(20) not null,
[demand_poe_type] varchar(20) not null,
[aggregation_period] varchar(20) not null,
[period_ending] datetime2 not null,
[regionid] varchar(20) not null,
[nativedemand] decimal(12,2) null,
[use_percentile10] decimal(12,2) null,
[use_percentile20] decimal(12,2) null,
[use_percentile30] decimal(12,2) null,
[use_percentile40] decimal(12,2) null,
[use_percentile50] decimal(12,2) null,
[use_percentile60] decimal(12,2) null,
[use_percentile70] decimal(12,2) null,
[use_percentile80] decimal(12,2) null,
[use_percentile90] decimal(12,2) null,
[use_percentile100] decimal(12,2) null,
[use_average] decimal(12,2) null,
[numberofiterations] decimal(12,2) null,
[use_numberofiterations] decimal(12,2) null,
[use_event_max] decimal(12,2) null,
[use_event_upperquartile] decimal(12,2) null,
[use_event_median] decimal(12,2) null,
[use_event_lowerquartile] decimal(12,2) null,
[use_event_min] decimal(12,2) null,
[weight] decimal(16,6) null,
[use_weighted_avg] decimal(16,6) null,
[lrc] decimal(12,2) null,
[lastchanged] datetime2 null,
    primary key ([aggregation_period],[demand_poe_type],[period_ending],[regionid],[run_datetime],[run_no],[runtype])
)
go
                        
create table mmsdm.NetworkEquipmentdetail1 (
file_log_id bigint not null,
    [substationid] varchar(30) not null,
[equipmenttype] varchar(10) not null,
[equipmentid] varchar(30) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[voltage] varchar(20) null,
[description] varchar(100) null,
[lastchanged] datetime2 null,
[elementid] decimal(15,0) not null,
    primary key ([elementid],[equipmentid],[equipmenttype],[substationid],[validfrom])
)
go
                        
create table mmsdm.NetworkOutageconstraintset1 (
file_log_id bigint not null,
    [outageid] decimal(15,0) not null,
[genconsetid] varchar(50) not null,
[startinterval] datetime2 null,
[endinterval] datetime2 null,
    primary key ([genconsetid],[outageid])
)
go
                        
create table mmsdm.NetworkOutagedetail3 (
file_log_id bigint not null,
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
[elementid] decimal(15,0) not null,
    primary key ([elementid],[equipmentid],[equipmenttype],[outageid],[starttime],[substationid])
)
go
                        
create table mmsdm.NetworkOutagestatuscode1 (
file_log_id bigint not null,
    [outagestatuscode] varchar(10) not null,
[description] varchar(100) null,
[lastchanged] datetime2 null,
    primary key ([outagestatuscode])
)
go
                        
create table mmsdm.NetworkRating1 (
file_log_id bigint not null,
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
    primary key ([spd_id],[validfrom])
)
go
                        
create table mmsdm.NetworkRealtimerating1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[spd_id] varchar(21) not null,
[ratingvalue] decimal(16,6) not null,
    primary key ([settlementdate],[spd_id])
)
go
                        
create table mmsdm.NetworkStaticrating1 (
file_log_id bigint not null,
    [substationid] varchar(30) not null,
[equipmenttype] varchar(10) not null,
[equipmentid] varchar(30) not null,
[ratinglevel] varchar(10) not null,
[applicationid] varchar(20) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[ratingvalue] decimal(16,6) null,
[lastchanged] datetime2 null,
    primary key ([applicationid],[equipmentid],[equipmenttype],[ratinglevel],[substationid],[validfrom])
)
go
                        
create table mmsdm.NetworkSubstationdetail1 (
file_log_id bigint not null,
    [substationid] varchar(30) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[description] varchar(100) null,
[regionid] varchar(10) null,
[ownerid] varchar(30) null,
[lastchanged] datetime2 null,
    primary key ([substationid],[validfrom])
)
go
                        
create table mmsdm.P5minBlockedConstraints1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
    primary key ([constraintid],[run_datetime])
)
go
                        
create table mmsdm.P5minCasesolution2 (
file_log_id bigint not null,
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
    primary key ([run_datetime])
)
go
                        
create table mmsdm.P5minConstraintsolution6 (
file_log_id bigint not null,
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
    primary key ([constraintid],[interval_datetime],[run_datetime])
)
go
                        
create table mmsdm.P5minInterconnectorsoln4 (
file_log_id bigint not null,
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
    primary key ([interconnectorid],[interval_datetime],[run_datetime])
)
go
                        
create table mmsdm.P5minIntersensitivities1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[interconnectorid] varchar(20) not null,
[interval_datetime] datetime2 not null,
[intervention] decimal(1,0) not null,
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
    primary key ([interconnectorid],[interval_datetime],[run_datetime])
)
go
                        
create table mmsdm.P5minLocalPrice1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[duid] varchar(20) not null,
[local_price_adjustment] decimal(10,2) null,
[locally_constrained] decimal(1,0) null,
    primary key ([duid],[interval_datetime],[run_datetime])
)
go
                        
create table mmsdm.P5minPricesensitivities1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[interval_datetime] datetime2 not null,
[intervention] decimal(1,0) not null,
[intervention_active] decimal(1,0) null,
[rrp1] decimal(15,5) null,
[rrp2] decimal(15,5) null,
[rrp3] decimal(15,5) null,
[rrp4] decimal(15,5) null,
[rrp5] decimal(15,5) null,
[rrp6] decimal(15,5) null,
[rrp7] decimal(15,5) null,
[rrp8] decimal(15,5) null,
[rrp9] decimal(15,5) null,
[rrp10] decimal(15,5) null,
[rrp11] decimal(15,5) null,
[rrp12] decimal(15,5) null,
[rrp13] decimal(15,5) null,
[rrp14] decimal(15,5) null,
[rrp15] decimal(15,5) null,
[rrp16] decimal(15,5) null,
[rrp17] decimal(15,5) null,
[rrp18] decimal(15,5) null,
[rrp19] decimal(15,5) null,
[rrp20] decimal(15,5) null,
[rrp21] decimal(15,5) null,
[rrp22] decimal(15,5) null,
[rrp23] decimal(15,5) null,
[rrp24] decimal(15,5) null,
[rrp25] decimal(15,5) null,
[rrp26] decimal(15,5) null,
[rrp27] decimal(15,5) null,
[rrp28] decimal(15,5) null,
[rrp29] decimal(15,5) null,
[rrp30] decimal(15,5) null,
[rrp31] decimal(15,5) null,
[rrp32] decimal(15,5) null,
[rrp33] decimal(15,5) null,
[rrp34] decimal(15,5) null,
[rrp35] decimal(15,5) null,
[rrp36] decimal(15,5) null,
[rrp37] decimal(15,5) null,
[rrp38] decimal(15,5) null,
[rrp39] decimal(15,5) null,
[rrp40] decimal(15,5) null,
[rrp41] decimal(15,5) null,
[rrp42] decimal(15,5) null,
[rrp43] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([interval_datetime],[regionid],[run_datetime])
)
go
                        
create table mmsdm.P5minRegionsolution6 (
file_log_id bigint not null,
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
[wdr_initialmw] decimal(15,5) null,
[wdr_available] decimal(15,5) null,
[wdr_dispatched] decimal(15,5) null,
    primary key ([interval_datetime],[regionid],[run_datetime])
)
go
                        
create table mmsdm.P5minScenariodemand1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[scenario] decimal(2,0) not null,
[regionid] varchar(20) not null,
[deltamw] decimal(4,0) null,
    primary key ([effectivedate],[regionid],[scenario],[version_datetime])
)
go
                        
create table mmsdm.P5minScenariodemandtrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[version_datetime])
)
go
                        
create table mmsdm.P5minUnitsolution3 (
file_log_id bigint not null,
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
[dispatchmodetime] decimal(4,0) null,
    primary key ([duid],[interval_datetime],[run_datetime])
)
go
                        
create table mmsdm.ParticipantRegistrationBidduiddetails1 (
file_log_id bigint not null,
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
    primary key ([bidtype],[duid],[effectivedate],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationBidduiddetailstrk1 (
file_log_id bigint not null,
    [duid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,
    primary key ([duid],[effectivedate],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationDispatchableunit1 (
file_log_id bigint not null,
    [duid] varchar(10) not null,
[duname] varchar(20) null,
[unittype] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([duid])
)
go
                        
create table mmsdm.ParticipantRegistrationDualloc1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[duid] varchar(10) not null,
[gensetid] varchar(20) not null,
[lastchanged] datetime2 null,
    primary key ([duid],[effectivedate],[gensetid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationDudetail3 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[duid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[connectionpointid] varchar(10) null,
[voltlevel] varchar(10) null,
[registeredcapacity] decimal(6,0) null,
[agccapability] varchar(1) null,
[dispatchtype] varchar(20) null,
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
[dispatchsubtype] varchar(20) null,
    primary key ([duid],[effectivedate],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationDudetailsummary4 (
file_log_id bigint not null,
    [duid] varchar(10) not null,
[start_date] datetime2 not null,
[end_date] datetime2 not null,
[dispatchtype] varchar(20) null,
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
[dispatchsubtype] varchar(20) null,
    primary key ([duid],[start_date])
)
go
                        
create table mmsdm.ParticipantRegistrationGenmeter1 (
file_log_id bigint not null,
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
    primary key ([applydate],[meterid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationGenunits2 (
file_log_id bigint not null,
    [gensetid] varchar(20) not null,
[stationid] varchar(10) null,
[setlossfactor] decimal(16,6) null,
[cdindicator] varchar(10) null,
[agcflag] varchar(2) null,
[spinningflag] varchar(2) null,
[voltlevel] decimal(6,0) null,
[registeredcapacity] decimal(6,0) null,
[dispatchtype] varchar(20) null,
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
    primary key ([gensetid])
)
go
                        
create table mmsdm.ParticipantRegistrationGenunitsUnit1 (
file_log_id bigint not null,
    [gensetid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(6,0) not null,
[unit_grouping_label] varchar(20) not null,
[unit_count] decimal(10,0) null,
[unit_size] decimal(8,3) null,
[unit_max_size] decimal(8,3) null,
[aggregation_flag] decimal(1,0) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[gensetid],[unit_grouping_label],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationMnspInterconnector2 (
file_log_id bigint not null,
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
    primary key ([effectivedate],[linkid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationMnspParticipant1 (
file_log_id bigint not null,
    [interconnectorid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[interconnectorid],[participantid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationParticipant1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[participantclassid] varchar(20) null,
[name] varchar(80) null,
[description] varchar(64) null,
[acn] varchar(9) null,
[primarybusiness] varchar(40) null,
[lastchanged] datetime2 null,
    primary key ([participantid])
)
go
                        
create table mmsdm.ParticipantRegistrationParticipantaccount1 (
file_log_id bigint not null,
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
    primary key ([participantid])
)
go
                        
create table mmsdm.ParticipantRegistrationParticipantcategory1 (
file_log_id bigint not null,
    [participantcategoryid] varchar(10) not null,
[description] varchar(64) null,
[lastchanged] datetime2 null,
    primary key ([participantcategoryid])
)
go
                        
create table mmsdm.ParticipantRegistrationParticipantcategoryalloc1 (
file_log_id bigint not null,
    [participantcategoryid] varchar(10) not null,
[participantid] varchar(10) not null,
[lastchanged] datetime2 null,
    primary key ([participantcategoryid],[participantid])
)
go
                        
create table mmsdm.ParticipantRegistrationParticipantclass1 (
file_log_id bigint not null,
    [participantclassid] varchar(20) not null,
[description] varchar(64) null,
[lastchanged] datetime2 null,
    primary key ([participantclassid])
)
go
                        
create table mmsdm.ParticipantRegistrationParticipantcreditdetail1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[participantid] varchar(10) not null,
[creditlimit] decimal(10,0) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid])
)
go
                        
create table mmsdm.ParticipantRegistrationStadualloc1 (
file_log_id bigint not null,
    [duid] varchar(10) not null,
[effectivedate] datetime2 not null,
[stationid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[lastchanged] datetime2 null,
    primary key ([duid],[effectivedate],[stationid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationStation1 (
file_log_id bigint not null,
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
    primary key ([stationid])
)
go
                        
create table mmsdm.ParticipantRegistrationStationoperatingstatus1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[stationid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[status] varchar(20) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[stationid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationStationowner1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[participantid] varchar(10) not null,
[stationid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid],[stationid],[versionno])
)
go
                        
create table mmsdm.ParticipantRegistrationStationownertrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[participantid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid],[versionno])
)
go
                        
create table mmsdm.PdpasaCasesolution3 (
file_log_id bigint not null,
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
    primary key ([run_datetime])
)
go
                        
create table mmsdm.PdpasaConstraintsolution1 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[capacityrhs] decimal(12,2) null,
[capacitymarginalvalue] decimal(12,2) null,
[capacityviolationdegree] decimal(12,2) null,
[lastchanged] datetime2 null,
[runtype] varchar(20) not null,
[studyregionid] varchar(20) not null,
    primary key ([constraintid],[interval_datetime],[run_datetime],[runtype],[studyregionid])
)
go
                        
create table mmsdm.PdpasaInterconnectorsoln1 (
file_log_id bigint not null,
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
[studyregionid] varchar(20) not null,
    primary key ([interconnectorid],[interval_datetime],[run_datetime],[runtype],[studyregionid])
)
go
                        
create table mmsdm.PdpasaRegionsolution6 (
file_log_id bigint not null,
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
[wdr_available] decimal(12,2) null,
[wdr_pasaavailable] decimal(12,2) null,
[wdr_capacity] decimal(12,2) null,
    primary key ([interval_datetime],[regionid],[run_datetime],[runtype])
)
go
                        
create table mmsdm.PredispatchBlockedConstraints1 (
file_log_id bigint not null,
    [predispatchseqno] varchar(20) not null,
[constraintid] varchar(20) not null,
    primary key ([constraintid],[predispatchseqno])
)
go
                        
create table mmsdm.PredispatchCaseSolution1 (
file_log_id bigint not null,
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
    primary key ([predispatchseqno],[runno])
)
go
                        
create table mmsdm.PredispatchConstraintSolution5 (
file_log_id bigint not null,
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
    primary key ([constraintid],[datetime])
)
go
                        
create table mmsdm.PredispatchInterconnectorSoln3 (
file_log_id bigint not null,
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
    primary key ([datetime],[interconnectorid])
)
go
                        
create table mmsdm.PredispatchInterconnectrSens1 (
file_log_id bigint not null,
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
    primary key ([datetime],[interconnectorid])
)
go
                        
create table mmsdm.PredispatchUnitSolution2 (
file_log_id bigint not null,
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
    primary key ([datetime],[duid])
)
go
                        
create table mmsdm.PredispatchOffertrk1 (
file_log_id bigint not null,
    [predispatchseqno] varchar(20) not null,
[duid] varchar(10) not null,
[bidtype] varchar(20) not null,
[periodid] varchar(20) not null,
[bidsettlementdate] datetime2 null,
[bidofferdate] datetime2 null,
[datetime] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([bidtype],[duid],[periodid],[predispatchseqno])
)
go
                        
create table mmsdm.PredispatchRegionPrices1 (
file_log_id bigint not null,
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
    primary key ([datetime],[regionid])
)
go
                        
create table mmsdm.PredispatchPricesensitivities1 (
file_log_id bigint not null,
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
    primary key ([datetime],[regionid])
)
go
                        
create table mmsdm.PredispatchRegionSolution5 (
file_log_id bigint not null,
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
[wdr_initialmw] decimal(15,5) null,
[wdr_available] decimal(15,5) null,
[wdr_dispatched] decimal(15,5) null,
    primary key ([datetime],[regionid])
)
go
                        
create table mmsdm.PredispatchScenarioDemand1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[scenario] decimal(2,0) not null,
[regionid] varchar(20) not null,
[deltamw] decimal(4,0) null,
    primary key ([effectivedate],[regionid],[scenario],[versionno])
)
go
                        
create table mmsdm.PredispatchScenarioDemandTrk1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[versionno])
)
go
                        
create table mmsdm.PredispatchRegionfcasrequirement2 (
file_log_id bigint not null,
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
    primary key ([bidtype],[datetime],[genconid],[regionid])
)
go
                        
create table mmsdm.PredispatchLocalPrice1 (
file_log_id bigint not null,
    [predispatchseqno] varchar(20) not null,
[datetime] datetime2 not null,
[duid] varchar(20) not null,
[periodid] varchar(20) null,
[local_price_adjustment] decimal(10,2) null,
[locally_constrained] decimal(1,0) null,
[lastchanged] datetime2 null,
    primary key ([datetime],[duid])
)
go
                        
create table mmsdm.PredispatchMnspbidtrk1 (
file_log_id bigint not null,
    [predispatchseqno] varchar(20) not null,
[linkid] varchar(10) not null,
[periodid] varchar(20) not null,
[participantid] varchar(10) null,
[settlementdate] datetime2 null,
[offerdate] datetime2 null,
[versionno] decimal(3,0) null,
[datetime] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([linkid],[periodid],[predispatchseqno])
)
go
                        
create table mmsdm.PrudentialCompanyPosition1 (
file_log_id bigint not null,
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
    primary key ([company_id],[prudential_date],[runno])
)
go
                        
create table mmsdm.PrudentialRuntrk1 (
file_log_id bigint not null,
    [prudential_date] datetime2 not null,
[runno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([prudential_date],[runno])
)
go
                        
create table mmsdm.MtpasaReservelimit1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[reservelimitid] varchar(20) not null,
[description] varchar(200) null,
[rhs] decimal(16,6) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[reservelimitid],[version_datetime])
)
go
                        
create table mmsdm.MtpasaReservelimitRegion1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[reservelimitid] varchar(20) not null,
[regionid] varchar(20) not null,
[coef] decimal(16,6) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[regionid],[reservelimitid],[version_datetime])
)
go
                        
create table mmsdm.MtpasaReservelimitSet1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[reservelimit_set_id] varchar(20) null,
[description] varchar(200) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[version_datetime])
)
go
                        
create table mmsdm.ReserveDataReserve1 (
file_log_id bigint not null,
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
    primary key ([periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementConfigAncillaryRecoverySplit1 (
file_log_id bigint not null,
    [effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[service] varchar(10) not null,
[paymenttype] varchar(20) not null,
[customer_portion] decimal(8,5) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[paymenttype],[service],[versionno])
)
go
                        
create table mmsdm.SettlementConfigMarketfee1 (
file_log_id bigint not null,
    [marketfeeid] varchar(10) not null,
[marketfeeperiod] varchar(20) null,
[marketfeetype] varchar(12) null,
[description] varchar(64) null,
[lastchanged] datetime2 null,
[gl_tcode] varchar(15) null,
[gl_financialcode] varchar(10) null,
[fee_class] varchar(40) null,
    primary key ([marketfeeid])
)
go
                        
create table mmsdm.SettlementConfigMarketfeedata1 (
file_log_id bigint not null,
    [marketfeeid] varchar(10) not null,
[marketfeeversionno] decimal(3,0) not null,
[effectivedate] datetime2 not null,
[marketfeevalue] decimal(22,8) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[marketfeeid],[marketfeeversionno])
)
go
                        
create table mmsdm.SettlementConfigMarketfeetrk1 (
file_log_id bigint not null,
    [marketfeeversionno] decimal(3,0) not null,
[effectivedate] datetime2 not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[marketfeeversionno])
)
go
                        
create table mmsdm.SettlementConfigMarketFeeCatExcl1 (
file_log_id bigint not null,
    [marketfeeid] varchar(20) not null,
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[participant_categoryid] varchar(20) not null,
    primary key ([effectivedate],[marketfeeid],[participant_categoryid],[version_datetime])
)
go
                        
create table mmsdm.SettlementConfigMarketFeeCatExclTrk1 (
file_log_id bigint not null,
    [marketfeeid] varchar(20) not null,
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[marketfeeid],[version_datetime])
)
go
                        
create table mmsdm.SettlementConfigMarketFeeExclusion1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[marketfeeid] varchar(10) not null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[marketfeeid],[participantid],[versionno])
)
go
                        
create table mmsdm.SettlementConfigMarketFeeExclusionTrk1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid],[versionno])
)
go
                        
create table mmsdm.SettlementConfigParticipantBandfeeAlloc1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[marketfeeid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantcategoryid] varchar(10) not null,
[marketfeevalue] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[marketfeeid],[participantcategoryid],[participantid],[versionno])
)
go
                        
create table mmsdm.SetcfgReallocation2 (
file_log_id bigint not null,
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
    primary key ([reallocationid])
)
go
                        
create table mmsdm.SetcfgReallocationinterval1 (
file_log_id bigint not null,
    [reallocationid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,
[nrp] decimal(15,5) null,
    primary key ([periodid],[reallocationid])
)
go
                        
create table mmsdm.SettlementConfigSetcfgParticipantMpf1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantcategoryid] varchar(10) not null,
[connectionpointid] varchar(10) not null,
[mpf] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([connectionpointid],[effectivedate],[participantcategoryid],[participantid],[versionno])
)
go
                        
create table mmsdm.SettlementConfigSetcfgParticipantMpftrk1 (
file_log_id bigint not null,
    [participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
    primary key ([effectivedate],[participantid],[versionno])
)
go
                        
create table mmsdm.SettlementsDaytrack6 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[regionid] varchar(10) null,
[exanterunstatus] varchar(15) null,
[exanterunno] decimal(3,0) null,
[expostrunstatus] varchar(15) null,
[expostrunno] decimal(3,0) not null,
[lastchanged] datetime2 null,
[settlementintervallength] decimal(3,0) null,
    primary key ([expostrunno],[settlementdate])
)
go
                        
create table mmsdm.SettlementsCpdata6 (
file_log_id bigint not null,
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
[afe] decimal(18,8) null,
[dme] decimal(18,8) null,
[ufea] decimal(18,8) null,
[age] decimal(18,8) null,
    primary key ([mda],[participantid],[periodid],[settlementdate],[tcpid],[versionno])
)
go
                        
create table mmsdm.SettlementsCpdataregion5 (
file_log_id bigint not null,
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
    primary key ([periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsFcasregionrecovery5 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[bidtype] varchar(10) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[generatorregionenergy] decimal(16,6) null,
[customerregionenergy] decimal(16,6) null,
[regionrecovery] decimal(18,8) null,
[lastchanged] datetime2 null,
    primary key ([bidtype],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsGendata6 (
file_log_id bigint not null,
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
    primary key ([duid],[gensetid],[periodid],[regionid],[settlementdate],[stationid],[versionno])
)
go
                        
create table mmsdm.SettlementsGendataregion5 (
file_log_id bigint not null,
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
    primary key ([periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsIntraregionresidues5 (
file_log_id bigint not null,
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
    primary key ([periodid],[regionid],[runno],[settlementdate])
)
go
                        
create table mmsdm.SettlementsIraucsurplus6 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[contractallocation] decimal(8,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,
    primary key ([contractid],[fromregionid],[interconnectorid],[participantid],[periodid],[settlementdate],[settlementrunno])
)
go
                        
create table mmsdm.SettlementsIrnspsurplus6 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[contractallocation] decimal(8,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,
    primary key ([contractid],[fromregionid],[interconnectorid],[participantid],[periodid],[settlementdate],[settlementrunno])
)
go
                        
create table mmsdm.SettlementsIrpartsurplus6 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[contractallocation] decimal(8,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,
    primary key ([contractid],[fromregionid],[interconnectorid],[participantid],[periodid],[settlementdate],[settlementrunno])
)
go
                        
create table mmsdm.SettlementsIrsurplus6 (
file_log_id bigint not null,
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
    primary key ([interconnectorid],[periodid],[regionid],[settlementdate],[settlementrunno])
)
go
                        
create table mmsdm.SettlementsLocalareaenergy1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[localareaid] varchar(30) not null,
[periodid] decimal(3,0) not null,
[ufe] decimal(18,8) null,
[ddme] decimal(18,8) null,
[tme] decimal(18,8) null,
[adme] decimal(18,8) null,
[admela] decimal(18,8) null,
[lastchanged] datetime2 null,
    primary key ([localareaid],[periodid],[settlementdate],[settlementrunno])
)
go
                        
create table mmsdm.SettlementsLocalareatni1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[localareaid] varchar(30) not null,
[tni] varchar(30) not null,
[lastchanged] datetime2 null,
    primary key ([localareaid],[settlementdate],[settlementrunno],[tni])
)
go
                        
create table mmsdm.SettlementsLshedpayment5 (
file_log_id bigint not null,
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
    primary key ([contractid],[participantid],[periodid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsLshedrecovery5 (
file_log_id bigint not null,
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
    primary key ([participantid],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsMarketfees6 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[marketfeeid] varchar(10) not null,
[marketfeevalue] decimal(15,5) null,
[energy] decimal(16,6) null,
[lastchanged] datetime2 null,
[participantcategoryid] varchar(10) not null,
[feerate] decimal(18,8) null,
[feeunits] decimal(18,8) null,
    primary key ([marketfeeid],[participantcategoryid],[participantid],[periodid],[runno],[settlementdate])
)
go
                        
create table mmsdm.SettlementsReallocations5 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(10) not null,
[reallocationid] varchar(20) not null,
[reallocationvalue] decimal(15,5) null,
[energy] decimal(15,5) null,
[rrp] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([participantid],[periodid],[reallocationid],[runno],[settlementdate])
)
go
                        
create table mmsdm.SettlementsRestartpayment6 (
file_log_id bigint not null,
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
    primary key ([contractid],[participantid],[periodid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsRestartrecovery6 (
file_log_id bigint not null,
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
    primary key ([participantid],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsRpowerpayment6 (
file_log_id bigint not null,
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
    primary key ([contractid],[participantid],[periodid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsRpowerrecovery5 (
file_log_id bigint not null,
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
    primary key ([participantid],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsSmallgendata1 (
file_log_id bigint not null,
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
    primary key ([connectionpointid],[participantid],[periodid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsAncillarySummary5 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[service] varchar(20) not null,
[paymenttype] varchar(20) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[paymentamount] decimal(18,8) null,
[lastchanged] datetime2 null,
    primary key ([paymenttype],[periodid],[regionid],[service],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsApcCompensation1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[apeventid] decimal(6,0) not null,
[claimid] decimal(6,0) not null,
[participantid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[compensation_amount] decimal(18,8) null,
    primary key ([apeventid],[claimid],[participantid],[periodid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsApcRecovery1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[apeventid] decimal(6,0) not null,
[claimid] decimal(6,0) not null,
[participantid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[regionid] varchar(20) not null,
[recovery_amount] decimal(18,8) null,
[region_recovery_br_amount] decimal(18,8) null,
    primary key ([apeventid],[claimid],[participantid],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsFcasPayment5 (
file_log_id bigint not null,
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
    primary key ([duid],[periodid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsFcasRecovery6 (
file_log_id bigint not null,
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
    primary key ([participantid],[periodid],[regionid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsSetFcasRegulationTrk1 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interval_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[cmpf] decimal(18,8) null,
[crmpf] decimal(18,8) null,
[recovery_factor_cmpf] decimal(18,8) null,
[recovery_factor_crmpf] decimal(18,8) null,
[lastchanged] datetime2 null,
[usesubstitutedemand] decimal(1,0) null,
[requirementdemand] decimal(18,8) null,
    primary key ([constraintid],[interval_datetime],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsNmasRecovery2 (
file_log_id bigint not null,
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
    primary key ([contractid],[participantid],[paymenttype],[periodid],[regionid],[service],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsNmasRecoveryRbf1 (
file_log_id bigint not null,
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
    primary key ([contractid],[paymenttype],[periodid],[regionid],[service],[settlementdate],[versionno])
)
go
                        
create table mmsdm.SettlementsRunParameter5 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[parameterid] varchar(20) not null,
[numvalue] decimal(18,8) null,
[lastchanged] datetime2 null,
    primary key ([parameterid],[settlementdate],[versionno])
)
go
                        
create table mmsdm.StpasaCasesolution3 (
file_log_id bigint not null,
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
    primary key ([run_datetime])
)
go
                        
create table mmsdm.StpasaConstraintsolution3 (
file_log_id bigint not null,
    [run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[capacityrhs] decimal(12,2) null,
[capacitymarginalvalue] decimal(12,2) null,
[capacityviolationdegree] decimal(12,2) null,
[lastchanged] datetime2 null,
[runtype] varchar(20) not null,
[studyregionid] varchar(20) not null,
    primary key ([constraintid],[interval_datetime],[run_datetime],[runtype],[studyregionid])
)
go
                        
create table mmsdm.StpasaInterconnectorsoln3 (
file_log_id bigint not null,
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
[studyregionid] varchar(20) not null,
    primary key ([interconnectorid],[interval_datetime],[run_datetime],[runtype],[studyregionid])
)
go
                        
create table mmsdm.StpasaRegionsolution6 (
file_log_id bigint not null,
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
[wdr_available] decimal(12,2) null,
[wdr_pasaavailable] decimal(12,2) null,
[wdr_capacity] decimal(12,2) null,
    primary key ([interval_datetime],[regionid],[run_datetime],[runtype])
)
go
                        
create table mmsdm.TradingAverageprice301 (
file_log_id bigint not null,
    [perioddate] datetime2 not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[rrp] decimal(15,5) null,
[price_confidence] varchar(20) null,
[lastchanged] datetime2 null,
    primary key ([perioddate],[regionid])
)
go
                        
create table mmsdm.TradingInterconnectorres2 (
file_log_id bigint not null,
    [settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[meteredmwflow] decimal(15,5) null,
[mwflow] decimal(15,5) null,
[mwlosses] decimal(15,5) null,
[lastchanged] datetime2 null,
    primary key ([interconnectorid],[periodid],[runno],[settlementdate])
)
go
                        
create table mmsdm.TradingPrice2 (
file_log_id bigint not null,
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
    primary key ([periodid],[regionid],[runno],[settlementdate])
)
go
                        
create table mmsdm.VoltageInstructionInstruction2 (
file_log_id bigint not null,
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
    primary key ([ems_id],[run_datetime],[version_datetime])
)
go
                        
create table mmsdm.VoltageInstructionTrack2 (
file_log_id bigint not null,
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
    primary key ([run_datetime],[version_datetime])
)
go
                        