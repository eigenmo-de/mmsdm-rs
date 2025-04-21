use mmsdm::{PartitionKey, PartitionValue};
use mmsdm_bids::OfferBiddayoffer3;
use mmsdm_core::FileReader;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::sync::Arc;
use zip::ZipArchive;

// PUBLIC_NEXT_DAY_DISPATCH_20230105_0000000378281515.zip 5.4mb zipped, 81mb unzipped, 1s raw, 3s parsed
// PUBLIC_DVD_PREDISPATCH_FCAS_REQ_D_202312010000.zip 200mb zipped, 7.0g unzipped, 10s raw, 66s parsed;
// PUBLIC_DVD_P5MIN_UNITSOLUTION_202312010000.zip 400mb zipped, 8.5g unzipped, 15s raw, 50s parsed;
// PUBLIC_DVD_SETFCASREGIONRECOVERY_202312010000.zip 800mb zipped, 5.5g unzipped, 13s raw, 52s parsed;
// PUBLIC_DVD_BIDPEROFFER1_202312010000.zip 2gb s raw, s parsed; --- BOH!

// PUBLIC_DVD_BIDDAYOFFER_202312010000.zip
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./PUBLIC_DVD_BIDPEROFFER1_202407010000.zip")?;

    let mut archive = ZipArchive::new(file)?;

    let mut fr = FileReader::new(&mut archive).unwrap();

    dbg!(fr.header(), fr.sub_files());

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

    // let mut partitions = BTreeSet::<<PredispatchRegionfcasrequirement2 as GetTable>::Partition>::new();
    // fr.iter_closest::<PredispatchRegionfcasrequirement2>()?.process_rows(|r| { partitions.insert(<PredispatchRegionfcasrequirement2 as GetTable>::partition_suffix(&r)); })?;

    // _ = dbg!(fr
    //     .iter_closest::<PredispatchRegionfcasrequirement2>()?
    //     .collect_partitions());

    // dbg!(fr
    //     .iter_closest::<PredispatchRegionfcasrequirement2>()?
    //     .collect().unwrap().len());

    _ = dbg!(
        (),
        // fr.iter_closest::<DispatchOffertrk1>().unwrap().count(),
        // fr.iter_closest::<DispatchUnitSolution4>().unwrap().count(),
        // fr.iter_closest::<DispatchConstraint5>().unwrap().count(),
        // fr.iter_closest::<DispatchMnspbidtrk1>().unwrap().count(),
        // fr.iter_closest::<DispatchLocalPrice1>().unwrap().count(),
        // fr.iter_closest::<PredispatchRegionfcasrequirement2>()
        //     .unwrap()
        //     .count(),
        // fr.iter_closest::<P5minUnitsolution5>().unwrap().count(),
        // fr.iter_closest::<SettlementsFcasregionrecovery5>().unwrap().count(),
        // fr.iter_closest::<BidsBidofferperiod1>().unwrap().count(),
        fr.iter_closest::<OfferBiddayoffer3>(Arc::new(OfferBiddayoffer3::new(
            PartitionKey("settlementdate"),
            |row| PartitionValue(row.settlementdate.date().to_string())
        )))
        .unwrap()
        .count(),
    );

    Ok(())
}
