use mmsdm::{data_model, ArrowSchema, GetTable, MmsFile};
use std::{collections, fs::File};

use arrow2::{
    io::parquet::write::{
        CompressionOptions, Encoding, FileWriter, RowGroupIterator, Version, WriteOptions,
    },
};

fn main() -> anyhow::Result<()> {
    // let db = sled::Config::new().path("./parquet-fs.db").open()?;

    let mut handle =
        mmsdm::AemoDiskFile::from_path("./data/PUBLIC_DVD_DISPATCHLOAD_202110010000.zip")?;
    dbg!(MmsFile::Disk(&mut handle).file_keys());
    let partitions = handle
        .get_table::<data_model::DispatchUnitSolution3>()?
        .map(|r| r.unwrap().partition_suffix())
        .collect::<collections::HashSet<_>>();
    dbg!(partitions);
    // let dispatch = handle.get_table::<data_model::DispatchUnitSolution3>()?;
    // for (num, row) in dispatch.enumerate() {
    //     if num % 100000 == 0 {
    //         println!("Up to row: {num}")
    //     }
    //     let row = row?;
    //     let partition  = row.partition_suffix();
    //     let tree = db.open_tree(format!("{:04}-{:02}", partition.year, partition.month.number_from_month()))?;
    //     tree.insert(
    //         bincode::serialize(&row.primary_key())?,
    //         bincode::serialize(&row)?,
    //         // serde_json::to_vec(&row)?,
    //     )?;
    // }

    // for tree in db.tree_names() {
    //     println!("Tree: {}", String::from_utf8(tree.to_vec()).unwrap());
    //     let partition = db.open_tree(tree)?.iter().map(|r| bincode::deserialize::<data_model::DispatchUnitSolution3>(&r.unwrap().1).unwrap());
    //     // let partition = db.open_tree(tree)?.iter().map(|r| serde_json::from_slice::<data_model::DispatchUnitSolution3>(&r.unwrap().1).unwrap());
    //     let chunk = mmsdm_core::ArrowSchema::partition_to_chunk(partition)?;
    //     for col in chunk.columns() {
    //         dbg!(col.data_type());
    //     }
    //     dbg!(chunk.len());
    // }

    let partition = handle
        .get_table::<data_model::DispatchUnitSolution3>()?
        .map(|r| r.unwrap());
    let chunk = mmsdm_core::ArrowSchema::partition_to_chunk(partition)?;
    for col in chunk.columns() {
        dbg!(col.data_type());
    }
    dbg!(chunk.len());

    // as per example at: https://github.com/jorgecarleitao/arrow2/blob/main/examples/parquet_write.rs
    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Snappy,
        version: Version::V2,
    };

    let encodings = data_model::DispatchUnitSolution3::arrow_schema()
        .fields
        .iter()
        .map(|f| match f.data_type {
            _ => Encoding::Plain,
        })
        .collect();

    let row_groups = RowGroupIterator::try_new(
        std::iter::once(Ok(chunk)),
        &data_model::DispatchUnitSolution3::arrow_schema(),
        options,
        encodings,
    )?;

    let file = File::create("./PUBLIC_DVD_DISPATCHLOAD_202110010000.parquet")?;

    let mut writer = FileWriter::try_new(
        file,
        data_model::DispatchUnitSolution3::arrow_schema(),
        options,
    )?;

    writer.start()?;
    for group in row_groups {
        writer.write(group?)?;
    }
    let _size = writer.end(None)?;
    Ok(())
}
