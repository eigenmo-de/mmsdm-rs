use mmsdm_core::FileReader;
use rc_zip_sync::ReadZip;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./PUBLIC_ARCHIVE#BIDOFFERPERIOD#FILE01#202409010000.zip")?;
    let archive = file.read_zip().unwrap();
    let handle = archive.entries().next().unwrap();

    let fr = FileReader::from_entry(handle).unwrap();

    dbg!(fr.header(), fr.sub_files());

    Ok(())
}
