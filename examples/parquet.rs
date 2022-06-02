use mmsdm::data_model;
use std::{collections, fs};

fn main() -> anyhow::Result<()> {
    let mut partitions = collections::HashMap::new();
    for entry in fs::read_dir("./data")? {
        let dir = entry?;
        if dir
            .path()
            .file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.starts_with("PUBLIC_DVD_DISPATCHLOAD"))
            // .map(|f| f.starts_with("PUBLIC_DISPATCHSCADA"))
            .unwrap_or(false)
        {
            let file = fs::File::open(dir.path())?;
            println!("Opened {:?}", dir.path());
            let mut zip = zip::ZipArchive::new(file)?;
            println!("Got zip handle");
            let inner_file = zip.by_index(0)?;
            println!("Got first inner file");
            let aemo = mmsdm::AemoFile::from_reader(inner_file)?;
            dbg!(aemo.file_keys());
            let dispatch: Vec<_> = aemo.get_table::<data_model::DispatchUnitSolution3>()?;
            dbg!(dispatch.len());
            partitions = mmsdm::merge_with_partitions(partitions, &dispatch);
            dbg!(partitions.len());
        }
    }
    for (partition_key, partition) in partitions {
        dbg!(partition_key);
        let record_batch = mmsdm_core::ArrowSchema::partition_to_chunk(partition)?;
        for col in record_batch.columns() {
            dbg!(col.data_type());
        }
        dbg!(record_batch.len());
    }
    Ok(())
}
