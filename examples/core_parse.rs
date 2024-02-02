use mmsdm_core::FileReader;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./PUBLIC_DVD_PREDISPATCH_FCAS_REQ_D_202312010000.zip")?;

    let mut archive = ZipArchive::new(file)?;

    let fr = FileReader::new(&mut archive).unwrap();

    dbg!(fr.header(), fr.sub_files());

    Ok(())
}
