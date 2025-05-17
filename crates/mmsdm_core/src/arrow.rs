#![allow(unused_imports)]
extern crate std;

use std::io::{Read, Seek};

use alloc::sync::Arc;
pub use arrow::{array::RecordBatch, datatypes::Schema};
use rc_zip_sync::HasCursor;

use crate::{FileReader, GetTable, PartitionValue};

pub trait ArrowSchema: GetTable {
    fn schema() -> arrow::datatypes::Schema;
    type Builder;
    fn new_builder() -> Self::Builder;
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>);
    fn finalize_builder(builder: &mut Self::Builder) -> crate::Result<arrow::array::RecordBatch>;
}

pub fn accumulate_batch<'a, R: HasCursor, T: ArrowSchema, F>(
    reader: &mut FileReader<'a, R>,
    mut filter: F,
    manager: Arc<T>,
) -> crate::Result<arrow::array::RecordBatch>
where
    F: FnMut(&T::Row<'_>) -> bool,
{
    let mut builder = T::new_builder();

    let mut iter = reader.iter_closest::<T>(manager)?;
    while let Some(maybe_row) = iter.next() {
        let Some(row) = maybe_row.transpose()? else {
            continue;
        };

        if filter(&row) {
            T::append_builder(&mut builder, row);
        }
    }

    T::finalize_builder(&mut builder)
}

pub fn partition_to_batch<'a, R: HasCursor, T: ArrowSchema>(
    reader: &mut FileReader<'a, R>,
    partition: PartitionValue,
    manager: Arc<T>,
) -> crate::Result<arrow::array::RecordBatch>
where
{
    accumulate_batch::<_, T, _>(
        reader,
        |row| manager.partition_value(&row) == partition,
        manager.clone(),
    )
}
