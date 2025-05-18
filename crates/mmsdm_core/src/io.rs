extern crate std;
use crate::{
    AemoHeader, CsvReader, Error, FileKey, GetTable, PartitionKey, PartitionValue, Result,
    RowValidation,
};
use alloc::{collections::BTreeSet, format, string::String, sync::Arc, vec::Vec};
use core::ops::AddAssign;
use rc_zip_sync::{EntryHandle, EntryReader, HasCursor};
use std::{
    collections::BTreeMap,
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
};


pub trait GetBufReader {
    type 
    fn reader() -> 
}

pub struct FileReader<'reader, F> {
    handle: EntryHandle<'reader, F>,
    header: AemoHeader,
    file_headings: BTreeMap<FileKey<'static>, usize>,
}

impl<'reader, F> FileReader<'reader, F>
where
    F: HasCursor,
{
    /// Create a new file reader
    /// this reads a whole, regular MMSDM file, including `C`, `I` and `D` rows
    /// this takes a `BufReader<R>` rather than `R` to discourage double wrapping
    /// in a BufReader. If it was to take any `R: Read` (and wrap interally),
    /// users of the API may not realise that it internally buffers and
    /// provide something already wrapped in `BufReader`
    pub fn from_entry(handle: EntryHandle<'reader, F>) -> Result<FileReader<'reader, F>> {
        // defensively reset to the start
        let mut header = None;
        let mut file_headings = BTreeMap::new();

        {
            let mut file_reader = BufReader::new(handle.reader());

            let mut count = 0;

            let mut row_holder = String::new();
            let mut indexes_backing = Vec::from([0; 1000]);
            let mut output_vec = Vec::from([0; 100_000]);

            let mut last_heading = None;

            let mut csv_reader = CsvReader::new();

            loop {
                row_holder.clear();

                let bytes_read = file_reader.read_line(&mut row_holder)?;

                if bytes_read == 0 {
                    break;
                }
                count += 1;

                match csv_reader.validate_row(&row_holder, &mut output_vec, &mut indexes_backing)? {
                    Some(RowValidation::Header(h)) => {
                        if header.is_none() {
                            header = Some(h);
                        } else {
                            return Err(
                                Error::UnexpectedRowType(format!("Extra header: {h:?}")).into()
                            );
                        }
                    }
                    Some(RowValidation::Footer(f)) => {
                        if f.line_count_inclusive != count {
                            return Err(Error::IncorrectLineCount {
                                got: count,
                                expected: f.line_count_inclusive,
                            }
                            .into());
                        }
                        break;
                    }
                    Some(RowValidation::Headings(k)) => {
                        last_heading = Some(k.to_owned());
                        file_headings.insert(k, 0);
                    }
                    None => {
                        if let Some(current_count) =
                            last_heading.as_ref().and_then(|h| file_headings.get_mut(h))
                        {
                            current_count.add_assign(1);
                        }
                    }
                }
            }
        }

        Ok(FileReader {
            handle,
            header: header.ok_or_else(|| Error::MissingHeaderRecord)?,
            file_headings,
        })
    }

    pub fn sub_files(&self) -> &BTreeMap<FileKey<'static>, usize> {
        &self.file_headings
    }

    pub fn header(&self) -> &AemoHeader {
        &self.header
    }

    pub fn iter_closest<'sub_reader, T>(
        &'sub_reader mut self,
        manager: Arc<T>,
    ) -> Result<IterTyped<<F as HasCursor>::Cursor<'sub_reader>, T>>
    where
        T: GetTable,
        // 'reader: 'sub_reader,
    {
        let closest_key = self
            .file_headings
            .keys()
            .filter(|k| T::matches_file_key(*k, k.version))
            .max_by_key(|k| k.version)
            .ok_or_else(|| Error::MissingFile {
                data_set_name: T::DATA_SET_NAME,
                table_name: T::TABLE_NAME,
                version: None,
            })?;

        let field_mapping = T::field_mapping_from_row(closest_key.backing_data().to_owned())?;

        Ok(IterTyped::new(
            manager,
            &closest_key,
            field_mapping,
            BufReader::new(self.handle.reader()),
        ))
    }

    pub fn iter_exact<'sub_reader, T>(
        &'sub_reader mut self,
        manager: Arc<T>,
    ) -> Result<IterTyped<<F as HasCursor>::Cursor<'sub_reader>, T>>
    where
        T: GetTable,
        'reader: 'sub_reader,
    {
        let Some(key) = self
            .file_headings
            .keys()
            .find(|k| T::matches_file_key(k, T::VERSION))
        else {
            return Err(Error::MissingFile {
                data_set_name: T::DATA_SET_NAME,
                table_name: T::TABLE_NAME,
                version: Some(T::VERSION),
            }
            .into());
        };

        let field_mapping = T::field_mapping_from_row(key.backing_data().to_owned())?;

        Ok(IterTyped::new(
            manager,
            &key,
            field_mapping,
            BufReader::new(self.handle.reader()),
        ))
    }
}

pub struct IterTyped<F, T>
where
    F: Read,
    T: GetTable,
{
    inner: BufReader<EntryReader<F>>,
    ty: PhantomData<T>,
    version: i32,
    indexes_backing: Vec<usize>,
    output: Vec<u8>,
    buf: String,
    field_mapping: T::FieldMapping,
    reader: CsvReader,
    manager: Arc<T>,
}

impl<F, T> IterTyped<F, T>
where
    F: Read,
    T: GetTable,
{
    pub fn new(
        manager: Arc<T>,
        key: &FileKey<'_>,
        mapping: T::FieldMapping,
        reader: BufReader<EntryReader<F>>,
    ) -> IterTyped<F, T> {
        IterTyped {
            inner: reader,
            ty: PhantomData,
            version: key.version,
            // must intialize enough spots to hold the most expected columns. this should be sufficient
            indexes_backing: Vec::from([0; 1000]),
            // must initialize enough bytes to hold the biggest expected row. this should be sufficient
            output: Vec::from([0; 100_000]),
            buf: String::new(),
            field_mapping: mapping,
            reader: CsvReader::new(),
            manager,
        }
    }
    pub fn next<'next, 'row>(&'next mut self) -> Option<Option<Result<<T as GetTable>::Row<'row>>>>
    where
        T: GetTable,
        'next: 'row,
    {
        self.buf.clear();

        match self.inner.read_line(&mut self.buf) {
            Ok(0) => None,
            Ok(_) => Some(
                self.reader
                    .read_row(&self.buf, &mut self.output, &mut self.indexes_backing)
                    .and_then(|csv| crate::handle_row::<T>(csv, self.version, &self.field_mapping))
                    .transpose(),
            ),
            Err(e) => Some(Some(Err(e.into()))),
        }
    }

    pub fn process_rows(mut self, mut on_row: impl FnMut(<T as GetTable>::Row<'_>)) -> Result<()> {
        // only want parsed rows
        while let Some(maybe_row) = self.next() {
            match maybe_row {
                Some(Ok(row)) => on_row(row),
                Some(Err(e)) => return Err(e),
                None => continue,
            }
        }
        Ok(())
    }

    #[deprecated(
        since = "0.0.0",
        note = "Avoid this where possible, high chance of causing OOM"
    )]
    pub fn collect(self) -> Result<Vec<<T as GetTable>::Row<'static>>> {
        let mut vec = Vec::new();

        self.process_rows(|row| {
            let cloned = T::to_static(&row);
            vec.push(cloned)
        })?;

        Ok(vec)
    }

    pub fn collect_partitions(self) -> Result<BTreeSet<(PartitionKey, PartitionValue)>> {
        let mut vec = BTreeSet::new();

        let manager = self.manager.clone();
        self.process_rows(|row| {
            vec.insert((manager.partition_key(), manager.partition_value(&row)));
        })?;

        Ok(vec)
    }

    pub fn count(self) -> Result<usize> {
        let mut count = 0;

        self.process_rows(|_| {
            count += 1;
        })?;

        Ok(count)
    }
}
