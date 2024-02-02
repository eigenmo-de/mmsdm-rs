use chrono::Month;
use mmsdm_bids::{BidBidperofferD2, BidsBidofferperiod1};
use mmsdm_core::{ArrowSchema, FileReader, IterTyped, YearMonth};
use mmsdm_dispatch::{
    DispatchConstraint5, DispatchConstraint5Builder, DispatchLocalPrice1,
    DispatchLocalPrice1Builder, DispatchMnspbidtrk1, DispatchMnspbidtrk1Builder, DispatchOffertrk1,
    DispatchOffertrk1Builder, DispatchUnitSolution4, DispatchUnitSolution4Builder,
};
use mmsdm_p5min::P5minUnitsolution5;
use mmsdm_pre_dispatch::PredispatchRegionfcasrequirement2;
use mmsdm_settlement_data::{SettlementsFcasRecovery7, SettlementsFcasregionrecovery5};
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use zip::ZipArchive;

// PUBLIC_NEXT_DAY_DISPATCH_20230105_0000000378281515.zip 5.4mb zipped, 81mb unzipped, 1s raw, 3s parsed
// PUBLIC_DVD_PREDISPATCH_FCAS_REQ_D_202312010000.zip 200mb zipped, 7.0g unzipped, 10s raw, 66s parsed;
// PUBLIC_DVD_P5MIN_UNITSOLUTION_202312010000.zip 400mb zipped, 8.5g unzipped, 15s raw, 50s parsed;
// PUBLIC_DVD_SETFCASREGIONRECOVERY_202312010000.zip 800mb zipped, 5.5g unzipped, 13s raw, 52s parsed;
// PUBLIC_DVD_BIDPEROFFER1_202312010000.zip 2gb s raw, s parsed; --- BOH!

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./PUBLIC_NEXT_DAY_DISPATCH_20230105_0000000378281515.zip")?;

    let mut archive = ZipArchive::new(file)?;

    // let mut br = BufReader::new(archive.by_index(0).unwrap());

    // let mut rh = String::with_capacity(500);

    // let mut rows = 0;
    // loop {
    //     let bytes_read = br.read_line(&mut rh)?;

    //     if bytes_read == 0 {
    //         break
    //     }
    //     rows += 1
    // }

    // dbg!(rows);

    let mut fr = FileReader::new(&mut archive).unwrap();

    dbg!(fr.header(), fr.sub_files());

    let batch = mmsdm_core::partition_to_batch::<_, DispatchOffertrk1>(
        &mut fr,
        YearMonth {
            year: 2023,
            month: Month::January,
        },
    )?;
    dbg!(batch.num_rows(), batch.num_columns());
    let batch = mmsdm_core::partition_to_batch::<_, DispatchUnitSolution4>(
        &mut fr,
        YearMonth {
            year: 2023,
            month: Month::January,
        },
    )?;
    dbg!(batch.num_rows(), batch.num_columns());
    let batch = mmsdm_core::partition_to_batch::<_, DispatchConstraint5>(
        &mut fr,
        YearMonth {
            year: 2023,
            month: Month::January,
        },
    )?;
    dbg!(batch.num_rows(), batch.num_columns());
    let batch = mmsdm_core::partition_to_batch::<_, DispatchMnspbidtrk1>(
        &mut fr,
        YearMonth {
            year: 2023,
            month: Month::January,
        },
    )?;
    dbg!(batch.num_rows(), batch.num_columns());
    let batch = mmsdm_core::partition_to_batch::<_, DispatchLocalPrice1>(
        &mut fr,
        YearMonth {
            year: 2023,
            month: Month::January,
        },
    )?;
    dbg!(batch.num_rows(), batch.num_columns());

    // {

    //     let mut iter_closest =  fr.iter_closest::<BidsBidofferperiod1>().unwrap();

    //     loop {
    //         let Some(maybe_res) = iter_closest.next() else {
    //             break;
    //         };

    //         match maybe_res {
    //             Some(Ok(t)) => {
    //                 // dbg!(t);

    //             }
    //             Some(Err(e)) => return Err(e.into()),
    //             None => continue,
    //         }
    //     }
    // }

    _ = dbg!(
        (),
        // fr.iter_closest::<DispatchOffertrk1>().unwrap().count(),
        // fr.iter_closest::<DispatchUnitSolution4>().unwrap().count(),
        // fr.iter_closest::<DispatchConstraint5>().unwrap().count(),
        // fr.iter_closest::<DispatchMnspbidtrk1>().unwrap().count(),
        // fr.iter_closest::<DispatchLocalPrice1>().unwrap().count(),
        // fr.iter_closest::<PredispatchRegionfcasrequirement2>().unwrap().count(),
        // fr.iter_closest::<P5minUnitsolution5>().unwrap().count(),
        // fr.iter_closest::<SettlementsFcasregionrecovery5>().unwrap().count(),
        // fr.iter_closest::<BidsBidofferperiod1>().unwrap().count(),
    );

    Ok(())
}
