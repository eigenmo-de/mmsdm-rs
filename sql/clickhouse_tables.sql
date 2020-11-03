
create table FileLog (
    file_id Uuid,
    file_name LowCardinality(String),
    data_set LowCardinality(String),
    sub_type LowCardinality(String),
    version LowCardinality(Int8),
    inserted DateTime64,
)
ENGINE = MergeTree()
ORDER BY (file_name, data_set, sub_type, version, id);
create table ParticipantRegistrationStationowner1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`participantid` String,
`stationid` String,
`versionno` Decimal(3,0),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`, `stationid`, `versionno`);
                        
create table ParticipantRegistrationMnspInterconnector2 (
file_id Uuid,
`linkid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`interconnectorid` Nullable(String),
`fromregion` Nullable(String),
`toregion` Nullable(String),
`maxcapacity` Nullable(Decimal(5,0)),
`tlf` Nullable(Decimal(12,7)),
`lhsfactor` Nullable(Decimal(12,7)),
`meterflowconstant` Nullable(Decimal(12,7)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`from_region_tlf` Nullable(Decimal(12,7)),
`to_region_tlf` Nullable(Decimal(12,7))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `linkid`, `versionno`);
                        
create table ParticipantRegistrationParticipantcreditdetail1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`participantid` String,
`creditlimit` Nullable(Decimal(10,0)),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`);
                        
create table ParticipantRegistrationGenunits2 (
file_id Uuid,
`gensetid` String,
`stationid` Nullable(String),
`setlossfactor` Nullable(Decimal(16,6)),
`cdindicator` Nullable(String),
`agcflag` Nullable(String),
`spinningflag` Nullable(String),
`voltlevel` Nullable(Decimal(6,0)),
`registeredcapacity` Nullable(Decimal(6,0)),
`dispatchtype` Nullable(String),
`starttype` Nullable(String),
`mktgeneratorind` Nullable(String),
`normalstatus` Nullable(String),
`maxcapacity` Nullable(Decimal(6,0)),
`gensettype` Nullable(String),
`gensetname` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`co2e_emissions_factor` Nullable(Decimal(18,8)),
`co2e_energy_source` Nullable(String),
`co2e_data_source` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`gensetid`);
                        
create table ParticipantRegistrationStationownertrk1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`participantid` String,
`versionno` Decimal(3,0),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`, `versionno`);
                        
create table ParticipantRegistrationStation1 (
file_id Uuid,
`stationid` String,
`stationname` Nullable(String),
`address1` Nullable(String),
`address2` Nullable(String),
`address3` Nullable(String),
`address4` Nullable(String),
`city` Nullable(String),
`state` Nullable(String),
`postcode` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`connectionpointid` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`stationid`);
                        
create table ParticipantRegistrationGenmeter1 (
file_id Uuid,
`meterid` String,
`gensetid` Nullable(String),
`connectionpointid` Nullable(String),
`stationid` Nullable(String),
`metertype` Nullable(String),
`meterclass` Nullable(String),
`voltagelevel` Nullable(Decimal(6,0)),
`applydate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`comdate` Nullable(DateTime('Australia/Brisbane')),
`decomdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`applydate`, `meterid`, `versionno`);
                        
create table ParticipantRegistrationParticipantcategory1 (
file_id Uuid,
`participantcategoryid` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantcategoryid`);
                        
create table ParticipantRegistrationDudetailsummary4 (
file_id Uuid,
`duid` String,
`start_date` DateTime('Australia/Brisbane'),
`end_date` DateTime('Australia/Brisbane'),
`dispatchtype` Nullable(String),
`connectionpointid` Nullable(String),
`regionid` Nullable(String),
`stationid` Nullable(String),
`participantid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`transmissionlossfactor` Nullable(Decimal(15,5)),
`starttype` Nullable(String),
`distributionlossfactor` Nullable(Decimal(15,5)),
`minimum_energy_price` Nullable(Decimal(9,2)),
`maximum_energy_price` Nullable(Decimal(9,2)),
`schedule_type` Nullable(String),
`min_ramp_rate_up` Nullable(Decimal(6,0)),
`min_ramp_rate_down` Nullable(Decimal(6,0)),
`max_ramp_rate_up` Nullable(Decimal(6,0)),
`max_ramp_rate_down` Nullable(Decimal(6,0)),
`is_aggregated` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `start_date`);
                        
create table ParticipantRegistrationStationoperatingstatus1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`stationid` String,
`versionno` Decimal(3,0),
`status` Nullable(String),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `stationid`, `versionno`);
                        
create table ParticipantRegistrationDualloc1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`duid` String,
`gensetid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `effectivedate`, `gensetid`, `versionno`);
                        
create table ParticipantRegistrationDispatchableunit1 (
file_id Uuid,
`duid` String,
`duname` Nullable(String),
`unittype` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`);
                        
create table ParticipantRegistrationBidduiddetailstrk1 (
file_id Uuid,
`duid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `effectivedate`, `versionno`);
                        
create table ParticipantRegistrationParticipant1 (
file_id Uuid,
`participantid` String,
`participantclassid` Nullable(String),
`name` Nullable(String),
`description` Nullable(String),
`acn` Nullable(String),
`primarybusiness` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantid`);
                        
create table ParticipantRegistrationGenunitsUnit1 (
file_id Uuid,
`gensetid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(6,0),
`unit_grouping_label` String,
`unit_count` Nullable(Decimal(3,0)),
`unit_size` Nullable(Decimal(8,3)),
`unit_max_size` Nullable(Decimal(8,3)),
`aggregation_flag` Nullable(Decimal(1,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `gensetid`, `unit_grouping_label`, `versionno`);
                        
create table ParticipantRegistrationParticipantcategoryalloc1 (
file_id Uuid,
`participantcategoryid` String,
`participantid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantcategoryid`, `participantid`);
                        
create table ParticipantRegistrationBidduiddetails1 (
file_id Uuid,
`duid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`bidtype` String,
`maxcapacity` Nullable(Decimal(22,0)),
`minenablementlevel` Nullable(Decimal(22,0)),
`maxenablementlevel` Nullable(Decimal(22,0)),
`maxlowerangle` Nullable(Decimal(3,0)),
`maxupperangle` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `effectivedate`, `versionno`);
                        
create table ParticipantRegistrationMnspParticipant1 (
file_id Uuid,
`interconnectorid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `interconnectorid`, `participantid`, `versionno`);
                        
create table ParticipantRegistrationStadualloc1 (
file_id Uuid,
`duid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`stationid` String,
`versionno` Decimal(3,0),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `effectivedate`, `stationid`, `versionno`);
                        
create table ParticipantRegistrationParticipantclass1 (
file_id Uuid,
`participantclassid` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantclassid`);
                        
create table ParticipantRegistrationDudetail3 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`duid` String,
`versionno` Decimal(3,0),
`connectionpointid` Nullable(String),
`voltlevel` Nullable(String),
`registeredcapacity` Nullable(Decimal(6,0)),
`agccapability` Nullable(String),
`dispatchtype` Nullable(String),
`maxcapacity` Nullable(Decimal(6,0)),
`starttype` Nullable(String),
`normallyonflag` Nullable(String),
`physicaldetailsflag` Nullable(String),
`spinningreserveflag` Nullable(String),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`intermittentflag` Nullable(String),
`semi_schedule_flag` Nullable(String),
`maxrateofchangeup` Nullable(Decimal(6,0)),
`maxrateofchangedown` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `effectivedate`, `versionno`);
                        
create table ParticipantRegistrationParticipantaccount1 (
file_id Uuid,
`accountname` Nullable(String),
`participantid` String,
`accountnumber` Nullable(String),
`bankname` Nullable(String),
`banknumber` Nullable(Decimal(10,0)),
`branchname` Nullable(String),
`branchnumber` Nullable(Decimal(10,0)),
`bsbnumber` Nullable(String),
`nemmcocreditaccountnumber` Nullable(Decimal(10,0)),
`nemmcodebitaccountnumber` Nullable(Decimal(10,0)),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`effectivedate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`abn` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`participantid`);
                        
create table SettlementsIrfmrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`irfmid` String,
`irmfversion` Nullable(Decimal(3,0)),
`participantid` String,
`participantdemand` Nullable(Decimal(12,5)),
`totaltcd` Nullable(Decimal(12,5)),
`totaltfd` Nullable(Decimal(12,5)),
`irfmamount` Nullable(Decimal(12,5)),
`irfmpayment` Nullable(Decimal(12,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`irfmid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsRestartpayment6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` String,
`periodid` Decimal(3,0),
`regionid` Nullable(String),
`restarttype` Nullable(Decimal(1,0)),
`avaflag` Nullable(Decimal(1,0)),
`availabilityprice` Nullable(Decimal(15,5)),
`tcf` Nullable(Decimal(1,0)),
`availabilitypayment` Nullable(Decimal(15,5)),
`contractversionno` Nullable(Decimal(3,0)),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`offerversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`enablingpayment` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsNmasRecovery2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`participantid` String,
`service` String,
`contractid` String,
`paymenttype` String,
`regionid` String,
`rbf` Nullable(Decimal(18,8)),
`payment_amount` Nullable(Decimal(18,8)),
`participant_energy` Nullable(Decimal(18,8)),
`region_energy` Nullable(Decimal(18,8)),
`recovery_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`participant_generation` Nullable(Decimal(18,8)),
`region_generation` Nullable(Decimal(18,8)),
`recovery_amount_customer` Nullable(Decimal(18,8)),
`recovery_amount_generator` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `paymenttype`, `periodid`, `regionid`, `service`, `settlementdate`, `versionno`);
                        
create table SettlementsApcCompensation1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`apeventid` Decimal(6,0),
`claimid` Decimal(6,0),
`participantid` String,
`periodid` Decimal(3,0),
`compensation_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`apeventid`, `claimid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsVicenergyfigures5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`totalgenoutput` Nullable(Decimal(15,5)),
`totalpcsd` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`tlr` Nullable(Decimal(15,6)),
`mlf` Nullable(Decimal(15,6))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsVicenergyflow5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`netflow` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsApcRecovery1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`apeventid` Decimal(6,0),
`claimid` Decimal(6,0),
`participantid` String,
`periodid` Decimal(3,0),
`regionid` String,
`recovery_amount` Nullable(Decimal(18,8)),
`region_recovery_br_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`apeventid`, `claimid`, `participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsIrnspsurplus6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`settlementrunno` Decimal(3,0),
`contractid` String,
`periodid` Decimal(2,0),
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`totalsurplus` Nullable(Decimal(15,5)),
`contractallocation` Nullable(Decimal(8,5)),
`surplusvalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `participantid`, `periodid`, `settlementdate`, `settlementrunno`);
                        
create table SettlementsRunParameter5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`parameterid` String,
`numvalue` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`parameterid`, `settlementdate`, `versionno`);
                        
create table SettlementsSetFcasRegulationTrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`interval_datetime` DateTime('Australia/Brisbane'),
`constraintid` String,
`cmpf` Nullable(Decimal(18,8)),
`crmpf` Nullable(Decimal(18,8)),
`recovery_factor_cmpf` Nullable(Decimal(18,8)),
`recovery_factor_crmpf` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `interval_datetime`, `settlementdate`, `versionno`);
                        
create table SettlementsAncillarySummary5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`service` String,
`paymenttype` String,
`regionid` String,
`periodid` Decimal(3,0),
`paymentamount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`paymenttype`, `periodid`, `regionid`, `service`, `settlementdate`, `versionno`);
                        
create table SettlementsIrsurplus6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`settlementrunno` Decimal(3,0),
`periodid` Decimal(3,0),
`interconnectorid` String,
`regionid` String,
`mwflow` Nullable(Decimal(15,6)),
`lossfactor` Nullable(Decimal(15,5)),
`surplusvalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`interconnectorid`, `periodid`, `regionid`, `settlementdate`, `settlementrunno`);
                        
create table SettlementsLunloadpayment5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` String,
`periodid` Decimal(3,0),
`duid` Nullable(String),
`regionid` Nullable(String),
`tlf` Nullable(Decimal(7,5)),
`ebp` Nullable(Decimal(15,5)),
`rrp` Nullable(Decimal(15,5)),
`enablingprice` Nullable(Decimal(15,5)),
`usageprice` Nullable(Decimal(15,5)),
`ccprice` Nullable(Decimal(15,5)),
`clearedmw` Nullable(Decimal(15,5)),
`unconstrainedmw` Nullable(Decimal(15,5)),
`controlrange` Nullable(Decimal(4,0)),
`enablingpayment` Nullable(Decimal(15,5)),
`usagepayment` Nullable(Decimal(15,5)),
`compensationpayment` Nullable(Decimal(15,5)),
`contractversionno` Nullable(Decimal(3,0)),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`offerversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsIntervention5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`contractid` Nullable(String),
`contractversion` Nullable(Decimal(3,0)),
`participantid` Nullable(String),
`regionid` Nullable(String),
`duid` String,
`rcf` Nullable(FixedString(1)),
`interventionpayment` Nullable(Decimal(12,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsDaytrack5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`regionid` Nullable(String),
`exanterunstatus` Nullable(String),
`exanterunno` Nullable(Decimal(3,0)),
`expostrunstatus` Nullable(String),
`expostrunno` Decimal(3,0),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`settlementintervallength` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`expostrunno`, `settlementdate`);
                        
create table SettlementsCpdataregion5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(22,10),
`periodid` Decimal(22,10),
`regionid` String,
`sumigenergy` Nullable(Decimal(27,5)),
`sumxgenergy` Nullable(Decimal(27,5)),
`suminenergy` Nullable(Decimal(27,5)),
`sumxnenergy` Nullable(Decimal(27,5)),
`sumipower` Nullable(Decimal(22,0)),
`sumxpower` Nullable(Decimal(22,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`sumep` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsRestartrecovery6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` Nullable(String),
`periodid` Decimal(3,0),
`regionid` String,
`availabilitypayment` Nullable(Decimal(15,5)),
`participantdemand` Nullable(Decimal(15,5)),
`regiondemand` Nullable(Decimal(15,5)),
`availabilityrecovery` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`availabilityrecovery_gen` Nullable(Decimal(15,5)),
`participantdemand_gen` Nullable(Decimal(15,5)),
`regiondemand_gen` Nullable(Decimal(15,5)),
`enablingpayment` Nullable(Decimal(18,8)),
`enablingrecovery` Nullable(Decimal(18,8)),
`enablingrecovery_gen` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsCpdata5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(10,0),
`periodid` Decimal(10,0),
`participantid` String,
`tcpid` String,
`regionid` Nullable(String),
`igenergy` Nullable(Decimal(16,6)),
`xgenergy` Nullable(Decimal(16,6)),
`inenergy` Nullable(Decimal(16,6)),
`xnenergy` Nullable(Decimal(16,6)),
`ipower` Nullable(Decimal(16,6)),
`xpower` Nullable(Decimal(16,6)),
`rrp` Nullable(Decimal(20,5)),
`eep` Nullable(Decimal(16,6)),
`tlf` Nullable(Decimal(7,5)),
`cprrp` Nullable(Decimal(16,6)),
`cpeep` Nullable(Decimal(16,6)),
`ta` Nullable(Decimal(16,6)),
`ep` Nullable(Decimal(16,6)),
`apc` Nullable(Decimal(16,6)),
`resc` Nullable(Decimal(16,6)),
`resp` Nullable(Decimal(16,6)),
`meterrunno` Nullable(Decimal(10,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`hostdistributor` Nullable(String),
`mda` String,
`afe` Nullable(Decimal(18,8)),
`dme` Nullable(Decimal(18,8)),
`ufea` Nullable(Decimal(18,8)),
`age` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`mda`, `participantid`, `periodid`, `settlementdate`, `tcpid`, `versionno`);
                        
create table SettlementsIraucsurplus6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`settlementrunno` Decimal(3,0),
`contractid` String,
`periodid` Decimal(2,0),
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`totalsurplus` Nullable(Decimal(15,5)),
`contractallocation` Nullable(Decimal(8,5)),
`surplusvalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `participantid`, `periodid`, `settlementdate`, `settlementrunno`);
                        
create table SettlementsLshedpayment5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` String,
`periodid` Decimal(3,0),
`duid` Nullable(String),
`regionid` Nullable(String),
`tlf` Nullable(Decimal(7,5)),
`rrp` Nullable(Decimal(15,5)),
`lseprice` Nullable(Decimal(15,5)),
`mcpprice` Nullable(Decimal(15,5)),
`lscr` Nullable(Decimal(4,0)),
`lsepayment` Nullable(Decimal(15,5)),
`ccpayment` Nullable(Decimal(15,5)),
`constrainedmw` Nullable(Decimal(15,5)),
`unconstrainedmw` Nullable(Decimal(15,5)),
`als` Nullable(Decimal(15,5)),
`initialdemand` Nullable(Decimal(15,5)),
`finaldemand` Nullable(Decimal(15,5)),
`contractversionno` Nullable(Decimal(3,0)),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`offerversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`availabilitypayment` Nullable(Decimal(16,6))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsMarketfees5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`participantid` String,
`periodid` Decimal(3,0),
`marketfeeid` String,
`marketfeevalue` Nullable(Decimal(15,5)),
`energy` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`participantcategoryid` String,
`feerate` Nullable(Decimal(18,8)),
`feeunits` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`marketfeeid`, `participantcategoryid`, `participantid`, `periodid`, `runno`, `settlementdate`);
                        
create table SettlementsAgcpayment5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` String,
`periodid` Decimal(3,0),
`duid` Nullable(String),
`regionid` Nullable(String),
`tlf` Nullable(Decimal(7,5)),
`ebp` Nullable(Decimal(15,5)),
`rrp` Nullable(Decimal(15,5)),
`clearedmw` Nullable(Decimal(15,5)),
`initialmw` Nullable(Decimal(15,5)),
`enablingpayment` Nullable(Decimal(15,5)),
`contractversionno` Nullable(Decimal(3,0)),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`offerversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsGendataregion5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(22,10),
`periodid` Decimal(22,10),
`regionid` String,
`genergy` Nullable(Decimal(22,0)),
`aenergy` Nullable(Decimal(22,0)),
`gpower` Nullable(Decimal(22,0)),
`apower` Nullable(Decimal(22,0)),
`netenergy` Nullable(Decimal(27,5)),
`energycost` Nullable(Decimal(27,5)),
`excessenergycost` Nullable(Decimal(27,5)),
`expenergy` Nullable(Decimal(27,6)),
`expenergycost` Nullable(Decimal(27,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsVicboundaryenergy5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`periodid` Decimal(3,0),
`boundaryenergy` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsFcasPayment5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` Nullable(String),
`duid` String,
`regionid` Nullable(String),
`periodid` Decimal(3,0),
`lower6sec_payment` Nullable(Decimal(18,8)),
`raise6sec_payment` Nullable(Decimal(18,8)),
`lower60sec_payment` Nullable(Decimal(18,8)),
`raise60sec_payment` Nullable(Decimal(18,8)),
`lower5min_payment` Nullable(Decimal(18,8)),
`raise5min_payment` Nullable(Decimal(18,8)),
`lowerreg_payment` Nullable(Decimal(18,8)),
`raisereg_payment` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsLshedrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` Nullable(String),
`periodid` Decimal(3,0),
`regionid` String,
`lsepayment` Nullable(Decimal(15,5)),
`ccpayment` Nullable(Decimal(15,5)),
`participantdemand` Nullable(Decimal(15,5)),
`regiondemand` Nullable(Decimal(15,5)),
`lserecovery` Nullable(Decimal(15,5)),
`ccrecovery` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`lserecovery_gen` Nullable(Decimal(15,5)),
`ccrecovery_gen` Nullable(Decimal(15,5)),
`participantdemand_gen` Nullable(Decimal(15,5)),
`regiondemand_gen` Nullable(Decimal(15,5)),
`availabilityrecovery` Nullable(Decimal(16,6)),
`availabilityrecovery_gen` Nullable(Decimal(16,6))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsRpowerrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` Nullable(String),
`periodid` Decimal(3,0),
`regionid` String,
`availabilitypayment` Nullable(Decimal(15,5)),
`enablingpayment` Nullable(Decimal(15,5)),
`ccpayment` Nullable(Decimal(15,5)),
`participantdemand` Nullable(Decimal(15,5)),
`regiondemand` Nullable(Decimal(15,5)),
`availabilityrecovery` Nullable(Decimal(15,5)),
`enablingrecovery` Nullable(Decimal(15,5)),
`ccrecovery` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`availabilityrecovery_gen` Nullable(Decimal(15,5)),
`enablingrecovery_gen` Nullable(Decimal(15,5)),
`ccrecovery_gen` Nullable(Decimal(15,5)),
`participantdemand_gen` Nullable(Decimal(15,5)),
`regiondemand_gen` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsSmallgendata1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`connectionpointid` String,
`periodid` Decimal(3,0),
`participantid` String,
`regionid` Nullable(String),
`importenergy` Nullable(Decimal(18,8)),
`exportenergy` Nullable(Decimal(18,8)),
`rrp` Nullable(Decimal(18,8)),
`tlf` Nullable(Decimal(18,8)),
`impenergycost` Nullable(Decimal(18,8)),
`expenergycost` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`connectionpointid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsGendata6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(10,0),
`periodid` Decimal(10,0),
`participantid` Nullable(String),
`stationid` String,
`duid` String,
`gensetid` String,
`regionid` String,
`genergy` Nullable(Decimal(16,6)),
`aenergy` Nullable(Decimal(16,6)),
`gpower` Nullable(Decimal(16,6)),
`apower` Nullable(Decimal(16,6)),
`rrp` Nullable(Decimal(20,5)),
`eep` Nullable(Decimal(16,6)),
`tlf` Nullable(Decimal(7,5)),
`cprrp` Nullable(Decimal(16,6)),
`cpeep` Nullable(Decimal(16,6)),
`netenergy` Nullable(Decimal(16,6)),
`energycost` Nullable(Decimal(16,6)),
`excessenergycost` Nullable(Decimal(16,6)),
`apc` Nullable(Decimal(16,6)),
`resc` Nullable(Decimal(16,6)),
`resp` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`expenergy` Nullable(Decimal(15,6)),
`expenergycost` Nullable(Decimal(15,6)),
`meterrunno` Nullable(Decimal(6,0)),
`mda` Nullable(String),
`secondary_tlf` Nullable(Decimal(7,5))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `gensetid`, `periodid`, `regionid`, `settlementdate`, `stationid`, `versionno`);
                        
create table SettlementsInterventionrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`contractid` String,
`rcf` Nullable(FixedString(1)),
`participantid` String,
`participantdemand` Nullable(Decimal(12,5)),
`totaldemand` Nullable(Decimal(12,5)),
`interventionpayment` Nullable(Decimal(12,5)),
`interventionamount` Nullable(Decimal(12,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`regionid` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsIntraregionresidues5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`periodid` Decimal(3,0),
`regionid` String,
`ep` Nullable(Decimal(15,5)),
`ec` Nullable(Decimal(15,5)),
`rrp` Nullable(Decimal(15,5)),
`exp` Nullable(Decimal(15,5)),
`irss` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `regionid`, `runno`, `settlementdate`);
                        
create table SettlementsNmasRecoveryRbf1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`service` String,
`contractid` String,
`paymenttype` String,
`regionid` String,
`rbf` Nullable(Decimal(18,8)),
`payment_amount` Nullable(Decimal(18,8)),
`recovery_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `paymenttype`, `periodid`, `regionid`, `service`, `settlementdate`, `versionno`);
                        
create table SettlementsMrPayment5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`regionid` String,
`participantid` Nullable(String),
`duid` String,
`periodid` Decimal(3,0),
`mr_capacity` Nullable(Decimal(16,6)),
`uncapped_payment` Nullable(Decimal(16,6)),
`capped_payment` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsMrRecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`regionid` String,
`participantid` Nullable(String),
`duid` String,
`periodid` Decimal(3,0),
`arodef` Nullable(Decimal(16,6)),
`nta` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsFcasRecovery6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` String,
`participantid` String,
`regionid` String,
`periodid` Decimal(3,0),
`lower6sec_recovery` Nullable(Decimal(18,8)),
`raise6sec_recovery` Nullable(Decimal(18,8)),
`lower60sec_recovery` Nullable(Decimal(18,8)),
`raise60sec_recovery` Nullable(Decimal(18,8)),
`lower5min_recovery` Nullable(Decimal(18,8)),
`raise5min_recovery` Nullable(Decimal(18,8)),
`lowerreg_recovery` Nullable(Decimal(18,8)),
`raisereg_recovery` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`lower6sec_recovery_gen` Nullable(Decimal(18,8)),
`raise6sec_recovery_gen` Nullable(Decimal(18,8)),
`lower60sec_recovery_gen` Nullable(Decimal(18,8)),
`raise60sec_recovery_gen` Nullable(Decimal(18,8)),
`lower5min_recovery_gen` Nullable(Decimal(18,8)),
`raise5min_recovery_gen` Nullable(Decimal(18,8)),
`lowerreg_recovery_gen` Nullable(Decimal(18,8)),
`raisereg_recovery_gen` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsLunloadrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` Nullable(String),
`periodid` Decimal(3,0),
`regionid` String,
`enablingpayment` Nullable(Decimal(15,5)),
`usagepayment` Nullable(Decimal(15,5)),
`compensationpayment` Nullable(Decimal(15,5)),
`participantdemand` Nullable(Decimal(15,5)),
`regiondemand` Nullable(Decimal(15,5)),
`enablingrecovery` Nullable(Decimal(15,5)),
`usagerecovery` Nullable(Decimal(15,5)),
`compensationrecovery` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`enablingrecovery_gen` Nullable(Decimal(15,5)),
`usagerecovery_gen` Nullable(Decimal(15,5)),
`compensationrecovery_gen` Nullable(Decimal(15,5)),
`participantdemand_gen` Nullable(Decimal(15,5)),
`regiondemand_gen` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsAgcrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` Nullable(String),
`periodid` Decimal(3,0),
`regionid` String,
`enablingpayment` Nullable(Decimal(15,5)),
`participantdemand` Nullable(Decimal(15,5)),
`regiondemand` Nullable(Decimal(15,5)),
`enablingrecovery` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`enablingrecovery_gen` Nullable(Decimal(15,5)),
`participantdemand_gen` Nullable(Decimal(15,5)),
`regiondemand_gen` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsRpowerpayment6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` String,
`periodid` Decimal(3,0),
`duid` Nullable(String),
`regionid` Nullable(String),
`tlf` Nullable(Decimal(7,5)),
`ebp` Nullable(Decimal(15,5)),
`rrp` Nullable(Decimal(15,5)),
`mvaraprice` Nullable(Decimal(15,5)),
`mvareprice` Nullable(Decimal(15,5)),
`mvargprice` Nullable(Decimal(15,5)),
`ccprice` Nullable(Decimal(15,5)),
`synccompensation` Nullable(Decimal(1,0)),
`mta` Nullable(Decimal(15,5)),
`mtg` Nullable(Decimal(15,5)),
`blocksize` Nullable(Decimal(4,0)),
`avaflag` Nullable(Decimal(1,0)),
`clearedmw` Nullable(Decimal(15,5)),
`unconstrainedmw` Nullable(Decimal(15,5)),
`availabilitypayment` Nullable(Decimal(15,5)),
`enablingpayment` Nullable(Decimal(15,5)),
`ccpayment` Nullable(Decimal(15,5)),
`contractversionno` Nullable(Decimal(3,0)),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`offerversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`availabilitypayment_rebate` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table SettlementsReallocations5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`periodid` Decimal(3,0),
`participantid` String,
`reallocationid` String,
`reallocationvalue` Nullable(Decimal(15,5)),
`energy` Nullable(Decimal(15,5)),
`rrp` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `reallocationid`, `runno`, `settlementdate`);
                        
create table SettlementsFcasregionrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`bidtype` String,
`regionid` String,
`periodid` Decimal(3,0),
`generatorregionenergy` Nullable(Decimal(16,6)),
`customerregionenergy` Nullable(Decimal(16,6)),
`regionrecovery` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsLuloadrecovery5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`contractid` Nullable(String),
`periodid` Decimal(3,0),
`regionid` String,
`enablingpayment` Nullable(Decimal(15,5)),
`usagepayment` Nullable(Decimal(15,5)),
`compensationpayment` Nullable(Decimal(15,5)),
`participantdemand` Nullable(Decimal(15,5)),
`regiondemand` Nullable(Decimal(15,5)),
`enablingrecovery` Nullable(Decimal(15,5)),
`usagerecovery` Nullable(Decimal(15,5)),
`compensationrecovery` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`enablingrecovery_gen` Nullable(Decimal(15,5)),
`usagerecovery_gen` Nullable(Decimal(15,5)),
`compensationrecovery_gen` Nullable(Decimal(15,5)),
`participantdemand_gen` Nullable(Decimal(15,5)),
`regiondemand_gen` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table SettlementsIrpartsurplus6 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`settlementrunno` Decimal(3,0),
`contractid` String,
`periodid` Decimal(2,0),
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`totalsurplus` Nullable(Decimal(15,5)),
`contractallocation` Nullable(Decimal(8,5)),
`surplusvalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `participantid`, `periodid`, `settlementdate`, `settlementrunno`);
                        
create table SettlementsFcascomp5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`duid` String,
`regionid` Nullable(String),
`periodid` Decimal(3,0),
`ccprice` Nullable(Decimal(15,5)),
`clearedmw` Nullable(Decimal(15,5)),
`unconstrainedmw` Nullable(Decimal(15,5)),
`ebp` Nullable(Decimal(15,5)),
`tlf` Nullable(Decimal(7,5)),
`rrp` Nullable(Decimal(15,5)),
`excessgen` Nullable(Decimal(15,5)),
`fcascomp` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table PredispatchOffertrk1 (
file_id Uuid,
`predispatchseqno` String,
`duid` String,
`bidtype` String,
`periodid` String,
`bidsettlementdate` Nullable(DateTime('Australia/Brisbane')),
`bidofferdate` Nullable(DateTime('Australia/Brisbane')),
`datetime` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `periodid`, `predispatchseqno`);
                        
create table PredispatchScenarioDemandTrk1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `versionno`);
                        
create table PredispatchRegionfcasrequirement2 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`intervention` Nullable(Decimal(2,0)),
`periodid` Nullable(String),
`genconid` String,
`regionid` String,
`bidtype` String,
`genconeffectivedate` Nullable(DateTime('Australia/Brisbane')),
`genconversionno` Nullable(Decimal(3,0)),
`marginalvalue` Nullable(Decimal(16,6)),
`datetime` DateTime('Australia/Brisbane'),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`base_cost` Nullable(Decimal(18,8)),
`adjusted_cost` Nullable(Decimal(18,8)),
`estimated_cmpf` Nullable(Decimal(18,8)),
`estimated_crmpf` Nullable(Decimal(18,8)),
`recovery_factor_cmpf` Nullable(Decimal(18,8)),
`recovery_factor_crmpf` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `datetime`, `genconid`, `regionid`);
                        
create table PredispatchCaseSolution1 (
file_id Uuid,
`predispatchseqno` String,
`runno` Decimal(3,0),
`solutionstatus` Nullable(Decimal(2,0)),
`spdversion` Nullable(String),
`nonphysicallosses` Nullable(Decimal(1,0)),
`totalobjective` Nullable(Decimal(27,10)),
`totalareagenviolation` Nullable(Decimal(15,5)),
`totalinterconnectorviolation` Nullable(Decimal(15,5)),
`totalgenericviolation` Nullable(Decimal(15,5)),
`totalramprateviolation` Nullable(Decimal(15,5)),
`totalunitmwcapacityviolation` Nullable(Decimal(15,5)),
`total5minviolation` Nullable(Decimal(15,5)),
`totalregviolation` Nullable(Decimal(15,5)),
`total6secviolation` Nullable(Decimal(15,5)),
`total60secviolation` Nullable(Decimal(15,5)),
`totalasprofileviolation` Nullable(Decimal(15,5)),
`totalenergyconstrviolation` Nullable(Decimal(15,5)),
`totalenergyofferviolation` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`intervention` Nullable(Decimal(2,0))
)
ENGINE = MergeTree()
ORDER BY (`predispatchseqno`, `runno`);
                        
create table PredispatchUnitSolution2 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`duid` String,
`tradetype` Nullable(Decimal(2,0)),
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`connectionpointid` Nullable(String),
`agcstatus` Nullable(Decimal(2,0)),
`dispatchmode` Nullable(Decimal(2,0)),
`initialmw` Nullable(Decimal(15,5)),
`totalcleared` Nullable(Decimal(15,5)),
`lower5min` Nullable(Decimal(15,5)),
`lower60sec` Nullable(Decimal(15,5)),
`lower6sec` Nullable(Decimal(15,5)),
`raise5min` Nullable(Decimal(15,5)),
`raise60sec` Nullable(Decimal(15,5)),
`raise6sec` Nullable(Decimal(15,5)),
`rampdownrate` Nullable(Decimal(15,5)),
`rampuprate` Nullable(Decimal(15,5)),
`downepf` Nullable(Decimal(15,5)),
`upepf` Nullable(Decimal(15,5)),
`marginal5minvalue` Nullable(Decimal(15,5)),
`marginal60secvalue` Nullable(Decimal(15,5)),
`marginal6secvalue` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violation5mindegree` Nullable(Decimal(15,5)),
`violation60secdegree` Nullable(Decimal(15,5)),
`violation6secdegree` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`datetime` DateTime('Australia/Brisbane'),
`lowerreg` Nullable(Decimal(15,5)),
`raisereg` Nullable(Decimal(15,5)),
`availability` Nullable(Decimal(15,5)),
`raise6secflags` Nullable(Decimal(3,0)),
`raise60secflags` Nullable(Decimal(3,0)),
`raise5minflags` Nullable(Decimal(3,0)),
`raiseregflags` Nullable(Decimal(3,0)),
`lower6secflags` Nullable(Decimal(3,0)),
`lower60secflags` Nullable(Decimal(3,0)),
`lower5minflags` Nullable(Decimal(3,0)),
`lowerregflags` Nullable(Decimal(3,0)),
`raise6secactualavailability` Nullable(Decimal(16,6)),
`raise60secactualavailability` Nullable(Decimal(16,6)),
`raise5minactualavailability` Nullable(Decimal(16,6)),
`raiseregactualavailability` Nullable(Decimal(16,6)),
`lower6secactualavailability` Nullable(Decimal(16,6)),
`lower60secactualavailability` Nullable(Decimal(16,6)),
`lower5minactualavailability` Nullable(Decimal(16,6)),
`lowerregactualavailability` Nullable(Decimal(16,6)),
`semidispatchcap` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `duid`);
                        
create table PredispatchBlockedConstraints1 (
file_id Uuid,
`predispatchseqno` String,
`constraintid` String
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `predispatchseqno`);
                        
create table PredispatchRegionPrices1 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`regionid` String,
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`rrp` Nullable(Decimal(15,5)),
`eep` Nullable(Decimal(15,5)),
`rrp1` Nullable(Decimal(15,5)),
`eep1` Nullable(Decimal(15,5)),
`rrp2` Nullable(Decimal(15,5)),
`eep2` Nullable(Decimal(15,5)),
`rrp3` Nullable(Decimal(15,5)),
`eep3` Nullable(Decimal(15,5)),
`rrp4` Nullable(Decimal(15,5)),
`eep4` Nullable(Decimal(15,5)),
`rrp5` Nullable(Decimal(15,5)),
`eep5` Nullable(Decimal(15,5)),
`rrp6` Nullable(Decimal(15,5)),
`eep6` Nullable(Decimal(15,5)),
`rrp7` Nullable(Decimal(15,5)),
`eep7` Nullable(Decimal(15,5)),
`rrp8` Nullable(Decimal(15,5)),
`eep8` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`datetime` DateTime('Australia/Brisbane'),
`raise6secrrp` Nullable(Decimal(15,5)),
`raise60secrrp` Nullable(Decimal(15,5)),
`raise5minrrp` Nullable(Decimal(15,5)),
`raiseregrrp` Nullable(Decimal(15,5)),
`lower6secrrp` Nullable(Decimal(15,5)),
`lower60secrrp` Nullable(Decimal(15,5)),
`lower5minrrp` Nullable(Decimal(15,5)),
`lowerregrrp` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `regionid`);
                        
create table PredispatchRegionSolution4 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`regionid` String,
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`totaldemand` Nullable(Decimal(15,5)),
`availablegeneration` Nullable(Decimal(15,5)),
`availableload` Nullable(Decimal(15,5)),
`demandforecast` Nullable(Decimal(15,5)),
`dispatchablegeneration` Nullable(Decimal(15,5)),
`dispatchableload` Nullable(Decimal(15,5)),
`netinterchange` Nullable(Decimal(15,5)),
`excessgeneration` Nullable(Decimal(15,5)),
`lower5mindispatch` Nullable(Decimal(15,5)),
`lower5minimport` Nullable(Decimal(15,5)),
`lower5minlocaldispatch` Nullable(Decimal(15,5)),
`lower5minlocalprice` Nullable(Decimal(15,5)),
`lower5minlocalreq` Nullable(Decimal(15,5)),
`lower5minprice` Nullable(Decimal(15,5)),
`lower5minreq` Nullable(Decimal(15,5)),
`lower5minsupplyprice` Nullable(Decimal(15,5)),
`lower60secdispatch` Nullable(Decimal(15,5)),
`lower60secimport` Nullable(Decimal(15,5)),
`lower60seclocaldispatch` Nullable(Decimal(15,5)),
`lower60seclocalprice` Nullable(Decimal(15,5)),
`lower60seclocalreq` Nullable(Decimal(15,5)),
`lower60secprice` Nullable(Decimal(15,5)),
`lower60secreq` Nullable(Decimal(15,5)),
`lower60secsupplyprice` Nullable(Decimal(15,5)),
`lower6secdispatch` Nullable(Decimal(15,5)),
`lower6secimport` Nullable(Decimal(15,5)),
`lower6seclocaldispatch` Nullable(Decimal(15,5)),
`lower6seclocalprice` Nullable(Decimal(15,5)),
`lower6seclocalreq` Nullable(Decimal(15,5)),
`lower6secprice` Nullable(Decimal(15,5)),
`lower6secreq` Nullable(Decimal(15,5)),
`lower6secsupplyprice` Nullable(Decimal(15,5)),
`raise5mindispatch` Nullable(Decimal(15,5)),
`raise5minimport` Nullable(Decimal(15,5)),
`raise5minlocaldispatch` Nullable(Decimal(15,5)),
`raise5minlocalprice` Nullable(Decimal(15,5)),
`raise5minlocalreq` Nullable(Decimal(15,5)),
`raise5minprice` Nullable(Decimal(15,5)),
`raise5minreq` Nullable(Decimal(15,5)),
`raise5minsupplyprice` Nullable(Decimal(15,5)),
`raise60secdispatch` Nullable(Decimal(15,5)),
`raise60secimport` Nullable(Decimal(15,5)),
`raise60seclocaldispatch` Nullable(Decimal(15,5)),
`raise60seclocalprice` Nullable(Decimal(15,5)),
`raise60seclocalreq` Nullable(Decimal(15,5)),
`raise60secprice` Nullable(Decimal(15,5)),
`raise60secreq` Nullable(Decimal(15,5)),
`raise60secsupplyprice` Nullable(Decimal(15,5)),
`raise6secdispatch` Nullable(Decimal(15,5)),
`raise6secimport` Nullable(Decimal(15,5)),
`raise6seclocaldispatch` Nullable(Decimal(15,5)),
`raise6seclocalprice` Nullable(Decimal(15,5)),
`raise6seclocalreq` Nullable(Decimal(15,5)),
`raise6secprice` Nullable(Decimal(15,5)),
`raise6secreq` Nullable(Decimal(15,5)),
`raise6secsupplyprice` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`datetime` DateTime('Australia/Brisbane'),
`initialsupply` Nullable(Decimal(15,5)),
`clearedsupply` Nullable(Decimal(15,5)),
`lowerregimport` Nullable(Decimal(15,5)),
`lowerreglocaldispatch` Nullable(Decimal(15,5)),
`lowerreglocalreq` Nullable(Decimal(15,5)),
`lowerregreq` Nullable(Decimal(15,5)),
`raiseregimport` Nullable(Decimal(15,5)),
`raisereglocaldispatch` Nullable(Decimal(15,5)),
`raisereglocalreq` Nullable(Decimal(15,5)),
`raiseregreq` Nullable(Decimal(15,5)),
`raise5minlocalviolation` Nullable(Decimal(15,5)),
`raisereglocalviolation` Nullable(Decimal(15,5)),
`raise60seclocalviolation` Nullable(Decimal(15,5)),
`raise6seclocalviolation` Nullable(Decimal(15,5)),
`lower5minlocalviolation` Nullable(Decimal(15,5)),
`lowerreglocalviolation` Nullable(Decimal(15,5)),
`lower60seclocalviolation` Nullable(Decimal(15,5)),
`lower6seclocalviolation` Nullable(Decimal(15,5)),
`raise5minviolation` Nullable(Decimal(15,5)),
`raiseregviolation` Nullable(Decimal(15,5)),
`raise60secviolation` Nullable(Decimal(15,5)),
`raise6secviolation` Nullable(Decimal(15,5)),
`lower5minviolation` Nullable(Decimal(15,5)),
`lowerregviolation` Nullable(Decimal(15,5)),
`lower60secviolation` Nullable(Decimal(15,5)),
`lower6secviolation` Nullable(Decimal(15,5)),
`raise6secactualavailability` Nullable(Decimal(16,6)),
`raise60secactualavailability` Nullable(Decimal(16,6)),
`raise5minactualavailability` Nullable(Decimal(16,6)),
`raiseregactualavailability` Nullable(Decimal(16,6)),
`lower6secactualavailability` Nullable(Decimal(16,6)),
`lower60secactualavailability` Nullable(Decimal(16,6)),
`lower5minactualavailability` Nullable(Decimal(16,6)),
`lowerregactualavailability` Nullable(Decimal(16,6)),
`decavailability` Nullable(Decimal(16,6)),
`lorsurplus` Nullable(Decimal(16,6)),
`lrcsurplus` Nullable(Decimal(16,6)),
`totalintermittentgeneration` Nullable(Decimal(15,5)),
`demand_and_nonschedgen` Nullable(Decimal(15,5)),
`uigf` Nullable(Decimal(15,5)),
`semischedule_clearedmw` Nullable(Decimal(15,5)),
`semischedule_compliancemw` Nullable(Decimal(15,5)),
`ss_solar_uigf` Nullable(Decimal(15,5)),
`ss_wind_uigf` Nullable(Decimal(15,5)),
`ss_solar_clearedmw` Nullable(Decimal(15,5)),
`ss_wind_clearedmw` Nullable(Decimal(15,5)),
`ss_solar_compliancemw` Nullable(Decimal(15,5)),
`ss_wind_compliancemw` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `regionid`);
                        
create table PredispatchLocalPrice1 (
file_id Uuid,
`predispatchseqno` String,
`datetime` DateTime('Australia/Brisbane'),
`duid` String,
`periodid` Nullable(String),
`local_price_adjustment` Nullable(Decimal(10,2)),
`locally_constrained` Nullable(Decimal(1,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `duid`);
                        
create table PredispatchConstraintSolution5 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`constraintid` String,
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`rhs` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`datetime` DateTime('Australia/Brisbane'),
`duid` Nullable(String),
`genconid_effectivedate` Nullable(DateTime('Australia/Brisbane')),
`genconid_versionno` Nullable(Decimal(22,0)),
`lhs` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `datetime`);
                        
create table PredispatchScenarioDemand1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`scenario` Decimal(2,0),
`regionid` String,
`deltamw` Nullable(Decimal(4,0))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `regionid`, `scenario`, `versionno`);
                        
create table PredispatchInterconnectorSoln3 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`interconnectorid` String,
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`meteredmwflow` Nullable(Decimal(15,5)),
`mwflow` Nullable(Decimal(15,5)),
`mwlosses` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`datetime` DateTime('Australia/Brisbane'),
`exportlimit` Nullable(Decimal(15,5)),
`importlimit` Nullable(Decimal(15,5)),
`marginalloss` Nullable(Decimal(15,5)),
`exportgenconid` Nullable(String),
`importgenconid` Nullable(String),
`fcasexportlimit` Nullable(Decimal(15,5)),
`fcasimportlimit` Nullable(Decimal(15,5)),
`local_price_adjustment_export` Nullable(Decimal(10,2)),
`locally_constrained_export` Nullable(Decimal(1,0)),
`local_price_adjustment_import` Nullable(Decimal(10,2)),
`locally_constrained_import` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `interconnectorid`);
                        
create table PredispatchInterconnectrSens1 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`interconnectorid` String,
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`datetime` DateTime('Australia/Brisbane'),
`intervention_active` Nullable(Decimal(1,0)),
`mwflow1` Nullable(Decimal(15,5)),
`mwflow2` Nullable(Decimal(15,5)),
`mwflow3` Nullable(Decimal(15,5)),
`mwflow4` Nullable(Decimal(15,5)),
`mwflow5` Nullable(Decimal(15,5)),
`mwflow6` Nullable(Decimal(15,5)),
`mwflow7` Nullable(Decimal(15,5)),
`mwflow8` Nullable(Decimal(15,5)),
`mwflow9` Nullable(Decimal(15,5)),
`mwflow10` Nullable(Decimal(15,5)),
`mwflow11` Nullable(Decimal(15,5)),
`mwflow12` Nullable(Decimal(15,5)),
`mwflow13` Nullable(Decimal(15,5)),
`mwflow14` Nullable(Decimal(15,5)),
`mwflow15` Nullable(Decimal(15,5)),
`mwflow16` Nullable(Decimal(15,5)),
`mwflow17` Nullable(Decimal(15,5)),
`mwflow18` Nullable(Decimal(15,5)),
`mwflow19` Nullable(Decimal(15,5)),
`mwflow20` Nullable(Decimal(15,5)),
`mwflow21` Nullable(Decimal(15,5)),
`mwflow22` Nullable(Decimal(15,5)),
`mwflow23` Nullable(Decimal(15,5)),
`mwflow24` Nullable(Decimal(15,5)),
`mwflow25` Nullable(Decimal(15,5)),
`mwflow26` Nullable(Decimal(15,5)),
`mwflow27` Nullable(Decimal(15,5)),
`mwflow28` Nullable(Decimal(15,5)),
`mwflow29` Nullable(Decimal(15,5)),
`mwflow30` Nullable(Decimal(15,5)),
`mwflow31` Nullable(Decimal(15,5)),
`mwflow32` Nullable(Decimal(15,5)),
`mwflow33` Nullable(Decimal(15,5)),
`mwflow34` Nullable(Decimal(15,5)),
`mwflow35` Nullable(Decimal(15,5)),
`mwflow36` Nullable(Decimal(15,5)),
`mwflow37` Nullable(Decimal(15,5)),
`mwflow38` Nullable(Decimal(15,5)),
`mwflow39` Nullable(Decimal(15,5)),
`mwflow40` Nullable(Decimal(15,5)),
`mwflow41` Nullable(Decimal(15,5)),
`mwflow42` Nullable(Decimal(15,5)),
`mwflow43` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `interconnectorid`);
                        
create table PredispatchMnspbidtrk1 (
file_id Uuid,
`predispatchseqno` String,
`linkid` String,
`periodid` String,
`participantid` Nullable(String),
`settlementdate` Nullable(DateTime('Australia/Brisbane')),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`versionno` Nullable(Decimal(3,0)),
`datetime` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`linkid`, `periodid`, `predispatchseqno`);
                        
create table PredispatchPricesensitivities1 (
file_id Uuid,
`predispatchseqno` Nullable(String),
`runno` Nullable(Decimal(3,0)),
`regionid` String,
`periodid` Nullable(String),
`intervention` Nullable(Decimal(2,0)),
`rrpeep1` Nullable(Decimal(15,5)),
`rrpeep2` Nullable(Decimal(15,5)),
`rrpeep3` Nullable(Decimal(15,5)),
`rrpeep4` Nullable(Decimal(15,5)),
`rrpeep5` Nullable(Decimal(15,5)),
`rrpeep6` Nullable(Decimal(15,5)),
`rrpeep7` Nullable(Decimal(15,5)),
`rrpeep8` Nullable(Decimal(15,5)),
`rrpeep9` Nullable(Decimal(15,5)),
`rrpeep10` Nullable(Decimal(15,5)),
`rrpeep11` Nullable(Decimal(15,5)),
`rrpeep12` Nullable(Decimal(15,5)),
`rrpeep13` Nullable(Decimal(15,5)),
`rrpeep14` Nullable(Decimal(15,5)),
`rrpeep15` Nullable(Decimal(15,5)),
`rrpeep16` Nullable(Decimal(15,5)),
`rrpeep17` Nullable(Decimal(15,5)),
`rrpeep18` Nullable(Decimal(15,5)),
`rrpeep19` Nullable(Decimal(15,5)),
`rrpeep20` Nullable(Decimal(15,5)),
`rrpeep21` Nullable(Decimal(15,5)),
`rrpeep22` Nullable(Decimal(15,5)),
`rrpeep23` Nullable(Decimal(15,5)),
`rrpeep24` Nullable(Decimal(15,5)),
`rrpeep25` Nullable(Decimal(15,5)),
`rrpeep26` Nullable(Decimal(15,5)),
`rrpeep27` Nullable(Decimal(15,5)),
`rrpeep28` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`datetime` DateTime('Australia/Brisbane'),
`rrpeep29` Nullable(Decimal(15,5)),
`rrpeep30` Nullable(Decimal(15,5)),
`rrpeep31` Nullable(Decimal(15,5)),
`rrpeep32` Nullable(Decimal(15,5)),
`rrpeep33` Nullable(Decimal(15,5)),
`rrpeep34` Nullable(Decimal(15,5)),
`rrpeep35` Nullable(Decimal(15,5)),
`intervention_active` Nullable(Decimal(1,0)),
`rrpeep36` Nullable(Decimal(15,5)),
`rrpeep37` Nullable(Decimal(15,5)),
`rrpeep38` Nullable(Decimal(15,5)),
`rrpeep39` Nullable(Decimal(15,5)),
`rrpeep40` Nullable(Decimal(15,5)),
`rrpeep41` Nullable(Decimal(15,5)),
`rrpeep42` Nullable(Decimal(15,5)),
`rrpeep43` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`datetime`, `regionid`);
                        
create table PrudentialRuntrk1 (
file_id Uuid,
`prudential_date` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`prudential_date`, `runno`);
                        
create table PrudentialCompanyPosition1 (
file_id Uuid,
`prudential_date` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`company_id` String,
`mcl` Nullable(Decimal(16,6)),
`credit_support` Nullable(Decimal(16,6)),
`trading_limit` Nullable(Decimal(16,6)),
`current_amount_balance` Nullable(Decimal(16,6)),
`security_deposit_provision` Nullable(Decimal(16,6)),
`security_deposit_offset` Nullable(Decimal(16,6)),
`security_deposit_balance` Nullable(Decimal(16,6)),
`expost_realloc_balance` Nullable(Decimal(16,6)),
`default_balance` Nullable(Decimal(16,6)),
`outstandings` Nullable(Decimal(16,6)),
`trading_margin` Nullable(Decimal(16,6)),
`typical_accrual` Nullable(Decimal(16,6)),
`prudential_margin` Nullable(Decimal(16,6)),
`early_payment_amount` Nullable(Decimal(18,8)),
`percentage_outstandings` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`company_id`, `prudential_date`, `runno`);
                        
create table PdpasaCasesolution3 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`pasaversion` Nullable(String),
`reservecondition` Nullable(Decimal(1,0)),
`lorcondition` Nullable(Decimal(1,0)),
`capacityobjfunction` Nullable(Decimal(12,3)),
`capacityoption` Nullable(Decimal(12,3)),
`maxsurplusreserveoption` Nullable(Decimal(12,3)),
`maxsparecapacityoption` Nullable(Decimal(12,3)),
`interconnectorflowpenalty` Nullable(Decimal(12,3)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`reliabilitylrcdemandoption` Nullable(Decimal(12,3)),
`outagelrcdemandoption` Nullable(Decimal(12,3)),
`lordemandoption` Nullable(Decimal(12,3)),
`reliabilitylrccapacityoption` Nullable(String),
`outagelrccapacityoption` Nullable(String),
`lorcapacityoption` Nullable(String),
`loruigf_option` Nullable(Decimal(3,0)),
`reliability_lrcuigf_option` Nullable(Decimal(3,0)),
`outage_lrcuigf_option` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`run_datetime`);
                        
create table PdpasaRegionsolution5 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`regionid` String,
`demand10` Nullable(Decimal(12,2)),
`demand50` Nullable(Decimal(12,2)),
`demand90` Nullable(Decimal(12,2)),
`reservereq` Nullable(Decimal(12,2)),
`capacityreq` Nullable(Decimal(12,2)),
`energyreqdemand50` Nullable(Decimal(12,2)),
`unconstrainedcapacity` Nullable(Decimal(12,0)),
`constrainedcapacity` Nullable(Decimal(12,0)),
`netinterchangeunderscarcity` Nullable(Decimal(12,2)),
`surpluscapacity` Nullable(Decimal(12,2)),
`surplusreserve` Nullable(Decimal(12,2)),
`reservecondition` Nullable(Decimal(1,0)),
`maxsurplusreserve` Nullable(Decimal(12,2)),
`maxsparecapacity` Nullable(Decimal(12,2)),
`lorcondition` Nullable(Decimal(1,0)),
`aggregatecapacityavailable` Nullable(Decimal(12,2)),
`aggregatescheduledload` Nullable(Decimal(12,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`aggregatepasaavailability` Nullable(Decimal(12,0)),
`runtype` String,
`energyreqdemand10` Nullable(Decimal(12,2)),
`calculatedlor1level` Nullable(Decimal(16,6)),
`calculatedlor2level` Nullable(Decimal(16,6)),
`msrnetinterchangeunderscarcity` Nullable(Decimal(12,2)),
`lornetinterchangeunderscarcity` Nullable(Decimal(12,2)),
`totalintermittentgeneration` Nullable(Decimal(15,5)),
`demand_and_nonschedgen` Nullable(Decimal(15,5)),
`uigf` Nullable(Decimal(12,2)),
`semi_scheduled_capacity` Nullable(Decimal(12,2)),
`lor_semi_scheduled_capacity` Nullable(Decimal(12,2)),
`lcr` Nullable(Decimal(16,6)),
`lcr2` Nullable(Decimal(16,6)),
`fum` Nullable(Decimal(16,6)),
`ss_solar_uigf` Nullable(Decimal(12,2)),
`ss_wind_uigf` Nullable(Decimal(12,2)),
`ss_solar_capacity` Nullable(Decimal(12,2)),
`ss_wind_capacity` Nullable(Decimal(12,2)),
`ss_solar_cleared` Nullable(Decimal(12,2)),
`ss_wind_cleared` Nullable(Decimal(12,2))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`, `run_datetime`, `runtype`);
                        
create table MeterdataIndividualReads1 (
file_id Uuid,
`case_id` Decimal(15,0),
`settlementdate` DateTime('Australia/Brisbane'),
`meter_id` String,
`meter_id_suffix` String,
`frmp` String,
`lr` String,
`periodid` Decimal(3,0),
`connectionpointid` String,
`meter_type` String,
`importvalue` Decimal(18,8),
`exportvalue` Decimal(18,8),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`case_id`, `meter_id`, `meter_id_suffix`, `periodid`, `settlementdate`);
                        
create table MeterdataTrk1 (
file_id Uuid,
`case_id` Decimal(15,0),
`aggregate_reads_load_datetime` Nullable(DateTime('Australia/Brisbane')),
`individual_reads_load_datetime` Nullable(DateTime('Australia/Brisbane')),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`case_id`);
                        
create table MeterdataInterconnector1 (
file_id Uuid,
`case_id` Decimal(15,0),
`settlementdate` DateTime('Australia/Brisbane'),
`interconnectorid` String,
`periodid` Decimal(3,0),
`importvalue` Nullable(Decimal(18,8)),
`exportvalue` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`case_id`, `interconnectorid`, `periodid`, `settlementdate`);
                        
create table MeterdataAggregateReads1 (
file_id Uuid,
`case_id` Decimal(15,0),
`settlementdate` DateTime('Australia/Brisbane'),
`connectionpointid` String,
`meter_type` String,
`frmp` String,
`lr` String,
`periodid` Decimal(3,0),
`importvalue` Decimal(18,8),
`exportvalue` Decimal(18,8),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`case_id`, `connectionpointid`, `frmp`, `lr`, `meter_type`, `periodid`, `settlementdate`);
                        
create table AsofferOfferlsheddata1 (
file_id Uuid,
`contractid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`availableload` Nullable(Decimal(4,0)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`filename` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`periodid` Decimal(3,0)
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `effectivedate`, `periodid`, `versionno`);
                        
create table AsofferOfferrpowerdata1 (
file_id Uuid,
`contractid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`availability` Nullable(Decimal(3,0)),
`mta` Nullable(Decimal(6,0)),
`mtg` Nullable(Decimal(6,0)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`filename` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `effectivedate`, `periodid`, `versionno`);
                        
create table AsofferOfferastrk1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`filename` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`, `versionno`);
                        
create table AsofferOfferrestartdata1 (
file_id Uuid,
`contractid` String,
`offerdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`availability` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`filename` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`periodid` Decimal(3,0)
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `offerdate`, `periodid`, `versionno`);
                        
create table AsofferOfferagcdata1 (
file_id Uuid,
`contractid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`availability` Nullable(Decimal(4,0)),
`upperlimit` Nullable(Decimal(4,0)),
`lowerlimit` Nullable(Decimal(4,0)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`filename` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`periodid` Decimal(3,0),
`agcup` Nullable(Decimal(3,0)),
`agcdown` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `effectivedate`, `periodid`, `versionno`);
                        
create table IrauctionSraFinancialAucMargin1 (
file_id Uuid,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`sra_runno` Decimal(3,0),
`participantid` String,
`total_cash_security` Nullable(Decimal(18,8)),
`required_margin` Nullable(Decimal(18,8)),
`returned_margin` Nullable(Decimal(18,8)),
`returned_margin_interest` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `sra_quarter`, `sra_runno`, `sra_year`);
                        
create table IrauctionResidueConData2 (
file_id Uuid,
`contractid` String,
`versionno` Decimal(3,0),
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`unitspurchased` Nullable(Decimal(17,5)),
`linkpayment` Nullable(Decimal(17,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`secondary_units_sold` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `participantid`, `versionno`);
                        
create table IrauctionConfigAuctionRevenueEstimate1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`valuationid` String,
`versionno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`monthno` Decimal(1,0),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`revenue` Nullable(Decimal(17,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `fromregionid`, `interconnectorid`, `monthno`, `quarter`, `valuationid`, `versionno`);
                        
create table IrauctionConfigAuctionTranche1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`versionno` Decimal(3,0),
`tranche` Decimal(2,0),
`auctiondate` Nullable(DateTime('Australia/Brisbane')),
`notifydate` Nullable(DateTime('Australia/Brisbane')),
`unitallocation` Nullable(Decimal(18,8)),
`changedate` Nullable(DateTime('Australia/Brisbane')),
`changedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `quarter`, `tranche`, `versionno`);
                        
create table IrauctionConfigAuctionRevenueTrack1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`valuationid` String,
`versionno` Decimal(3,0),
`effectivedate` Nullable(DateTime('Australia/Brisbane')),
`status` Nullable(String),
`documentref` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `quarter`, `valuationid`, `versionno`);
                        
create table IrauctionSraFinancialAucReceipts1 (
file_id Uuid,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`sra_runno` Decimal(3,0),
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`contractid` String,
`units_purchased` Nullable(Decimal(18,8)),
`clearing_price` Nullable(Decimal(18,8)),
`receipt_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`proceeds_amount` Nullable(Decimal(18,8)),
`units_sold` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `participantid`, `sra_quarter`, `sra_runno`, `sra_year`);
                        
create table IrauctionSraFinancialAucpaySum1 (
file_id Uuid,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`sra_runno` Decimal(3,0),
`participantid` String,
`gross_proceeds_amount` Nullable(Decimal(18,8)),
`total_gross_proceeds_amount` Nullable(Decimal(18,8)),
`shortfall_amount` Nullable(Decimal(18,8)),
`total_shortfall_amount` Nullable(Decimal(18,8)),
`net_payment_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `sra_quarter`, `sra_runno`, `sra_year`);
                        
create table IrauctionSraFinancialAucMardetail1 (
file_id Uuid,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`sra_runno` Decimal(3,0),
`participantid` String,
`cash_security_id` String,
`returned_amount` Nullable(Decimal(18,8)),
`returned_interest` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`cash_security_id`, `participantid`, `sra_quarter`, `sra_runno`, `sra_year`);
                        
create table IrauctionSraPrudentialExposure1 (
file_id Uuid,
`prudential_date` DateTime('Australia/Brisbane'),
`prudential_runno` Decimal(8,0),
`participantid` String,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`max_tranche` Nullable(Decimal(2,0)),
`auctionid` Nullable(String),
`offer_submissiontime` Nullable(DateTime('Australia/Brisbane')),
`average_purchase_price` Nullable(Decimal(18,8)),
`average_cancellation_price` Nullable(Decimal(18,8)),
`cancellation_volume` Nullable(Decimal(18,8)),
`trading_position` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`fromregionid`, `interconnectorid`, `participantid`, `prudential_date`, `prudential_runno`, `sra_quarter`, `sra_year`);
                        
create table IrauctionResidueBidTrk1 (
file_id Uuid,
`contractid` Nullable(String),
`versionno` Decimal(3,0),
`participantid` String,
`bidloaddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`auctionid` String
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `participantid`, `versionno`);
                        
create table IrauctionSraCashSecurity1 (
file_id Uuid,
`cash_security_id` String,
`participantid` Nullable(String),
`provision_date` Nullable(DateTime('Australia/Brisbane')),
`cash_amount` Nullable(Decimal(18,8)),
`interest_acct_id` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`finalreturndate` Nullable(DateTime('Australia/Brisbane')),
`cash_security_returned` Nullable(Decimal(18,8)),
`deletiondate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`cash_security_id`);
                        
create table IrauctionResidueTrk1 (
file_id Uuid,
`contractid` Nullable(String),
`versionno` Decimal(3,0),
`rundate` Nullable(DateTime('Australia/Brisbane')),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`postdate` Nullable(DateTime('Australia/Brisbane')),
`postedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`status` Nullable(String),
`auctionid` String
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `versionno`);
                        
create table IrauctionSraOfferProfile1 (
file_id Uuid,
`auctionid` String,
`participantid` String,
`loaddate` DateTime('Australia/Brisbane'),
`filename` Nullable(String),
`ackfilename` Nullable(String),
`transactionid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `loaddate`, `participantid`);
                        
create table IrauctionSraFinancialRuntrk1 (
file_id Uuid,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`sra_runno` Decimal(3,0),
`runtype` Nullable(String),
`rundate` Nullable(DateTime('Australia/Brisbane')),
`posteddate` Nullable(DateTime('Australia/Brisbane')),
`interest_versionno` Nullable(Decimal(3,0)),
`makeup_versionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`sra_quarter`, `sra_runno`, `sra_year`);
                        
create table IrauctionSraPrudentialCompPosition1 (
file_id Uuid,
`prudential_date` DateTime('Australia/Brisbane'),
`prudential_runno` Decimal(8,0),
`participantid` String,
`trading_limit` Nullable(Decimal(18,8)),
`prudential_exposure_amount` Nullable(Decimal(18,8)),
`trading_margin` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `prudential_date`, `prudential_runno`);
                        
create table IrauctionSraPrudentialRun1 (
file_id Uuid,
`prudential_date` DateTime('Australia/Brisbane'),
`prudential_runno` Decimal(8,0)
)
ENGINE = MergeTree()
ORDER BY (`prudential_date`, `prudential_runno`);
                        
create table IrauctionBidsFileTrk1 (
file_id Uuid,
`contractid` Nullable(String),
`participantid` String,
`loaddate` DateTime('Australia/Brisbane'),
`filename` Nullable(String),
`ackfilename` Nullable(String),
`status` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`auctionid` String
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `loaddate`, `participantid`);
                        
create table IrauctionSraFinancialAucpayDetail1 (
file_id Uuid,
`sra_year` Decimal(4,0),
`sra_quarter` Decimal(3,0),
`sra_runno` Decimal(3,0),
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`contractid` String,
`maximum_units` Nullable(Decimal(18,8)),
`units_sold` Nullable(Decimal(18,8)),
`shortfall_units` Nullable(Decimal(18,8)),
`reserve_price` Nullable(Decimal(18,8)),
`clearing_price` Nullable(Decimal(18,8)),
`payment_amount` Nullable(Decimal(18,8)),
`shortfall_amount` Nullable(Decimal(18,8)),
`allocation` Nullable(Decimal(18,8)),
`net_payment_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `participantid`, `sra_quarter`, `sra_runno`, `sra_year`);
                        
create table IrauctionSraPrudentialCashSecurity1 (
file_id Uuid,
`prudential_date` DateTime('Australia/Brisbane'),
`prudential_runno` Decimal(8,0),
`participantid` String,
`cash_security_id` String,
`cash_security_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`cash_security_id`, `participantid`, `prudential_date`, `prudential_runno`);
                        
create table IrauctionResidueContracts1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`tranche` Decimal(2,0),
`contractid` Nullable(String),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`notifydate` Nullable(DateTime('Australia/Brisbane')),
`auctiondate` Nullable(DateTime('Australia/Brisbane')),
`calcmethod` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`notifypostdate` Nullable(DateTime('Australia/Brisbane')),
`notifyby` Nullable(String),
`postdate` Nullable(DateTime('Australia/Brisbane')),
`postedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`description` Nullable(String),
`auctionid` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `quarter`, `tranche`);
                        
create table IrauctionValuationid1 (
file_id Uuid,
`valuationid` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`valuationid`);
                        
create table IrauctionConfigAuctionRpEstimate1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`valuationid` String,
`versionno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`rpestimate` Nullable(Decimal(17,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `fromregionid`, `interconnectorid`, `quarter`, `valuationid`, `versionno`);
                        
create table IrauctionResidueConFunds1 (
file_id Uuid,
`contractid` String,
`interconnectorid` String,
`fromregionid` String,
`defaultunits` Nullable(Decimal(5,0)),
`rolloverunits` Nullable(Decimal(5,0)),
`reallocatedunits` Nullable(Decimal(5,0)),
`unitsoffered` Nullable(Decimal(5,0)),
`meanreserveprice` Nullable(Decimal(9,2)),
`scalefactor` Nullable(Decimal(8,5)),
`actualreserveprice` Nullable(Decimal(9,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`);
                        
create table IrauctionBidsFundsBid1 (
file_id Uuid,
`contractid` String,
`participantid` String,
`loaddate` DateTime('Australia/Brisbane'),
`optionid` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`units` Nullable(Decimal(5,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `loaddate`, `optionid`, `participantid`);
                        
create table SettlementConfigResiduecontractpayments1 (
file_id Uuid,
`contractid` String,
`participantid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `participantid`);
                        
create table IrauctionSraOfferProduct1 (
file_id Uuid,
`auctionid` String,
`participantid` String,
`loaddate` DateTime('Australia/Brisbane'),
`optionid` Decimal(4,0),
`interconnectorid` Nullable(String),
`fromregionid` Nullable(String),
`offer_quantity` Nullable(Decimal(5,0)),
`offer_price` Nullable(Decimal(18,8)),
`trancheid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `loaddate`, `optionid`, `participantid`);
                        
create table IrauctionResiduePriceFundsBid1 (
file_id Uuid,
`contractid` String,
`interconnectorid` String,
`fromregionid` String,
`units` Nullable(Decimal(5,0)),
`bidprice` Nullable(Decimal(17,5)),
`linkedbidflag` Decimal(6,0),
`auctionid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `contractid`, `fromregionid`, `interconnectorid`, `linkedbidflag`);
                        
create table IrauctionConfigAuctionIcAllocations2 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`versionno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`maximumunits` Nullable(Decimal(5,0)),
`proportion` Nullable(Decimal(8,5)),
`auctionfee` Nullable(Decimal(17,5)),
`changedate` Nullable(DateTime('Australia/Brisbane')),
`changedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`auctionfee_sales` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `fromregionid`, `interconnectorid`, `quarter`, `versionno`);
                        
create table IrauctionConfigAuctionCalendar2 (
file_id Uuid,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`notifydate` Nullable(DateTime('Australia/Brisbane')),
`paymentdate` Nullable(DateTime('Australia/Brisbane')),
`reconciliationdate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`prelimpurchasestmtdate` Nullable(DateTime('Australia/Brisbane')),
`prelimproceedsstmtdate` Nullable(DateTime('Australia/Brisbane')),
`finalpurchasestmtdate` Nullable(DateTime('Australia/Brisbane')),
`finalproceedsstmtdate` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `quarter`);
                        
create table IrauctionResiduePublicData1 (
file_id Uuid,
`contractid` String,
`versionno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`unitsoffered` Nullable(Decimal(5,0)),
`unitssold` Nullable(Decimal(16,6)),
`clearingprice` Nullable(Decimal(17,5)),
`reserveprice` Nullable(Decimal(17,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `fromregionid`, `interconnectorid`, `versionno`);
                        
create table IrauctionResidueConEstimatesTrk1 (
file_id Uuid,
`contractid` String,
`contractyear` Decimal(4,0),
`quarter` Decimal(1,0),
`valuationid` String,
`versionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractid`, `contractyear`, `quarter`, `valuationid`);
                        
create table IrauctionBidsPriceBid1 (
file_id Uuid,
`contractid` Nullable(String),
`participantid` String,
`loaddate` DateTime('Australia/Brisbane'),
`optionid` Decimal(3,0),
`bidprice` Nullable(Decimal(17,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`auctionid` String
)
ENGINE = MergeTree()
ORDER BY (`auctionid`, `loaddate`, `optionid`, `participantid`);
                        
create table IrauctionConfigAuction1 (
file_id Uuid,
`auctionid` String,
`auctiondate` Nullable(DateTime('Australia/Brisbane')),
`notifydate` Nullable(DateTime('Australia/Brisbane')),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`description` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`auctionid`);
                        
create table ForceMajeureMarketSuspendSchedule1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`day_type` String,
`regionid` String,
`periodid` Decimal(3,0),
`energy_rrp` Nullable(Decimal(15,5)),
`r6_rrp` Nullable(Decimal(15,5)),
`r60_rrp` Nullable(Decimal(15,5)),
`r5_rrp` Nullable(Decimal(15,5)),
`rreg_rrp` Nullable(Decimal(15,5)),
`l6_rrp` Nullable(Decimal(15,5)),
`l60_rrp` Nullable(Decimal(15,5)),
`l5_rrp` Nullable(Decimal(15,5)),
`lreg_rrp` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`day_type`, `effectivedate`, `periodid`, `regionid`);
                        
create table ForceMajeureMarketSuspendScheduleTrk1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`source_start_date` Nullable(DateTime('Australia/Brisbane')),
`source_end_date` Nullable(DateTime('Australia/Brisbane')),
`comments` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`);
                        
create table ForceMajeureOverriderrp1 (
file_id Uuid,
`regionid` String,
`startdate` DateTime('Australia/Brisbane'),
`startperiod` Decimal(3,0),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`endperiod` Nullable(Decimal(3,0)),
`rrp` Nullable(Decimal(15,0)),
`description` Nullable(String),
`authorisestart` Nullable(String),
`authoriseend` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`regionid`, `startdate`, `startperiod`);
                        
create table ApApeventregion1 (
file_id Uuid,
`apeventid` Decimal(22,0),
`regionid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`energyapflag` Nullable(Decimal(1,0)),
`raise6secapflag` Nullable(Decimal(1,0)),
`raise60secapflag` Nullable(Decimal(1,0)),
`raise5minapflag` Nullable(Decimal(1,0)),
`raiseregapflag` Nullable(Decimal(1,0)),
`lower6secapflag` Nullable(Decimal(1,0)),
`lower60secapflag` Nullable(Decimal(1,0)),
`lower5minapflag` Nullable(Decimal(1,0)),
`lowerregapflag` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`apeventid`, `regionid`);
                        
create table ForceMajeureIrfmevents1 (
file_id Uuid,
`irfmid` String,
`startdate` Nullable(DateTime('Australia/Brisbane')),
`startperiod` Nullable(Decimal(3,0)),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`endperiod` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`irfmid`);
                        
create table ApApevent1 (
file_id Uuid,
`apeventid` Decimal(22,0),
`effectivefrominterval` Nullable(DateTime('Australia/Brisbane')),
`effectivetointerval` Nullable(DateTime('Australia/Brisbane')),
`reason` Nullable(String),
`startauthorisedby` Nullable(String),
`startauthoriseddate` Nullable(DateTime('Australia/Brisbane')),
`endauthorisedby` Nullable(String),
`endauthoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`apeventid`);
                        
create table ForceMajeureIrfmamount1 (
file_id Uuid,
`irfmid` String,
`effectivedate` Nullable(DateTime('Australia/Brisbane')),
`versionno` Decimal(3,0),
`periodid` Decimal(4,0),
`amount` Nullable(Decimal(15,5)),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`irfmid`, `periodid`, `versionno`);
                        
create table ApRegionapc1 (
file_id Uuid,
`regionid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `regionid`, `versionno`);
                        
create table ForceMajeureMarketSuspendRegionSum1 (
file_id Uuid,
`suspension_id` String,
`regionid` String,
`initial_interval` Nullable(DateTime('Australia/Brisbane')),
`end_region_interval` Nullable(DateTime('Australia/Brisbane')),
`end_suspension_interval` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`regionid`, `suspension_id`);
                        
create table ApRegionapcintervals1 (
file_id Uuid,
`regionid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`periodid` Decimal(3,0),
`apcvalue` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`apctype` Nullable(Decimal(3,0)),
`fcasapcvalue` Nullable(Decimal(16,6)),
`apfvalue` Nullable(Decimal(16,6))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `periodid`, `regionid`, `versionno`);
                        
create table ForceMajeureMarketSuspendRegimeSum1 (
file_id Uuid,
`suspension_id` String,
`regionid` String,
`start_interval` DateTime('Australia/Brisbane'),
`end_interval` Nullable(DateTime('Australia/Brisbane')),
`pricing_regime` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`regionid`, `start_interval`, `suspension_id`);
                        
create table MarketConfigInterconnectoralloc1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(5,0),
`interconnectorid` String,
`regionid` String,
`participantid` String,
`allocation` Nullable(Decimal(12,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `interconnectorid`, `participantid`, `regionid`, `versionno`);
                        
create table MarketConfigIntraregionalloc1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(5,0),
`regionid` String,
`participantid` String,
`allocation` Nullable(Decimal(12,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`, `regionid`, `versionno`);
                        
create table MarketConfigLossmodel1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`interconnectorid` String,
`periodid` Nullable(String),
`losssegment` Decimal(6,0),
`mwbreakpoint` Nullable(Decimal(6,0)),
`lossfactor` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `interconnectorid`, `losssegment`, `versionno`);
                        
create table MarketConfigRegion1 (
file_id Uuid,
`regionid` String,
`description` Nullable(String),
`regionstatus` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`regionid`);
                        
create table MarketConfigInterconnector1 (
file_id Uuid,
`interconnectorid` String,
`regionfrom` Nullable(String),
`rsoid` Nullable(String),
`regionto` Nullable(String),
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`interconnectorid`);
                        
create table MarketConfigRegionstandingdata1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`regionid` String,
`rsoid` Nullable(String),
`regionalreferencepointid` Nullable(String),
`peaktradingperiod` Nullable(Decimal(3,0)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`scalingfactor` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `regionid`, `versionno`);
                        
create table MarketConfigInterconnectorconstraint1 (
file_id Uuid,
`reserveoverallloadfactor` Nullable(Decimal(5,2)),
`fromregionlossshare` Nullable(Decimal(5,2)),
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`interconnectorid` String,
`maxmwin` Nullable(Decimal(15,5)),
`maxmwout` Nullable(Decimal(15,5)),
`lossconstant` Nullable(Decimal(15,6)),
`lossflowcoefficient` Nullable(Decimal(27,17)),
`emsmeasurand` Nullable(String),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`dynamicrhs` Nullable(String),
`importlimit` Nullable(Decimal(6,0)),
`exportlimit` Nullable(Decimal(6,0)),
`outagederationfactor` Nullable(Decimal(15,5)),
`nonphysicallossfactor` Nullable(Decimal(15,5)),
`overloadfactor60sec` Nullable(Decimal(15,5)),
`overloadfactor6sec` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`fcassupportunavailable` Nullable(Decimal(1,0)),
`ictype` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `interconnectorid`, `versionno`);
                        
create table MarketConfigLossfactormodel1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`interconnectorid` String,
`regionid` String,
`demandcoefficient` Nullable(Decimal(27,17)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `interconnectorid`, `regionid`, `versionno`);
                        
create table MarketConfigBidtypes1 (
file_id Uuid,
`bidtype` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`description` Nullable(String),
`numberofbands` Nullable(Decimal(3,0)),
`numdaysaheadpricelocked` Nullable(Decimal(2,0)),
`validationrule` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`spdalias` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `effectivedate`, `versionno`);
                        
create table MarketConfigMarketPriceThresholds1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(4,0),
`voll` Nullable(Decimal(15,5)),
`marketpricefloor` Nullable(Decimal(15,5)),
`administered_price_threshold` Nullable(Decimal(15,5)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `versionno`);
                        
create table MarketConfigBidtypestrk1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `versionno`);
                        
create table MarketConfigTransmissionlossfactor2 (
file_id Uuid,
`transmissionlossfactor` Decimal(15,5),
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(22,0),
`connectionpointid` String,
`regionid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`secondary_tlf` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`connectionpointid`, `effectivedate`, `versionno`);
                        
create table GdInstructInstructionsubtype1 (
file_id Uuid,
`instructiontypeid` String,
`instructionsubtypeid` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`instructionsubtypeid`, `instructiontypeid`);
                        
create table GdInstructInstructiontype1 (
file_id Uuid,
`instructiontypeid` String,
`description` Nullable(String),
`regionid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`instructiontypeid`);
                        
create table GdInstructGdinstruct1 (
file_id Uuid,
`duid` Nullable(String),
`stationid` Nullable(String),
`regionid` Nullable(String),
`id` Decimal(22,0),
`instructiontypeid` Nullable(String),
`instructionsubtypeid` Nullable(String),
`instructionclassid` Nullable(String),
`reason` Nullable(String),
`instlevel` Nullable(Decimal(6,0)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`participantid` Nullable(String),
`issuedtime` Nullable(DateTime('Australia/Brisbane')),
`targettime` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`id`);
                        
create table SettlementConfigMarketfeetrk1 (
file_id Uuid,
`marketfeeversionno` Decimal(3,0),
`effectivedate` DateTime('Australia/Brisbane'),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `marketfeeversionno`);
                        
create table SettlementConfigMarketfee1 (
file_id Uuid,
`marketfeeid` String,
`marketfeeperiod` Nullable(String),
`marketfeetype` Nullable(String),
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`gl_tcode` Nullable(String),
`gl_financialcode` Nullable(String),
`fee_class` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`marketfeeid`);
                        
create table SetcfgReallocation2 (
file_id Uuid,
`reallocationid` String,
`creditparticipantid` Nullable(String),
`debitparticipantid` Nullable(String),
`regionid` Nullable(String),
`agreementtype` Nullable(String),
`creditreference` Nullable(String),
`debitreference` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`current_stepid` Nullable(String),
`daytype` Nullable(String),
`reallocation_type` Nullable(String),
`calendarid` Nullable(String),
`intervallength` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`reallocationid`);
                        
create table SettlementConfigSetcfgParticipantMpf1 (
file_id Uuid,
`participantid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantcategoryid` String,
`connectionpointid` String,
`mpf` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`connectionpointid`, `effectivedate`, `participantcategoryid`, `participantid`, `versionno`);
                        
create table SettlementConfigAncillaryRecoverySplit1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`service` String,
`paymenttype` String,
`customer_portion` Nullable(Decimal(8,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `paymenttype`, `service`, `versionno`);
                        
create table SettlementConfigMarketfeedata1 (
file_id Uuid,
`marketfeeid` String,
`marketfeeversionno` Decimal(3,0),
`effectivedate` DateTime('Australia/Brisbane'),
`marketfeevalue` Nullable(Decimal(22,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `marketfeeid`, `marketfeeversionno`);
                        
create table SetcfgReallocationinterval1 (
file_id Uuid,
`reallocationid` String,
`periodid` Decimal(3,0),
`value` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`nrp` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `reallocationid`);
                        
create table SettlementConfigMarketFeeExclusion1 (
file_id Uuid,
`participantid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`marketfeeid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `marketfeeid`, `participantid`, `versionno`);
                        
create table SettlementConfigMarketFeeCatExclTrk1 (
file_id Uuid,
`marketfeeid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`version_datetime` DateTime('Australia/Brisbane'),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `marketfeeid`, `version_datetime`);
                        
create table SettlementConfigMarketFeeCatExcl1 (
file_id Uuid,
`marketfeeid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`version_datetime` DateTime('Australia/Brisbane'),
`participant_categoryid` String
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `marketfeeid`, `participant_categoryid`, `version_datetime`);
                        
create table SettlementConfigParticipantBandfeeAlloc1 (
file_id Uuid,
`participantid` String,
`marketfeeid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantcategoryid` String,
`marketfeevalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `marketfeeid`, `participantcategoryid`, `participantid`, `versionno`);
                        
create table SettlementConfigSetcfgParticipantMpftrk1 (
file_id Uuid,
`participantid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`, `versionno`);
                        
create table SettlementConfigMarketFeeExclusionTrk1 (
file_id Uuid,
`participantid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `participantid`, `versionno`);
                        
create table TradingUnitSolution2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`duid` String,
`tradetype` Decimal(2,0),
`periodid` Decimal(3,0),
`initialmw` Nullable(Decimal(15,5)),
`totalcleared` Nullable(Decimal(15,5)),
`rampdownrate` Nullable(Decimal(15,5)),
`rampuprate` Nullable(Decimal(15,5)),
`lower5min` Nullable(Decimal(15,5)),
`lower60sec` Nullable(Decimal(15,5)),
`lower6sec` Nullable(Decimal(15,5)),
`raise5min` Nullable(Decimal(15,5)),
`raise60sec` Nullable(Decimal(15,5)),
`raise6sec` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`lowerreg` Nullable(Decimal(15,5)),
`raisereg` Nullable(Decimal(15,5)),
`availability` Nullable(Decimal(15,5)),
`semidispatchcap` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `periodid`, `runno`, `settlementdate`, `tradetype`);
                        
create table TradingPrice2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`regionid` String,
`periodid` Decimal(3,0),
`rrp` Nullable(Decimal(15,5)),
`eep` Nullable(Decimal(15,5)),
`invalidflag` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`rop` Nullable(Decimal(15,5)),
`raise6secrrp` Nullable(Decimal(15,5)),
`raise6secrop` Nullable(Decimal(15,5)),
`raise60secrrp` Nullable(Decimal(15,5)),
`raise60secrop` Nullable(Decimal(15,5)),
`raise5minrrp` Nullable(Decimal(15,5)),
`raise5minrop` Nullable(Decimal(15,5)),
`raiseregrrp` Nullable(Decimal(15,5)),
`raiseregrop` Nullable(Decimal(15,5)),
`lower6secrrp` Nullable(Decimal(15,5)),
`lower6secrop` Nullable(Decimal(15,5)),
`lower60secrrp` Nullable(Decimal(15,5)),
`lower60secrop` Nullable(Decimal(15,5)),
`lower5minrrp` Nullable(Decimal(15,5)),
`lower5minrop` Nullable(Decimal(15,5)),
`lowerregrrp` Nullable(Decimal(15,5)),
`lowerregrop` Nullable(Decimal(15,5)),
`price_status` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `regionid`, `runno`, `settlementdate`);
                        
create table TradingInterconnectorres2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`interconnectorid` String,
`periodid` Decimal(3,0),
`meteredmwflow` Nullable(Decimal(15,5)),
`mwflow` Nullable(Decimal(15,5)),
`mwlosses` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`interconnectorid`, `periodid`, `runno`, `settlementdate`);
                        
create table TradingRegionsum4 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`regionid` String,
`periodid` Decimal(3,0),
`totaldemand` Nullable(Decimal(15,5)),
`availablegeneration` Nullable(Decimal(15,5)),
`availableload` Nullable(Decimal(15,5)),
`demandforecast` Nullable(Decimal(15,5)),
`dispatchablegeneration` Nullable(Decimal(15,5)),
`dispatchableload` Nullable(Decimal(15,5)),
`netinterchange` Nullable(Decimal(15,5)),
`excessgeneration` Nullable(Decimal(15,5)),
`lower5mindispatch` Nullable(Decimal(15,5)),
`lower5minimport` Nullable(Decimal(15,5)),
`lower5minlocaldispatch` Nullable(Decimal(15,5)),
`lower5minlocalprice` Nullable(Decimal(15,5)),
`lower5minlocalreq` Nullable(Decimal(15,5)),
`lower5minprice` Nullable(Decimal(15,5)),
`lower5minreq` Nullable(Decimal(15,5)),
`lower5minsupplyprice` Nullable(Decimal(15,5)),
`lower60secdispatch` Nullable(Decimal(15,5)),
`lower60secimport` Nullable(Decimal(15,5)),
`lower60seclocaldispatch` Nullable(Decimal(15,5)),
`lower60seclocalprice` Nullable(Decimal(15,5)),
`lower60seclocalreq` Nullable(Decimal(15,5)),
`lower60secprice` Nullable(Decimal(15,5)),
`lower60secreq` Nullable(Decimal(15,5)),
`lower60secsupplyprice` Nullable(Decimal(15,5)),
`lower6secdispatch` Nullable(Decimal(15,5)),
`lower6secimport` Nullable(Decimal(15,5)),
`lower6seclocaldispatch` Nullable(Decimal(15,5)),
`lower6seclocalprice` Nullable(Decimal(15,5)),
`lower6seclocalreq` Nullable(Decimal(15,5)),
`lower6secprice` Nullable(Decimal(15,5)),
`lower6secreq` Nullable(Decimal(15,5)),
`lower6secsupplyprice` Nullable(Decimal(15,5)),
`raise5mindispatch` Nullable(Decimal(15,5)),
`raise5minimport` Nullable(Decimal(15,5)),
`raise5minlocaldispatch` Nullable(Decimal(15,5)),
`raise5minlocalprice` Nullable(Decimal(15,5)),
`raise5minlocalreq` Nullable(Decimal(15,5)),
`raise5minprice` Nullable(Decimal(15,5)),
`raise5minreq` Nullable(Decimal(15,5)),
`raise5minsupplyprice` Nullable(Decimal(15,5)),
`raise60secdispatch` Nullable(Decimal(15,5)),
`raise60secimport` Nullable(Decimal(15,5)),
`raise60seclocaldispatch` Nullable(Decimal(15,5)),
`raise60seclocalprice` Nullable(Decimal(15,5)),
`raise60seclocalreq` Nullable(Decimal(15,5)),
`raise60secprice` Nullable(Decimal(15,5)),
`raise60secreq` Nullable(Decimal(15,5)),
`raise60secsupplyprice` Nullable(Decimal(15,5)),
`raise6secdispatch` Nullable(Decimal(15,5)),
`raise6secimport` Nullable(Decimal(15,5)),
`raise6seclocaldispatch` Nullable(Decimal(15,5)),
`raise6seclocalprice` Nullable(Decimal(15,5)),
`raise6seclocalreq` Nullable(Decimal(15,5)),
`raise6secprice` Nullable(Decimal(15,5)),
`raise6secreq` Nullable(Decimal(15,5)),
`raise6secsupplyprice` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`initialsupply` Nullable(Decimal(15,5)),
`clearedsupply` Nullable(Decimal(15,5)),
`lowerregimport` Nullable(Decimal(15,5)),
`lowerreglocaldispatch` Nullable(Decimal(15,5)),
`lowerreglocalreq` Nullable(Decimal(15,5)),
`lowerregreq` Nullable(Decimal(15,5)),
`raiseregimport` Nullable(Decimal(15,5)),
`raisereglocaldispatch` Nullable(Decimal(15,5)),
`raisereglocalreq` Nullable(Decimal(15,5)),
`raiseregreq` Nullable(Decimal(15,5)),
`raise5minlocalviolation` Nullable(Decimal(15,5)),
`raisereglocalviolation` Nullable(Decimal(15,5)),
`raise60seclocalviolation` Nullable(Decimal(15,5)),
`raise6seclocalviolation` Nullable(Decimal(15,5)),
`lower5minlocalviolation` Nullable(Decimal(15,5)),
`lowerreglocalviolation` Nullable(Decimal(15,5)),
`lower60seclocalviolation` Nullable(Decimal(15,5)),
`lower6seclocalviolation` Nullable(Decimal(15,5)),
`raise5minviolation` Nullable(Decimal(15,5)),
`raiseregviolation` Nullable(Decimal(15,5)),
`raise60secviolation` Nullable(Decimal(15,5)),
`raise6secviolation` Nullable(Decimal(15,5)),
`lower5minviolation` Nullable(Decimal(15,5)),
`lowerregviolation` Nullable(Decimal(15,5)),
`lower60secviolation` Nullable(Decimal(15,5)),
`lower6secviolation` Nullable(Decimal(15,5)),
`totalintermittentgeneration` Nullable(Decimal(15,5)),
`demand_and_nonschedgen` Nullable(Decimal(15,5)),
`uigf` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `regionid`, `runno`, `settlementdate`);
                        
create table BillingMrSummary5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`total_payments` Nullable(Decimal(16,6)),
`total_recovery` Nullable(Decimal(16,6)),
`total_rsa` Nullable(Decimal(16,6)),
`aage` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `mr_date`, `regionid`, `weekno`);
                        
create table BillingSecdepInterestPay1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`security_deposit_id` String,
`participantid` String,
`interest_amount` Nullable(Decimal(18,8)),
`interest_calc_type` Nullable(String),
`interest_acct_id` Nullable(String),
`interest_rate` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `security_deposit_id`, `weekno`);
                        
create table BillingIntraresidues5 (
file_id Uuid,
`allocation` Nullable(Decimal(6,3)),
`totalsurplus` Nullable(Decimal(15,5)),
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`surplusvalue` Nullable(Decimal(15,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`regionid` String
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `regionid`, `weekno`);
                        
create table BillingApcCompensation2 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`apeventid` Decimal(6,0),
`claimid` Decimal(6,0),
`participantid` Nullable(String),
`compensation_amount` Nullable(Decimal(18,8)),
`event_type` Nullable(String),
`compensation_type` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`apeventid`, `billrunno`, `claimid`, `contractyear`, `weekno`);
                        
create table BillingSecdepInterestRate1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`interest_acct_id` String,
`effectivedate` DateTime('Australia/Brisbane'),
`interest_rate` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `effectivedate`, `interest_acct_id`, `weekno`);
                        
create table BillingRuntrk5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`status` Nullable(String),
`adj_cleared` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`postdate` Nullable(DateTime('Australia/Brisbane')),
`postby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`receiptpostdate` Nullable(DateTime('Australia/Brisbane')),
`receiptpostby` Nullable(String),
`paymentpostdate` Nullable(DateTime('Australia/Brisbane')),
`paymentpostby` Nullable(String),
`shortfall` Nullable(Decimal(16,6)),
`makeup` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `weekno`);
                        
create table BillingMrShortfall5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`participantid` String,
`age` Nullable(Decimal(16,6)),
`rsa` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `mr_date`, `participantid`, `regionid`, `weekno`);
                        
create table BillingMrRecovery5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`participantid` Nullable(String),
`duid` String,
`mr_amount` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `duid`, `mr_date`, `regionid`, `weekno`);
                        
create table BillingBillingCo2ePublication1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`settlementdate` DateTime('Australia/Brisbane'),
`regionid` String,
`sentoutenergy` Nullable(Decimal(18,8)),
`generatoremissions` Nullable(Decimal(18,8)),
`intensityindex` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `regionid`, `settlementdate`, `weekno`);
                        
create table BillingSmelterreduction5 (
file_id Uuid,
`contractyear` Decimal(22,0),
`weekno` Decimal(22,0),
`billrunno` Decimal(22,0),
`participantid` String,
`rate1` Nullable(Decimal(15,6)),
`ra1` Nullable(Decimal(15,6)),
`rate2` Nullable(Decimal(15,6)),
`ra2` Nullable(Decimal(15,6)),
`te` Nullable(Decimal(15,6)),
`pcsd` Nullable(Decimal(15,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingWhitehole5 (
file_id Uuid,
`contractyear` Decimal(22,0),
`weekno` Decimal(22,0),
`billrunno` Decimal(22,0),
`participantid` String,
`nl` Nullable(Decimal(15,6)),
`participantdemand` Nullable(Decimal(15,6)),
`regiondemand` Nullable(Decimal(15,6)),
`whiteholepayment` Nullable(Decimal(15,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`interconnectorid` String
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `interconnectorid`, `participantid`, `weekno`);
                        
create table BillingIrnspsurplussum6 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`residueyear` Decimal(4,0),
`quarter` Decimal(2,0),
`billrunno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`participantid` String,
`totalsurplus` Nullable(Decimal(15,5)),
`auctionfees` Nullable(Decimal(15,5)),
`auctionfees_gst` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `fromregionid`, `interconnectorid`, `participantid`, `quarter`, `residueyear`, `weekno`);
                        
create table BillingGstSummary5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`bas_class` String,
`gst_exclusive_amount` Nullable(Decimal(15,5)),
`gst_amount` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bas_class`, `billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingIraucsurplus5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(2,0),
`residueyear` Nullable(Decimal(4,0)),
`quarter` Nullable(Decimal(2,0)),
`billrunno` Decimal(3,0),
`contractid` String,
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`totalresidues` Nullable(Decimal(15,5)),
`adjustment` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `fromregionid`, `interconnectorid`, `participantid`, `weekno`);
                        
create table BillingBillingCo2ePublicationTrk1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `weekno`);
                        
create table BillingApcRecovery2 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`apeventid` Decimal(6,0),
`claimid` Decimal(6,0),
`participantid` String,
`regionid` String,
`recovery_amount` Nullable(Decimal(18,8)),
`eligibility_start_interval` Nullable(DateTime('Australia/Brisbane')),
`eligibility_end_interval` Nullable(DateTime('Australia/Brisbane')),
`participant_demand` Nullable(Decimal(18,8)),
`region_demand` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`apeventid`, `billrunno`, `claimid`, `contractyear`, `participantid`, `regionid`, `weekno`);
                        
create table BillingGstDetail5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`bas_class` String,
`transaction_type` String,
`gst_exclusive_amount` Nullable(Decimal(15,5)),
`gst_amount` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bas_class`, `billrunno`, `contractyear`, `participantid`, `transaction_type`, `weekno`);
                        
create table BillingEftshortfallAmount1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`shortfall_amount` Nullable(Decimal(18,8)),
`shortfall` Nullable(Decimal(18,8)),
`shortfall_company_id` Nullable(String),
`company_shortfall_amount` Nullable(Decimal(18,8)),
`participant_net_energy` Nullable(Decimal(18,8)),
`company_net_energy` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingIraucsurplussum7 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`residueyear` Decimal(4,0),
`quarter` Decimal(2,0),
`billrunno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`participantid` String,
`totalsurplus` Nullable(Decimal(15,5)),
`auctionfees` Nullable(Decimal(15,5)),
`actualpayment` Nullable(Decimal(15,5)),
`auctionfees_gst` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8)),
`negative_residues` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `fromregionid`, `interconnectorid`, `participantid`, `quarter`, `residueyear`, `weekno`);
                        
create table BillingIrfm5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`irfmpayment` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingCpdata5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`connectionpointid` String,
`aggregateenergy` Nullable(Decimal(16,6)),
`purchases` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`mda` String,
`afe` Nullable(Decimal(18,8)),
`dme` Nullable(Decimal(18,8)),
`ufea` Nullable(Decimal(18,8)),
`age` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `connectionpointid`, `contractyear`, `mda`, `participantid`, `weekno`);
                        
create table BillingAspayments6 (
file_id Uuid,
`regionid` Nullable(String),
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`connectionpointid` String,
`raise6sec` Nullable(Decimal(15,5)),
`lower6sec` Nullable(Decimal(15,5)),
`raise60sec` Nullable(Decimal(15,5)),
`lower60sec` Nullable(Decimal(15,5)),
`agc` Nullable(Decimal(15,5)),
`fcascomp` Nullable(Decimal(15,5)),
`loadshed` Nullable(Decimal(15,5)),
`rgul` Nullable(Decimal(15,5)),
`rguu` Nullable(Decimal(15,5)),
`reactivepower` Nullable(Decimal(15,5)),
`systemrestart` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`lower5min` Nullable(Decimal(15,5)),
`raise5min` Nullable(Decimal(15,5)),
`lowerreg` Nullable(Decimal(15,5)),
`raisereg` Nullable(Decimal(15,5)),
`availability_reactive` Nullable(Decimal(18,8)),
`availability_reactive_rbt` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `connectionpointid`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingRegionexports5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`regionid` String,
`exportto` String,
`energy` Nullable(Decimal(16,6)),
`value` Nullable(Decimal(15,5)),
`surplusenergy` Nullable(Decimal(16,6)),
`surplusvalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `exportto`, `regionid`, `weekno`);
                        
create table BillingMrPayment5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`participantid` Nullable(String),
`duid` String,
`mr_amount` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `duid`, `mr_date`, `regionid`, `weekno`);
                        
create table BillingFees5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`marketfeeid` String,
`rate` Nullable(Decimal(15,5)),
`energy` Nullable(Decimal(16,6)),
`value` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`participantcategoryid` String
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `marketfeeid`, `participantcategoryid`, `participantid`, `weekno`);
                        
create table BillingResTraderPayment1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`contractid` String,
`payment_type` String,
`participantid` String,
`payment_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `participantid`, `payment_type`, `weekno`);
                        
create table BillingBillingDirectionReconOther1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`direction_id` String,
`regionid` String,
`direction_desc` Nullable(String),
`direction_type_id` Nullable(String),
`direction_start_date` Nullable(DateTime('Australia/Brisbane')),
`direction_end_date` Nullable(DateTime('Australia/Brisbane')),
`direction_start_interval` Nullable(DateTime('Australia/Brisbane')),
`direction_end_interval` Nullable(DateTime('Australia/Brisbane')),
`compensation_amount` Nullable(Decimal(18,8)),
`interest_amount` Nullable(Decimal(18,8)),
`independent_expert_fee` Nullable(Decimal(18,8)),
`cra` Nullable(Decimal(18,8)),
`regional_customer_energy` Nullable(Decimal(18,8)),
`regional_generator_energy` Nullable(Decimal(18,8)),
`regional_benefit_factor` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `direction_id`, `regionid`, `weekno`);
                        
create table BillingRealloc5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`counterparty` String,
`value` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `counterparty`, `participantid`, `weekno`);
                        
create table BillingFinancialadjustments5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`participanttype` Nullable(String),
`adjustmentitem` String,
`amount` Nullable(Decimal(15,5)),
`value` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`financialcode` Nullable(Decimal(10,0)),
`bas_class` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`adjustmentitem`, `billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingNmasTstRecovery1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`service` String,
`contractid` String,
`regionid` String,
`rbf` Nullable(Decimal(18,8)),
`test_payment` Nullable(Decimal(18,8)),
`recovery_start_date` Nullable(DateTime('Australia/Brisbane')),
`recovery_end_date` Nullable(DateTime('Australia/Brisbane')),
`participant_energy` Nullable(Decimal(18,8)),
`region_energy` Nullable(Decimal(18,8)),
`nem_energy` Nullable(Decimal(18,8)),
`customer_proportion` Nullable(Decimal(18,8)),
`generator_proportion` Nullable(Decimal(18,8)),
`participant_generation` Nullable(Decimal(18,8)),
`nem_generation` Nullable(Decimal(18,8)),
`recovery_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `participantid`, `regionid`, `service`, `weekno`);
                        
create table BillingAsrecovery7 (
file_id Uuid,
`regionid` String,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`raise6sec` Nullable(Decimal(15,5)),
`lower6sec` Nullable(Decimal(15,5)),
`raise60sec` Nullable(Decimal(15,5)),
`lower60sec` Nullable(Decimal(15,5)),
`agc` Nullable(Decimal(15,5)),
`fcascomp` Nullable(Decimal(15,5)),
`loadshed` Nullable(Decimal(15,5)),
`rgul` Nullable(Decimal(15,5)),
`rguu` Nullable(Decimal(15,5)),
`reactivepower` Nullable(Decimal(15,5)),
`systemrestart` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`raise6sec_gen` Nullable(Decimal(15,5)),
`lower6sec_gen` Nullable(Decimal(15,5)),
`raise60sec_gen` Nullable(Decimal(15,5)),
`lower60sec_gen` Nullable(Decimal(15,5)),
`agc_gen` Nullable(Decimal(15,5)),
`fcascomp_gen` Nullable(Decimal(15,5)),
`loadshed_gen` Nullable(Decimal(15,5)),
`rgul_gen` Nullable(Decimal(15,5)),
`rguu_gen` Nullable(Decimal(15,5)),
`reactivepower_gen` Nullable(Decimal(15,5)),
`systemrestart_gen` Nullable(Decimal(15,5)),
`lower5min` Nullable(Decimal(15,5)),
`raise5min` Nullable(Decimal(15,5)),
`lowerreg` Nullable(Decimal(15,5)),
`raisereg` Nullable(Decimal(15,5)),
`lower5min_gen` Nullable(Decimal(16,6)),
`raise5min_gen` Nullable(Decimal(16,6)),
`lowerreg_gen` Nullable(Decimal(16,6)),
`raisereg_gen` Nullable(Decimal(16,6)),
`availability_reactive` Nullable(Decimal(18,8)),
`availability_reactive_rbt` Nullable(Decimal(18,8)),
`availability_reactive_gen` Nullable(Decimal(18,8)),
`availability_reactive_rbt_gen` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `regionid`, `weekno`);
                        
create table BillingReallocDetail5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`counterparty` String,
`reallocationid` String,
`value` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `counterparty`, `participantid`, `reallocationid`, `weekno`);
                        
create table BillingIrpartsurplussum7 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`residueyear` Decimal(4,0),
`quarter` Decimal(2,0),
`billrunno` Decimal(3,0),
`interconnectorid` String,
`fromregionid` String,
`participantid` String,
`totalsurplus` Nullable(Decimal(15,5)),
`auctionfees` Nullable(Decimal(15,5)),
`actualpayment` Nullable(Decimal(15,5)),
`auctionfees_gst` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`csp_derogation_amount` Nullable(Decimal(18,8)),
`unadjusted_irsr` Nullable(Decimal(18,8)),
`auctionfees_totalgross_adj` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `fromregionid`, `interconnectorid`, `participantid`, `quarter`, `residueyear`, `weekno`);
                        
create table BillingGendata5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`connectionpointid` String,
`stationid` Nullable(String),
`duid` Nullable(String),
`aggregateenergy` Nullable(Decimal(16,6)),
`sales` Nullable(Decimal(16,6)),
`purchases` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`purchasedenergy` Nullable(Decimal(16,6)),
`mda` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `connectionpointid`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingResTraderRecovery1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`regionid` String,
`participantid` String,
`recovery_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `regionid`, `weekno`);
                        
create table BillingIrpartsurplus5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(2,0),
`residueyear` Nullable(Decimal(4,0)),
`quarter` Nullable(Decimal(2,0)),
`billrunno` Decimal(3,0),
`contractid` String,
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`totalresidues` Nullable(Decimal(15,5)),
`adjustment` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`actualpayment` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `fromregionid`, `interconnectorid`, `participantid`, `weekno`);
                        
create table BillingDaytrk5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `settlementdate`, `weekno`);
                        
create table BillingPrioradjustments5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`adjcontractyear` Decimal(4,0),
`adjweekno` Decimal(3,0),
`adjbillrunno` Decimal(3,0),
`participantid` String,
`prevamount` Nullable(Decimal(15,5)),
`adjamount` Nullable(Decimal(15,5)),
`irn` Nullable(Decimal(15,5)),
`irp` Nullable(Decimal(15,5)),
`interestamount` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`irsr_prevamount` Nullable(Decimal(15,5)),
`irsr_adjamount` Nullable(Decimal(15,5)),
`irsr_interestamount` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`adjbillrunno`, `adjcontractyear`, `adjweekno`, `billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingNmasTstRecvryRbf1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`service` String,
`contractid` String,
`regionid` String,
`rbf` Nullable(Decimal(18,8)),
`payment_amount` Nullable(Decimal(18,8)),
`recovery_amount` Nullable(Decimal(18,8)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `regionid`, `service`, `weekno`);
                        
create table BillingInterresidues5 (
file_id Uuid,
`allocation` Nullable(Decimal(6,3)),
`totalsurplus` Nullable(Decimal(15,5)),
`interconnectorid` String,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`surplusvalue` Nullable(Decimal(15,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`regionid` String
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `interconnectorid`, `participantid`, `regionid`, `weekno`);
                        
create table BillingDirectionReconciliatn1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`direction_id` String,
`direction_desc` Nullable(String),
`direction_start_date` Nullable(DateTime('Australia/Brisbane')),
`direction_end_date` Nullable(DateTime('Australia/Brisbane')),
`compensation_amount` Nullable(Decimal(16,6)),
`independent_expert_fee` Nullable(Decimal(16,6)),
`interest_amount` Nullable(Decimal(16,6)),
`cra` Nullable(Decimal(16,6)),
`nem_fee_id` Nullable(String),
`nem_fixed_fee_amount` Nullable(Decimal(16,6)),
`mkt_customer_perc` Nullable(Decimal(16,6)),
`generator_perc` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `direction_id`, `weekno`);
                        
create table BillingRegionfigures5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`regionid` String,
`energyout` Nullable(Decimal(16,6)),
`valueout` Nullable(Decimal(16,6)),
`energypurchased` Nullable(Decimal(16,6)),
`valuepurchased` Nullable(Decimal(16,6)),
`excessgen` Nullable(Decimal(16,6)),
`reservetrading` Nullable(Decimal(16,6)),
`intcompo` Nullable(Decimal(16,6)),
`adminpricecompo` Nullable(Decimal(16,6)),
`settsurplus` Nullable(Decimal(16,6)),
`aspayment` Nullable(Decimal(16,6)),
`poolfees` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `regionid`, `weekno`);
                        
create table BillingRegionimports5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`regionid` String,
`importfrom` String,
`energy` Nullable(Decimal(16,6)),
`value` Nullable(Decimal(15,5)),
`surplusenergy` Nullable(Decimal(16,6)),
`surplusvalue` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `importfrom`, `regionid`, `weekno`);
                        
create table BillingSecdepositApplication1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`application_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `weekno`);
                        
create table BillingNmasTstRecvryTrk1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`recovery_contractyear` Decimal(4,0),
`recovery_weekno` Decimal(3,0),
`recovery_billrunno` Decimal(3,0)
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `recovery_billrunno`, `recovery_contractyear`, `recovery_weekno`, `weekno`);
                        
create table BillingIrnspsurplus5 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(2,0),
`residueyear` Nullable(Decimal(4,0)),
`quarter` Nullable(Decimal(2,0)),
`billrunno` Decimal(3,0),
`contractid` String,
`participantid` String,
`interconnectorid` String,
`fromregionid` String,
`totalresidues` Nullable(Decimal(15,5)),
`adjustment` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `fromregionid`, `interconnectorid`, `participantid`, `weekno`);
                        
create table BillingDailyEnergySummary1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`settlementdate` DateTime('Australia/Brisbane'),
`participantid` String,
`regionid` String,
`customer_energy_purchased` Nullable(Decimal(18,8)),
`generator_energy_sold` Nullable(Decimal(18,8)),
`generator_energy_purchased` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `regionid`, `settlementdate`, `weekno`);
                        
create table BillingNmasTstPayments1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`service` String,
`contractid` String,
`payment_amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractid`, `contractyear`, `participantid`, `service`, `weekno`);
                        
create table BillingEftshortfallDetail1 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`billrunno` Decimal(3,0),
`participantid` String,
`transaction_type` String,
`amount` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`billrunno`, `contractyear`, `participantid`, `transaction_type`, `weekno`);
                        
create table MccCasesolution1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane')
)
ENGINE = MergeTree()
ORDER BY (`run_datetime`);
                        
create table MccConstraintsolution1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`constraintid` String,
`rhs` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `run_datetime`);
                        
create table StpasaRegionsolution5 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`regionid` String,
`demand10` Nullable(Decimal(12,2)),
`demand50` Nullable(Decimal(12,2)),
`demand90` Nullable(Decimal(12,2)),
`reservereq` Nullable(Decimal(12,2)),
`capacityreq` Nullable(Decimal(12,2)),
`energyreqdemand50` Nullable(Decimal(12,2)),
`unconstrainedcapacity` Nullable(Decimal(12,0)),
`constrainedcapacity` Nullable(Decimal(12,0)),
`netinterchangeunderscarcity` Nullable(Decimal(12,2)),
`surpluscapacity` Nullable(Decimal(12,2)),
`surplusreserve` Nullable(Decimal(12,2)),
`reservecondition` Nullable(Decimal(1,0)),
`maxsurplusreserve` Nullable(Decimal(12,2)),
`maxsparecapacity` Nullable(Decimal(12,2)),
`lorcondition` Nullable(Decimal(1,0)),
`aggregatecapacityavailable` Nullable(Decimal(12,2)),
`aggregatescheduledload` Nullable(Decimal(12,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`aggregatepasaavailability` Nullable(Decimal(12,0)),
`runtype` String,
`energyreqdemand10` Nullable(Decimal(12,2)),
`calculatedlor1level` Nullable(Decimal(16,6)),
`calculatedlor2level` Nullable(Decimal(16,6)),
`msrnetinterchangeunderscarcity` Nullable(Decimal(12,2)),
`lornetinterchangeunderscarcity` Nullable(Decimal(12,2)),
`totalintermittentgeneration` Nullable(Decimal(15,5)),
`demand_and_nonschedgen` Nullable(Decimal(15,5)),
`uigf` Nullable(Decimal(12,2)),
`semi_scheduled_capacity` Nullable(Decimal(12,2)),
`lor_semi_scheduled_capacity` Nullable(Decimal(12,2)),
`lcr` Nullable(Decimal(16,6)),
`lcr2` Nullable(Decimal(16,6)),
`fum` Nullable(Decimal(16,6)),
`ss_solar_uigf` Nullable(Decimal(12,2)),
`ss_wind_uigf` Nullable(Decimal(12,2)),
`ss_solar_capacity` Nullable(Decimal(12,2)),
`ss_wind_capacity` Nullable(Decimal(12,2)),
`ss_solar_cleared` Nullable(Decimal(12,2)),
`ss_wind_cleared` Nullable(Decimal(12,2))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`, `run_datetime`, `runtype`);
                        
create table StpasaInterconnectorsoln2 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`interconnectorid` String,
`capacitymwflow` Nullable(Decimal(12,2)),
`capacitymarginalvalue` Nullable(Decimal(12,2)),
`capacityviolationdegree` Nullable(Decimal(12,2)),
`calculatedexportlimit` Nullable(Decimal(12,2)),
`calculatedimportlimit` Nullable(Decimal(12,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`runtype` String,
`exportlimitconstraintid` Nullable(String),
`importlimitconstraintid` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`interconnectorid`, `interval_datetime`, `run_datetime`, `runtype`);
                        
create table StpasaConstraintsolution2 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`constraintid` String,
`capacityrhs` Nullable(Decimal(12,2)),
`capacitymarginalvalue` Nullable(Decimal(12,2)),
`capacityviolationdegree` Nullable(Decimal(12,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`runtype` String
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `interval_datetime`, `run_datetime`, `runtype`);
                        
create table StpasaCasesolution3 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`pasaversion` Nullable(String),
`reservecondition` Nullable(Decimal(1,0)),
`lorcondition` Nullable(Decimal(1,0)),
`capacityobjfunction` Nullable(Decimal(12,3)),
`capacityoption` Nullable(Decimal(12,3)),
`maxsurplusreserveoption` Nullable(Decimal(12,3)),
`maxsparecapacityoption` Nullable(Decimal(12,3)),
`interconnectorflowpenalty` Nullable(Decimal(12,3)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`reliabilitylrcdemandoption` Nullable(Decimal(12,3)),
`outagelrcdemandoption` Nullable(Decimal(12,3)),
`lordemandoption` Nullable(Decimal(12,3)),
`reliabilitylrccapacityoption` Nullable(String),
`outagelrccapacityoption` Nullable(String),
`lorcapacityoption` Nullable(String),
`loruigf_option` Nullable(Decimal(3,0)),
`reliability_lrcuigf_option` Nullable(Decimal(3,0)),
`outage_lrcuigf_option` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`run_datetime`);
                        
create table OfferMnspDayoffer2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`offerdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`linkid` String,
`entrytype` Nullable(String),
`rebidexplanation` Nullable(String),
`priceband1` Nullable(Decimal(9,2)),
`priceband2` Nullable(Decimal(9,2)),
`priceband3` Nullable(Decimal(9,2)),
`priceband4` Nullable(Decimal(9,2)),
`priceband5` Nullable(Decimal(9,2)),
`priceband6` Nullable(Decimal(9,2)),
`priceband7` Nullable(Decimal(9,2)),
`priceband8` Nullable(Decimal(9,2)),
`priceband9` Nullable(Decimal(9,2)),
`priceband10` Nullable(Decimal(9,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`mr_factor` Nullable(Decimal(16,6)),
`rebid_event_time` Nullable(String),
`rebid_aware_time` Nullable(String),
`rebid_decision_time` Nullable(String),
`rebid_category` Nullable(String),
`reference_id` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`linkid`, `offerdate`, `participantid`, `settlementdate`, `versionno`);
                        
create table OfferBiddayoffer2 (
file_id Uuid,
`duid` String,
`bidtype` String,
`settlementdate` DateTime('Australia/Brisbane'),
`offerdate` DateTime('Australia/Brisbane'),
`versionno` Nullable(Decimal(22,0)),
`participantid` Nullable(String),
`dailyenergyconstraint` Nullable(Decimal(12,6)),
`rebidexplanation` Nullable(String),
`priceband1` Nullable(Decimal(9,2)),
`priceband2` Nullable(Decimal(9,2)),
`priceband3` Nullable(Decimal(9,2)),
`priceband4` Nullable(Decimal(9,2)),
`priceband5` Nullable(Decimal(9,2)),
`priceband6` Nullable(Decimal(9,2)),
`priceband7` Nullable(Decimal(9,2)),
`priceband8` Nullable(Decimal(9,2)),
`priceband9` Nullable(Decimal(9,2)),
`priceband10` Nullable(Decimal(9,2)),
`minimumload` Nullable(Decimal(22,0)),
`t1` Nullable(Decimal(22,0)),
`t2` Nullable(Decimal(22,0)),
`t3` Nullable(Decimal(22,0)),
`t4` Nullable(Decimal(22,0)),
`normalstatus` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`mr_factor` Nullable(Decimal(16,6)),
`entrytype` Nullable(String),
`rebid_event_time` Nullable(String),
`rebid_aware_time` Nullable(String),
`rebid_decision_time` Nullable(String),
`rebid_category` Nullable(String),
`reference_id` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `offerdate`, `settlementdate`);
                        
create table OfferMnspPeroffer1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`offerdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`linkid` String,
`periodid` Decimal(22,0),
`maxavail` Nullable(Decimal(6,0)),
`bandavail1` Nullable(Decimal(6,0)),
`bandavail2` Nullable(Decimal(6,0)),
`bandavail3` Nullable(Decimal(6,0)),
`bandavail4` Nullable(Decimal(6,0)),
`bandavail5` Nullable(Decimal(6,0)),
`bandavail6` Nullable(Decimal(6,0)),
`bandavail7` Nullable(Decimal(6,0)),
`bandavail8` Nullable(Decimal(6,0)),
`bandavail9` Nullable(Decimal(6,0)),
`bandavail10` Nullable(Decimal(6,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`fixedload` Nullable(Decimal(12,6)),
`rampuprate` Nullable(Decimal(6,0)),
`pasaavailability` Nullable(Decimal(12,0)),
`mr_capacity` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`linkid`, `offerdate`, `participantid`, `periodid`, `settlementdate`, `versionno`);
                        
create table BidBidperofferD2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`duid` String,
`bidtype` String,
`bidsettlementdate` Nullable(DateTime('Australia/Brisbane')),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`periodid` Nullable(Decimal(22,0)),
`versionno` Nullable(Decimal(22,0)),
`maxavail` Nullable(Decimal(12,6)),
`fixedload` Nullable(Decimal(12,6)),
`rocup` Nullable(Decimal(6,0)),
`rocdown` Nullable(Decimal(6,0)),
`enablementmin` Nullable(Decimal(6,0)),
`enablementmax` Nullable(Decimal(6,0)),
`lowbreakpoint` Nullable(Decimal(6,0)),
`highbreakpoint` Nullable(Decimal(6,0)),
`bandavail1` Nullable(Decimal(22,0)),
`bandavail2` Nullable(Decimal(22,0)),
`bandavail3` Nullable(Decimal(22,0)),
`bandavail4` Nullable(Decimal(22,0)),
`bandavail5` Nullable(Decimal(22,0)),
`bandavail6` Nullable(Decimal(22,0)),
`bandavail7` Nullable(Decimal(22,0)),
`bandavail8` Nullable(Decimal(22,0)),
`bandavail9` Nullable(Decimal(22,0)),
`bandavail10` Nullable(Decimal(22,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`pasaavailability` Nullable(Decimal(12,0)),
`interval_datetime` DateTime('Australia/Brisbane'),
`mr_capacity` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `interval_datetime`, `settlementdate`);
                        
create table BidMnspFiletrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`offerdate` DateTime('Australia/Brisbane'),
`participantid` String,
`filename` String,
`status` Nullable(String),
`ackfilename` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`filename`, `offerdate`, `participantid`, `settlementdate`);
                        
create table OfferBidperoffer1 (
file_id Uuid,
`duid` String,
`bidtype` String,
`settlementdate` DateTime('Australia/Brisbane'),
`offerdate` DateTime('Australia/Brisbane'),
`periodid` Decimal(22,0),
`versionno` Nullable(Decimal(22,0)),
`maxavail` Nullable(Decimal(12,6)),
`fixedload` Nullable(Decimal(12,6)),
`rocup` Nullable(Decimal(6,0)),
`rocdown` Nullable(Decimal(6,0)),
`enablementmin` Nullable(Decimal(6,0)),
`enablementmax` Nullable(Decimal(6,0)),
`lowbreakpoint` Nullable(Decimal(6,0)),
`highbreakpoint` Nullable(Decimal(6,0)),
`bandavail1` Nullable(Decimal(22,0)),
`bandavail2` Nullable(Decimal(22,0)),
`bandavail3` Nullable(Decimal(22,0)),
`bandavail4` Nullable(Decimal(22,0)),
`bandavail5` Nullable(Decimal(22,0)),
`bandavail6` Nullable(Decimal(22,0)),
`bandavail7` Nullable(Decimal(22,0)),
`bandavail8` Nullable(Decimal(22,0)),
`bandavail9` Nullable(Decimal(22,0)),
`bandavail10` Nullable(Decimal(22,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`pasaavailability` Nullable(Decimal(12,0)),
`mr_capacity` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `offerdate`, `periodid`, `settlementdate`);
                        
create table OfferMtpasaOfferfiletrk1 (
file_id Uuid,
`participantid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`filename` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`offerdatetime`, `participantid`);
                        
create table OfferMnspOffertrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`offerdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`participantid` String,
`filename` String,
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`filename`, `offerdate`, `participantid`, `settlementdate`, `versionno`);
                        
create table BidBiddayofferD2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`duid` String,
`bidtype` String,
`bidsettlementdate` Nullable(DateTime('Australia/Brisbane')),
`offerdate` Nullable(DateTime('Australia/Brisbane')),
`versionno` Nullable(Decimal(22,0)),
`participantid` Nullable(String),
`dailyenergyconstraint` Nullable(Decimal(12,6)),
`rebidexplanation` Nullable(String),
`priceband1` Nullable(Decimal(9,2)),
`priceband2` Nullable(Decimal(9,2)),
`priceband3` Nullable(Decimal(9,2)),
`priceband4` Nullable(Decimal(9,2)),
`priceband5` Nullable(Decimal(9,2)),
`priceband6` Nullable(Decimal(9,2)),
`priceband7` Nullable(Decimal(9,2)),
`priceband8` Nullable(Decimal(9,2)),
`priceband9` Nullable(Decimal(9,2)),
`priceband10` Nullable(Decimal(9,2)),
`minimumload` Nullable(Decimal(22,0)),
`t1` Nullable(Decimal(22,0)),
`t2` Nullable(Decimal(22,0)),
`t3` Nullable(Decimal(22,0)),
`t4` Nullable(Decimal(22,0)),
`normalstatus` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`mr_factor` Nullable(Decimal(16,6)),
`entrytype` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `settlementdate`);
                        
create table OfferBidofferfiletrk1 (
file_id Uuid,
`participantid` String,
`offerdate` DateTime('Australia/Brisbane'),
`filename` String,
`status` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`transaction_id` Nullable(String),
`reference_id` Nullable(String),
`submission_timestamp` Nullable(DateTime('Australia/Brisbane')),
`comments` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`offerdate`, `participantid`);
                        
create table OfferMtpasaOfferdata1 (
file_id Uuid,
`participantid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`unitid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`energy` Nullable(Decimal(9,0)),
`capacity1` Nullable(Decimal(9,0)),
`capacity2` Nullable(Decimal(9,0)),
`capacity3` Nullable(Decimal(9,0)),
`capacity4` Nullable(Decimal(9,0)),
`capacity5` Nullable(Decimal(9,0)),
`capacity6` Nullable(Decimal(9,0)),
`capacity7` Nullable(Decimal(9,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `offerdatetime`, `participantid`, `unitid`);
                        
create table MarketNoticeMarketnoticedata1 (
file_id Uuid,
`noticeid` Decimal(10,0),
`effectivedate` Nullable(DateTime('Australia/Brisbane')),
`typeid` Nullable(String),
`noticetype` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`reason` Nullable(String),
`externalreference` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`noticeid`);
                        
create table MarketNoticeParticipantnoticetrk1 (
file_id Uuid,
`participantid` String,
`noticeid` Decimal(10,0),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`noticeid`, `participantid`);
                        
create table MarketNoticeMarketnoticetype1 (
file_id Uuid,
`typeid` String,
`description` Nullable(String),
`raisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`typeid`);
                        
create table SpdcpcNull2 (
file_id Uuid,
`connectionpointid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`genconid` String,
`factor` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`bidtype` String
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `connectionpointid`, `effectivedate`, `genconid`, `versionno`);
                        
create table GenconsetNull1 (
file_id Uuid,
`genconsetid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`genconid` String,
`genconeffdate` Nullable(DateTime('Australia/Brisbane')),
`genconversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `genconid`, `genconsetid`, `versionno`);
                        
create table GcrhsNull1 (
file_id Uuid,
`genconid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(22,0),
`scope` String,
`termid` Decimal(4,0),
`groupid` Nullable(Decimal(3,0)),
`spd_id` Nullable(String),
`spd_type` Nullable(String),
`factor` Nullable(Decimal(16,6)),
`operation` Nullable(String),
`defaultvalue` Nullable(Decimal(16,6)),
`parameterterm1` Nullable(String),
`parameterterm2` Nullable(String),
`parameterterm3` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `genconid`, `scope`, `termid`, `versionno`);
                        
create table GeqrhsNull1 (
file_id Uuid,
`equationid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`termid` Decimal(3,0),
`groupid` Nullable(Decimal(3,0)),
`spd_id` Nullable(String),
`spd_type` Nullable(String),
`factor` Nullable(Decimal(16,6)),
`operation` Nullable(String),
`defaultvalue` Nullable(Decimal(16,6)),
`parameterterm1` Nullable(String),
`parameterterm2` Nullable(String),
`parameterterm3` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `equationid`, `termid`, `versionno`);
                        
create table SpdiccNull1 (
file_id Uuid,
`interconnectorid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`genconid` String,
`factor` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `genconid`, `interconnectorid`, `versionno`);
                        
create table GenericConstraintEmsmaster1 (
file_id Uuid,
`spd_id` String,
`spd_type` String,
`description` Nullable(String),
`grouping_id` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`spd_id`, `spd_type`);
                        
create table SpdrcNull2 (
file_id Uuid,
`regionid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`genconid` String,
`factor` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`bidtype` String
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `effectivedate`, `genconid`, `regionid`, `versionno`);
                        
create table GencondataNull6 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`genconid` String,
`constrainttype` Nullable(String),
`constraintvalue` Nullable(Decimal(16,6)),
`description` Nullable(String),
`status` Nullable(String),
`genericconstraintweight` Nullable(Decimal(16,6)),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`dynamicrhs` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`dispatch` Nullable(String),
`predispatch` Nullable(String),
`stpasa` Nullable(String),
`mtpasa` Nullable(String),
`impact` Nullable(String),
`source` Nullable(String),
`limittype` Nullable(String),
`reason` Nullable(String),
`modifications` Nullable(String),
`additionalnotes` Nullable(String),
`p5min_scope_override` Nullable(String),
`lrc` Nullable(String),
`lor` Nullable(String),
`force_scada` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `genconid`, `versionno`);
                        
create table GeqdescNull2 (
file_id Uuid,
`equationid` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`impact` Nullable(String),
`source` Nullable(String),
`limittype` Nullable(String),
`reason` Nullable(String),
`modifications` Nullable(String),
`additionalnotes` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`equationid`);
                        
create table GenconsettrkNull2 (
file_id Uuid,
`genconsetid` String,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`description` Nullable(String),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`coverage` Nullable(String),
`modifications` Nullable(String),
`systemnormal` Nullable(String),
`outage` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `genconsetid`, `versionno`);
                        
create table GenericConstraintGenconsetinvoke2 (
file_id Uuid,
`invocation_id` Decimal(9,0),
`startdate` DateTime('Australia/Brisbane'),
`startperiod` Decimal(3,0),
`genconsetid` String,
`enddate` Nullable(DateTime('Australia/Brisbane')),
`endperiod` Nullable(Decimal(3,0)),
`startauthorisedby` Nullable(String),
`endauthorisedby` Nullable(String),
`intervention` Nullable(String),
`asconstrainttype` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`startintervaldatetime` Nullable(DateTime('Australia/Brisbane')),
`endintervaldatetime` Nullable(DateTime('Australia/Brisbane')),
`systemnormal` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`invocation_id`);
                        
create table DispatchLocalPrice1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`duid` String,
`local_price_adjustment` Nullable(Decimal(10,2)),
`locally_constrained` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `settlementdate`);
                        
create table PriceloadConstraintFcasOcd1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`intervention` Decimal(2,0),
`constraintid` String,
`versionno` Decimal(3,0),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`rhs` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `intervention`, `runno`, `settlementdate`, `versionno`);
                        
create table DispatchUnitConformance1 (
file_id Uuid,
`interval_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`totalcleared` Nullable(Decimal(16,6)),
`actualmw` Nullable(Decimal(16,6)),
`roc` Nullable(Decimal(16,6)),
`availability` Nullable(Decimal(16,6)),
`lowerreg` Nullable(Decimal(16,6)),
`raisereg` Nullable(Decimal(16,6)),
`striglm` Nullable(Decimal(16,6)),
`ltriglm` Nullable(Decimal(16,6)),
`mwerror` Nullable(Decimal(16,6)),
`max_mwerror` Nullable(Decimal(16,6)),
`lecount` Nullable(Decimal(6,0)),
`secount` Nullable(Decimal(6,0)),
`status` Nullable(String),
`participant_status_action` Nullable(String),
`operating_mode` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `interval_datetime`);
                        
create table DispatchConstraint5 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`constraintid` String,
`dispatchinterval` Decimal(22,0),
`intervention` Decimal(2,0),
`rhs` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`duid` Nullable(String),
`genconid_effectivedate` Nullable(DateTime('Australia/Brisbane')),
`genconid_versionno` Nullable(Decimal(22,0)),
`lhs` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `dispatchinterval`, `intervention`, `runno`, `settlementdate`);
                        
create table PriceloadConstraintrelaxation1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`constraintid` String,
`rhs` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`versionno` Decimal(3,0)
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `runno`, `settlementdate`, `versionno`);
                        
create table DispatchCaseSolution2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`intervention` Decimal(2,0),
`casesubtype` Nullable(String),
`solutionstatus` Nullable(Decimal(2,0)),
`spdversion` Nullable(String),
`nonphysicallosses` Nullable(Decimal(1,0)),
`totalobjective` Nullable(Decimal(27,10)),
`totalareagenviolation` Nullable(Decimal(15,5)),
`totalinterconnectorviolation` Nullable(Decimal(15,5)),
`totalgenericviolation` Nullable(Decimal(15,5)),
`totalramprateviolation` Nullable(Decimal(15,5)),
`totalunitmwcapacityviolation` Nullable(Decimal(15,5)),
`total5minviolation` Nullable(Decimal(15,5)),
`totalregviolation` Nullable(Decimal(15,5)),
`total6secviolation` Nullable(Decimal(15,5)),
`total60secviolation` Nullable(Decimal(15,5)),
`totalasprofileviolation` Nullable(Decimal(15,5)),
`totalfaststartviolation` Nullable(Decimal(15,5)),
`totalenergyofferviolation` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`switchruninitialstatus` Nullable(Decimal(1,0)),
`switchrunbeststatus` Nullable(Decimal(1,0)),
`switchrunbeststatus_int` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`runno`, `settlementdate`);
                        
create table DispatchNegativeResidue1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`nrm_datetime` DateTime('Australia/Brisbane'),
`directional_interconnectorid` String,
`nrm_activated_flag` Nullable(Decimal(1,0)),
`cumul_negresidue_amount` Nullable(Decimal(15,5)),
`cumul_negresidue_prev_ti` Nullable(Decimal(15,5)),
`negresidue_current_ti` Nullable(Decimal(15,5)),
`negresidue_pd_next_ti` Nullable(Decimal(15,5)),
`price_revision` Nullable(String),
`predispatchseqno` Nullable(String),
`event_activated_di` Nullable(DateTime('Australia/Brisbane')),
`event_deactivated_di` Nullable(DateTime('Australia/Brisbane')),
`di_notbinding_count` Nullable(Decimal(2,0)),
`di_violated_count` Nullable(Decimal(2,0)),
`nrmconstraint_blocked_flag` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`directional_interconnectorid`, `nrm_datetime`, `settlementdate`);
                        
create table DispatchMnspbidtrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`participantid` String,
`linkid` String,
`offersettlementdate` Nullable(DateTime('Australia/Brisbane')),
`offereffectivedate` Nullable(DateTime('Australia/Brisbane')),
`offerversionno` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`linkid`, `participantid`, `runno`, `settlementdate`);
                        
create table DispatchPrice4 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`regionid` String,
`dispatchinterval` String,
`intervention` Decimal(2,0),
`rrp` Nullable(Decimal(15,5)),
`eep` Nullable(Decimal(15,5)),
`rop` Nullable(Decimal(15,5)),
`apcflag` Nullable(Decimal(3,0)),
`marketsuspendedflag` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`raise6secrrp` Nullable(Decimal(15,5)),
`raise6secrop` Nullable(Decimal(15,5)),
`raise6secapcflag` Nullable(Decimal(3,0)),
`raise60secrrp` Nullable(Decimal(15,5)),
`raise60secrop` Nullable(Decimal(15,5)),
`raise60secapcflag` Nullable(Decimal(3,0)),
`raise5minrrp` Nullable(Decimal(15,5)),
`raise5minrop` Nullable(Decimal(15,5)),
`raise5minapcflag` Nullable(Decimal(3,0)),
`raiseregrrp` Nullable(Decimal(15,5)),
`raiseregrop` Nullable(Decimal(15,5)),
`raiseregapcflag` Nullable(Decimal(3,0)),
`lower6secrrp` Nullable(Decimal(15,5)),
`lower6secrop` Nullable(Decimal(15,5)),
`lower6secapcflag` Nullable(Decimal(3,0)),
`lower60secrrp` Nullable(Decimal(15,5)),
`lower60secrop` Nullable(Decimal(15,5)),
`lower60secapcflag` Nullable(Decimal(3,0)),
`lower5minrrp` Nullable(Decimal(15,5)),
`lower5minrop` Nullable(Decimal(15,5)),
`lower5minapcflag` Nullable(Decimal(3,0)),
`lowerregrrp` Nullable(Decimal(15,5)),
`lowerregrop` Nullable(Decimal(15,5)),
`lowerregapcflag` Nullable(Decimal(3,0)),
`price_status` Nullable(String),
`pre_ap_energy_price` Nullable(Decimal(15,5)),
`pre_ap_raise6_price` Nullable(Decimal(15,5)),
`pre_ap_raise60_price` Nullable(Decimal(15,5)),
`pre_ap_raise5min_price` Nullable(Decimal(15,5)),
`pre_ap_raisereg_price` Nullable(Decimal(15,5)),
`pre_ap_lower6_price` Nullable(Decimal(15,5)),
`pre_ap_lower60_price` Nullable(Decimal(15,5)),
`pre_ap_lower5min_price` Nullable(Decimal(15,5)),
`pre_ap_lowerreg_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_energy_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_raise6_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_raise60_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_raise5min_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_raisereg_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_lower6_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_lower60_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_lower5min_price` Nullable(Decimal(15,5)),
`cumul_pre_ap_lowerreg_price` Nullable(Decimal(15,5)),
`ocd_status` Nullable(String),
`mii_status` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`dispatchinterval`, `intervention`, `regionid`, `runno`, `settlementdate`);
                        
create table DispatchInterconnectorres3 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`interconnectorid` String,
`dispatchinterval` Decimal(22,0),
`intervention` Decimal(2,0),
`meteredmwflow` Nullable(Decimal(15,5)),
`mwflow` Nullable(Decimal(15,5)),
`mwlosses` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`exportlimit` Nullable(Decimal(15,5)),
`importlimit` Nullable(Decimal(15,5)),
`marginalloss` Nullable(Decimal(15,5)),
`exportgenconid` Nullable(String),
`importgenconid` Nullable(String),
`fcasexportlimit` Nullable(Decimal(15,5)),
`fcasimportlimit` Nullable(Decimal(15,5)),
`local_price_adjustment_export` Nullable(Decimal(10,2)),
`locally_constrained_export` Nullable(Decimal(1,0)),
`local_price_adjustment_import` Nullable(Decimal(10,2)),
`locally_constrained_import` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`dispatchinterval`, `interconnectorid`, `intervention`, `runno`, `settlementdate`);
                        
create table DispatchMrScheduleTrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`regionid` String,
`mr_date` Nullable(DateTime('Australia/Brisbane')),
`version_datetime` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`regionid`, `settlementdate`);
                        
create table DispatchIntermittentForecastTrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`duid` String,
`origin` Nullable(String),
`forecast_priority` Nullable(Decimal(10,0)),
`offerdatetime` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `settlementdate`);
                        
create table DispatchFcasReq2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`intervention` Decimal(2,0),
`genconid` String,
`regionid` String,
`bidtype` String,
`genconeffectivedate` Nullable(DateTime('Australia/Brisbane')),
`genconversionno` Nullable(Decimal(3,0)),
`marginalvalue` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`base_cost` Nullable(Decimal(18,8)),
`adjusted_cost` Nullable(Decimal(18,8)),
`estimated_cmpf` Nullable(Decimal(18,8)),
`estimated_crmpf` Nullable(Decimal(18,8)),
`recovery_factor_cmpf` Nullable(Decimal(18,8)),
`recovery_factor_crmpf` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `genconid`, `intervention`, `regionid`, `runno`, `settlementdate`);
                        
create table PriceloadPriceRevision1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`intervention` Decimal(2,0),
`regionid` String,
`bidtype` String,
`versionno` Decimal(3,0),
`rrp_new` Nullable(Decimal(15,5)),
`rrp_old` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `intervention`, `regionid`, `runno`, `settlementdate`, `versionno`);
                        
create table DispatchRegionsum4 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`regionid` String,
`dispatchinterval` Decimal(22,0),
`intervention` Decimal(2,0),
`totaldemand` Nullable(Decimal(15,5)),
`availablegeneration` Nullable(Decimal(15,5)),
`availableload` Nullable(Decimal(15,5)),
`demandforecast` Nullable(Decimal(15,5)),
`dispatchablegeneration` Nullable(Decimal(15,5)),
`dispatchableload` Nullable(Decimal(15,5)),
`netinterchange` Nullable(Decimal(15,5)),
`excessgeneration` Nullable(Decimal(15,5)),
`lower5mindispatch` Nullable(Decimal(15,5)),
`lower5minimport` Nullable(Decimal(15,5)),
`lower5minlocaldispatch` Nullable(Decimal(15,5)),
`lower5minlocalprice` Nullable(Decimal(15,5)),
`lower5minlocalreq` Nullable(Decimal(15,5)),
`lower5minprice` Nullable(Decimal(15,5)),
`lower5minreq` Nullable(Decimal(15,5)),
`lower5minsupplyprice` Nullable(Decimal(15,5)),
`lower60secdispatch` Nullable(Decimal(15,5)),
`lower60secimport` Nullable(Decimal(15,5)),
`lower60seclocaldispatch` Nullable(Decimal(15,5)),
`lower60seclocalprice` Nullable(Decimal(15,5)),
`lower60seclocalreq` Nullable(Decimal(15,5)),
`lower60secprice` Nullable(Decimal(15,5)),
`lower60secreq` Nullable(Decimal(15,5)),
`lower60secsupplyprice` Nullable(Decimal(15,5)),
`lower6secdispatch` Nullable(Decimal(15,5)),
`lower6secimport` Nullable(Decimal(15,5)),
`lower6seclocaldispatch` Nullable(Decimal(15,5)),
`lower6seclocalprice` Nullable(Decimal(15,5)),
`lower6seclocalreq` Nullable(Decimal(15,5)),
`lower6secprice` Nullable(Decimal(15,5)),
`lower6secreq` Nullable(Decimal(15,5)),
`lower6secsupplyprice` Nullable(Decimal(15,5)),
`raise5mindispatch` Nullable(Decimal(15,5)),
`raise5minimport` Nullable(Decimal(15,5)),
`raise5minlocaldispatch` Nullable(Decimal(15,5)),
`raise5minlocalprice` Nullable(Decimal(15,5)),
`raise5minlocalreq` Nullable(Decimal(15,5)),
`raise5minprice` Nullable(Decimal(15,5)),
`raise5minreq` Nullable(Decimal(15,5)),
`raise5minsupplyprice` Nullable(Decimal(15,5)),
`raise60secdispatch` Nullable(Decimal(15,5)),
`raise60secimport` Nullable(Decimal(15,5)),
`raise60seclocaldispatch` Nullable(Decimal(15,5)),
`raise60seclocalprice` Nullable(Decimal(15,5)),
`raise60seclocalreq` Nullable(Decimal(15,5)),
`raise60secprice` Nullable(Decimal(15,5)),
`raise60secreq` Nullable(Decimal(15,5)),
`raise60secsupplyprice` Nullable(Decimal(15,5)),
`raise6secdispatch` Nullable(Decimal(15,5)),
`raise6secimport` Nullable(Decimal(15,5)),
`raise6seclocaldispatch` Nullable(Decimal(15,5)),
`raise6seclocalprice` Nullable(Decimal(15,5)),
`raise6seclocalreq` Nullable(Decimal(15,5)),
`raise6secprice` Nullable(Decimal(15,5)),
`raise6secreq` Nullable(Decimal(15,5)),
`raise6secsupplyprice` Nullable(Decimal(15,5)),
`aggegatedispatcherror` Nullable(Decimal(15,5)),
`aggregatedispatcherror` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`initialsupply` Nullable(Decimal(15,5)),
`clearedsupply` Nullable(Decimal(15,5)),
`lowerregimport` Nullable(Decimal(15,5)),
`lowerreglocaldispatch` Nullable(Decimal(15,5)),
`lowerreglocalreq` Nullable(Decimal(15,5)),
`lowerregreq` Nullable(Decimal(15,5)),
`raiseregimport` Nullable(Decimal(15,5)),
`raisereglocaldispatch` Nullable(Decimal(15,5)),
`raisereglocalreq` Nullable(Decimal(15,5)),
`raiseregreq` Nullable(Decimal(15,5)),
`raise5minlocalviolation` Nullable(Decimal(15,5)),
`raisereglocalviolation` Nullable(Decimal(15,5)),
`raise60seclocalviolation` Nullable(Decimal(15,5)),
`raise6seclocalviolation` Nullable(Decimal(15,5)),
`lower5minlocalviolation` Nullable(Decimal(15,5)),
`lowerreglocalviolation` Nullable(Decimal(15,5)),
`lower60seclocalviolation` Nullable(Decimal(15,5)),
`lower6seclocalviolation` Nullable(Decimal(15,5)),
`raise5minviolation` Nullable(Decimal(15,5)),
`raiseregviolation` Nullable(Decimal(15,5)),
`raise60secviolation` Nullable(Decimal(15,5)),
`raise6secviolation` Nullable(Decimal(15,5)),
`lower5minviolation` Nullable(Decimal(15,5)),
`lowerregviolation` Nullable(Decimal(15,5)),
`lower60secviolation` Nullable(Decimal(15,5)),
`lower6secviolation` Nullable(Decimal(15,5)),
`raise6secactualavailability` Nullable(Decimal(16,6)),
`raise60secactualavailability` Nullable(Decimal(16,6)),
`raise5minactualavailability` Nullable(Decimal(16,6)),
`raiseregactualavailability` Nullable(Decimal(16,6)),
`lower6secactualavailability` Nullable(Decimal(16,6)),
`lower60secactualavailability` Nullable(Decimal(16,6)),
`lower5minactualavailability` Nullable(Decimal(16,6)),
`lowerregactualavailability` Nullable(Decimal(16,6)),
`lorsurplus` Nullable(Decimal(16,6)),
`lrcsurplus` Nullable(Decimal(16,6)),
`totalintermittentgeneration` Nullable(Decimal(15,5)),
`demand_and_nonschedgen` Nullable(Decimal(15,5)),
`uigf` Nullable(Decimal(15,5)),
`semischedule_clearedmw` Nullable(Decimal(15,5)),
`semischedule_compliancemw` Nullable(Decimal(15,5)),
`ss_solar_uigf` Nullable(Decimal(15,5)),
`ss_wind_uigf` Nullable(Decimal(15,5)),
`ss_solar_clearedmw` Nullable(Decimal(15,5)),
`ss_wind_clearedmw` Nullable(Decimal(15,5)),
`ss_solar_compliancemw` Nullable(Decimal(15,5)),
`ss_wind_compliancemw` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`dispatchinterval`, `intervention`, `regionid`, `runno`, `settlementdate`);
                        
create table DispatchOffertrk1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`duid` String,
`bidtype` String,
`bidsettlementdate` Nullable(DateTime('Australia/Brisbane')),
`bidofferdate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bidtype`, `duid`, `settlementdate`);
                        
create table DispatchBlockedConstraints1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`constraintid` String
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `runno`, `settlementdate`);
                        
create table DispatchInterconnection1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`intervention` Decimal(2,0),
`from_regionid` String,
`to_regionid` String,
`dispatchinterval` Nullable(Decimal(22,0)),
`irlf` Nullable(Decimal(15,5)),
`mwflow` Nullable(Decimal(16,6)),
`meteredmwflow` Nullable(Decimal(16,6)),
`from_region_mw_losses` Nullable(Decimal(16,6)),
`to_region_mw_losses` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`from_regionid`, `intervention`, `runno`, `settlementdate`, `to_regionid`);
                        
create table DispatchUnitSolution2 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`runno` Decimal(3,0),
`duid` String,
`tradetype` Nullable(Decimal(2,0)),
`dispatchinterval` Nullable(Decimal(22,0)),
`intervention` Decimal(2,0),
`connectionpointid` Nullable(String),
`dispatchmode` Nullable(Decimal(2,0)),
`agcstatus` Nullable(Decimal(2,0)),
`initialmw` Nullable(Decimal(15,5)),
`totalcleared` Nullable(Decimal(15,5)),
`rampdownrate` Nullable(Decimal(15,5)),
`rampuprate` Nullable(Decimal(15,5)),
`lower5min` Nullable(Decimal(15,5)),
`lower60sec` Nullable(Decimal(15,5)),
`lower6sec` Nullable(Decimal(15,5)),
`raise5min` Nullable(Decimal(15,5)),
`raise60sec` Nullable(Decimal(15,5)),
`raise6sec` Nullable(Decimal(15,5)),
`downepf` Nullable(Decimal(15,5)),
`upepf` Nullable(Decimal(15,5)),
`marginal5minvalue` Nullable(Decimal(15,5)),
`marginal60secvalue` Nullable(Decimal(15,5)),
`marginal6secvalue` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violation5mindegree` Nullable(Decimal(15,5)),
`violation60secdegree` Nullable(Decimal(15,5)),
`violation6secdegree` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`lowerreg` Nullable(Decimal(15,5)),
`raisereg` Nullable(Decimal(15,5)),
`availability` Nullable(Decimal(15,5)),
`raise6secflags` Nullable(Decimal(3,0)),
`raise60secflags` Nullable(Decimal(3,0)),
`raise5minflags` Nullable(Decimal(3,0)),
`raiseregflags` Nullable(Decimal(3,0)),
`lower6secflags` Nullable(Decimal(3,0)),
`lower60secflags` Nullable(Decimal(3,0)),
`lower5minflags` Nullable(Decimal(3,0)),
`lowerregflags` Nullable(Decimal(3,0)),
`raiseregavailability` Nullable(Decimal(15,5)),
`raiseregenablementmax` Nullable(Decimal(15,5)),
`raiseregenablementmin` Nullable(Decimal(15,5)),
`lowerregavailability` Nullable(Decimal(15,5)),
`lowerregenablementmax` Nullable(Decimal(15,5)),
`lowerregenablementmin` Nullable(Decimal(15,5)),
`raise6secactualavailability` Nullable(Decimal(16,6)),
`raise60secactualavailability` Nullable(Decimal(16,6)),
`raise5minactualavailability` Nullable(Decimal(16,6)),
`raiseregactualavailability` Nullable(Decimal(16,6)),
`lower6secactualavailability` Nullable(Decimal(16,6)),
`lower60secactualavailability` Nullable(Decimal(16,6)),
`lower5minactualavailability` Nullable(Decimal(16,6)),
`lowerregactualavailability` Nullable(Decimal(16,6)),
`semidispatchcap` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `intervention`, `runno`, `settlementdate`);
                        
create table DispatchUnitScada1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`duid` String,
`scadavalue` Nullable(Decimal(16,6))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `settlementdate`);
                        
create table VoltageInstructionTrack2 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`file_type` Nullable(String),
`version_datetime` DateTime('Australia/Brisbane'),
`se_datetime` Nullable(DateTime('Australia/Brisbane')),
`solution_category` Nullable(String),
`solution_status` Nullable(String),
`operating_mode` Nullable(String),
`operating_status` Nullable(String),
`est_expiry` Nullable(DateTime('Australia/Brisbane')),
`est_next_instruction` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`run_datetime`, `version_datetime`);
                        
create table VoltageInstructionInstruction2 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`ems_id` String,
`participantid` Nullable(String),
`station_id` Nullable(String),
`device_id` Nullable(String),
`device_type` Nullable(String),
`control_type` Nullable(String),
`target` Nullable(Decimal(15,0)),
`conforming` Nullable(Decimal(1,0)),
`instruction_summary` Nullable(String),
`version_datetime` DateTime('Australia/Brisbane'),
`instruction_sequence` Nullable(Decimal(4,0)),
`additional_notes` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`ems_id`, `run_datetime`, `version_datetime`);
                        
create table P5minUnitsolution3 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`connectionpointid` Nullable(String),
`tradetype` Nullable(Decimal(2,0)),
`agcstatus` Nullable(Decimal(2,0)),
`initialmw` Nullable(Decimal(15,5)),
`totalcleared` Nullable(Decimal(15,5)),
`rampdownrate` Nullable(Decimal(15,5)),
`rampuprate` Nullable(Decimal(15,5)),
`lower5min` Nullable(Decimal(15,5)),
`lower60sec` Nullable(Decimal(15,5)),
`lower6sec` Nullable(Decimal(15,5)),
`raise5min` Nullable(Decimal(15,5)),
`raise60sec` Nullable(Decimal(15,5)),
`raise6sec` Nullable(Decimal(15,5)),
`lowerreg` Nullable(Decimal(15,5)),
`raisereg` Nullable(Decimal(15,5)),
`availability` Nullable(Decimal(15,5)),
`raise6secflags` Nullable(Decimal(3,0)),
`raise60secflags` Nullable(Decimal(3,0)),
`raise5minflags` Nullable(Decimal(3,0)),
`raiseregflags` Nullable(Decimal(3,0)),
`lower6secflags` Nullable(Decimal(3,0)),
`lower60secflags` Nullable(Decimal(3,0)),
`lower5minflags` Nullable(Decimal(3,0)),
`lowerregflags` Nullable(Decimal(3,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`semidispatchcap` Nullable(Decimal(3,0)),
`intervention` Nullable(Decimal(2,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `interval_datetime`, `run_datetime`);
                        
create table P5minConstraintsolution6 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`constraintid` String,
`rhs` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`duid` Nullable(String),
`genconid_effectivedate` Nullable(DateTime('Australia/Brisbane')),
`genconid_versionno` Nullable(Decimal(22,0)),
`lhs` Nullable(Decimal(15,5)),
`intervention` Nullable(Decimal(2,0))
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `interval_datetime`, `run_datetime`);
                        
create table P5minLocalPrice1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`local_price_adjustment` Nullable(Decimal(10,2)),
`locally_constrained` Nullable(Decimal(1,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `interval_datetime`, `run_datetime`);
                        
create table P5minRegionsolution5 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`regionid` String,
`rrp` Nullable(Decimal(15,5)),
`rop` Nullable(Decimal(15,5)),
`excessgeneration` Nullable(Decimal(15,5)),
`raise6secrrp` Nullable(Decimal(15,5)),
`raise6secrop` Nullable(Decimal(15,5)),
`raise60secrrp` Nullable(Decimal(15,5)),
`raise60secrop` Nullable(Decimal(15,5)),
`raise5minrrp` Nullable(Decimal(15,5)),
`raise5minrop` Nullable(Decimal(15,5)),
`raiseregrrp` Nullable(Decimal(15,5)),
`raiseregrop` Nullable(Decimal(15,5)),
`lower6secrrp` Nullable(Decimal(15,5)),
`lower6secrop` Nullable(Decimal(15,5)),
`lower60secrrp` Nullable(Decimal(15,5)),
`lower60secrop` Nullable(Decimal(15,5)),
`lower5minrrp` Nullable(Decimal(15,5)),
`lower5minrop` Nullable(Decimal(15,5)),
`lowerregrrp` Nullable(Decimal(15,5)),
`lowerregrop` Nullable(Decimal(15,5)),
`totaldemand` Nullable(Decimal(15,5)),
`availablegeneration` Nullable(Decimal(15,5)),
`availableload` Nullable(Decimal(15,5)),
`demandforecast` Nullable(Decimal(15,5)),
`dispatchablegeneration` Nullable(Decimal(15,5)),
`dispatchableload` Nullable(Decimal(15,5)),
`netinterchange` Nullable(Decimal(15,5)),
`lower5mindispatch` Nullable(Decimal(15,5)),
`lower5minimport` Nullable(Decimal(15,5)),
`lower5minlocaldispatch` Nullable(Decimal(15,5)),
`lower5minlocalreq` Nullable(Decimal(15,5)),
`lower5minreq` Nullable(Decimal(15,5)),
`lower60secdispatch` Nullable(Decimal(15,5)),
`lower60secimport` Nullable(Decimal(15,5)),
`lower60seclocaldispatch` Nullable(Decimal(15,5)),
`lower60seclocalreq` Nullable(Decimal(15,5)),
`lower60secreq` Nullable(Decimal(15,5)),
`lower6secdispatch` Nullable(Decimal(15,5)),
`lower6secimport` Nullable(Decimal(15,5)),
`lower6seclocaldispatch` Nullable(Decimal(15,5)),
`lower6seclocalreq` Nullable(Decimal(15,5)),
`lower6secreq` Nullable(Decimal(15,5)),
`raise5mindispatch` Nullable(Decimal(15,5)),
`raise5minimport` Nullable(Decimal(15,5)),
`raise5minlocaldispatch` Nullable(Decimal(15,5)),
`raise5minlocalreq` Nullable(Decimal(15,5)),
`raise5minreq` Nullable(Decimal(15,5)),
`raise60secdispatch` Nullable(Decimal(15,5)),
`raise60secimport` Nullable(Decimal(15,5)),
`raise60seclocaldispatch` Nullable(Decimal(15,5)),
`raise60seclocalreq` Nullable(Decimal(15,5)),
`raise60secreq` Nullable(Decimal(15,5)),
`raise6secdispatch` Nullable(Decimal(15,5)),
`raise6secimport` Nullable(Decimal(15,5)),
`raise6seclocaldispatch` Nullable(Decimal(15,5)),
`raise6seclocalreq` Nullable(Decimal(15,5)),
`raise6secreq` Nullable(Decimal(15,5)),
`aggregatedispatcherror` Nullable(Decimal(15,5)),
`initialsupply` Nullable(Decimal(15,5)),
`clearedsupply` Nullable(Decimal(15,5)),
`lowerregimport` Nullable(Decimal(15,5)),
`lowerregdispatch` Nullable(Decimal(15,5)),
`lowerreglocaldispatch` Nullable(Decimal(15,5)),
`lowerreglocalreq` Nullable(Decimal(15,5)),
`lowerregreq` Nullable(Decimal(15,5)),
`raiseregimport` Nullable(Decimal(15,5)),
`raiseregdispatch` Nullable(Decimal(15,5)),
`raisereglocaldispatch` Nullable(Decimal(15,5)),
`raisereglocalreq` Nullable(Decimal(15,5)),
`raiseregreq` Nullable(Decimal(15,5)),
`raise5minlocalviolation` Nullable(Decimal(15,5)),
`raisereglocalviolation` Nullable(Decimal(15,5)),
`raise60seclocalviolation` Nullable(Decimal(15,5)),
`raise6seclocalviolation` Nullable(Decimal(15,5)),
`lower5minlocalviolation` Nullable(Decimal(15,5)),
`lowerreglocalviolation` Nullable(Decimal(15,5)),
`lower60seclocalviolation` Nullable(Decimal(15,5)),
`lower6seclocalviolation` Nullable(Decimal(15,5)),
`raise5minviolation` Nullable(Decimal(15,5)),
`raiseregviolation` Nullable(Decimal(15,5)),
`raise60secviolation` Nullable(Decimal(15,5)),
`raise6secviolation` Nullable(Decimal(15,5)),
`lower5minviolation` Nullable(Decimal(15,5)),
`lowerregviolation` Nullable(Decimal(15,5)),
`lower60secviolation` Nullable(Decimal(15,5)),
`lower6secviolation` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`totalintermittentgeneration` Nullable(Decimal(15,5)),
`demand_and_nonschedgen` Nullable(Decimal(15,5)),
`uigf` Nullable(Decimal(15,5)),
`semischedule_clearedmw` Nullable(Decimal(15,5)),
`semischedule_compliancemw` Nullable(Decimal(15,5)),
`intervention` Nullable(Decimal(2,0)),
`ss_solar_uigf` Nullable(Decimal(15,5)),
`ss_wind_uigf` Nullable(Decimal(15,5)),
`ss_solar_clearedmw` Nullable(Decimal(15,5)),
`ss_wind_clearedmw` Nullable(Decimal(15,5)),
`ss_solar_compliancemw` Nullable(Decimal(15,5)),
`ss_wind_compliancemw` Nullable(Decimal(15,5))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`, `run_datetime`);
                        
create table P5minBlockedConstraints1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`constraintid` String
)
ENGINE = MergeTree()
ORDER BY (`constraintid`, `run_datetime`);
                        
create table P5minCasesolution2 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`startinterval_datetime` Nullable(String),
`totalobjective` Nullable(Decimal(27,10)),
`nonphysicallosses` Nullable(Decimal(1,0)),
`totalareagenviolation` Nullable(Decimal(15,5)),
`totalinterconnectorviolation` Nullable(Decimal(15,5)),
`totalgenericviolation` Nullable(Decimal(15,5)),
`totalramprateviolation` Nullable(Decimal(15,5)),
`totalunitmwcapacityviolation` Nullable(Decimal(15,5)),
`total5minviolation` Nullable(Decimal(15,5)),
`totalregviolation` Nullable(Decimal(15,5)),
`total6secviolation` Nullable(Decimal(15,5)),
`total60secviolation` Nullable(Decimal(15,5)),
`totalenergyconstrviolation` Nullable(Decimal(15,5)),
`totalenergyofferviolation` Nullable(Decimal(15,5)),
`totalasprofileviolation` Nullable(Decimal(15,5)),
`totalfaststartviolation` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`intervention` Nullable(Decimal(2,0))
)
ENGINE = MergeTree()
ORDER BY (`run_datetime`);
                        
create table P5minInterconnectorsoln4 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`interconnectorid` String,
`interval_datetime` DateTime('Australia/Brisbane'),
`meteredmwflow` Nullable(Decimal(15,5)),
`mwflow` Nullable(Decimal(15,5)),
`mwlosses` Nullable(Decimal(15,5)),
`marginalvalue` Nullable(Decimal(15,5)),
`violationdegree` Nullable(Decimal(15,5)),
`mnsp` Nullable(Decimal(1,0)),
`exportlimit` Nullable(Decimal(15,5)),
`importlimit` Nullable(Decimal(15,5)),
`marginalloss` Nullable(Decimal(15,5)),
`exportgenconid` Nullable(String),
`importgenconid` Nullable(String),
`fcasexportlimit` Nullable(Decimal(15,5)),
`fcasimportlimit` Nullable(Decimal(15,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`local_price_adjustment_export` Nullable(Decimal(10,2)),
`locally_constrained_export` Nullable(Decimal(1,0)),
`local_price_adjustment_import` Nullable(Decimal(10,2)),
`locally_constrained_import` Nullable(Decimal(1,0)),
`intervention` Nullable(Decimal(2,0))
)
ENGINE = MergeTree()
ORDER BY (`interconnectorid`, `interval_datetime`, `run_datetime`);
                        
create table MrEvent1 (
file_id Uuid,
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`description` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`offer_cut_off_time` Nullable(DateTime('Australia/Brisbane')),
`settlement_complete` Nullable(Decimal(1,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`mr_date`, `regionid`);
                        
create table MrPerofferStack1 (
file_id Uuid,
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`version_datetime` DateTime('Australia/Brisbane'),
`stack_position` Decimal(3,0),
`periodid` Decimal(3,0),
`duid` Nullable(String),
`accepted_capacity` Nullable(Decimal(6,0)),
`deducted_capacity` Nullable(Decimal(6,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`mr_date`, `periodid`, `regionid`, `stack_position`, `version_datetime`);
                        
create table MrEventSchedule1 (
file_id Uuid,
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`version_datetime` DateTime('Australia/Brisbane'),
`demand_effectivedate` Nullable(DateTime('Australia/Brisbane')),
`demand_offerdate` Nullable(DateTime('Australia/Brisbane')),
`demand_versionno` Nullable(Decimal(3,0)),
`authorisedby` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`mr_date`, `regionid`, `version_datetime`);
                        
create table MrDayofferStack1 (
file_id Uuid,
`mr_date` DateTime('Australia/Brisbane'),
`regionid` String,
`version_datetime` DateTime('Australia/Brisbane'),
`stack_position` Decimal(3,0),
`duid` Nullable(String),
`authorised` Nullable(Decimal(1,0)),
`offer_settlementdate` Nullable(DateTime('Australia/Brisbane')),
`offer_offerdate` Nullable(DateTime('Australia/Brisbane')),
`offer_versionno` Nullable(Decimal(3,0)),
`offer_type` Nullable(String),
`laof` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`mr_date`, `regionid`, `stack_position`, `version_datetime`);
                        
create table OperationalDemandForecast1 (
file_id Uuid,
`interval_datetime` DateTime('Australia/Brisbane'),
`regionid` String,
`load_date` Nullable(DateTime('Australia/Brisbane')),
`operational_demand_poe10` Nullable(Decimal(15,2)),
`operational_demand_poe50` Nullable(Decimal(15,2)),
`operational_demand_poe90` Nullable(Decimal(15,2)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`);
                        
create table DemandIntermittentClusterAvailDay1 (
file_id Uuid,
`tradingdate` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`clusterid` String
)
ENGINE = MergeTree()
ORDER BY (`clusterid`, `duid`, `offerdatetime`, `tradingdate`);
                        
create table ForecastIntermittentGen1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`start_interval_datetime` DateTime('Australia/Brisbane'),
`end_interval_datetime` DateTime('Australia/Brisbane'),
`versionno` Nullable(Decimal(10,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `run_datetime`);
                        
create table DemandIntermittentGenLimitDay1 (
file_id Uuid,
`tradingdate` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`participantid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`authorisedbyuser` Nullable(String),
`authorisedbyparticipantid` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`duid`, `offerdatetime`, `tradingdate`);
                        
create table DemandPeriod1 (
file_id Uuid,
`effectivedate` Nullable(DateTime('Australia/Brisbane')),
`settlementdate` DateTime('Australia/Brisbane'),
`regionid` String,
`offerdate` DateTime('Australia/Brisbane'),
`periodid` Decimal(3,0),
`versionno` Decimal(3,0),
`resdemand` Nullable(Decimal(10,0)),
`demand90probability` Nullable(Decimal(10,0)),
`demand10probability` Nullable(Decimal(10,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`mr_schedule` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`offerdate`, `periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table ForecastIntermittentGenData1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`interval_datetime` DateTime('Australia/Brisbane'),
`powermean` Nullable(Decimal(9,3)),
`powerpoe50` Nullable(Decimal(9,3)),
`powerpoelow` Nullable(Decimal(9,3)),
`powerpoehigh` Nullable(Decimal(9,3)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `interval_datetime`, `run_datetime`);
                        
create table OperationalDemandActual1 (
file_id Uuid,
`interval_datetime` DateTime('Australia/Brisbane'),
`regionid` String,
`operational_demand` Nullable(Decimal(10,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`);
                        
create table DemandMtpasaIntermittentAvail1 (
file_id Uuid,
`tradingdate` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`clusterid` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`elements_unavailable` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`clusterid`, `duid`, `offerdatetime`, `tradingdate`);
                        
create table DemandTrk1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`regionid` String,
`offerdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`filename` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `offerdate`, `regionid`, `versionno`);
                        
create table RooftopActual2 (
file_id Uuid,
`interval_datetime` DateTime('Australia/Brisbane'),
`type` String,
`regionid` String,
`power` Nullable(Decimal(12,3)),
`qi` Nullable(Decimal(2,1)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`, `type`);
                        
create table DemandIntermittentGenLimit1 (
file_id Uuid,
`tradingdate` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`periodid` Decimal(3,0),
`uppermwlimit` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `offerdatetime`, `periodid`, `tradingdate`);
                        
create table DemandIntermittentClusterAvail1 (
file_id Uuid,
`tradingdate` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`clusterid` String,
`periodid` Decimal(3,0),
`elements_unavailable` Nullable(Decimal(3,0))
)
ENGINE = MergeTree()
ORDER BY (`clusterid`, `duid`, `offerdatetime`, `periodid`, `tradingdate`);
                        
create table DemandIntermittentDsPred1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`interval_datetime` DateTime('Australia/Brisbane'),
`origin` String,
`forecast_priority` Decimal(10,0),
`forecast_mean` Nullable(Decimal(18,8)),
`forecast_poe10` Nullable(Decimal(18,8)),
`forecast_poe50` Nullable(Decimal(18,8)),
`forecast_poe90` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`duid`, `forecast_priority`, `interval_datetime`, `offerdatetime`, `origin`, `run_datetime`);
                        
create table RooftopForecast1 (
file_id Uuid,
`version_datetime` DateTime('Australia/Brisbane'),
`regionid` String,
`interval_datetime` DateTime('Australia/Brisbane'),
`powermean` Nullable(Decimal(12,3)),
`powerpoe50` Nullable(Decimal(12,3)),
`powerpoelow` Nullable(Decimal(12,3)),
`powerpoehigh` Nullable(Decimal(12,3)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`interval_datetime`, `regionid`, `version_datetime`);
                        
create table DemandIntermittentDsRun1 (
file_id Uuid,
`run_datetime` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`origin` String,
`forecast_priority` Decimal(10,0),
`authorisedby` Nullable(String),
`comments` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`model` Nullable(String),
`participant_timestamp` Nullable(DateTime('Australia/Brisbane')),
`suppressed_aemo` Nullable(Decimal(1,0)),
`suppressed_participant` Nullable(Decimal(1,0)),
`transaction_id` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`duid`, `forecast_priority`, `offerdatetime`, `origin`, `run_datetime`);
                        
create table DemandMtpasaIntermittentLimit1 (
file_id Uuid,
`tradingdate` DateTime('Australia/Brisbane'),
`duid` String,
`offerdatetime` DateTime('Australia/Brisbane'),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`uppermwlimit` Nullable(Decimal(6,0)),
`authorisedbyuser` Nullable(String),
`authorisedbyparticipantid` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`duid`, `offerdatetime`, `tradingdate`);
                        
create table NetworkStaticrating1 (
file_id Uuid,
`substationid` String,
`equipmenttype` String,
`equipmentid` String,
`ratinglevel` String,
`applicationid` String,
`validfrom` DateTime('Australia/Brisbane'),
`validto` Nullable(DateTime('Australia/Brisbane')),
`ratingvalue` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`applicationid`, `equipmentid`, `equipmenttype`, `ratinglevel`, `substationid`, `validfrom`);
                        
create table NetworkEquipmentdetail1 (
file_id Uuid,
`substationid` String,
`equipmenttype` String,
`equipmentid` String,
`validfrom` DateTime('Australia/Brisbane'),
`validto` Nullable(DateTime('Australia/Brisbane')),
`voltage` Nullable(String),
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`equipmentid`, `equipmenttype`, `substationid`, `validfrom`);
                        
create table NetworkRating1 (
file_id Uuid,
`spd_id` String,
`validfrom` DateTime('Australia/Brisbane'),
`validto` Nullable(DateTime('Australia/Brisbane')),
`regionid` Nullable(String),
`substationid` Nullable(String),
`equipmenttype` Nullable(String),
`equipmentid` Nullable(String),
`ratinglevel` Nullable(String),
`isdynamic` Nullable(Decimal(1,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`spd_id`, `validfrom`);
                        
create table NetworkRealtimerating1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`spd_id` String,
`ratingvalue` Decimal(16,6)
)
ENGINE = MergeTree()
ORDER BY (`settlementdate`, `spd_id`);
                        
create table NetworkOutagedetail3 (
file_id Uuid,
`outageid` Decimal(15,0),
`substationid` String,
`equipmenttype` String,
`equipmentid` String,
`starttime` DateTime('Australia/Brisbane'),
`endtime` Nullable(DateTime('Australia/Brisbane')),
`submitteddate` Nullable(DateTime('Australia/Brisbane')),
`outagestatuscode` Nullable(String),
`resubmitreason` Nullable(String),
`resubmitoutageid` Nullable(Decimal(15,0)),
`recalltimeday` Nullable(Decimal(10,0)),
`recalltimenight` Nullable(Decimal(10,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`reason` Nullable(String),
`issecondary` Nullable(Decimal(1,0)),
`actual_starttime` Nullable(DateTime('Australia/Brisbane')),
`actual_endtime` Nullable(DateTime('Australia/Brisbane')),
`companyrefcode` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`equipmentid`, `equipmenttype`, `outageid`, `starttime`, `substationid`);
                        
create table NetworkOutagestatuscode1 (
file_id Uuid,
`outagestatuscode` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`outagestatuscode`);
                        
create table NetworkSubstationdetail1 (
file_id Uuid,
`substationid` String,
`validfrom` DateTime('Australia/Brisbane'),
`validto` Nullable(DateTime('Australia/Brisbane')),
`description` Nullable(String),
`regionid` Nullable(String),
`ownerid` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`substationid`, `validfrom`);
                        
create table NetworkOutageconstraintset1 (
file_id Uuid,
`outageid` Decimal(15,0),
`genconsetid` String,
`startinterval` Nullable(DateTime('Australia/Brisbane')),
`endinterval` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`genconsetid`, `outageid`);
                        
create table BillingConfigSecdepositProvision1 (
file_id Uuid,
`security_deposit_id` String,
`participantid` String,
`transaction_date` Nullable(DateTime('Australia/Brisbane')),
`maturity_contractyear` Nullable(Decimal(4,0)),
`maturity_weekno` Nullable(Decimal(3,0)),
`amount` Nullable(Decimal(18,8)),
`interest_rate` Nullable(Decimal(18,8)),
`interest_calc_type` Nullable(String),
`interest_acct_id` Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (`participantid`, `security_deposit_id`);
                        
create table BillingConfigGstBasClass1 (
file_id Uuid,
`bas_class` String,
`description` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bas_class`);
                        
create table BillingConfigBillingcalendar2 (
file_id Uuid,
`contractyear` Decimal(4,0),
`weekno` Decimal(3,0),
`startdate` Nullable(DateTime('Australia/Brisbane')),
`enddate` Nullable(DateTime('Australia/Brisbane')),
`preliminarystatementdate` Nullable(DateTime('Australia/Brisbane')),
`finalstatementdate` Nullable(DateTime('Australia/Brisbane')),
`paymentdate` Nullable(DateTime('Australia/Brisbane')),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`revision1_statementdate` Nullable(DateTime('Australia/Brisbane')),
`revision2_statementdate` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`contractyear`, `weekno`);
                        
create table BillingConfigGstTransactionClass1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`transaction_type` String,
`bas_class` String,
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bas_class`, `effectivedate`, `transaction_type`, `versionno`);
                        
create table BillingConfigGstTransactionType1 (
file_id Uuid,
`transaction_type` String,
`description` Nullable(String),
`gl_financialcode` Nullable(String),
`gl_tcode` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`transaction_type`);
                        
create table BillingConfigGstRate1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`bas_class` String,
`gst_rate` Nullable(Decimal(8,5)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`bas_class`, `effectivedate`, `versionno`);
                        
create table BillingConfigSecdepositInterestRate1 (
file_id Uuid,
`interest_acct_id` String,
`effectivedate` DateTime('Australia/Brisbane'),
`version_datetime` DateTime('Australia/Brisbane'),
`interest_rate` Nullable(Decimal(18,8))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `interest_acct_id`, `version_datetime`);
                        
create table MtpasaReservelimitRegion1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`version_datetime` DateTime('Australia/Brisbane'),
`reservelimitid` String,
`regionid` String,
`coef` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `regionid`, `reservelimitid`, `version_datetime`);
                        
create table ReserveDataReserve1 (
file_id Uuid,
`settlementdate` DateTime('Australia/Brisbane'),
`versionno` Decimal(3,0),
`regionid` String,
`periodid` Decimal(2,0),
`lower5min` Nullable(Decimal(6,0)),
`lower60sec` Nullable(Decimal(6,0)),
`lower6sec` Nullable(Decimal(6,0)),
`raise5min` Nullable(Decimal(6,0)),
`raise60sec` Nullable(Decimal(6,0)),
`raise6sec` Nullable(Decimal(6,0)),
`lastchanged` Nullable(DateTime('Australia/Brisbane')),
`pasareserve` Nullable(Decimal(6,0)),
`loadrejectionreservereq` Nullable(Decimal(10,0)),
`raisereg` Nullable(Decimal(6,0)),
`lowerreg` Nullable(Decimal(6,0)),
`lor1level` Nullable(Decimal(6,0)),
`lor2level` Nullable(Decimal(6,0))
)
ENGINE = MergeTree()
ORDER BY (`periodid`, `regionid`, `settlementdate`, `versionno`);
                        
create table MtpasaReservelimitSet1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`version_datetime` DateTime('Australia/Brisbane'),
`reservelimit_set_id` Nullable(String),
`description` Nullable(String),
`authoriseddate` Nullable(DateTime('Australia/Brisbane')),
`authorisedby` Nullable(String),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `version_datetime`);
                        
create table MtpasaReservelimit1 (
file_id Uuid,
`effectivedate` DateTime('Australia/Brisbane'),
`version_datetime` DateTime('Australia/Brisbane'),
`reservelimitid` String,
`description` Nullable(String),
`rhs` Nullable(Decimal(16,6)),
`lastchanged` Nullable(DateTime('Australia/Brisbane'))
)
ENGINE = MergeTree()
ORDER BY (`effectivedate`, `reservelimitid`, `version_datetime`);
                        