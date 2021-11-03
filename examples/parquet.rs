use mmsdm::data_model;
use std::{collections, fs};

fn main() -> anyhow::Result<()> {
    let mut partitions = collections::HashMap::new();
    for entry in fs::read_dir("./data")? {
        let dir = entry?;
        dbg!(dir.path());
        if dir
            .path()
            .file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.starts_with("PUBLIC_NEXT_DAY_DISPATCH"))
            .unwrap_or(false)
        {
            let file = fs::File::open(dir.path())?;
            let aemo = mmsdm::AemoFile::from_reader(file)?;
            dbg!(aemo.file_keys());
            let dispatch = aemo.get_table::<data_model::DispatchUnitSolution2>()?;
            partitions = mmsdm::merge_with_partitions(partitions, &dispatch);
        }
    }
    for (partition_key, partition) in partitions {
        dbg!(partition_key);
        let record_batch = mmsdm::ArrowSchema::partition_to_record_batch(partition)?;
        dbg!(record_batch.schema());
        dbg!(record_batch.num_rows());
    }
    Ok(())
}
