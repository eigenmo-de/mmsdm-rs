from typing import Optional
import pyarrow
import pyarrow.csv as pc

def ancilliary_services_contractagc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("crr", pyarrow.decimal128(4,0), True),
        pyarrow.field("crl", pyarrow.decimal128(4,0), True),
        pyarrow.field("rlprice", pyarrow.decimal128(10,2), True),
        pyarrow.field("ccprice", pyarrow.decimal128(10,2), True),
        pyarrow.field("bs", pyarrow.decimal128(10,2), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ancilliary_services_contractloadshed_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("lseprice", pyarrow.decimal128(6,2), True),
        pyarrow.field("mcpprice", pyarrow.decimal128(12,2), True),
        pyarrow.field("tenderedprice", pyarrow.decimal128(6,2), True),
        pyarrow.field("lscr", pyarrow.decimal128(6,2), True),
        pyarrow.field("ilscalingfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secbreakpoint", pyarrow.decimal128(9,6), True),
        pyarrow.field("lower60secmax", pyarrow.decimal128(9,6), True),
        pyarrow.field("lower6secbreakpoint", pyarrow.decimal128(9,6), True),
        pyarrow.field("lower6secmax", pyarrow.decimal128(9,6), True),
        pyarrow.field("raise60secbreakpoint", pyarrow.decimal128(9,6), True),
        pyarrow.field("raise60seccapacity", pyarrow.decimal128(9,6), True),
        pyarrow.field("raise60secmax", pyarrow.decimal128(9,6), True),
        pyarrow.field("raise6secbreakpoint", pyarrow.decimal128(9,6), True),
        pyarrow.field("raise6seccapacity", pyarrow.decimal128(9,6), True),
        pyarrow.field("raise6secmax", pyarrow.decimal128(9,6), True),
        pyarrow.field("price6secraisemandatory", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant6secraisemandatory", pyarrow.decimal128(9,6), True),
        pyarrow.field("price6secraisecontract", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant6secraisecontract", pyarrow.decimal128(9,6), True),
        pyarrow.field("price60secraisemandatory", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant60secraisemandatory", pyarrow.decimal128(9,6), True),
        pyarrow.field("price60secraisecontract", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant60secraisecontract", pyarrow.decimal128(9,6), True),
        pyarrow.field("price6seclowermandatory", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant6seclowermandatory", pyarrow.decimal128(9,6), True),
        pyarrow.field("price6seclowercontract", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant6seclowercontract", pyarrow.decimal128(9,6), True),
        pyarrow.field("price60seclowermandatory", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant60seclowermandatory", pyarrow.decimal128(9,6), True),
        pyarrow.field("price60seclowercontract", pyarrow.decimal128(16,6), True),
        pyarrow.field("quant60seclowercontract", pyarrow.decimal128(9,6), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("default_testingpayment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("service_start_date", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ancilliary_services_contractreactivepower_v4(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("synccompensation", pyarrow.large_utf8(), True),
        pyarrow.field("mvaraprice", pyarrow.decimal128(10,2), True),
        pyarrow.field("mvareprice", pyarrow.decimal128(10,2), True),
        pyarrow.field("mvargprice", pyarrow.decimal128(10,2), True),
        pyarrow.field("ccprice", pyarrow.decimal128(10,2), True),
        pyarrow.field("mta", pyarrow.decimal128(10,2), True),
        pyarrow.field("mtg", pyarrow.decimal128(10,2), True),
        pyarrow.field("mmca", pyarrow.decimal128(10,2), True),
        pyarrow.field("mmcg", pyarrow.decimal128(10,2), True),
        pyarrow.field("eu", pyarrow.decimal128(10,2), True),
        pyarrow.field("pp", pyarrow.decimal128(10,2), True),
        pyarrow.field("bs", pyarrow.decimal128(10,2), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("default_testingpayment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("service_start_date", pyarrow.timestamp('s'), True),
        pyarrow.field("availability_mwh_threshold", pyarrow.decimal128(18,8), True),
        pyarrow.field("mvar_threshold", pyarrow.decimal128(18,8), True),
        pyarrow.field("rebate_cap", pyarrow.decimal128(18,8), True),
        pyarrow.field("rebate_amount_per_mvar", pyarrow.decimal128(18,8), True),
        pyarrow.field("isrebateapplicable", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ancilliary_services_contractrestartservices_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("restarttype", pyarrow.decimal128(1,0), True),
        pyarrow.field("rcprice", pyarrow.decimal128(6,2), True),
        pyarrow.field("triptohouselevel", pyarrow.decimal128(5,0), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("default_testingpayment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("service_start_date", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ancilliary_services_contractrestartunits_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def asoffer_offeragcdata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("availability", pyarrow.decimal128(4,0), True),
        pyarrow.field("upperlimit", pyarrow.decimal128(4,0), True),
        pyarrow.field("lowerlimit", pyarrow.decimal128(4,0), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("agcup", pyarrow.decimal128(3,0), True),
        pyarrow.field("agcdown", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def asoffer_offerastrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def asoffer_offerlsheddata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("availableload", pyarrow.decimal128(4,0), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def asoffer_offerrestartdata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("availability", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def asoffer_offerrpowerdata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("availability", pyarrow.decimal128(3,0), True),
        pyarrow.field("mta", pyarrow.decimal128(6,0), True),
        pyarrow.field("mtg", pyarrow.decimal128(6,0), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bids_biddayoffer_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(22,0), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("dailyenergyconstraint", pyarrow.decimal128(12,6), True),
        pyarrow.field("rebidexplanation", pyarrow.large_utf8(), True),
        pyarrow.field("priceband1", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband2", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband3", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband4", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband5", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband6", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband7", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband8", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband9", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband10", pyarrow.decimal128(9,2), True),
        pyarrow.field("minimumload", pyarrow.decimal128(22,0), True),
        pyarrow.field("t1", pyarrow.decimal128(22,0), True),
        pyarrow.field("t2", pyarrow.decimal128(22,0), True),
        pyarrow.field("t3", pyarrow.decimal128(22,0), True),
        pyarrow.field("t4", pyarrow.decimal128(22,0), True),
        pyarrow.field("normalstatus", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("mr_factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("entrytype", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_event_time", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_aware_time", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_decision_time", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_category", pyarrow.large_utf8(), True),
        pyarrow.field("reference_id", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bids_bidofferfiletrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("filename", pyarrow.large_utf8(), False),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("transaction_id", pyarrow.large_utf8(), True),
        pyarrow.field("reference_id", pyarrow.large_utf8(), True),
        pyarrow.field("submission_timestamp", pyarrow.timestamp('s'), True),
        pyarrow.field("comments", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bids_bidofferperiod_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("maxavail", pyarrow.decimal128(8,3), True),
        pyarrow.field("fixedload", pyarrow.decimal128(8,3), True),
        pyarrow.field("rampuprate", pyarrow.int64(), True),
        pyarrow.field("rampdownrate", pyarrow.int64(), True),
        pyarrow.field("enablementmin", pyarrow.decimal128(8,3), True),
        pyarrow.field("enablementmax", pyarrow.decimal128(8,3), True),
        pyarrow.field("lowbreakpoint", pyarrow.decimal128(8,3), True),
        pyarrow.field("highbreakpoint", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail1", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail2", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail3", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail4", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail5", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail6", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail7", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail8", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail9", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail10", pyarrow.decimal128(8,3), True),
        pyarrow.field("pasaavailability", pyarrow.decimal128(8,3), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def offer_bidperoffer_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("periodid", pyarrow.decimal128(22,0), False),
        pyarrow.field("versionno", pyarrow.decimal128(22,0), True),
        pyarrow.field("maxavail", pyarrow.decimal128(12,6), True),
        pyarrow.field("fixedload", pyarrow.decimal128(12,6), True),
        pyarrow.field("rocup", pyarrow.decimal128(6,0), True),
        pyarrow.field("rocdown", pyarrow.decimal128(6,0), True),
        pyarrow.field("enablementmin", pyarrow.decimal128(6,0), True),
        pyarrow.field("enablementmax", pyarrow.decimal128(6,0), True),
        pyarrow.field("lowbreakpoint", pyarrow.decimal128(6,0), True),
        pyarrow.field("highbreakpoint", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail1", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail2", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail3", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail4", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail5", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail6", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail7", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail8", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail9", pyarrow.decimal128(22,0), True),
        pyarrow.field("bandavail10", pyarrow.decimal128(22,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("pasaavailability", pyarrow.decimal128(12,0), True),
        pyarrow.field("mr_capacity", pyarrow.decimal128(6,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bids_mnsp_bidofferperiod_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("linkid", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("maxavail", pyarrow.decimal128(8,3), True),
        pyarrow.field("fixedload", pyarrow.decimal128(8,3), True),
        pyarrow.field("rampuprate", pyarrow.int64(), True),
        pyarrow.field("bandavail1", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail2", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail3", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail4", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail5", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail6", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail7", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail8", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail9", pyarrow.decimal128(8,3), True),
        pyarrow.field("bandavail10", pyarrow.decimal128(8,3), True),
        pyarrow.field("pasaavailability", pyarrow.decimal128(8,3), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bids_mnsp_dayoffer_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("linkid", pyarrow.large_utf8(), False),
        pyarrow.field("entrytype", pyarrow.large_utf8(), True),
        pyarrow.field("rebidexplanation", pyarrow.large_utf8(), True),
        pyarrow.field("priceband1", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband2", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband3", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband4", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband5", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband6", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband7", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband8", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband9", pyarrow.decimal128(9,2), True),
        pyarrow.field("priceband10", pyarrow.decimal128(9,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("mr_factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("rebid_event_time", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_aware_time", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_decision_time", pyarrow.large_utf8(), True),
        pyarrow.field("rebid_category", pyarrow.large_utf8(), True),
        pyarrow.field("reference_id", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bid_mnsp_filetrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("filename", pyarrow.large_utf8(), False),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("ackfilename", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bid_mnsp_offertrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("filename", pyarrow.large_utf8(), False),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def bid_mnsp_peroffer_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("linkid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(22,0), False),
        pyarrow.field("maxavail", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail1", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail2", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail3", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail4", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail5", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail6", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail7", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail8", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail9", pyarrow.decimal128(6,0), True),
        pyarrow.field("bandavail10", pyarrow.decimal128(6,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("fixedload", pyarrow.decimal128(12,6), True),
        pyarrow.field("rampuprate", pyarrow.decimal128(6,0), True),
        pyarrow.field("pasaavailability", pyarrow.decimal128(12,0), True),
        pyarrow.field("mr_capacity", pyarrow.decimal128(6,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def offer_mtpasa_offerdata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("unitid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("energy", pyarrow.int64(), True),
        pyarrow.field("capacity1", pyarrow.int64(), True),
        pyarrow.field("capacity2", pyarrow.int64(), True),
        pyarrow.field("capacity3", pyarrow.int64(), True),
        pyarrow.field("capacity4", pyarrow.int64(), True),
        pyarrow.field("capacity5", pyarrow.int64(), True),
        pyarrow.field("capacity6", pyarrow.int64(), True),
        pyarrow.field("capacity7", pyarrow.int64(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def offer_mtpasa_offerfiletrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("filename", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_billingcalendar_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("preliminarystatementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("finalstatementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("paymentdate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("revision1_statementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("revision2_statementdate", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_gst_bas_class_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("bas_class", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_gst_rate_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("bas_class", pyarrow.large_utf8(), False),
        pyarrow.field("gst_rate", pyarrow.decimal128(8,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_gst_transaction_class_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("transaction_type", pyarrow.large_utf8(), False),
        pyarrow.field("bas_class", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_gst_transaction_type_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("transaction_type", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("gl_financialcode", pyarrow.large_utf8(), True),
        pyarrow.field("gl_tcode", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_secdeposit_interest_rate_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interest_acct_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interest_rate", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_config_secdeposit_provision_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("security_deposit_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("transaction_date", pyarrow.timestamp('s'), True),
        pyarrow.field("maturity_contractyear", pyarrow.decimal128(4,0), True),
        pyarrow.field("maturity_weekno", pyarrow.decimal128(3,0), True),
        pyarrow.field("amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("interest_rate", pyarrow.decimal128(18,8), True),
        pyarrow.field("interest_calc_type", pyarrow.large_utf8(), True),
        pyarrow.field("interest_acct_id", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_aspayments_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("raise6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("agc", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcascomp", pyarrow.decimal128(15,5), True),
        pyarrow.field("loadshed", pyarrow.decimal128(15,5), True),
        pyarrow.field("rgul", pyarrow.decimal128(15,5), True),
        pyarrow.field("rguu", pyarrow.decimal128(15,5), True),
        pyarrow.field("reactivepower", pyarrow.decimal128(15,5), True),
        pyarrow.field("systemrestart", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("lower5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereg", pyarrow.decimal128(15,5), True),
        pyarrow.field("availability_reactive", pyarrow.decimal128(18,8), True),
        pyarrow.field("availability_reactive_rbt", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_asrecovery_v7(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("raise6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("agc", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcascomp", pyarrow.decimal128(15,5), True),
        pyarrow.field("loadshed", pyarrow.decimal128(15,5), True),
        pyarrow.field("rgul", pyarrow.decimal128(15,5), True),
        pyarrow.field("rguu", pyarrow.decimal128(15,5), True),
        pyarrow.field("reactivepower", pyarrow.decimal128(15,5), True),
        pyarrow.field("systemrestart", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("raise6sec_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("agc_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcascomp_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("loadshed_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("rgul_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("rguu_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("reactivepower_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("systemrestart_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereg", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5min_gen", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise5min_gen", pyarrow.decimal128(16,6), True),
        pyarrow.field("lowerreg_gen", pyarrow.decimal128(16,6), True),
        pyarrow.field("raisereg_gen", pyarrow.decimal128(16,6), True),
        pyarrow.field("availability_reactive", pyarrow.decimal128(18,8), True),
        pyarrow.field("availability_reactive_rbt", pyarrow.decimal128(18,8), True),
        pyarrow.field("availability_reactive_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("availability_reactive_rbt_gen", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_cpdata_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("aggregateenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("purchases", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("mda", pyarrow.large_utf8(), False),
        pyarrow.field("afe", pyarrow.decimal128(18,8), True),
        pyarrow.field("dme", pyarrow.decimal128(18,8), True),
        pyarrow.field("ufea", pyarrow.decimal128(18,8), True),
        pyarrow.field("age", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_daytrk_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_fees_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("rate", pyarrow.decimal128(15,5), True),
        pyarrow.field("energy", pyarrow.decimal128(16,6), True),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("participantcategoryid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_financialadjustments_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("participanttype", pyarrow.large_utf8(), True),
        pyarrow.field("adjustmentitem", pyarrow.large_utf8(), False),
        pyarrow.field("amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("financialcode", pyarrow.decimal128(10,0), True),
        pyarrow.field("bas_class", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_gendata_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("stationid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("aggregateenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("sales", pyarrow.decimal128(16,6), True),
        pyarrow.field("purchases", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("purchasedenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("mda", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_interresidues_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("allocation", pyarrow.decimal128(6,3), True),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_intraresidues_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("allocation", pyarrow.decimal128(6,3), True),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_iraucsurplus_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(2,0), False),
        pyarrow.field("residueyear", pyarrow.decimal128(4,0), True),
        pyarrow.field("quarter", pyarrow.decimal128(2,0), True),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("totalresidues", pyarrow.decimal128(15,5), True),
        pyarrow.field("adjustment", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_iraucsurplussum_v7(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("residueyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(2,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("auctionfees", pyarrow.decimal128(15,5), True),
        pyarrow.field("actualpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("auctionfees_gst", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True),
        pyarrow.field("negative_residues", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_irfm_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("irfmpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_irnspsurplus_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(2,0), False),
        pyarrow.field("residueyear", pyarrow.decimal128(4,0), True),
        pyarrow.field("quarter", pyarrow.decimal128(2,0), True),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("totalresidues", pyarrow.decimal128(15,5), True),
        pyarrow.field("adjustment", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_irnspsurplussum_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("residueyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(2,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("auctionfees", pyarrow.decimal128(15,5), True),
        pyarrow.field("auctionfees_gst", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_irpartsurplus_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(2,0), False),
        pyarrow.field("residueyear", pyarrow.decimal128(4,0), True),
        pyarrow.field("quarter", pyarrow.decimal128(2,0), True),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("totalresidues", pyarrow.decimal128(15,5), True),
        pyarrow.field("adjustment", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("actualpayment", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_irpartsurplussum_v7(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("residueyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(2,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("auctionfees", pyarrow.decimal128(15,5), True),
        pyarrow.field("actualpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("auctionfees_gst", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True),
        pyarrow.field("auctionfees_totalgross_adj", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_prioradjustments_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("adjcontractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("adjweekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("adjbillrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("prevamount", pyarrow.decimal128(15,5), True),
        pyarrow.field("adjamount", pyarrow.decimal128(15,5), True),
        pyarrow.field("irn", pyarrow.decimal128(15,5), True),
        pyarrow.field("irp", pyarrow.decimal128(15,5), True),
        pyarrow.field("interestamount", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("irsr_prevamount", pyarrow.decimal128(15,5), True),
        pyarrow.field("irsr_adjamount", pyarrow.decimal128(15,5), True),
        pyarrow.field("irsr_interestamount", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_realloc_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("counterparty", pyarrow.large_utf8(), False),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_realloc_detail_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("counterparty", pyarrow.large_utf8(), False),
        pyarrow.field("reallocationid", pyarrow.large_utf8(), False),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_regionexports_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("exportto", pyarrow.large_utf8(), False),
        pyarrow.field("energy", pyarrow.decimal128(16,6), True),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("surplusenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_regionfigures_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("energyout", pyarrow.decimal128(16,6), True),
        pyarrow.field("valueout", pyarrow.decimal128(16,6), True),
        pyarrow.field("energypurchased", pyarrow.decimal128(16,6), True),
        pyarrow.field("valuepurchased", pyarrow.decimal128(16,6), True),
        pyarrow.field("excessgen", pyarrow.decimal128(16,6), True),
        pyarrow.field("reservetrading", pyarrow.decimal128(16,6), True),
        pyarrow.field("intcompo", pyarrow.decimal128(16,6), True),
        pyarrow.field("adminpricecompo", pyarrow.decimal128(16,6), True),
        pyarrow.field("settsurplus", pyarrow.decimal128(16,6), True),
        pyarrow.field("aspayment", pyarrow.decimal128(16,6), True),
        pyarrow.field("poolfees", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_regionimports_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("importfrom", pyarrow.large_utf8(), False),
        pyarrow.field("energy", pyarrow.decimal128(16,6), True),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("surplusenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_runtrk_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("adj_cleared", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("postdate", pyarrow.timestamp('s'), True),
        pyarrow.field("postby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("receiptpostdate", pyarrow.timestamp('s'), True),
        pyarrow.field("receiptpostby", pyarrow.large_utf8(), True),
        pyarrow.field("paymentpostdate", pyarrow.timestamp('s'), True),
        pyarrow.field("paymentpostby", pyarrow.large_utf8(), True),
        pyarrow.field("shortfall", pyarrow.decimal128(16,6), True),
        pyarrow.field("makeup", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_smelterreduction_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(22,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(22,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(22,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("rate1", pyarrow.decimal128(15,6), True),
        pyarrow.field("ra1", pyarrow.decimal128(15,6), True),
        pyarrow.field("rate2", pyarrow.decimal128(15,6), True),
        pyarrow.field("ra2", pyarrow.decimal128(15,6), True),
        pyarrow.field("te", pyarrow.decimal128(15,6), True),
        pyarrow.field("pcsd", pyarrow.decimal128(15,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_apc_compensation_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.int64(), False),
        pyarrow.field("weekno", pyarrow.int64(), False),
        pyarrow.field("billrunno", pyarrow.int64(), False),
        pyarrow.field("apeventid", pyarrow.int64(), False),
        pyarrow.field("claimid", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("compensation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("event_type", pyarrow.large_utf8(), True),
        pyarrow.field("compensation_type", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_apc_recovery_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.int64(), False),
        pyarrow.field("weekno", pyarrow.int64(), False),
        pyarrow.field("billrunno", pyarrow.int64(), False),
        pyarrow.field("apeventid", pyarrow.int64(), False),
        pyarrow.field("claimid", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("eligibility_start_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("eligibility_end_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("participant_demand", pyarrow.decimal128(18,8), True),
        pyarrow.field("region_demand", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_billing_co2e_publication_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.int64(), False),
        pyarrow.field("weekno", pyarrow.int64(), False),
        pyarrow.field("billrunno", pyarrow.int64(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("sentoutenergy", pyarrow.decimal128(18,8), True),
        pyarrow.field("generatoremissions", pyarrow.decimal128(18,8), True),
        pyarrow.field("intensityindex", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_billing_co2e_publication_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.int64(), False),
        pyarrow.field("weekno", pyarrow.int64(), False),
        pyarrow.field("billrunno", pyarrow.int64(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_daily_energy_summary_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("customer_energy_purchased", pyarrow.decimal128(18,8), True),
        pyarrow.field("generator_energy_sold", pyarrow.decimal128(18,8), True),
        pyarrow.field("generator_energy_purchased", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_direction_reconciliatn_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.int64(), False),
        pyarrow.field("weekno", pyarrow.int64(), False),
        pyarrow.field("billrunno", pyarrow.int64(), False),
        pyarrow.field("direction_id", pyarrow.large_utf8(), False),
        pyarrow.field("direction_desc", pyarrow.large_utf8(), True),
        pyarrow.field("direction_start_date", pyarrow.timestamp('s'), True),
        pyarrow.field("direction_end_date", pyarrow.timestamp('s'), True),
        pyarrow.field("compensation_amount", pyarrow.decimal128(16,6), True),
        pyarrow.field("independent_expert_fee", pyarrow.decimal128(16,6), True),
        pyarrow.field("interest_amount", pyarrow.decimal128(16,6), True),
        pyarrow.field("cra", pyarrow.decimal128(16,6), True),
        pyarrow.field("nem_fee_id", pyarrow.large_utf8(), True),
        pyarrow.field("nem_fixed_fee_amount", pyarrow.decimal128(16,6), True),
        pyarrow.field("mkt_customer_perc", pyarrow.decimal128(16,6), True),
        pyarrow.field("generator_perc", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_billing_direction_recon_other_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.int64(), False),
        pyarrow.field("weekno", pyarrow.int64(), False),
        pyarrow.field("billrunno", pyarrow.int64(), False),
        pyarrow.field("direction_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("direction_desc", pyarrow.large_utf8(), True),
        pyarrow.field("direction_type_id", pyarrow.large_utf8(), True),
        pyarrow.field("direction_start_date", pyarrow.timestamp('s'), True),
        pyarrow.field("direction_end_date", pyarrow.timestamp('s'), True),
        pyarrow.field("direction_start_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("direction_end_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("compensation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("interest_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("independent_expert_fee", pyarrow.decimal128(18,8), True),
        pyarrow.field("cra", pyarrow.decimal128(18,8), True),
        pyarrow.field("regional_customer_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("regional_generator_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("regional_benefit_factor", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_eftshortfall_amount_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("shortfall_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("shortfall", pyarrow.decimal128(18,8), True),
        pyarrow.field("shortfall_company_id", pyarrow.large_utf8(), True),
        pyarrow.field("company_shortfall_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("participant_net_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("company_net_energy", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_eftshortfall_detail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("transaction_type", pyarrow.large_utf8(), False),
        pyarrow.field("amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_gst_detail_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("bas_class", pyarrow.large_utf8(), False),
        pyarrow.field("transaction_type", pyarrow.large_utf8(), False),
        pyarrow.field("gst_exclusive_amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("gst_amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_gst_summary_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("bas_class", pyarrow.large_utf8(), False),
        pyarrow.field("gst_exclusive_amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("gst_amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_mr_payment_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("mr_amount", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_mr_recovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("mr_amount", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_mr_shortfall_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("age", pyarrow.decimal128(16,6), True),
        pyarrow.field("rsa", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_mr_summary_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("total_payments", pyarrow.decimal128(16,6), True),
        pyarrow.field("total_recovery", pyarrow.decimal128(16,6), True),
        pyarrow.field("total_rsa", pyarrow.decimal128(16,6), True),
        pyarrow.field("aage", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_nmas_tst_payments_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_nmas_tst_recovery_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("rbf", pyarrow.decimal128(18,8), True),
        pyarrow.field("test_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_start_date", pyarrow.timestamp('s'), True),
        pyarrow.field("recovery_end_date", pyarrow.timestamp('s'), True),
        pyarrow.field("participant_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("region_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("nem_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("customer_proportion", pyarrow.decimal128(18,8), True),
        pyarrow.field("generator_proportion", pyarrow.decimal128(18,8), True),
        pyarrow.field("participant_generation", pyarrow.decimal128(18,8), True),
        pyarrow.field("nem_generation", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_nmas_tst_recvry_rbf_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("rbf", pyarrow.decimal128(18,8), True),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_nmas_tst_recvry_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("recovery_contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("recovery_weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("recovery_billrunno", pyarrow.decimal128(3,0), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_secdeposit_application_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("application_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_secdep_interest_pay_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("security_deposit_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interest_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("interest_calc_type", pyarrow.large_utf8(), True),
        pyarrow.field("interest_acct_id", pyarrow.large_utf8(), True),
        pyarrow.field("interest_rate", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_secdep_interest_rate_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interest_acct_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("interest_rate", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_reservetraderpayment_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("payment_id", pyarrow.decimal128(3,0), False),
        pyarrow.field("payment_type", pyarrow.large_utf8(), True),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_reservetraderrecovery_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(3,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("publication_id", pyarrow.large_utf8(), False),
        pyarrow.field("payment_id", pyarrow.decimal128(3,0), False),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participant_demand", pyarrow.decimal128(18,8), True),
        pyarrow.field("region_demand", pyarrow.decimal128(18,8), True),
        pyarrow.field("eligibility_start_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("eligibility_end_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def billing_whitehole_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(22,0), False),
        pyarrow.field("weekno", pyarrow.decimal128(22,0), False),
        pyarrow.field("billrunno", pyarrow.decimal128(22,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("nl", pyarrow.decimal128(15,6), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,6), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,6), True),
        pyarrow.field("whiteholepayment", pyarrow.decimal128(15,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def operational_demand_actual_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("operational_demand", pyarrow.decimal128(10,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("operational_demand_adjustment", pyarrow.decimal128(10,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def operational_demand_forecast_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("load_date", pyarrow.timestamp('s'), True),
        pyarrow.field("operational_demand_poe10", pyarrow.decimal128(15,2), True),
        pyarrow.field("operational_demand_poe50", pyarrow.decimal128(15,2), True),
        pyarrow.field("operational_demand_poe90", pyarrow.decimal128(15,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_intermittent_cluster_avail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("clusterid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("elements_unavailable", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_intermittent_cluster_avail_day_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("clusterid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_intermittent_ds_pred_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("origin", pyarrow.large_utf8(), False),
        pyarrow.field("forecast_priority", pyarrow.decimal128(10,0), False),
        pyarrow.field("forecast_mean", pyarrow.decimal128(18,8), True),
        pyarrow.field("forecast_poe10", pyarrow.decimal128(18,8), True),
        pyarrow.field("forecast_poe50", pyarrow.decimal128(18,8), True),
        pyarrow.field("forecast_poe90", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_intermittent_ds_run_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("origin", pyarrow.large_utf8(), False),
        pyarrow.field("forecast_priority", pyarrow.decimal128(10,0), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("comments", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("model", pyarrow.large_utf8(), True),
        pyarrow.field("participant_timestamp", pyarrow.timestamp('s'), True),
        pyarrow.field("suppressed_aemo", pyarrow.decimal128(1,0), True),
        pyarrow.field("suppressed_participant", pyarrow.decimal128(1,0), True),
        pyarrow.field("transaction_id", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def forecast_intermittent_gen_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("start_interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("end_interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(10,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def forecast_intermittent_gen_data_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("powermean", pyarrow.decimal128(9,3), True),
        pyarrow.field("powerpoe50", pyarrow.decimal128(9,3), True),
        pyarrow.field("powerpoelow", pyarrow.decimal128(9,3), True),
        pyarrow.field("powerpoehigh", pyarrow.decimal128(9,3), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_intermittent_gen_limit_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("uppermwlimit", pyarrow.int64(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_intermittent_gen_limit_day_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedbyuser", pyarrow.large_utf8(), True),
        pyarrow.field("authorisedbyparticipantid", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_mtpasa_intermittent_avail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("clusterid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("elements_unavailable", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_mtpasa_intermittent_limit_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("tradingdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("uppermwlimit", pyarrow.int64(), True),
        pyarrow.field("authorisedbyuser", pyarrow.large_utf8(), True),
        pyarrow.field("authorisedbyparticipantid", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_period_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("resdemand", pyarrow.decimal128(10,0), True),
        pyarrow.field("demand90probability", pyarrow.decimal128(10,0), True),
        pyarrow.field("demand10probability", pyarrow.decimal128(10,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("mr_schedule", pyarrow.decimal128(6,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def demand_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def rooftop_actual_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("type", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("power", pyarrow.decimal128(12,3), True),
        pyarrow.field("qi", pyarrow.decimal128(2,1), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def rooftop_forecast_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("powermean", pyarrow.decimal128(12,3), True),
        pyarrow.field("powerpoe50", pyarrow.decimal128(12,3), True),
        pyarrow.field("powerpoelow", pyarrow.decimal128(12,3), True),
        pyarrow.field("powerpoehigh", pyarrow.decimal128(12,3), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def priceload_constraintrelaxation_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("rhs", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_blocked_constraints_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_case_solution_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("casesubtype", pyarrow.large_utf8(), True),
        pyarrow.field("solutionstatus", pyarrow.decimal128(2,0), True),
        pyarrow.field("spdversion", pyarrow.large_utf8(), True),
        pyarrow.field("nonphysicallosses", pyarrow.decimal128(1,0), True),
        pyarrow.field("totalobjective", pyarrow.decimal128(27,10), True),
        pyarrow.field("totalareagenviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalinterconnectorviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalgenericviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalramprateviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalunitmwcapacityviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalasprofileviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalfaststartviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalenergyofferviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("switchruninitialstatus", pyarrow.decimal128(1,0), True),
        pyarrow.field("switchrunbeststatus", pyarrow.decimal128(1,0), True),
        pyarrow.field("switchrunbeststatus_int", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_constraint_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("dispatchinterval", pyarrow.decimal128(22,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("rhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("genconid_effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("genconid_versionno", pyarrow.decimal128(22,0), True),
        pyarrow.field("lhs", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_interconnectorres_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("dispatchinterval", pyarrow.decimal128(22,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("meteredmwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwlosses", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("exportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("importlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalloss", pyarrow.decimal128(15,5), True),
        pyarrow.field("exportgenconid", pyarrow.large_utf8(), True),
        pyarrow.field("importgenconid", pyarrow.large_utf8(), True),
        pyarrow.field("fcasexportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcasimportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("local_price_adjustment_export", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained_export", pyarrow.decimal128(1,0), True),
        pyarrow.field("local_price_adjustment_import", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained_import", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_unit_solution_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("tradetype", pyarrow.decimal128(2,0), True),
        pyarrow.field("dispatchinterval", pyarrow.decimal128(22,0), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True),
        pyarrow.field("dispatchmode", pyarrow.decimal128(2,0), True),
        pyarrow.field("agcstatus", pyarrow.decimal128(2,0), True),
        pyarrow.field("initialmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalcleared", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampdownrate", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampuprate", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("downepf", pyarrow.decimal128(15,5), True),
        pyarrow.field("upepf", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginal5minvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginal60secvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginal6secvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violation5mindegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("violation60secdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("violation6secdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereg", pyarrow.decimal128(15,5), True),
        pyarrow.field("availability", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise60secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise5minflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raiseregflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower6secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower60secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower5minflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lowerregflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raiseregavailability", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregenablementmax", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregenablementmin", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregavailability", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregenablementmax", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregenablementmin", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raiseregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lowerregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("semidispatchcap", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_offertrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("bidsettlementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("bidofferdate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_price_v4(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("dispatchinterval", pyarrow.large_utf8(), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep", pyarrow.decimal128(15,5), True),
        pyarrow.field("rop", pyarrow.decimal128(15,5), True),
        pyarrow.field("apcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("marketsuspendedflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("raise6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("raiseregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("lowerregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregapcflag", pyarrow.decimal128(3,0), True),
        pyarrow.field("price_status", pyarrow.large_utf8(), True),
        pyarrow.field("pre_ap_energy_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_raise6_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_raise60_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_raise5min_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_raisereg_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_lower6_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_lower60_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_lower5min_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("pre_ap_lowerreg_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_energy_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_raise6_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_raise60_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_raise5min_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_raisereg_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_lower6_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_lower60_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_lower5min_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_pre_ap_lowerreg_price", pyarrow.decimal128(15,5), True),
        pyarrow.field("ocd_status", pyarrow.large_utf8(), True),
        pyarrow.field("mii_status", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_regionsum_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("dispatchinterval", pyarrow.decimal128(22,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("totaldemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("availablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("availableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("demandforecast", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("netinterchange", pyarrow.decimal128(15,5), True),
        pyarrow.field("excessgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("aggegatedispatcherror", pyarrow.decimal128(15,5), True),
        pyarrow.field("aggregatedispatcherror", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("initialsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raiseregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lowerregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lorsurplus", pyarrow.decimal128(16,6), True),
        pyarrow.field("lrcsurplus", pyarrow.decimal128(16,6), True),
        pyarrow.field("totalintermittentgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("demand_and_nonschedgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("semischedule_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("semischedule_compliancemw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_compliancemw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_compliancemw", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def priceload_constraint_fcas_ocd_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.int64(), False),
        pyarrow.field("intervention", pyarrow.int64(), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("rhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_fcas_req_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("genconeffectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("genconversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("base_cost", pyarrow.decimal128(18,8), True),
        pyarrow.field("adjusted_cost", pyarrow.decimal128(18,8), True),
        pyarrow.field("estimated_cmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("estimated_crmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_factor_cmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_factor_crmpf", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_interconnection_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("from_regionid", pyarrow.large_utf8(), False),
        pyarrow.field("to_regionid", pyarrow.large_utf8(), False),
        pyarrow.field("dispatchinterval", pyarrow.decimal128(22,0), True),
        pyarrow.field("irlf", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow", pyarrow.decimal128(16,6), True),
        pyarrow.field("meteredmwflow", pyarrow.decimal128(16,6), True),
        pyarrow.field("from_region_mw_losses", pyarrow.decimal128(16,6), True),
        pyarrow.field("to_region_mw_losses", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_local_price_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("local_price_adjustment", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_mnspbidtrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("linkid", pyarrow.large_utf8(), False),
        pyarrow.field("offersettlementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offereffectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_mr_schedule_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), True),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def priceload_price_revision_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("rrp_new", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp_old", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_unit_conformance_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("totalcleared", pyarrow.decimal128(16,6), True),
        pyarrow.field("actualmw", pyarrow.decimal128(16,6), True),
        pyarrow.field("roc", pyarrow.decimal128(16,6), True),
        pyarrow.field("availability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(16,6), True),
        pyarrow.field("raisereg", pyarrow.decimal128(16,6), True),
        pyarrow.field("striglm", pyarrow.decimal128(16,6), True),
        pyarrow.field("ltriglm", pyarrow.decimal128(16,6), True),
        pyarrow.field("mwerror", pyarrow.decimal128(16,6), True),
        pyarrow.field("max_mwerror", pyarrow.decimal128(16,6), True),
        pyarrow.field("lecount", pyarrow.int64(), True),
        pyarrow.field("secount", pyarrow.int64(), True),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("participant_status_action", pyarrow.large_utf8(), True),
        pyarrow.field("operating_mode", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_unit_scada_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("scadavalue", pyarrow.decimal128(16,6), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_intermittent_forecast_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("origin", pyarrow.large_utf8(), True),
        pyarrow.field("forecast_priority", pyarrow.decimal128(10,0), True),
        pyarrow.field("offerdatetime", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def dispatch_negative_residue_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("nrm_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("directional_interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("nrm_activated_flag", pyarrow.decimal128(1,0), True),
        pyarrow.field("cumul_negresidue_amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("cumul_negresidue_prev_ti", pyarrow.decimal128(15,5), True),
        pyarrow.field("negresidue_current_ti", pyarrow.decimal128(15,5), True),
        pyarrow.field("negresidue_pd_next_ti", pyarrow.decimal128(15,5), True),
        pyarrow.field("price_revision", pyarrow.large_utf8(), True),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("event_activated_di", pyarrow.timestamp('s'), True),
        pyarrow.field("event_deactivated_di", pyarrow.timestamp('s'), True),
        pyarrow.field("di_notbinding_count", pyarrow.decimal128(2,0), True),
        pyarrow.field("di_violated_count", pyarrow.decimal128(2,0), True),
        pyarrow.field("nrmconstraint_blocked_flag", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ap_apevent_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("apeventid", pyarrow.decimal128(22,0), False),
        pyarrow.field("effectivefrominterval", pyarrow.timestamp('s'), True),
        pyarrow.field("effectivetointerval", pyarrow.timestamp('s'), True),
        pyarrow.field("reason", pyarrow.large_utf8(), True),
        pyarrow.field("startauthorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("startauthoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("endauthorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("endauthoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ap_apeventregion_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("apeventid", pyarrow.decimal128(22,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("energyapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("raise6secapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("raise60secapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("raise5minapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("raiseregapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("lower6secapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("lower60secapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("lower5minapflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("lowerregapflag", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_irfmamount_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("irfmid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(4,0), False),
        pyarrow.field("amount", pyarrow.decimal128(15,5), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_irfmevents_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("irfmid", pyarrow.large_utf8(), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("startperiod", pyarrow.decimal128(3,0), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("endperiod", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_market_suspend_regime_sum_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("suspension_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("start_interval", pyarrow.timestamp('s'), False),
        pyarrow.field("end_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("pricing_regime", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_market_suspend_region_sum_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("suspension_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("initial_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("end_region_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("end_suspension_interval", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_market_suspend_schedule_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("day_type", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("energy_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("r6_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("r60_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("r5_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("rreg_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("l6_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("l60_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("l5_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lreg_rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_market_suspend_schedule_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("source_start_date", pyarrow.timestamp('s'), True),
        pyarrow.field("source_end_date", pyarrow.timestamp('s'), True),
        pyarrow.field("comments", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def force_majeure_overriderrp_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), False),
        pyarrow.field("startperiod", pyarrow.decimal128(3,0), False),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("endperiod", pyarrow.decimal128(3,0), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,0), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("authorisestart", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseend", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ap_regionapc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def ap_regionapcintervals_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("apcvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("apctype", pyarrow.decimal128(3,0), True),
        pyarrow.field("fcasapcvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("apfvalue", pyarrow.decimal128(16,6), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def gd_instruct_gdinstruct_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("stationid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("id", pyarrow.decimal128(22,0), False),
        pyarrow.field("instructiontypeid", pyarrow.large_utf8(), True),
        pyarrow.field("instructionsubtypeid", pyarrow.large_utf8(), True),
        pyarrow.field("instructionclassid", pyarrow.large_utf8(), True),
        pyarrow.field("reason", pyarrow.large_utf8(), True),
        pyarrow.field("instlevel", pyarrow.decimal128(6,0), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("issuedtime", pyarrow.timestamp('s'), True),
        pyarrow.field("targettime", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def gd_instruct_instructionsubtype_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("instructiontypeid", pyarrow.large_utf8(), False),
        pyarrow.field("instructionsubtypeid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def gd_instruct_instructiontype_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("instructiontypeid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def generic_constraint_emsmaster_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("spd_id", pyarrow.large_utf8(), False),
        pyarrow.field("spd_type", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("grouping_id", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def gencondata_null_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("constrainttype", pyarrow.large_utf8(), True),
        pyarrow.field("constraintvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("genericconstraintweight", pyarrow.decimal128(16,6), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("dynamicrhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("dispatch", pyarrow.large_utf8(), True),
        pyarrow.field("predispatch", pyarrow.large_utf8(), True),
        pyarrow.field("stpasa", pyarrow.large_utf8(), True),
        pyarrow.field("mtpasa", pyarrow.large_utf8(), True),
        pyarrow.field("impact", pyarrow.large_utf8(), True),
        pyarrow.field("source", pyarrow.large_utf8(), True),
        pyarrow.field("limittype", pyarrow.large_utf8(), True),
        pyarrow.field("reason", pyarrow.large_utf8(), True),
        pyarrow.field("modifications", pyarrow.large_utf8(), True),
        pyarrow.field("additionalnotes", pyarrow.large_utf8(), True),
        pyarrow.field("p5min_scope_override", pyarrow.large_utf8(), True),
        pyarrow.field("lrc", pyarrow.large_utf8(), True),
        pyarrow.field("lor", pyarrow.large_utf8(), True),
        pyarrow.field("force_scada", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def genconset_null_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("genconsetid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("genconeffdate", pyarrow.timestamp('s'), True),
        pyarrow.field("genconversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def generic_constraint_genconsetinvoke_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("invocation_id", pyarrow.int64(), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), False),
        pyarrow.field("startperiod", pyarrow.decimal128(3,0), False),
        pyarrow.field("genconsetid", pyarrow.large_utf8(), False),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("endperiod", pyarrow.decimal128(3,0), True),
        pyarrow.field("startauthorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("endauthorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.large_utf8(), True),
        pyarrow.field("asconstrainttype", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("startintervaldatetime", pyarrow.timestamp('s'), True),
        pyarrow.field("endintervaldatetime", pyarrow.timestamp('s'), True),
        pyarrow.field("systemnormal", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def genconsettrk_null_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("genconsetid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("coverage", pyarrow.large_utf8(), True),
        pyarrow.field("modifications", pyarrow.large_utf8(), True),
        pyarrow.field("systemnormal", pyarrow.large_utf8(), True),
        pyarrow.field("outage", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def gcrhs_null_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(22,0), False),
        pyarrow.field("scope", pyarrow.large_utf8(), False),
        pyarrow.field("termid", pyarrow.decimal128(4,0), False),
        pyarrow.field("groupid", pyarrow.decimal128(3,0), True),
        pyarrow.field("spd_id", pyarrow.large_utf8(), True),
        pyarrow.field("spd_type", pyarrow.large_utf8(), True),
        pyarrow.field("factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("operation", pyarrow.large_utf8(), True),
        pyarrow.field("defaultvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("parameterterm1", pyarrow.large_utf8(), True),
        pyarrow.field("parameterterm2", pyarrow.large_utf8(), True),
        pyarrow.field("parameterterm3", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def geqdesc_null_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("equationid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("impact", pyarrow.large_utf8(), True),
        pyarrow.field("source", pyarrow.large_utf8(), True),
        pyarrow.field("limittype", pyarrow.large_utf8(), True),
        pyarrow.field("reason", pyarrow.large_utf8(), True),
        pyarrow.field("modifications", pyarrow.large_utf8(), True),
        pyarrow.field("additionalnotes", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def geqrhs_null_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("equationid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("termid", pyarrow.decimal128(3,0), False),
        pyarrow.field("groupid", pyarrow.decimal128(3,0), True),
        pyarrow.field("spd_id", pyarrow.large_utf8(), True),
        pyarrow.field("spd_type", pyarrow.large_utf8(), True),
        pyarrow.field("factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("operation", pyarrow.large_utf8(), True),
        pyarrow.field("defaultvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("parameterterm1", pyarrow.large_utf8(), True),
        pyarrow.field("parameterterm2", pyarrow.large_utf8(), True),
        pyarrow.field("parameterterm3", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def spdcpc_null_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def spdicc_null_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def spdrc_null_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("factor", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False),
        pyarrow.field("auctiondate", pyarrow.timestamp('s'), True),
        pyarrow.field("notifydate", pyarrow.timestamp('s'), True),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_calendar_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("notifydate", pyarrow.timestamp('s'), True),
        pyarrow.field("paymentdate", pyarrow.timestamp('s'), True),
        pyarrow.field("reconciliationdate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("prelimpurchasestmtdate", pyarrow.timestamp('s'), True),
        pyarrow.field("prelimproceedsstmtdate", pyarrow.timestamp('s'), True),
        pyarrow.field("finalpurchasestmtdate", pyarrow.timestamp('s'), True),
        pyarrow.field("finalproceedsstmtdate", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_ic_allocations_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("maximumunits", pyarrow.decimal128(5,0), True),
        pyarrow.field("proportion", pyarrow.decimal128(8,5), True),
        pyarrow.field("auctionfee", pyarrow.decimal128(17,5), True),
        pyarrow.field("changedate", pyarrow.timestamp('s'), True),
        pyarrow.field("changedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("auctionfee_sales", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_revenue_estimate_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("valuationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("monthno", pyarrow.decimal128(1,0), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("revenue", pyarrow.decimal128(17,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_revenue_track_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("valuationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("documentref", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_rp_estimate_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("valuationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("rpestimate", pyarrow.decimal128(17,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_config_auction_tranche_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("tranche", pyarrow.decimal128(2,0), False),
        pyarrow.field("auctiondate", pyarrow.timestamp('s'), True),
        pyarrow.field("notifydate", pyarrow.timestamp('s'), True),
        pyarrow.field("unitallocation", pyarrow.decimal128(18,8), True),
        pyarrow.field("changedate", pyarrow.timestamp('s'), True),
        pyarrow.field("changedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_residuecontractpayments_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_bids_file_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("loaddate", pyarrow.timestamp('s'), False),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("ackfilename", pyarrow.large_utf8(), True),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_bid_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("bidloaddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_contracts_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("tranche", pyarrow.decimal128(2,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("notifydate", pyarrow.timestamp('s'), True),
        pyarrow.field("auctiondate", pyarrow.timestamp('s'), True),
        pyarrow.field("calcmethod", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("notifypostdate", pyarrow.timestamp('s'), True),
        pyarrow.field("notifyby", pyarrow.large_utf8(), True),
        pyarrow.field("postdate", pyarrow.timestamp('s'), True),
        pyarrow.field("postedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("auctionid", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_con_data_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("unitspurchased", pyarrow.decimal128(17,5), True),
        pyarrow.field("linkpayment", pyarrow.decimal128(17,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("secondary_units_sold", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_con_estimates_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("contractyear", pyarrow.decimal128(4,0), False),
        pyarrow.field("quarter", pyarrow.decimal128(1,0), False),
        pyarrow.field("valuationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_con_funds_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("defaultunits", pyarrow.decimal128(5,0), True),
        pyarrow.field("rolloverunits", pyarrow.decimal128(5,0), True),
        pyarrow.field("reallocatedunits", pyarrow.decimal128(5,0), True),
        pyarrow.field("unitsoffered", pyarrow.decimal128(5,0), True),
        pyarrow.field("meanreserveprice", pyarrow.decimal128(9,2), True),
        pyarrow.field("scalefactor", pyarrow.decimal128(8,5), True),
        pyarrow.field("actualreserveprice", pyarrow.decimal128(9,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_bids_funds_bid_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("loaddate", pyarrow.timestamp('s'), False),
        pyarrow.field("optionid", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("units", pyarrow.decimal128(5,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_bids_price_bid_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("loaddate", pyarrow.timestamp('s'), False),
        pyarrow.field("optionid", pyarrow.decimal128(3,0), False),
        pyarrow.field("bidprice", pyarrow.decimal128(17,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_price_funds_bid_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("units", pyarrow.decimal128(5,0), True),
        pyarrow.field("bidprice", pyarrow.decimal128(17,5), True),
        pyarrow.field("linkedbidflag", pyarrow.decimal128(6,0), False),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_public_data_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("unitsoffered", pyarrow.decimal128(5,0), True),
        pyarrow.field("unitssold", pyarrow.decimal128(16,6), True),
        pyarrow.field("clearingprice", pyarrow.decimal128(17,5), True),
        pyarrow.field("reserveprice", pyarrow.decimal128(17,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_residue_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("rundate", pyarrow.timestamp('s'), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("postdate", pyarrow.timestamp('s'), True),
        pyarrow.field("postedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_cash_security_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("cash_security_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("provision_date", pyarrow.timestamp('s'), True),
        pyarrow.field("cash_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("interest_acct_id", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("finalreturndate", pyarrow.timestamp('s'), True),
        pyarrow.field("cash_security_returned", pyarrow.decimal128(18,8), True),
        pyarrow.field("deletiondate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_financial_aucpay_detail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("sra_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("maximum_units", pyarrow.decimal128(18,8), True),
        pyarrow.field("units_sold", pyarrow.decimal128(18,8), True),
        pyarrow.field("shortfall_units", pyarrow.decimal128(18,8), True),
        pyarrow.field("reserve_price", pyarrow.decimal128(18,8), True),
        pyarrow.field("clearing_price", pyarrow.decimal128(18,8), True),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("shortfall_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("allocation", pyarrow.decimal128(18,8), True),
        pyarrow.field("net_payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_financial_aucpay_sum_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("sra_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("gross_proceeds_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("total_gross_proceeds_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("shortfall_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("total_shortfall_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("net_payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_financial_auc_mardetail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("sra_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("cash_security_id", pyarrow.large_utf8(), False),
        pyarrow.field("returned_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("returned_interest", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_financial_auc_margin_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("sra_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("total_cash_security", pyarrow.decimal128(18,8), True),
        pyarrow.field("required_margin", pyarrow.decimal128(18,8), True),
        pyarrow.field("returned_margin", pyarrow.decimal128(18,8), True),
        pyarrow.field("returned_margin_interest", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_financial_auc_receipts_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("sra_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("units_purchased", pyarrow.decimal128(18,8), True),
        pyarrow.field("clearing_price", pyarrow.decimal128(18,8), True),
        pyarrow.field("receipt_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("proceeds_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("units_sold", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_financial_runtrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("sra_runno", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), True),
        pyarrow.field("rundate", pyarrow.timestamp('s'), True),
        pyarrow.field("posteddate", pyarrow.timestamp('s'), True),
        pyarrow.field("interest_versionno", pyarrow.int64(), True),
        pyarrow.field("makeup_versionno", pyarrow.int64(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_offer_product_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("loaddate", pyarrow.timestamp('s'), False),
        pyarrow.field("optionid", pyarrow.int64(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), True),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), True),
        pyarrow.field("offer_quantity", pyarrow.int64(), True),
        pyarrow.field("offer_price", pyarrow.decimal128(18,8), True),
        pyarrow.field("trancheid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_offer_profile_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("auctionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("loaddate", pyarrow.timestamp('s'), False),
        pyarrow.field("filename", pyarrow.large_utf8(), True),
        pyarrow.field("ackfilename", pyarrow.large_utf8(), True),
        pyarrow.field("transactionid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_prudential_cash_security_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("prudential_date", pyarrow.timestamp('s'), False),
        pyarrow.field("prudential_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("cash_security_id", pyarrow.large_utf8(), False),
        pyarrow.field("cash_security_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_prudential_comp_position_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("prudential_date", pyarrow.timestamp('s'), False),
        pyarrow.field("prudential_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("trading_limit", pyarrow.decimal128(18,8), True),
        pyarrow.field("prudential_exposure_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("trading_margin", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_prudential_exposure_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("prudential_date", pyarrow.timestamp('s'), False),
        pyarrow.field("prudential_runno", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("sra_year", pyarrow.int64(), False),
        pyarrow.field("sra_quarter", pyarrow.int64(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("max_tranche", pyarrow.int64(), True),
        pyarrow.field("auctionid", pyarrow.large_utf8(), True),
        pyarrow.field("offer_submissiontime", pyarrow.timestamp('s'), True),
        pyarrow.field("average_purchase_price", pyarrow.decimal128(18,8), True),
        pyarrow.field("average_cancellation_price", pyarrow.decimal128(18,8), True),
        pyarrow.field("cancellation_volume", pyarrow.decimal128(18,8), True),
        pyarrow.field("trading_position", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_sra_prudential_run_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("prudential_date", pyarrow.timestamp('s'), False),
        pyarrow.field("prudential_runno", pyarrow.int64(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def irauction_valuationid_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("valuationid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_bidtypes_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("numberofbands", pyarrow.decimal128(3,0), True),
        pyarrow.field("numdaysaheadpricelocked", pyarrow.decimal128(2,0), True),
        pyarrow.field("validationrule", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("spdalias", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_bidtypestrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_interconnector_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("regionfrom", pyarrow.large_utf8(), True),
        pyarrow.field("rsoid", pyarrow.large_utf8(), True),
        pyarrow.field("regionto", pyarrow.large_utf8(), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_interconnectoralloc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(5,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("allocation", pyarrow.decimal128(12,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_interconnectorconstraint_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("reserveoverallloadfactor", pyarrow.decimal128(5,2), True),
        pyarrow.field("fromregionlossshare", pyarrow.decimal128(5,2), True),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("maxmwin", pyarrow.decimal128(15,5), True),
        pyarrow.field("maxmwout", pyarrow.decimal128(15,5), True),
        pyarrow.field("lossconstant", pyarrow.decimal128(15,6), True),
        pyarrow.field("lossflowcoefficient", pyarrow.decimal128(27,17), True),
        pyarrow.field("emsmeasurand", pyarrow.large_utf8(), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("dynamicrhs", pyarrow.large_utf8(), True),
        pyarrow.field("importlimit", pyarrow.decimal128(6,0), True),
        pyarrow.field("exportlimit", pyarrow.decimal128(6,0), True),
        pyarrow.field("outagederationfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("nonphysicallossfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("overloadfactor60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("overloadfactor6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("fcassupportunavailable", pyarrow.decimal128(1,0), True),
        pyarrow.field("ictype", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_intraregionalloc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(5,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("allocation", pyarrow.decimal128(12,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_lossfactormodel_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("demandcoefficient", pyarrow.decimal128(27,17), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_lossmodel_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("losssegment", pyarrow.decimal128(6,0), False),
        pyarrow.field("mwbreakpoint", pyarrow.decimal128(6,0), True),
        pyarrow.field("lossfactor", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_market_price_thresholds_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(4,0), False),
        pyarrow.field("voll", pyarrow.decimal128(15,5), True),
        pyarrow.field("marketpricefloor", pyarrow.decimal128(15,5), True),
        pyarrow.field("administered_price_threshold", pyarrow.decimal128(15,5), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_region_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("regionstatus", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_regionstandingdata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("rsoid", pyarrow.large_utf8(), True),
        pyarrow.field("regionalreferencepointid", pyarrow.large_utf8(), True),
        pyarrow.field("peaktradingperiod", pyarrow.decimal128(3,0), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("scalingfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_config_transmissionlossfactor_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("transmissionlossfactor", pyarrow.decimal128(15,5), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(22,0), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("secondary_tlf", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_notice_marketnoticedata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("noticeid", pyarrow.decimal128(10,0), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("typeid", pyarrow.large_utf8(), True),
        pyarrow.field("noticetype", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("reason", pyarrow.large_utf8(), True),
        pyarrow.field("externalreference", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_notice_marketnoticetype_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("typeid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("raisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def market_notice_participantnoticetrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("noticeid", pyarrow.decimal128(10,0), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mcc_casesolution_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mcc_constraintsolution_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("rhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def meterdata_aggregate_reads_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("case_id", pyarrow.decimal128(15,0), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("meter_type", pyarrow.large_utf8(), False),
        pyarrow.field("frmp", pyarrow.large_utf8(), False),
        pyarrow.field("lr", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("importvalue", pyarrow.decimal128(18,8), False),
        pyarrow.field("exportvalue", pyarrow.decimal128(18,8), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def meterdata_individual_reads_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("case_id", pyarrow.decimal128(15,0), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("meter_id", pyarrow.large_utf8(), False),
        pyarrow.field("meter_id_suffix", pyarrow.large_utf8(), False),
        pyarrow.field("frmp", pyarrow.large_utf8(), False),
        pyarrow.field("lr", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("meter_type", pyarrow.large_utf8(), False),
        pyarrow.field("importvalue", pyarrow.decimal128(18,8), False),
        pyarrow.field("exportvalue", pyarrow.decimal128(18,8), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def meterdata_interconnector_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("case_id", pyarrow.decimal128(15,0), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("importvalue", pyarrow.decimal128(18,8), True),
        pyarrow.field("exportvalue", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def meterdata_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("case_id", pyarrow.decimal128(15,0), False),
        pyarrow.field("aggregate_reads_load_datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("individual_reads_load_datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mr_dayoffer_stack_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("stack_position", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("authorised", pyarrow.decimal128(1,0), True),
        pyarrow.field("offer_settlementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offer_offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offer_versionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("offer_type", pyarrow.large_utf8(), True),
        pyarrow.field("laof", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mr_event_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("offer_cut_off_time", pyarrow.timestamp('s'), True),
        pyarrow.field("settlement_complete", pyarrow.decimal128(1,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mr_event_schedule_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("demand_effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("demand_offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("demand_versionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mr_peroffer_stack_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("mr_date", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("stack_position", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("accepted_capacity", pyarrow.decimal128(6,0), True),
        pyarrow.field("deducted_capacity", pyarrow.decimal128(6,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_caseresult_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("plexos_version", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_constraintresult_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("demand_poe_type", pyarrow.large_utf8(), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), True),
        pyarrow.field("probabilityofbinding", pyarrow.decimal128(8,5), True),
        pyarrow.field("probabilityofviolation", pyarrow.decimal128(8,5), True),
        pyarrow.field("constraintviolation90", pyarrow.decimal128(12,2), True),
        pyarrow.field("constraintviolation50", pyarrow.decimal128(12,2), True),
        pyarrow.field("constraintviolation10", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_constraintsummary_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("demand_poe_type", pyarrow.large_utf8(), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("aggregation_period", pyarrow.large_utf8(), False),
        pyarrow.field("constrainthoursbinding", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_duidavailability_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("publish_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("pasaavailability", pyarrow.decimal128(12,0), True),
        pyarrow.field("latest_offer_datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_interconnectorresult_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("demand_poe_type", pyarrow.large_utf8(), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), True),
        pyarrow.field("flow90", pyarrow.decimal128(12,2), True),
        pyarrow.field("flow50", pyarrow.decimal128(12,2), True),
        pyarrow.field("flow10", pyarrow.decimal128(12,2), True),
        pyarrow.field("probabilityofbindingexport", pyarrow.decimal128(8,5), True),
        pyarrow.field("probabilityofbindingimport", pyarrow.decimal128(8,5), True),
        pyarrow.field("calculatedexportlimit", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedimportlimit", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_lolpresult_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("worst_interval_periodid", pyarrow.decimal128(3,0), True),
        pyarrow.field("worst_interval_demand", pyarrow.decimal128(12,2), True),
        pyarrow.field("worst_interval_intgen", pyarrow.decimal128(12,2), True),
        pyarrow.field("worst_interval_dsp", pyarrow.decimal128(12,2), True),
        pyarrow.field("lossofloadprobability", pyarrow.decimal128(8,5), True),
        pyarrow.field("lossofloadmagnitude", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_regionavailability_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("publish_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("pasaavailability_scheduled", pyarrow.decimal128(12,0), True),
        pyarrow.field("latest_offer_datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("energyunconstrainedcapacity", pyarrow.decimal128(12,0), True),
        pyarrow.field("energyconstrainedcapacity", pyarrow.decimal128(12,0), True),
        pyarrow.field("nonscheduledgeneration", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand10", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand50", pyarrow.decimal128(12,2), True),
        pyarrow.field("energyreqdemand10", pyarrow.decimal128(12,2), True),
        pyarrow.field("energyreqdemand50", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("demand10min", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand10max", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand50min", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand50max", pyarrow.decimal128(12,2), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_regionavailtrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("publish_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("latest_offer_datetime", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_regioniteration_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("demand_poe_type", pyarrow.large_utf8(), False),
        pyarrow.field("aggregation_period", pyarrow.large_utf8(), False),
        pyarrow.field("period_ending", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("use_iteration_id", pyarrow.int64(), False),
        pyarrow.field("use_iteration_event_number", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_iteration_event_average", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_regionresult_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("demand_poe_type", pyarrow.large_utf8(), False),
        pyarrow.field("day", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), True),
        pyarrow.field("demand", pyarrow.decimal128(12,2), True),
        pyarrow.field("aggregateinstalledcapacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("numberofiterations", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_numberofiterations", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_max", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_upperquartile", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_median", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_lowerquartile", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_min", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_average", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_event_average", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalscheduledgen90", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalscheduledgen50", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalscheduledgen10", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalintermittentgen90", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalintermittentgen50", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalintermittentgen10", pyarrow.decimal128(12,2), True),
        pyarrow.field("demandsideparticipation90", pyarrow.decimal128(12,2), True),
        pyarrow.field("demandsideparticipation50", pyarrow.decimal128(12,2), True),
        pyarrow.field("demandsideparticipation10", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("totalsemischedulegen90", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalsemischedulegen50", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalsemischedulegen10", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalavailablegenmin", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalavailablegen10", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalavailablegen50", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalavailablegen90", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalavailablegenmax", pyarrow.decimal128(12,2), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_regionsummary_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("run_no", pyarrow.int64(), False),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("demand_poe_type", pyarrow.large_utf8(), False),
        pyarrow.field("aggregation_period", pyarrow.large_utf8(), False),
        pyarrow.field("period_ending", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("nativedemand", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile10", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile20", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile30", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile40", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile50", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile60", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile70", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile80", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile90", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_percentile100", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_average", pyarrow.decimal128(12,2), True),
        pyarrow.field("numberofiterations", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_numberofiterations", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_event_max", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_event_upperquartile", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_event_median", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_event_lowerquartile", pyarrow.decimal128(12,2), True),
        pyarrow.field("use_event_min", pyarrow.decimal128(12,2), True),
        pyarrow.field("weight", pyarrow.decimal128(16,6), True),
        pyarrow.field("use_weighted_avg", pyarrow.decimal128(16,6), True),
        pyarrow.field("lrc", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_equipmentdetail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("substationid", pyarrow.large_utf8(), False),
        pyarrow.field("equipmenttype", pyarrow.large_utf8(), False),
        pyarrow.field("equipmentid", pyarrow.large_utf8(), False),
        pyarrow.field("validfrom", pyarrow.timestamp('s'), False),
        pyarrow.field("validto", pyarrow.timestamp('s'), True),
        pyarrow.field("voltage", pyarrow.large_utf8(), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_outageconstraintset_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("outageid", pyarrow.decimal128(15,0), False),
        pyarrow.field("genconsetid", pyarrow.large_utf8(), False),
        pyarrow.field("startinterval", pyarrow.timestamp('s'), True),
        pyarrow.field("endinterval", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_outagedetail_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("outageid", pyarrow.decimal128(15,0), False),
        pyarrow.field("substationid", pyarrow.large_utf8(), False),
        pyarrow.field("equipmenttype", pyarrow.large_utf8(), False),
        pyarrow.field("equipmentid", pyarrow.large_utf8(), False),
        pyarrow.field("starttime", pyarrow.timestamp('s'), False),
        pyarrow.field("endtime", pyarrow.timestamp('s'), True),
        pyarrow.field("submitteddate", pyarrow.timestamp('s'), True),
        pyarrow.field("outagestatuscode", pyarrow.large_utf8(), True),
        pyarrow.field("resubmitreason", pyarrow.large_utf8(), True),
        pyarrow.field("resubmitoutageid", pyarrow.decimal128(15,0), True),
        pyarrow.field("recalltimeday", pyarrow.decimal128(10,0), True),
        pyarrow.field("recalltimenight", pyarrow.decimal128(10,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("reason", pyarrow.large_utf8(), True),
        pyarrow.field("issecondary", pyarrow.decimal128(1,0), True),
        pyarrow.field("actual_starttime", pyarrow.timestamp('s'), True),
        pyarrow.field("actual_endtime", pyarrow.timestamp('s'), True),
        pyarrow.field("companyrefcode", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_outagestatuscode_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("outagestatuscode", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_rating_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("spd_id", pyarrow.large_utf8(), False),
        pyarrow.field("validfrom", pyarrow.timestamp('s'), False),
        pyarrow.field("validto", pyarrow.timestamp('s'), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("substationid", pyarrow.large_utf8(), True),
        pyarrow.field("equipmenttype", pyarrow.large_utf8(), True),
        pyarrow.field("equipmentid", pyarrow.large_utf8(), True),
        pyarrow.field("ratinglevel", pyarrow.large_utf8(), True),
        pyarrow.field("isdynamic", pyarrow.decimal128(1,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_realtimerating_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("spd_id", pyarrow.large_utf8(), False),
        pyarrow.field("ratingvalue", pyarrow.decimal128(16,6), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_staticrating_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("substationid", pyarrow.large_utf8(), False),
        pyarrow.field("equipmenttype", pyarrow.large_utf8(), False),
        pyarrow.field("equipmentid", pyarrow.large_utf8(), False),
        pyarrow.field("ratinglevel", pyarrow.large_utf8(), False),
        pyarrow.field("applicationid", pyarrow.large_utf8(), False),
        pyarrow.field("validfrom", pyarrow.timestamp('s'), False),
        pyarrow.field("validto", pyarrow.timestamp('s'), True),
        pyarrow.field("ratingvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def network_substationdetail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("substationid", pyarrow.large_utf8(), False),
        pyarrow.field("validfrom", pyarrow.timestamp('s'), False),
        pyarrow.field("validto", pyarrow.timestamp('s'), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("ownerid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_blocked_constraints_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_casesolution_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("startinterval_datetime", pyarrow.large_utf8(), True),
        pyarrow.field("totalobjective", pyarrow.decimal128(27,10), True),
        pyarrow.field("nonphysicallosses", pyarrow.decimal128(1,0), True),
        pyarrow.field("totalareagenviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalinterconnectorviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalgenericviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalramprateviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalunitmwcapacityviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalenergyconstrviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalenergyofferviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalasprofileviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalfaststartviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_constraintsolution_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("rhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("genconid_effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("genconid_versionno", pyarrow.decimal128(22,0), True),
        pyarrow.field("lhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_interconnectorsoln_v4(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("meteredmwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwlosses", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("mnsp", pyarrow.decimal128(1,0), True),
        pyarrow.field("exportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("importlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalloss", pyarrow.decimal128(15,5), True),
        pyarrow.field("exportgenconid", pyarrow.large_utf8(), True),
        pyarrow.field("importgenconid", pyarrow.large_utf8(), True),
        pyarrow.field("fcasexportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcasimportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("local_price_adjustment_export", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained_export", pyarrow.decimal128(1,0), True),
        pyarrow.field("local_price_adjustment_import", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained_import", pyarrow.decimal128(1,0), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_intersensitivities_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("intervention", pyarrow.decimal128(1,0), False),
        pyarrow.field("intervention_active", pyarrow.decimal128(1,0), True),
        pyarrow.field("mwflow1", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow2", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow3", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow4", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow5", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow6", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow7", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow8", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow9", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow10", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow11", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow12", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow13", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow14", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow15", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow16", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow17", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow18", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow19", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow20", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow21", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow22", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow23", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow24", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow25", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow26", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow27", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow28", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow29", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow30", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow31", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow32", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow33", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow34", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow35", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow36", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow37", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow38", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow39", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow40", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow41", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow42", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow43", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_local_price_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("local_price_adjustment", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_pricesensitivities_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("intervention", pyarrow.decimal128(1,0), False),
        pyarrow.field("intervention_active", pyarrow.decimal128(1,0), True),
        pyarrow.field("rrp1", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp2", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp3", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp4", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp5", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp6", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp7", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp8", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp9", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp10", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp11", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp12", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp13", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp14", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp15", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp16", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp17", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp18", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp19", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp20", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp21", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp22", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp23", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp24", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp25", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp26", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp27", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp28", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp29", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp30", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp31", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp32", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp33", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp34", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp35", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp36", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp37", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp38", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp39", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp40", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp41", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp42", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp43", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_regionsolution_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("rop", pyarrow.decimal128(15,5), True),
        pyarrow.field("excessgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("totaldemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("availablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("availableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("demandforecast", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("netinterchange", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("aggregatedispatcherror", pyarrow.decimal128(15,5), True),
        pyarrow.field("initialsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("totalintermittentgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("demand_and_nonschedgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("semischedule_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("semischedule_compliancemw", pyarrow.decimal128(15,5), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("ss_solar_uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_compliancemw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_compliancemw", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_scenariodemand_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("scenario", pyarrow.decimal128(2,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("deltamw", pyarrow.decimal128(4,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_scenariodemandtrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def p5min_unitsolution_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True),
        pyarrow.field("tradetype", pyarrow.decimal128(2,0), True),
        pyarrow.field("agcstatus", pyarrow.decimal128(2,0), True),
        pyarrow.field("initialmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalcleared", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampdownrate", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampuprate", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereg", pyarrow.decimal128(15,5), True),
        pyarrow.field("availability", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise60secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise5minflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raiseregflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower6secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower60secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower5minflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lowerregflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("semidispatchcap", pyarrow.decimal128(3,0), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_bidduiddetails_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("maxcapacity", pyarrow.decimal128(22,0), True),
        pyarrow.field("minenablementlevel", pyarrow.decimal128(22,0), True),
        pyarrow.field("maxenablementlevel", pyarrow.decimal128(22,0), True),
        pyarrow.field("maxlowerangle", pyarrow.decimal128(3,0), True),
        pyarrow.field("maxupperangle", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_bidduiddetailstrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_dispatchableunit_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("duname", pyarrow.large_utf8(), True),
        pyarrow.field("unittype", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_dualloc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("gensetid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_dudetail_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True),
        pyarrow.field("voltlevel", pyarrow.large_utf8(), True),
        pyarrow.field("registeredcapacity", pyarrow.decimal128(6,0), True),
        pyarrow.field("agccapability", pyarrow.large_utf8(), True),
        pyarrow.field("dispatchtype", pyarrow.large_utf8(), True),
        pyarrow.field("maxcapacity", pyarrow.decimal128(6,0), True),
        pyarrow.field("starttype", pyarrow.large_utf8(), True),
        pyarrow.field("normallyonflag", pyarrow.large_utf8(), True),
        pyarrow.field("physicaldetailsflag", pyarrow.large_utf8(), True),
        pyarrow.field("spinningreserveflag", pyarrow.large_utf8(), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("intermittentflag", pyarrow.large_utf8(), True),
        pyarrow.field("semi_schedule_flag", pyarrow.large_utf8(), True),
        pyarrow.field("maxrateofchangeup", pyarrow.decimal128(6,0), True),
        pyarrow.field("maxrateofchangedown", pyarrow.decimal128(6,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_dudetailsummary_v4(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("start_date", pyarrow.timestamp('s'), False),
        pyarrow.field("end_date", pyarrow.timestamp('s'), False),
        pyarrow.field("dispatchtype", pyarrow.large_utf8(), True),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("stationid", pyarrow.large_utf8(), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("transmissionlossfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("starttype", pyarrow.large_utf8(), True),
        pyarrow.field("distributionlossfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("minimum_energy_price", pyarrow.decimal128(9,2), True),
        pyarrow.field("maximum_energy_price", pyarrow.decimal128(9,2), True),
        pyarrow.field("schedule_type", pyarrow.large_utf8(), True),
        pyarrow.field("min_ramp_rate_up", pyarrow.decimal128(6,0), True),
        pyarrow.field("min_ramp_rate_down", pyarrow.decimal128(6,0), True),
        pyarrow.field("max_ramp_rate_up", pyarrow.decimal128(6,0), True),
        pyarrow.field("max_ramp_rate_down", pyarrow.decimal128(6,0), True),
        pyarrow.field("is_aggregated", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_genmeter_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("meterid", pyarrow.large_utf8(), False),
        pyarrow.field("gensetid", pyarrow.large_utf8(), True),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True),
        pyarrow.field("stationid", pyarrow.large_utf8(), True),
        pyarrow.field("metertype", pyarrow.large_utf8(), True),
        pyarrow.field("meterclass", pyarrow.large_utf8(), True),
        pyarrow.field("voltagelevel", pyarrow.decimal128(6,0), True),
        pyarrow.field("applydate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("comdate", pyarrow.timestamp('s'), True),
        pyarrow.field("decomdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_genunits_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("gensetid", pyarrow.large_utf8(), False),
        pyarrow.field("stationid", pyarrow.large_utf8(), True),
        pyarrow.field("setlossfactor", pyarrow.decimal128(16,6), True),
        pyarrow.field("cdindicator", pyarrow.large_utf8(), True),
        pyarrow.field("agcflag", pyarrow.large_utf8(), True),
        pyarrow.field("spinningflag", pyarrow.large_utf8(), True),
        pyarrow.field("voltlevel", pyarrow.decimal128(6,0), True),
        pyarrow.field("registeredcapacity", pyarrow.decimal128(6,0), True),
        pyarrow.field("dispatchtype", pyarrow.large_utf8(), True),
        pyarrow.field("starttype", pyarrow.large_utf8(), True),
        pyarrow.field("mktgeneratorind", pyarrow.large_utf8(), True),
        pyarrow.field("normalstatus", pyarrow.large_utf8(), True),
        pyarrow.field("maxcapacity", pyarrow.decimal128(6,0), True),
        pyarrow.field("gensettype", pyarrow.large_utf8(), True),
        pyarrow.field("gensetname", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("co2e_emissions_factor", pyarrow.decimal128(18,8), True),
        pyarrow.field("co2e_energy_source", pyarrow.large_utf8(), True),
        pyarrow.field("co2e_data_source", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_genunits_unit_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("gensetid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(6,0), False),
        pyarrow.field("unit_grouping_label", pyarrow.large_utf8(), False),
        pyarrow.field("unit_count", pyarrow.decimal128(3,0), True),
        pyarrow.field("unit_size", pyarrow.decimal128(8,3), True),
        pyarrow.field("unit_max_size", pyarrow.decimal128(8,3), True),
        pyarrow.field("aggregation_flag", pyarrow.decimal128(1,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_mnsp_interconnector_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("linkid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), True),
        pyarrow.field("fromregion", pyarrow.large_utf8(), True),
        pyarrow.field("toregion", pyarrow.large_utf8(), True),
        pyarrow.field("maxcapacity", pyarrow.decimal128(5,0), True),
        pyarrow.field("tlf", pyarrow.decimal128(12,7), True),
        pyarrow.field("lhsfactor", pyarrow.decimal128(12,7), True),
        pyarrow.field("meterflowconstant", pyarrow.decimal128(12,7), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("from_region_tlf", pyarrow.decimal128(12,7), True),
        pyarrow.field("to_region_tlf", pyarrow.decimal128(12,7), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_mnsp_participant_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_participant_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("participantclassid", pyarrow.large_utf8(), True),
        pyarrow.field("name", pyarrow.large_utf8(), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("acn", pyarrow.large_utf8(), True),
        pyarrow.field("primarybusiness", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_participantaccount_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("accountname", pyarrow.large_utf8(), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("accountnumber", pyarrow.large_utf8(), True),
        pyarrow.field("bankname", pyarrow.large_utf8(), True),
        pyarrow.field("banknumber", pyarrow.decimal128(10,0), True),
        pyarrow.field("branchname", pyarrow.large_utf8(), True),
        pyarrow.field("branchnumber", pyarrow.decimal128(10,0), True),
        pyarrow.field("bsbnumber", pyarrow.large_utf8(), True),
        pyarrow.field("nemmcocreditaccountnumber", pyarrow.decimal128(10,0), True),
        pyarrow.field("nemmcodebitaccountnumber", pyarrow.decimal128(10,0), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("abn", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_participantcategory_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantcategoryid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_participantcategoryalloc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantcategoryid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_participantclass_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantclassid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_participantcreditdetail_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("creditlimit", pyarrow.decimal128(10,0), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_stadualloc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("stationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_station_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("stationid", pyarrow.large_utf8(), False),
        pyarrow.field("stationname", pyarrow.large_utf8(), True),
        pyarrow.field("address1", pyarrow.large_utf8(), True),
        pyarrow.field("address2", pyarrow.large_utf8(), True),
        pyarrow.field("address3", pyarrow.large_utf8(), True),
        pyarrow.field("address4", pyarrow.large_utf8(), True),
        pyarrow.field("city", pyarrow.large_utf8(), True),
        pyarrow.field("state", pyarrow.large_utf8(), True),
        pyarrow.field("postcode", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_stationoperatingstatus_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("stationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("status", pyarrow.large_utf8(), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_stationowner_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("stationid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def participant_registration_stationownertrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def pdpasa_casesolution_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("pasaversion", pyarrow.large_utf8(), True),
        pyarrow.field("reservecondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("lorcondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("capacityobjfunction", pyarrow.decimal128(12,3), True),
        pyarrow.field("capacityoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("maxsurplusreserveoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("maxsparecapacityoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("interconnectorflowpenalty", pyarrow.decimal128(12,3), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("reliabilitylrcdemandoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("outagelrcdemandoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("lordemandoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("reliabilitylrccapacityoption", pyarrow.large_utf8(), True),
        pyarrow.field("outagelrccapacityoption", pyarrow.large_utf8(), True),
        pyarrow.field("lorcapacityoption", pyarrow.large_utf8(), True),
        pyarrow.field("loruigf_option", pyarrow.decimal128(3,0), True),
        pyarrow.field("reliability_lrcuigf_option", pyarrow.decimal128(3,0), True),
        pyarrow.field("outage_lrcuigf_option", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def pdpasa_constraintsolution_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("capacityrhs", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacitymarginalvalue", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacityviolationdegree", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("studyregionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def pdpasa_interconnectorsoln_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("capacitymwflow", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacitymarginalvalue", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacityviolationdegree", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedexportlimit", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedimportlimit", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("exportlimitconstraintid", pyarrow.large_utf8(), True),
        pyarrow.field("importlimitconstraintid", pyarrow.large_utf8(), True),
        pyarrow.field("studyregionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def pdpasa_regionsolution_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("demand10", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand50", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand90", pyarrow.decimal128(12,2), True),
        pyarrow.field("reservereq", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacityreq", pyarrow.decimal128(12,2), True),
        pyarrow.field("energyreqdemand50", pyarrow.decimal128(12,2), True),
        pyarrow.field("unconstrainedcapacity", pyarrow.decimal128(12,0), True),
        pyarrow.field("constrainedcapacity", pyarrow.decimal128(12,0), True),
        pyarrow.field("netinterchangeunderscarcity", pyarrow.decimal128(12,2), True),
        pyarrow.field("surpluscapacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("surplusreserve", pyarrow.decimal128(12,2), True),
        pyarrow.field("reservecondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("maxsurplusreserve", pyarrow.decimal128(12,2), True),
        pyarrow.field("maxsparecapacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lorcondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("aggregatecapacityavailable", pyarrow.decimal128(12,2), True),
        pyarrow.field("aggregatescheduledload", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("aggregatepasaavailability", pyarrow.decimal128(12,0), True),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("energyreqdemand10", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedlor1level", pyarrow.decimal128(16,6), True),
        pyarrow.field("calculatedlor2level", pyarrow.decimal128(16,6), True),
        pyarrow.field("msrnetinterchangeunderscarcity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lornetinterchangeunderscarcity", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalintermittentgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("demand_and_nonschedgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("uigf", pyarrow.decimal128(12,2), True),
        pyarrow.field("semi_scheduled_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lor_semi_scheduled_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lcr", pyarrow.decimal128(16,6), True),
        pyarrow.field("lcr2", pyarrow.decimal128(16,6), True),
        pyarrow.field("fum", pyarrow.decimal128(16,6), True),
        pyarrow.field("ss_solar_uigf", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_wind_uigf", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_solar_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_wind_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_solar_cleared", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_wind_cleared", pyarrow.decimal128(12,2), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_blocked_constraints_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_case_solution_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("solutionstatus", pyarrow.decimal128(2,0), True),
        pyarrow.field("spdversion", pyarrow.large_utf8(), True),
        pyarrow.field("nonphysicallosses", pyarrow.decimal128(1,0), True),
        pyarrow.field("totalobjective", pyarrow.decimal128(27,10), True),
        pyarrow.field("totalareagenviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalinterconnectorviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalgenericviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalramprateviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalunitmwcapacityviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("total60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalasprofileviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalenergyconstrviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalenergyofferviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_constraint_solution_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("rhs", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("genconid_effectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("genconid_versionno", pyarrow.decimal128(22,0), True),
        pyarrow.field("lhs", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_interconnector_soln_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("meteredmwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwlosses", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("exportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("importlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalloss", pyarrow.decimal128(15,5), True),
        pyarrow.field("exportgenconid", pyarrow.large_utf8(), True),
        pyarrow.field("importgenconid", pyarrow.large_utf8(), True),
        pyarrow.field("fcasexportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcasimportlimit", pyarrow.decimal128(15,5), True),
        pyarrow.field("local_price_adjustment_export", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained_export", pyarrow.decimal128(1,0), True),
        pyarrow.field("local_price_adjustment_import", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained_import", pyarrow.decimal128(1,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_interconnectr_sens_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("intervention_active", pyarrow.decimal128(1,0), True),
        pyarrow.field("mwflow1", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow2", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow3", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow4", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow5", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow6", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow7", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow8", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow9", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow10", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow11", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow12", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow13", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow14", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow15", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow16", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow17", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow18", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow19", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow20", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow21", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow22", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow23", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow24", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow25", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow26", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow27", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow28", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow29", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow30", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow31", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow32", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow33", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow34", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow35", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow36", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow37", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow38", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow39", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow40", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow41", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow42", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow43", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_unit_solution_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("tradetype", pyarrow.decimal128(2,0), True),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), True),
        pyarrow.field("agcstatus", pyarrow.decimal128(2,0), True),
        pyarrow.field("dispatchmode", pyarrow.decimal128(2,0), True),
        pyarrow.field("initialmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalcleared", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampdownrate", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampuprate", pyarrow.decimal128(15,5), True),
        pyarrow.field("downepf", pyarrow.decimal128(15,5), True),
        pyarrow.field("upepf", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginal5minvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginal60secvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginal6secvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("violation5mindegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("violation60secdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("violation6secdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("violationdegree", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("lowerreg", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereg", pyarrow.decimal128(15,5), True),
        pyarrow.field("availability", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise60secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise5minflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raiseregflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower6secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower60secflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lower5minflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("lowerregflags", pyarrow.decimal128(3,0), True),
        pyarrow.field("raise6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raiseregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lowerregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("semidispatchcap", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_offertrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), False),
        pyarrow.field("bidsettlementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("bidofferdate", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_region_prices_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp1", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep1", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp2", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep2", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp3", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep3", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp4", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep4", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp5", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep5", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp6", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep6", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp7", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep7", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp8", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep8", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("raise6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregrrp", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_pricesensitivities_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("rrpeep1", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep2", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep3", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep4", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep5", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep6", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep7", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep8", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep9", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep10", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep11", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep12", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep13", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep14", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep15", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep16", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep17", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep18", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep19", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep20", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep21", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep22", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep23", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep24", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep25", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep26", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep27", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep28", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("rrpeep29", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep30", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep31", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep32", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep33", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep34", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep35", pyarrow.decimal128(15,5), True),
        pyarrow.field("intervention_active", pyarrow.decimal128(1,0), True),
        pyarrow.field("rrpeep36", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep37", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep38", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep39", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep40", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep41", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep42", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrpeep43", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_region_solution_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("totaldemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("availablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("availableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("demandforecast", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("netinterchange", pyarrow.decimal128(15,5), True),
        pyarrow.field("excessgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("initialsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raise5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("raiseregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower6secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower60secactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lower5minactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lowerregactualavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("decavailability", pyarrow.decimal128(16,6), True),
        pyarrow.field("lorsurplus", pyarrow.decimal128(16,6), True),
        pyarrow.field("lrcsurplus", pyarrow.decimal128(16,6), True),
        pyarrow.field("totalintermittentgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("demand_and_nonschedgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("semischedule_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("semischedule_compliancemw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_uigf", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_solar_compliancemw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ss_wind_compliancemw", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_scenario_demand_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("scenario", pyarrow.int64(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("deltamw", pyarrow.int64(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_scenario_demand_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_regionfcasrequirement_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), True),
        pyarrow.field("runno", pyarrow.decimal128(3,0), True),
        pyarrow.field("intervention", pyarrow.decimal128(2,0), True),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("genconid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("genconeffectivedate", pyarrow.timestamp('s'), True),
        pyarrow.field("genconversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("marginalvalue", pyarrow.decimal128(16,6), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("base_cost", pyarrow.decimal128(18,8), True),
        pyarrow.field("adjusted_cost", pyarrow.decimal128(18,8), True),
        pyarrow.field("estimated_cmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("estimated_crmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_factor_cmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_factor_crmpf", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_local_price_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), False),
        pyarrow.field("datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), True),
        pyarrow.field("local_price_adjustment", pyarrow.decimal128(10,2), True),
        pyarrow.field("locally_constrained", pyarrow.decimal128(1,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def predispatch_mnspbidtrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("predispatchseqno", pyarrow.large_utf8(), False),
        pyarrow.field("linkid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def prudential_company_position_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("prudential_date", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.int64(), False),
        pyarrow.field("company_id", pyarrow.large_utf8(), False),
        pyarrow.field("mcl", pyarrow.decimal128(16,6), True),
        pyarrow.field("credit_support", pyarrow.decimal128(16,6), True),
        pyarrow.field("trading_limit", pyarrow.decimal128(16,6), True),
        pyarrow.field("current_amount_balance", pyarrow.decimal128(16,6), True),
        pyarrow.field("security_deposit_provision", pyarrow.decimal128(16,6), True),
        pyarrow.field("security_deposit_offset", pyarrow.decimal128(16,6), True),
        pyarrow.field("security_deposit_balance", pyarrow.decimal128(16,6), True),
        pyarrow.field("expost_realloc_balance", pyarrow.decimal128(16,6), True),
        pyarrow.field("default_balance", pyarrow.decimal128(16,6), True),
        pyarrow.field("outstandings", pyarrow.decimal128(16,6), True),
        pyarrow.field("trading_margin", pyarrow.decimal128(16,6), True),
        pyarrow.field("typical_accrual", pyarrow.decimal128(16,6), True),
        pyarrow.field("prudential_margin", pyarrow.decimal128(16,6), True),
        pyarrow.field("early_payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("percentage_outstandings", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def prudential_runtrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("prudential_date", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.int64(), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_reservelimit_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("reservelimitid", pyarrow.large_utf8(), False),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("rhs", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_reservelimit_region_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("reservelimitid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("coef", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def mtpasa_reservelimit_set_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("reservelimit_set_id", pyarrow.large_utf8(), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def reserve_data_reserve_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(2,0), False),
        pyarrow.field("lower5min", pyarrow.decimal128(6,0), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(6,0), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(6,0), True),
        pyarrow.field("raise5min", pyarrow.decimal128(6,0), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(6,0), True),
        pyarrow.field("raise6sec", pyarrow.decimal128(6,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("pasareserve", pyarrow.decimal128(6,0), True),
        pyarrow.field("loadrejectionreservereq", pyarrow.decimal128(10,0), True),
        pyarrow.field("raisereg", pyarrow.decimal128(6,0), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(6,0), True),
        pyarrow.field("lor1level", pyarrow.decimal128(6,0), True),
        pyarrow.field("lor2level", pyarrow.decimal128(6,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_ancillary_recovery_split_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("paymenttype", pyarrow.large_utf8(), False),
        pyarrow.field("customer_portion", pyarrow.decimal128(8,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_marketfee_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeperiod", pyarrow.large_utf8(), True),
        pyarrow.field("marketfeetype", pyarrow.large_utf8(), True),
        pyarrow.field("description", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("gl_tcode", pyarrow.large_utf8(), True),
        pyarrow.field("gl_financialcode", pyarrow.large_utf8(), True),
        pyarrow.field("fee_class", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_marketfeedata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeversionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("marketfeevalue", pyarrow.decimal128(22,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_marketfeetrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeversionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_market_fee_cat_excl_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("participant_categoryid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_market_fee_cat_excl_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_market_fee_exclusion_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_market_fee_exclusion_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_participant_bandfee_alloc_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantcategoryid", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeevalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def setcfg_reallocation_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("reallocationid", pyarrow.large_utf8(), False),
        pyarrow.field("creditparticipantid", pyarrow.large_utf8(), True),
        pyarrow.field("debitparticipantid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("agreementtype", pyarrow.large_utf8(), True),
        pyarrow.field("creditreference", pyarrow.large_utf8(), True),
        pyarrow.field("debitreference", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("startdate", pyarrow.timestamp('s'), True),
        pyarrow.field("enddate", pyarrow.timestamp('s'), True),
        pyarrow.field("current_stepid", pyarrow.large_utf8(), True),
        pyarrow.field("daytype", pyarrow.large_utf8(), True),
        pyarrow.field("reallocation_type", pyarrow.large_utf8(), True),
        pyarrow.field("calendarid", pyarrow.large_utf8(), True),
        pyarrow.field("intervallength", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def setcfg_reallocationinterval_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("reallocationid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.int64(), False),
        pyarrow.field("value", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("nrp", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_setcfg_participant_mpf_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantcategoryid", pyarrow.large_utf8(), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("mpf", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlement_config_setcfg_participant_mpftrk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("effectivedate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("authorisedby", pyarrow.large_utf8(), True),
        pyarrow.field("authoriseddate", pyarrow.timestamp('s'), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_daytrack_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("exanterunstatus", pyarrow.large_utf8(), True),
        pyarrow.field("exanterunno", pyarrow.decimal128(3,0), True),
        pyarrow.field("expostrunstatus", pyarrow.large_utf8(), True),
        pyarrow.field("expostrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("settlementintervallength", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_agcpayment_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("ebp", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("initialmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_agcrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("enablingrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand_gen", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_cpdata_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(10,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(10,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("tcpid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("igenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("xgenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("inenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("xnenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("ipower", pyarrow.decimal128(16,6), True),
        pyarrow.field("xpower", pyarrow.decimal128(16,6), True),
        pyarrow.field("rrp", pyarrow.decimal128(20,5), True),
        pyarrow.field("eep", pyarrow.decimal128(16,6), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("cprrp", pyarrow.decimal128(16,6), True),
        pyarrow.field("cpeep", pyarrow.decimal128(16,6), True),
        pyarrow.field("ta", pyarrow.decimal128(16,6), True),
        pyarrow.field("ep", pyarrow.decimal128(16,6), True),
        pyarrow.field("apc", pyarrow.decimal128(16,6), True),
        pyarrow.field("resc", pyarrow.decimal128(16,6), True),
        pyarrow.field("resp", pyarrow.decimal128(16,6), True),
        pyarrow.field("meterrunno", pyarrow.decimal128(10,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("hostdistributor", pyarrow.large_utf8(), True),
        pyarrow.field("mda", pyarrow.large_utf8(), False),
        pyarrow.field("afe", pyarrow.decimal128(18,8), True),
        pyarrow.field("dme", pyarrow.decimal128(18,8), True),
        pyarrow.field("ufea", pyarrow.decimal128(18,8), True),
        pyarrow.field("age", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_cpdataregion_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(22,10), False),
        pyarrow.field("periodid", pyarrow.decimal128(22,10), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("sumigenergy", pyarrow.decimal128(27,5), True),
        pyarrow.field("sumxgenergy", pyarrow.decimal128(27,5), True),
        pyarrow.field("suminenergy", pyarrow.decimal128(27,5), True),
        pyarrow.field("sumxnenergy", pyarrow.decimal128(27,5), True),
        pyarrow.field("sumipower", pyarrow.decimal128(22,0), True),
        pyarrow.field("sumxpower", pyarrow.decimal128(22,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("sumep", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_fcascomp_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("ccprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("unconstrainedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("ebp", pyarrow.decimal128(15,5), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("excessgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("fcascomp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_fcasregionrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("bidtype", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("generatorregionenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("customerregionenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("regionrecovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_gendata_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(10,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(10,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("stationid", pyarrow.large_utf8(), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("gensetid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("genergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("aenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("gpower", pyarrow.decimal128(16,6), True),
        pyarrow.field("apower", pyarrow.decimal128(16,6), True),
        pyarrow.field("rrp", pyarrow.decimal128(20,5), True),
        pyarrow.field("eep", pyarrow.decimal128(16,6), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("cprrp", pyarrow.decimal128(16,6), True),
        pyarrow.field("cpeep", pyarrow.decimal128(16,6), True),
        pyarrow.field("netenergy", pyarrow.decimal128(16,6), True),
        pyarrow.field("energycost", pyarrow.decimal128(16,6), True),
        pyarrow.field("excessenergycost", pyarrow.decimal128(16,6), True),
        pyarrow.field("apc", pyarrow.decimal128(16,6), True),
        pyarrow.field("resc", pyarrow.decimal128(16,6), True),
        pyarrow.field("resp", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("expenergy", pyarrow.decimal128(15,6), True),
        pyarrow.field("expenergycost", pyarrow.decimal128(15,6), True),
        pyarrow.field("meterrunno", pyarrow.decimal128(6,0), True),
        pyarrow.field("mda", pyarrow.large_utf8(), True),
        pyarrow.field("secondary_tlf", pyarrow.decimal128(7,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_gendataregion_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(22,10), False),
        pyarrow.field("periodid", pyarrow.decimal128(22,10), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("genergy", pyarrow.decimal128(22,0), True),
        pyarrow.field("aenergy", pyarrow.decimal128(22,0), True),
        pyarrow.field("gpower", pyarrow.decimal128(22,0), True),
        pyarrow.field("apower", pyarrow.decimal128(22,0), True),
        pyarrow.field("netenergy", pyarrow.decimal128(27,5), True),
        pyarrow.field("energycost", pyarrow.decimal128(27,5), True),
        pyarrow.field("excessenergycost", pyarrow.decimal128(27,5), True),
        pyarrow.field("expenergy", pyarrow.decimal128(27,6), True),
        pyarrow.field("expenergycost", pyarrow.decimal128(27,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_intervention_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("contractversion", pyarrow.decimal128(3,0), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("rcf", pyarrow.large_utf8(), True),
        pyarrow.field("interventionpayment", pyarrow.decimal128(12,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_interventionrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("rcf", pyarrow.large_utf8(), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("participantdemand", pyarrow.decimal128(12,5), True),
        pyarrow.field("totaldemand", pyarrow.decimal128(12,5), True),
        pyarrow.field("interventionpayment", pyarrow.decimal128(12,5), True),
        pyarrow.field("interventionamount", pyarrow.decimal128(12,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_intraregionresidues_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.int64(), False),
        pyarrow.field("periodid", pyarrow.int64(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("ep", pyarrow.decimal128(15,5), True),
        pyarrow.field("ec", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("exp", pyarrow.decimal128(15,5), True),
        pyarrow.field("irss", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_iraucsurplus_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("settlementrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractallocation", pyarrow.decimal128(8,5), True),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_irfmrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("irfmid", pyarrow.large_utf8(), False),
        pyarrow.field("irmfversion", pyarrow.decimal128(3,0), True),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("participantdemand", pyarrow.decimal128(12,5), True),
        pyarrow.field("totaltcd", pyarrow.decimal128(12,5), True),
        pyarrow.field("totaltfd", pyarrow.decimal128(12,5), True),
        pyarrow.field("irfmamount", pyarrow.decimal128(12,5), True),
        pyarrow.field("irfmpayment", pyarrow.decimal128(12,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_irnspsurplus_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("settlementrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractallocation", pyarrow.decimal128(8,5), True),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_irpartsurplus_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("settlementrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("fromregionid", pyarrow.large_utf8(), False),
        pyarrow.field("totalsurplus", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractallocation", pyarrow.decimal128(8,5), True),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_irsurplus_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("settlementrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("mwflow", pyarrow.decimal128(15,6), True),
        pyarrow.field("lossfactor", pyarrow.decimal128(15,5), True),
        pyarrow.field("surplusvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("csp_derogation_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("unadjusted_irsr", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_localareaenergy_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("settlementrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("localareaid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("ufe", pyarrow.decimal128(18,8), True),
        pyarrow.field("ddme", pyarrow.decimal128(18,8), True),
        pyarrow.field("tme", pyarrow.decimal128(18,8), True),
        pyarrow.field("adme", pyarrow.decimal128(18,8), True),
        pyarrow.field("admela", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_localareatni_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("settlementrunno", pyarrow.decimal128(3,0), False),
        pyarrow.field("localareaid", pyarrow.large_utf8(), False),
        pyarrow.field("tni", pyarrow.large_utf8(), False),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_lshedpayment_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lseprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("mcpprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lscr", pyarrow.decimal128(4,0), True),
        pyarrow.field("lsepayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("constrainedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("unconstrainedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("als", pyarrow.decimal128(15,5), True),
        pyarrow.field("initialdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("finaldemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("availabilitypayment", pyarrow.decimal128(16,6), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_lshedrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("lsepayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("lserecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("lserecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("availabilityrecovery", pyarrow.decimal128(16,6), True),
        pyarrow.field("availabilityrecovery_gen", pyarrow.decimal128(16,6), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_luloadrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagepayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagerecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("enablingrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagerecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand_gen", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_lunloadpayment_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("ebp", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("usageprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("unconstrainedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("controlrange", pyarrow.decimal128(4,0), True),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagepayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_lunloadrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagepayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagerecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("enablingrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("usagerecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("compensationrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand_gen", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_marketfees_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("marketfeeid", pyarrow.large_utf8(), False),
        pyarrow.field("marketfeevalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("energy", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("participantcategoryid", pyarrow.large_utf8(), False),
        pyarrow.field("feerate", pyarrow.decimal128(18,8), True),
        pyarrow.field("feeunits", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_reallocations_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("reallocationid", pyarrow.large_utf8(), False),
        pyarrow.field("reallocationvalue", pyarrow.decimal128(15,5), True),
        pyarrow.field("energy", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_restartpayment_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("restarttype", pyarrow.decimal128(1,0), True),
        pyarrow.field("avaflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("availabilityprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("tcf", pyarrow.decimal128(1,0), True),
        pyarrow.field("availabilitypayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("enablingpayment", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_restartrecovery_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("availabilitypayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("availabilityrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("availabilityrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingpayment", pyarrow.decimal128(18,8), True),
        pyarrow.field("enablingrecovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("enablingrecovery_gen", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_rpowerpayment_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), True),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("tlf", pyarrow.decimal128(7,5), True),
        pyarrow.field("ebp", pyarrow.decimal128(15,5), True),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("mvaraprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("mvareprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("mvargprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("synccompensation", pyarrow.decimal128(1,0), True),
        pyarrow.field("mta", pyarrow.decimal128(15,5), True),
        pyarrow.field("mtg", pyarrow.decimal128(15,5), True),
        pyarrow.field("blocksize", pyarrow.decimal128(4,0), True),
        pyarrow.field("avaflag", pyarrow.decimal128(1,0), True),
        pyarrow.field("clearedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("unconstrainedmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("availabilitypayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("contractversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("offerdate", pyarrow.timestamp('s'), True),
        pyarrow.field("offerversionno", pyarrow.decimal128(3,0), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("availabilitypayment_rebate", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_rpowerrecovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("availabilitypayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccpayment", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("availabilityrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccrecovery", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("availabilityrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("enablingrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("ccrecovery_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("participantdemand_gen", pyarrow.decimal128(15,5), True),
        pyarrow.field("regiondemand_gen", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_smallgendata_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("connectionpointid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("importenergy", pyarrow.decimal128(18,8), True),
        pyarrow.field("exportenergy", pyarrow.decimal128(18,8), True),
        pyarrow.field("rrp", pyarrow.decimal128(18,8), True),
        pyarrow.field("tlf", pyarrow.decimal128(18,8), True),
        pyarrow.field("impenergycost", pyarrow.decimal128(18,8), True),
        pyarrow.field("expenergycost", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_vicboundaryenergy_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("boundaryenergy", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_vicenergyfigures_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("totalgenoutput", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalpcsd", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("tlr", pyarrow.decimal128(15,6), True),
        pyarrow.field("mlf", pyarrow.decimal128(15,6), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_vicenergyflow_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("netflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_ancillary_summary_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("paymenttype", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("paymentamount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_apc_compensation_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("apeventid", pyarrow.int64(), False),
        pyarrow.field("claimid", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.int64(), False),
        pyarrow.field("compensation_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_apc_recovery_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("apeventid", pyarrow.int64(), False),
        pyarrow.field("claimid", pyarrow.int64(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.int64(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("region_recovery_br_amount", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_fcas_payment_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), True),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("lower6sec_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise6sec_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("lower60sec_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise60sec_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("lower5min_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise5min_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("lowerreg_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("raisereg_payment", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_fcas_recovery_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("lower6sec_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise6sec_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("lower60sec_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise60sec_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("lower5min_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise5min_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("lowerreg_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("raisereg_recovery", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("lower6sec_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise6sec_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("lower60sec_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise60sec_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("lower5min_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("raise5min_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("lowerreg_recovery_gen", pyarrow.decimal128(18,8), True),
        pyarrow.field("raisereg_recovery_gen", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_set_fcas_regulation_trk_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("cmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("crmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_factor_cmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_factor_crmpf", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_mr_payment_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("mr_capacity", pyarrow.decimal128(16,6), True),
        pyarrow.field("uncapped_payment", pyarrow.decimal128(16,6), True),
        pyarrow.field("capped_payment", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_mr_recovery_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("arodef", pyarrow.decimal128(16,6), True),
        pyarrow.field("nta", pyarrow.decimal128(16,6), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_nmas_recovery_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("paymenttype", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("rbf", pyarrow.decimal128(18,8), True),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("participant_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("region_energy", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("participant_generation", pyarrow.decimal128(18,8), True),
        pyarrow.field("region_generation", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_amount_customer", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_amount_generator", pyarrow.decimal128(18,8), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_nmas_recovery_rbf_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.decimal128(3,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("service", pyarrow.large_utf8(), False),
        pyarrow.field("contractid", pyarrow.large_utf8(), False),
        pyarrow.field("paymenttype", pyarrow.large_utf8(), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("rbf", pyarrow.decimal128(18,8), True),
        pyarrow.field("payment_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("recovery_amount", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def settlements_run_parameter_v5(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("versionno", pyarrow.int64(), False),
        pyarrow.field("parameterid", pyarrow.large_utf8(), False),
        pyarrow.field("numvalue", pyarrow.decimal128(18,8), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def stpasa_casesolution_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("pasaversion", pyarrow.large_utf8(), True),
        pyarrow.field("reservecondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("lorcondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("capacityobjfunction", pyarrow.decimal128(12,3), True),
        pyarrow.field("capacityoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("maxsurplusreserveoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("maxsparecapacityoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("interconnectorflowpenalty", pyarrow.decimal128(12,3), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("reliabilitylrcdemandoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("outagelrcdemandoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("lordemandoption", pyarrow.decimal128(12,3), True),
        pyarrow.field("reliabilitylrccapacityoption", pyarrow.large_utf8(), True),
        pyarrow.field("outagelrccapacityoption", pyarrow.large_utf8(), True),
        pyarrow.field("lorcapacityoption", pyarrow.large_utf8(), True),
        pyarrow.field("loruigf_option", pyarrow.decimal128(3,0), True),
        pyarrow.field("reliability_lrcuigf_option", pyarrow.decimal128(3,0), True),
        pyarrow.field("outage_lrcuigf_option", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def stpasa_constraintsolution_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("constraintid", pyarrow.large_utf8(), False),
        pyarrow.field("capacityrhs", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacitymarginalvalue", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacityviolationdegree", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("studyregionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def stpasa_interconnectorsoln_v3(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("capacitymwflow", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacitymarginalvalue", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacityviolationdegree", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedexportlimit", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedimportlimit", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("exportlimitconstraintid", pyarrow.large_utf8(), True),
        pyarrow.field("importlimitconstraintid", pyarrow.large_utf8(), True),
        pyarrow.field("studyregionid", pyarrow.large_utf8(), False)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def stpasa_regionsolution_v6(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("interval_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("demand10", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand50", pyarrow.decimal128(12,2), True),
        pyarrow.field("demand90", pyarrow.decimal128(12,2), True),
        pyarrow.field("reservereq", pyarrow.decimal128(12,2), True),
        pyarrow.field("capacityreq", pyarrow.decimal128(12,2), True),
        pyarrow.field("energyreqdemand50", pyarrow.decimal128(12,2), True),
        pyarrow.field("unconstrainedcapacity", pyarrow.decimal128(12,0), True),
        pyarrow.field("constrainedcapacity", pyarrow.decimal128(12,0), True),
        pyarrow.field("netinterchangeunderscarcity", pyarrow.decimal128(12,2), True),
        pyarrow.field("surpluscapacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("surplusreserve", pyarrow.decimal128(12,2), True),
        pyarrow.field("reservecondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("maxsurplusreserve", pyarrow.decimal128(12,2), True),
        pyarrow.field("maxsparecapacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lorcondition", pyarrow.decimal128(1,0), True),
        pyarrow.field("aggregatecapacityavailable", pyarrow.decimal128(12,2), True),
        pyarrow.field("aggregatescheduledload", pyarrow.decimal128(12,2), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("aggregatepasaavailability", pyarrow.decimal128(12,0), True),
        pyarrow.field("runtype", pyarrow.large_utf8(), False),
        pyarrow.field("energyreqdemand10", pyarrow.decimal128(12,2), True),
        pyarrow.field("calculatedlor1level", pyarrow.decimal128(16,6), True),
        pyarrow.field("calculatedlor2level", pyarrow.decimal128(16,6), True),
        pyarrow.field("msrnetinterchangeunderscarcity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lornetinterchangeunderscarcity", pyarrow.decimal128(12,2), True),
        pyarrow.field("totalintermittentgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("demand_and_nonschedgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("uigf", pyarrow.decimal128(12,2), True),
        pyarrow.field("semi_scheduled_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lor_semi_scheduled_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("lcr", pyarrow.decimal128(16,6), True),
        pyarrow.field("lcr2", pyarrow.decimal128(16,6), True),
        pyarrow.field("fum", pyarrow.decimal128(16,6), True),
        pyarrow.field("ss_solar_uigf", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_wind_uigf", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_solar_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_wind_capacity", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_solar_cleared", pyarrow.decimal128(12,2), True),
        pyarrow.field("ss_wind_cleared", pyarrow.decimal128(12,2), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def trading_averageprice30_v1(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("perioddate", pyarrow.timestamp('s'), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("price_confidence", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def trading_interconnectorres_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("interconnectorid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("meteredmwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwflow", pyarrow.decimal128(15,5), True),
        pyarrow.field("mwlosses", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def trading_unit_solution_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("duid", pyarrow.large_utf8(), False),
        pyarrow.field("tradetype", pyarrow.decimal128(2,0), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("initialmw", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalcleared", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampdownrate", pyarrow.decimal128(15,5), True),
        pyarrow.field("rampuprate", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5min", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6sec", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("lowerreg", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereg", pyarrow.decimal128(15,5), True),
        pyarrow.field("availability", pyarrow.decimal128(15,5), True),
        pyarrow.field("semidispatchcap", pyarrow.decimal128(3,0), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def trading_price_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("rrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("eep", pyarrow.decimal128(15,5), True),
        pyarrow.field("invalidflag", pyarrow.large_utf8(), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("rop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregrrp", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregrop", pyarrow.decimal128(15,5), True),
        pyarrow.field("price_status", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def trading_regionsum_v4(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("settlementdate", pyarrow.timestamp('s'), False),
        pyarrow.field("runno", pyarrow.decimal128(3,0), False),
        pyarrow.field("regionid", pyarrow.large_utf8(), False),
        pyarrow.field("periodid", pyarrow.decimal128(3,0), False),
        pyarrow.field("totaldemand", pyarrow.decimal128(15,5), True),
        pyarrow.field("availablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("availableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("demandforecast", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchablegeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("dispatchableload", pyarrow.decimal128(15,5), True),
        pyarrow.field("netinterchange", pyarrow.decimal128(15,5), True),
        pyarrow.field("excessgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5mindispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secdispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secsupplyprice", pyarrow.decimal128(15,5), True),
        pyarrow.field("lastchanged", pyarrow.timestamp('s'), True),
        pyarrow.field("initialsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("clearedsupply", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregimport", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocaldispatch", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregreq", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raisereglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minlocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerreglocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6seclocalviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raiseregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("raise6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower5minviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lowerregviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower60secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("lower6secviolation", pyarrow.decimal128(15,5), True),
        pyarrow.field("totalintermittentgeneration", pyarrow.decimal128(15,5), True),
        pyarrow.field("demand_and_nonschedgen", pyarrow.decimal128(15,5), True),
        pyarrow.field("uigf", pyarrow.decimal128(15,5), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def voltage_instruction_instruction_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("ems_id", pyarrow.large_utf8(), False),
        pyarrow.field("participantid", pyarrow.large_utf8(), True),
        pyarrow.field("station_id", pyarrow.large_utf8(), True),
        pyarrow.field("device_id", pyarrow.large_utf8(), True),
        pyarrow.field("device_type", pyarrow.large_utf8(), True),
        pyarrow.field("control_type", pyarrow.large_utf8(), True),
        pyarrow.field("target", pyarrow.decimal128(15,0), True),
        pyarrow.field("conforming", pyarrow.decimal128(1,0), True),
        pyarrow.field("instruction_summary", pyarrow.large_utf8(), True),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("instruction_sequence", pyarrow.decimal128(4,0), True),
        pyarrow.field("additional_notes", pyarrow.large_utf8(), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def voltage_instruction_track_v2(file_path):
    schema = pyarrow.schema([
        pyarrow.field("file_id", pyarrow.large_utf8(), False),
        pyarrow.field("run_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("file_type", pyarrow.large_utf8(), True),
        pyarrow.field("version_datetime", pyarrow.timestamp('s'), False),
        pyarrow.field("se_datetime", pyarrow.timestamp('s'), True),
        pyarrow.field("solution_category", pyarrow.large_utf8(), True),
        pyarrow.field("solution_status", pyarrow.large_utf8(), True),
        pyarrow.field("operating_mode", pyarrow.large_utf8(), True),
        pyarrow.field("operating_status", pyarrow.large_utf8(), True),
        pyarrow.field("est_expiry", pyarrow.timestamp('s'), True),
        pyarrow.field("est_next_instruction", pyarrow.timestamp('s'), True)
    ])
    table = pc.read_csv(file_path, convert_options=pc.ConvertOptions(column_types={ schema.field(i).name: schema.field(i).type for i in range(0, len(schema.names)) }, timestamp_parsers=["%Y/%m/%d %H:%M:%S"]))
    return table.cast(schema)

def get_csv_reader(*,file_path: str, data_set: str, sub_type: Optional[str] = None):
    mapping = {
        ("ancilliary_services", "contractagc"): ancilliary_services_contractagc_v1,
        ("ancilliary_services", "contractloadshed"): ancilliary_services_contractloadshed_v2,
        ("ancilliary_services", "contractreactivepower"): ancilliary_services_contractreactivepower_v4,
        ("ancilliary_services", "contractrestartservices"): ancilliary_services_contractrestartservices_v2,
        ("ancilliary_services", "contractrestartunits"): ancilliary_services_contractrestartunits_v1,
        ("asoffer", "offeragcdata"): asoffer_offeragcdata_v1,
        ("asoffer", "offerastrk"): asoffer_offerastrk_v1,
        ("asoffer", "offerlsheddata"): asoffer_offerlsheddata_v1,
        ("asoffer", "offerrestartdata"): asoffer_offerrestartdata_v1,
        ("asoffer", "offerrpowerdata"): asoffer_offerrpowerdata_v1,
        ("bids", "biddayoffer"): bids_biddayoffer_v1,
        ("bids", "bidofferfiletrk"): bids_bidofferfiletrk_v1,
        ("bids", "bidofferperiod"): bids_bidofferperiod_v1,
        ("offer", "bidperoffer"): offer_bidperoffer_v1,
        ("bids", "mnsp_bidofferperiod"): bids_mnsp_bidofferperiod_v1,
        ("bids", "mnsp_dayoffer"): bids_mnsp_dayoffer_v1,
        ("bid", "mnsp_filetrk"): bid_mnsp_filetrk_v1,
        ("bid", "mnsp_offertrk"): bid_mnsp_offertrk_v1,
        ("bid", "mnsp_peroffer"): bid_mnsp_peroffer_v1,
        ("offer", "mtpasa_offerdata"): offer_mtpasa_offerdata_v1,
        ("offer", "mtpasa_offerfiletrk"): offer_mtpasa_offerfiletrk_v1,
        ("billing_config", "billingcalendar"): billing_config_billingcalendar_v2,
        ("billing_config", "gst_bas_class"): billing_config_gst_bas_class_v1,
        ("billing_config", "gst_rate"): billing_config_gst_rate_v1,
        ("billing_config", "gst_transaction_class"): billing_config_gst_transaction_class_v1,
        ("billing_config", "gst_transaction_type"): billing_config_gst_transaction_type_v1,
        ("billing_config", "secdeposit_interest_rate"): billing_config_secdeposit_interest_rate_v1,
        ("billing_config", "secdeposit_provision"): billing_config_secdeposit_provision_v1,
        ("billing", "aspayments"): billing_aspayments_v6,
        ("billing", "asrecovery"): billing_asrecovery_v7,
        ("billing", "cpdata"): billing_cpdata_v6,
        ("billing", "daytrk"): billing_daytrk_v5,
        ("billing", "fees"): billing_fees_v5,
        ("billing", "financialadjustments"): billing_financialadjustments_v5,
        ("billing", "gendata"): billing_gendata_v5,
        ("billing", "interresidues"): billing_interresidues_v5,
        ("billing", "intraresidues"): billing_intraresidues_v5,
        ("billing", "iraucsurplus"): billing_iraucsurplus_v5,
        ("billing", "iraucsurplussum"): billing_iraucsurplussum_v7,
        ("billing", "irfm"): billing_irfm_v5,
        ("billing", "irnspsurplus"): billing_irnspsurplus_v5,
        ("billing", "irnspsurplussum"): billing_irnspsurplussum_v6,
        ("billing", "irpartsurplus"): billing_irpartsurplus_v5,
        ("billing", "irpartsurplussum"): billing_irpartsurplussum_v7,
        ("billing", "prioradjustments"): billing_prioradjustments_v5,
        ("billing", "realloc"): billing_realloc_v5,
        ("billing", "realloc_detail"): billing_realloc_detail_v5,
        ("billing", "regionexports"): billing_regionexports_v5,
        ("billing", "regionfigures"): billing_regionfigures_v5,
        ("billing", "regionimports"): billing_regionimports_v5,
        ("billing", "runtrk"): billing_runtrk_v5,
        ("billing", "smelterreduction"): billing_smelterreduction_v5,
        ("billing", "apc_compensation"): billing_apc_compensation_v2,
        ("billing", "apc_recovery"): billing_apc_recovery_v2,
        ("billing", "billing_co2e_publication"): billing_billing_co2e_publication_v1,
        ("billing", "billing_co2e_publication_trk"): billing_billing_co2e_publication_trk_v1,
        ("billing", "daily_energy_summary"): billing_daily_energy_summary_v1,
        ("billing", "direction_reconciliatn"): billing_direction_reconciliatn_v1,
        ("billing", "billing_direction_recon_other"): billing_billing_direction_recon_other_v1,
        ("billing", "eftshortfall_amount"): billing_eftshortfall_amount_v1,
        ("billing", "eftshortfall_detail"): billing_eftshortfall_detail_v1,
        ("billing", "gst_detail"): billing_gst_detail_v5,
        ("billing", "gst_summary"): billing_gst_summary_v5,
        ("billing", "mr_payment"): billing_mr_payment_v5,
        ("billing", "mr_recovery"): billing_mr_recovery_v5,
        ("billing", "mr_shortfall"): billing_mr_shortfall_v5,
        ("billing", "mr_summary"): billing_mr_summary_v5,
        ("billing", "nmas_tst_payments"): billing_nmas_tst_payments_v1,
        ("billing", "nmas_tst_recovery"): billing_nmas_tst_recovery_v1,
        ("billing", "nmas_tst_recvry_rbf"): billing_nmas_tst_recvry_rbf_v1,
        ("billing", "nmas_tst_recvry_trk"): billing_nmas_tst_recvry_trk_v1,
        ("billing", "secdeposit_application"): billing_secdeposit_application_v1,
        ("billing", "secdep_interest_pay"): billing_secdep_interest_pay_v1,
        ("billing", "secdep_interest_rate"): billing_secdep_interest_rate_v1,
        ("billing", "reservetraderpayment"): billing_reservetraderpayment_v1,
        ("billing", "reservetraderrecovery"): billing_reservetraderrecovery_v1,
        ("billing", "whitehole"): billing_whitehole_v5,
        ("operational_demand", "actual"): operational_demand_actual_v2,
        ("operational_demand", "forecast"): operational_demand_forecast_v1,
        ("demand", "intermittent_cluster_avail"): demand_intermittent_cluster_avail_v1,
        ("demand", "intermittent_cluster_avail_day"): demand_intermittent_cluster_avail_day_v1,
        ("demand", "intermittent_ds_pred"): demand_intermittent_ds_pred_v1,
        ("demand", "intermittent_ds_run"): demand_intermittent_ds_run_v1,
        ("forecast", "intermittent_gen"): forecast_intermittent_gen_v1,
        ("forecast", "intermittent_gen_data"): forecast_intermittent_gen_data_v1,
        ("demand", "intermittent_gen_limit"): demand_intermittent_gen_limit_v1,
        ("demand", "intermittent_gen_limit_day"): demand_intermittent_gen_limit_day_v1,
        ("demand", "mtpasa_intermittent_avail"): demand_mtpasa_intermittent_avail_v1,
        ("demand", "mtpasa_intermittent_limit"): demand_mtpasa_intermittent_limit_v1,
        ("demand", "period"): demand_period_v1,
        ("demand", "trk"): demand_trk_v1,
        ("rooftop", "actual"): rooftop_actual_v2,
        ("rooftop", "forecast"): rooftop_forecast_v1,
        ("priceload", "constraintrelaxation"): priceload_constraintrelaxation_v1,
        ("dispatch", "blocked_constraints"): dispatch_blocked_constraints_v1,
        ("dispatch", "case_solution"): dispatch_case_solution_v2,
        ("dispatch", "constraint"): dispatch_constraint_v5,
        ("dispatch", "interconnectorres"): dispatch_interconnectorres_v3,
        ("dispatch", "unit_solution"): dispatch_unit_solution_v2,
        ("dispatch", "offertrk"): dispatch_offertrk_v1,
        ("dispatch", "price"): dispatch_price_v4,
        ("dispatch", "regionsum"): dispatch_regionsum_v5,
        ("priceload", "constraint_fcas_ocd"): priceload_constraint_fcas_ocd_v1,
        ("dispatch", "fcas_req"): dispatch_fcas_req_v2,
        ("dispatch", "interconnection"): dispatch_interconnection_v1,
        ("dispatch", "local_price"): dispatch_local_price_v1,
        ("dispatch", "mnspbidtrk"): dispatch_mnspbidtrk_v1,
        ("dispatch", "mr_schedule_trk"): dispatch_mr_schedule_trk_v1,
        ("priceload", "price_revision"): priceload_price_revision_v1,
        ("dispatch", "unit_conformance"): dispatch_unit_conformance_v1,
        ("dispatch", "unit_scada"): dispatch_unit_scada_v1,
        ("dispatch", "intermittent_forecast_trk"): dispatch_intermittent_forecast_trk_v1,
        ("dispatch", "negative_residue"): dispatch_negative_residue_v1,
        ("ap", "apevent"): ap_apevent_v1,
        ("ap", "apeventregion"): ap_apeventregion_v1,
        ("force_majeure", "irfmamount"): force_majeure_irfmamount_v1,
        ("force_majeure", "irfmevents"): force_majeure_irfmevents_v1,
        ("force_majeure", "market_suspend_regime_sum"): force_majeure_market_suspend_regime_sum_v1,
        ("force_majeure", "market_suspend_region_sum"): force_majeure_market_suspend_region_sum_v1,
        ("force_majeure", "market_suspend_schedule"): force_majeure_market_suspend_schedule_v1,
        ("force_majeure", "market_suspend_schedule_trk"): force_majeure_market_suspend_schedule_trk_v1,
        ("force_majeure", "overriderrp"): force_majeure_overriderrp_v1,
        ("ap", "regionapc"): ap_regionapc_v1,
        ("ap", "regionapcintervals"): ap_regionapcintervals_v1,
        ("gd_instruct", "gdinstruct"): gd_instruct_gdinstruct_v1,
        ("gd_instruct", "instructionsubtype"): gd_instruct_instructionsubtype_v1,
        ("gd_instruct", "instructiontype"): gd_instruct_instructiontype_v1,
        ("generic_constraint", "emsmaster"): generic_constraint_emsmaster_v1,
        ("gencondata", "null"): gencondata_null_v6,
        ("genconset", "null"): genconset_null_v1,
        ("generic_constraint", "genconsetinvoke"): generic_constraint_genconsetinvoke_v2,
        ("genconsettrk", "null"): genconsettrk_null_v2,
        ("gcrhs", "null"): gcrhs_null_v1,
        ("geqdesc", "null"): geqdesc_null_v2,
        ("geqrhs", "null"): geqrhs_null_v1,
        ("spdcpc", "null"): spdcpc_null_v2,
        ("spdicc", "null"): spdicc_null_v1,
        ("spdrc", "null"): spdrc_null_v2,
        ("irauction_config", "auction"): irauction_config_auction_v1,
        ("irauction_config", "auction_calendar"): irauction_config_auction_calendar_v2,
        ("irauction_config", "auction_ic_allocations"): irauction_config_auction_ic_allocations_v2,
        ("irauction_config", "auction_revenue_estimate"): irauction_config_auction_revenue_estimate_v1,
        ("irauction_config", "auction_revenue_track"): irauction_config_auction_revenue_track_v1,
        ("irauction_config", "auction_rp_estimate"): irauction_config_auction_rp_estimate_v1,
        ("irauction_config", "auction_tranche"): irauction_config_auction_tranche_v1,
        ("settlement_config", "residuecontractpayments"): settlement_config_residuecontractpayments_v1,
        ("irauction_bids", "file_trk"): irauction_bids_file_trk_v1,
        ("irauction", "residue_bid_trk"): irauction_residue_bid_trk_v1,
        ("irauction", "residue_contracts"): irauction_residue_contracts_v1,
        ("irauction", "residue_con_data"): irauction_residue_con_data_v2,
        ("irauction", "residue_con_estimates_trk"): irauction_residue_con_estimates_trk_v1,
        ("irauction", "residue_con_funds"): irauction_residue_con_funds_v1,
        ("irauction_bids", "funds_bid"): irauction_bids_funds_bid_v1,
        ("irauction_bids", "price_bid"): irauction_bids_price_bid_v1,
        ("irauction", "residue_price_funds_bid"): irauction_residue_price_funds_bid_v1,
        ("irauction", "residue_public_data"): irauction_residue_public_data_v1,
        ("irauction", "residue_trk"): irauction_residue_trk_v1,
        ("irauction", "sra_cash_security"): irauction_sra_cash_security_v1,
        ("irauction", "sra_financial_aucpay_detail"): irauction_sra_financial_aucpay_detail_v1,
        ("irauction", "sra_financial_aucpay_sum"): irauction_sra_financial_aucpay_sum_v1,
        ("irauction", "sra_financial_auc_mardetail"): irauction_sra_financial_auc_mardetail_v1,
        ("irauction", "sra_financial_auc_margin"): irauction_sra_financial_auc_margin_v1,
        ("irauction", "sra_financial_auc_receipts"): irauction_sra_financial_auc_receipts_v1,
        ("irauction", "sra_financial_runtrk"): irauction_sra_financial_runtrk_v1,
        ("irauction", "sra_offer_product"): irauction_sra_offer_product_v1,
        ("irauction", "sra_offer_profile"): irauction_sra_offer_profile_v1,
        ("irauction", "sra_prudential_cash_security"): irauction_sra_prudential_cash_security_v1,
        ("irauction", "sra_prudential_comp_position"): irauction_sra_prudential_comp_position_v1,
        ("irauction", "sra_prudential_exposure"): irauction_sra_prudential_exposure_v1,
        ("irauction", "sra_prudential_run"): irauction_sra_prudential_run_v1,
        ("irauction", "valuationid"): irauction_valuationid_v1,
        ("market_config", "bidtypes"): market_config_bidtypes_v1,
        ("market_config", "bidtypestrk"): market_config_bidtypestrk_v1,
        ("market_config", "interconnector"): market_config_interconnector_v1,
        ("market_config", "interconnectoralloc"): market_config_interconnectoralloc_v1,
        ("market_config", "interconnectorconstraint"): market_config_interconnectorconstraint_v1,
        ("market_config", "intraregionalloc"): market_config_intraregionalloc_v1,
        ("market_config", "lossfactormodel"): market_config_lossfactormodel_v1,
        ("market_config", "lossmodel"): market_config_lossmodel_v1,
        ("market_config", "market_price_thresholds"): market_config_market_price_thresholds_v1,
        ("market_config", "region"): market_config_region_v1,
        ("market_config", "regionstandingdata"): market_config_regionstandingdata_v1,
        ("market_config", "transmissionlossfactor"): market_config_transmissionlossfactor_v2,
        ("market_notice", "marketnoticedata"): market_notice_marketnoticedata_v1,
        ("market_notice", "marketnoticetype"): market_notice_marketnoticetype_v1,
        ("market_notice", "participantnoticetrk"): market_notice_participantnoticetrk_v1,
        ("mcc", "casesolution"): mcc_casesolution_v1,
        ("mcc", "constraintsolution"): mcc_constraintsolution_v1,
        ("meterdata", "aggregate_reads"): meterdata_aggregate_reads_v1,
        ("meterdata", "individual_reads"): meterdata_individual_reads_v1,
        ("meterdata", "interconnector"): meterdata_interconnector_v1,
        ("meterdata", "trk"): meterdata_trk_v1,
        ("mr", "dayoffer_stack"): mr_dayoffer_stack_v1,
        ("mr", "event"): mr_event_v1,
        ("mr", "event_schedule"): mr_event_schedule_v1,
        ("mr", "peroffer_stack"): mr_peroffer_stack_v1,
        ("mtpasa", "caseresult"): mtpasa_caseresult_v1,
        ("mtpasa", "constraintresult"): mtpasa_constraintresult_v1,
        ("mtpasa", "constraintsummary"): mtpasa_constraintsummary_v1,
        ("mtpasa", "duidavailability"): mtpasa_duidavailability_v1,
        ("mtpasa", "interconnectorresult"): mtpasa_interconnectorresult_v1,
        ("mtpasa", "lolpresult"): mtpasa_lolpresult_v1,
        ("mtpasa", "regionavailability"): mtpasa_regionavailability_v3,
        ("mtpasa", "regionavailtrk"): mtpasa_regionavailtrk_v1,
        ("mtpasa", "regioniteration"): mtpasa_regioniteration_v1,
        ("mtpasa", "regionresult"): mtpasa_regionresult_v2,
        ("mtpasa", "regionsummary"): mtpasa_regionsummary_v1,
        ("network", "equipmentdetail"): network_equipmentdetail_v1,
        ("network", "outageconstraintset"): network_outageconstraintset_v1,
        ("network", "outagedetail"): network_outagedetail_v3,
        ("network", "outagestatuscode"): network_outagestatuscode_v1,
        ("network", "rating"): network_rating_v1,
        ("network", "realtimerating"): network_realtimerating_v1,
        ("network", "staticrating"): network_staticrating_v1,
        ("network", "substationdetail"): network_substationdetail_v1,
        ("p5min", "blocked_constraints"): p5min_blocked_constraints_v1,
        ("p5min", "casesolution"): p5min_casesolution_v2,
        ("p5min", "constraintsolution"): p5min_constraintsolution_v6,
        ("p5min", "interconnectorsoln"): p5min_interconnectorsoln_v4,
        ("p5min", "intersensitivities"): p5min_intersensitivities_v1,
        ("p5min", "local_price"): p5min_local_price_v1,
        ("p5min", "pricesensitivities"): p5min_pricesensitivities_v1,
        ("p5min", "regionsolution"): p5min_regionsolution_v6,
        ("p5min", "scenariodemand"): p5min_scenariodemand_v1,
        ("p5min", "scenariodemandtrk"): p5min_scenariodemandtrk_v1,
        ("p5min", "unitsolution"): p5min_unitsolution_v3,
        ("participant_registration", "bidduiddetails"): participant_registration_bidduiddetails_v1,
        ("participant_registration", "bidduiddetailstrk"): participant_registration_bidduiddetailstrk_v1,
        ("participant_registration", "dispatchableunit"): participant_registration_dispatchableunit_v1,
        ("participant_registration", "dualloc"): participant_registration_dualloc_v1,
        ("participant_registration", "dudetail"): participant_registration_dudetail_v3,
        ("participant_registration", "dudetailsummary"): participant_registration_dudetailsummary_v4,
        ("participant_registration", "genmeter"): participant_registration_genmeter_v1,
        ("participant_registration", "genunits"): participant_registration_genunits_v2,
        ("participant_registration", "genunits_unit"): participant_registration_genunits_unit_v1,
        ("participant_registration", "mnsp_interconnector"): participant_registration_mnsp_interconnector_v2,
        ("participant_registration", "mnsp_participant"): participant_registration_mnsp_participant_v1,
        ("participant_registration", "participant"): participant_registration_participant_v1,
        ("participant_registration", "participantaccount"): participant_registration_participantaccount_v1,
        ("participant_registration", "participantcategory"): participant_registration_participantcategory_v1,
        ("participant_registration", "participantcategoryalloc"): participant_registration_participantcategoryalloc_v1,
        ("participant_registration", "participantclass"): participant_registration_participantclass_v1,
        ("participant_registration", "participantcreditdetail"): participant_registration_participantcreditdetail_v1,
        ("participant_registration", "stadualloc"): participant_registration_stadualloc_v1,
        ("participant_registration", "station"): participant_registration_station_v1,
        ("participant_registration", "stationoperatingstatus"): participant_registration_stationoperatingstatus_v1,
        ("participant_registration", "stationowner"): participant_registration_stationowner_v1,
        ("participant_registration", "stationownertrk"): participant_registration_stationownertrk_v1,
        ("pdpasa", "casesolution"): pdpasa_casesolution_v3,
        ("pdpasa", "constraintsolution"): pdpasa_constraintsolution_v1,
        ("pdpasa", "interconnectorsoln"): pdpasa_interconnectorsoln_v1,
        ("pdpasa", "regionsolution"): pdpasa_regionsolution_v6,
        ("predispatch", "blocked_constraints"): predispatch_blocked_constraints_v1,
        ("predispatch", "case_solution"): predispatch_case_solution_v1,
        ("predispatch", "constraint_solution"): predispatch_constraint_solution_v5,
        ("predispatch", "interconnector_soln"): predispatch_interconnector_soln_v3,
        ("predispatch", "interconnectr_sens"): predispatch_interconnectr_sens_v1,
        ("predispatch", "unit_solution"): predispatch_unit_solution_v2,
        ("predispatch", "offertrk"): predispatch_offertrk_v1,
        ("predispatch", "region_prices"): predispatch_region_prices_v1,
        ("predispatch", "pricesensitivities"): predispatch_pricesensitivities_v1,
        ("predispatch", "region_solution"): predispatch_region_solution_v5,
        ("predispatch", "scenario_demand"): predispatch_scenario_demand_v1,
        ("predispatch", "scenario_demand_trk"): predispatch_scenario_demand_trk_v1,
        ("predispatch", "regionfcasrequirement"): predispatch_regionfcasrequirement_v2,
        ("predispatch", "local_price"): predispatch_local_price_v1,
        ("predispatch", "mnspbidtrk"): predispatch_mnspbidtrk_v1,
        ("prudential", "company_position"): prudential_company_position_v1,
        ("prudential", "runtrk"): prudential_runtrk_v1,
        ("mtpasa", "reservelimit"): mtpasa_reservelimit_v1,
        ("mtpasa", "reservelimit_region"): mtpasa_reservelimit_region_v1,
        ("mtpasa", "reservelimit_set"): mtpasa_reservelimit_set_v1,
        ("reserve_data", "reserve"): reserve_data_reserve_v1,
        ("settlement_config", "ancillary_recovery_split"): settlement_config_ancillary_recovery_split_v1,
        ("settlement_config", "marketfee"): settlement_config_marketfee_v1,
        ("settlement_config", "marketfeedata"): settlement_config_marketfeedata_v1,
        ("settlement_config", "marketfeetrk"): settlement_config_marketfeetrk_v1,
        ("settlement_config", "market_fee_cat_excl"): settlement_config_market_fee_cat_excl_v1,
        ("settlement_config", "market_fee_cat_excl_trk"): settlement_config_market_fee_cat_excl_trk_v1,
        ("settlement_config", "market_fee_exclusion"): settlement_config_market_fee_exclusion_v1,
        ("settlement_config", "market_fee_exclusion_trk"): settlement_config_market_fee_exclusion_trk_v1,
        ("settlement_config", "participant_bandfee_alloc"): settlement_config_participant_bandfee_alloc_v1,
        ("setcfg", "reallocation"): setcfg_reallocation_v2,
        ("setcfg", "reallocationinterval"): setcfg_reallocationinterval_v1,
        ("settlement_config", "setcfg_participant_mpf"): settlement_config_setcfg_participant_mpf_v1,
        ("settlement_config", "setcfg_participant_mpftrk"): settlement_config_setcfg_participant_mpftrk_v1,
        ("settlements", "daytrack"): settlements_daytrack_v6,
        ("settlements", "agcpayment"): settlements_agcpayment_v5,
        ("settlements", "agcrecovery"): settlements_agcrecovery_v5,
        ("settlements", "cpdata"): settlements_cpdata_v6,
        ("settlements", "cpdataregion"): settlements_cpdataregion_v5,
        ("settlements", "fcascomp"): settlements_fcascomp_v5,
        ("settlements", "fcasregionrecovery"): settlements_fcasregionrecovery_v5,
        ("settlements", "gendata"): settlements_gendata_v6,
        ("settlements", "gendataregion"): settlements_gendataregion_v5,
        ("settlements", "intervention"): settlements_intervention_v5,
        ("settlements", "interventionrecovery"): settlements_interventionrecovery_v5,
        ("settlements", "intraregionresidues"): settlements_intraregionresidues_v5,
        ("settlements", "iraucsurplus"): settlements_iraucsurplus_v6,
        ("settlements", "irfmrecovery"): settlements_irfmrecovery_v5,
        ("settlements", "irnspsurplus"): settlements_irnspsurplus_v6,
        ("settlements", "irpartsurplus"): settlements_irpartsurplus_v6,
        ("settlements", "irsurplus"): settlements_irsurplus_v6,
        ("settlements", "localareaenergy"): settlements_localareaenergy_v1,
        ("settlements", "localareatni"): settlements_localareatni_v1,
        ("settlements", "lshedpayment"): settlements_lshedpayment_v5,
        ("settlements", "lshedrecovery"): settlements_lshedrecovery_v5,
        ("settlements", "luloadrecovery"): settlements_luloadrecovery_v5,
        ("settlements", "lunloadpayment"): settlements_lunloadpayment_v5,
        ("settlements", "lunloadrecovery"): settlements_lunloadrecovery_v5,
        ("settlements", "marketfees"): settlements_marketfees_v6,
        ("settlements", "reallocations"): settlements_reallocations_v5,
        ("settlements", "restartpayment"): settlements_restartpayment_v6,
        ("settlements", "restartrecovery"): settlements_restartrecovery_v6,
        ("settlements", "rpowerpayment"): settlements_rpowerpayment_v6,
        ("settlements", "rpowerrecovery"): settlements_rpowerrecovery_v5,
        ("settlements", "smallgendata"): settlements_smallgendata_v1,
        ("settlements", "vicboundaryenergy"): settlements_vicboundaryenergy_v5,
        ("settlements", "vicenergyfigures"): settlements_vicenergyfigures_v5,
        ("settlements", "vicenergyflow"): settlements_vicenergyflow_v5,
        ("settlements", "ancillary_summary"): settlements_ancillary_summary_v5,
        ("settlements", "apc_compensation"): settlements_apc_compensation_v1,
        ("settlements", "apc_recovery"): settlements_apc_recovery_v1,
        ("settlements", "fcas_payment"): settlements_fcas_payment_v5,
        ("settlements", "fcas_recovery"): settlements_fcas_recovery_v6,
        ("settlements", "set_fcas_regulation_trk"): settlements_set_fcas_regulation_trk_v1,
        ("settlements", "mr_payment"): settlements_mr_payment_v5,
        ("settlements", "mr_recovery"): settlements_mr_recovery_v5,
        ("settlements", "nmas_recovery"): settlements_nmas_recovery_v2,
        ("settlements", "nmas_recovery_rbf"): settlements_nmas_recovery_rbf_v1,
        ("settlements", "run_parameter"): settlements_run_parameter_v5,
        ("stpasa", "casesolution"): stpasa_casesolution_v3,
        ("stpasa", "constraintsolution"): stpasa_constraintsolution_v3,
        ("stpasa", "interconnectorsoln"): stpasa_interconnectorsoln_v3,
        ("stpasa", "regionsolution"): stpasa_regionsolution_v6,
        ("trading", "averageprice30"): trading_averageprice30_v1,
        ("trading", "interconnectorres"): trading_interconnectorres_v2,
        ("trading", "unit_solution"): trading_unit_solution_v2,
        ("trading", "price"): trading_price_v2,
        ("trading", "regionsum"): trading_regionsum_v4,
        ("voltage_instruction", "instruction"): voltage_instruction_instruction_v2,
        ("voltage_instruction", "track"): voltage_instruction_track_v2,
    }
    if sub_type is not None:
        return mapping[(data_set.lower(), sub_type.lower())](file_path)
    else: 
        return mapping[(data_set.lower(), "null")](file_path)
    