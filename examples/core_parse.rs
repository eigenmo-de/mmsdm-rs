use mmsdm_core::{FileReader, UnzipFile};
use rc_zip_sync::ReadZip;
use std::boxed::Box;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    {
        let file = File::open("./PUBLIC_ARCHIVE#BIDOFFERPERIOD#FILE01#202409010000.zip")?;

        let start = Instant::now();
        let archive = file.read_zip().unwrap();
        let handle = archive.entries().next().unwrap();
        let fr = FileReader::from_entry(handle).unwrap();
        dbg!(fr.header(), fr.sub_files());
        println!(
            "rczip done in {}ms",
            Instant::now()
                .checked_duration_since(start)
                .unwrap()
                .as_millis()
        );
    }

    {
        let file = File::open("./PUBLIC_ARCHIVE#BIDOFFERPERIOD#FILE01#202409010000.zip")?;

        let start = Instant::now();
        let unzipper = UnzipFile::new(file);
        let fr = FileReader::from_get_buf_reader(unzipper)?;
        dbg!(fr.header(), fr.sub_files());
        println!(
            "unzip done in {}ms",
            Instant::now()
                .checked_duration_since(start)
                .unwrap()
                .as_millis()
        );
    }

    Ok(())
}
