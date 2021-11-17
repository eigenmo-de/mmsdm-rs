
from typing import List, Optional
import datetime

def get_row_partition_name(row: List[str]) -> str:
    if len(row) < 4:
        raise Exception("Row of length {} is too short, should be at least 4".format(len(row)))
    if row[0] != 'D':
        raise Exception("Row should be a data row but was instead `{}`".format(row[0]))
    mapping = {
        ("BIDS","BIDDAYOFFER"): 6,
        ("BIDS","MNSP_DAYOFFER"): 4,
        ("BILLING","DAYTRK"): 7,
        ("BILLING","BILLING_CO2E_PUBLICATION"): 7,
        ("BILLING","DAILY_ENERGY_SUMMARY"): 7,
        ("DEMAND","PERIOD"): 5,
        ("PRICELOAD","CONSTRAINTRELAXATION"): 4,
        ("DISPATCH","BLOCKED_CONSTRAINTS"): 4,
        ("DISPATCH","CASE_SOLUTION"): 4,
        ("DISPATCH","CONSTRAINT"): 4,
        ("DISPATCH","INTERCONNECTORRES"): 4,
        ("DISPATCH","UNIT_SOLUTION"): 4,
        ("DISPATCH","OFFERTRK"): 4,
        ("DISPATCH","PRICE"): 4,
        ("DISPATCH","REGIONSUM"): 4,
        ("PRICELOAD","CONSTRAINT_FCAS_OCD"): 4,
        ("DISPATCH","FCAS_REQ"): 4,
        ("DISPATCH","INTERCONNECTION"): 4,
        ("DISPATCH","LOCAL_PRICE"): 4,
        ("DISPATCH","MNSPBIDTRK"): 4,
        ("DISPATCH","MR_SCHEDULE_TRK"): 4,
        ("PRICELOAD","PRICE_REVISION"): 4,
        ("DISPATCH","UNIT_SCADA"): 4,
        ("DISPATCH","INTERMITTENT_FORECAST_TRK"): 4,
        ("DISPATCH","NEGATIVE_RESIDUE"): 4,
        ("METERDATA","AGGREGATE_READS"): 5,
        ("METERDATA","INDIVIDUAL_READS"): 5,
        ("METERDATA","INTERCONNECTOR"): 5,
        ("NETWORK","REALTIMERATING"): 4,
        ("PREDISPATCH","MNSPBIDTRK"): 8,
        ("RESERVE_DATA","RESERVE"): 4,
        ("SETTLEMENTS","DAYTRACK"): 4,
        ("SETTLEMENTS","CPDATA"): 4,
        ("SETTLEMENTS","CPDATAREGION"): 4,
        ("SETTLEMENTS","FCASREGIONRECOVERY"): 4,
        ("SETTLEMENTS","GENDATA"): 4,
        ("SETTLEMENTS","GENDATAREGION"): 4,
        ("SETTLEMENTS","INTRAREGIONRESIDUES"): 4,
        ("SETTLEMENTS","IRAUCSURPLUS"): 4,
        ("SETTLEMENTS","IRNSPSURPLUS"): 4,
        ("SETTLEMENTS","IRPARTSURPLUS"): 4,
        ("SETTLEMENTS","IRSURPLUS"): 4,
        ("SETTLEMENTS","LOCALAREAENERGY"): 4,
        ("SETTLEMENTS","LOCALAREATNI"): 4,
        ("SETTLEMENTS","LSHEDPAYMENT"): 4,
        ("SETTLEMENTS","LSHEDRECOVERY"): 4,
        ("SETTLEMENTS","MARKETFEES"): 4,
        ("SETTLEMENTS","REALLOCATIONS"): 4,
        ("SETTLEMENTS","RESTARTPAYMENT"): 4,
        ("SETTLEMENTS","RESTARTRECOVERY"): 4,
        ("SETTLEMENTS","RPOWERPAYMENT"): 4,
        ("SETTLEMENTS","RPOWERRECOVERY"): 4,
        ("SETTLEMENTS","SMALLGENDATA"): 4,
        ("SETTLEMENTS","ANCILLARY_SUMMARY"): 4,
        ("SETTLEMENTS","APC_COMPENSATION"): 4,
        ("SETTLEMENTS","APC_RECOVERY"): 4,
        ("SETTLEMENTS","FCAS_PAYMENT"): 4,
        ("SETTLEMENTS","FCAS_RECOVERY"): 4,
        ("SETTLEMENTS","SET_FCAS_REGULATION_TRK"): 4,
        ("SETTLEMENTS","NMAS_RECOVERY"): 4,
        ("SETTLEMENTS","NMAS_RECOVERY_RBF"): 4,
        ("SETTLEMENTS","RUN_PARAMETER"): 4,
        ("TRADING","INTERCONNECTORRES"): 4,
        ("TRADING","PRICE"): 4,
    } 

    if mapping.get((row[1], row[2])) is not None:
        sd = row[mapping[(row[1], row[2])]]
        parsed = datetime.datetime.strptime(sd, "%Y/%m/%d %H:%M:%S").date()
        return "{}-{}-v{}-{}-{}".format(row[1], row[2], row[3], parsed.year, parsed.month)
    else:
        return "{}-{}-v{}".format(row[1], row[2], row[3])
    