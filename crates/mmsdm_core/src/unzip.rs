#![allow(warnings)]
extern crate std;
use core::{iter, num::NonZeroUsize};
use std::{
    io::{self, Read},
    u32,
};

use alloc::{format, vec::Vec};
use flate2::read::DeflateDecoder;

use crate::Error;

#[derive(Debug)]
struct LocalFileHeader {
    _version: u16,
    _general: u16,
    compression: u16,
    _msdos_time: [u8; 2], // ignored
    _msdos_date: [u8; 2], // ignored
    _crc_32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_len: u16,
    extra_field_len: u16,
}

impl LocalFileHeader {
    fn new(sig: [u8; 30]) -> Option<LocalFileHeader> {
        if sig[0..4] != [0x50, 0x4b, 0x03, 0x04] {
            return None;
        }

        Some(LocalFileHeader {
            _version: u16::from_le_bytes([sig[4], sig[5]]),
            _general: u16::from_le_bytes([sig[6], sig[7]]),
            compression: u16::from_le_bytes([sig[8], sig[9]]),
            _msdos_time: [sig[10], sig[11]],
            _msdos_date: [sig[12], sig[13]],
            _crc_32: u32::from_le_bytes([sig[14], sig[15], sig[16], sig[17]]),
            compressed_size: u32::from_le_bytes([sig[18], sig[19], sig[20], sig[21]]),
            uncompressed_size: u32::from_le_bytes([sig[22], sig[23], sig[24], sig[25]]),
            file_name_len: u16::from_le_bytes([sig[26], sig[27]]),
            extra_field_len: u16::from_le_bytes([sig[28], sig[29]]),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AemoZipStreamerConfig {
    // DDOS protection
    unrestricted_length_reader: Option<NonZeroUsize>,
}

impl Default for AemoZipStreamerConfig {
    fn default() -> Self {
        AemoZipStreamerConfig {
            unrestricted_length_reader: None,
        }
    }
}

struct Headers<R> {
    lfh: LocalFileHeader,
    file_name: Vec<u8>,
    extra_fields: Vec<u8>,
    file: R,
}
impl AemoZipStreamerConfig {
    fn read_headers<R>(mut file: R) -> crate::Result<Headers<R>>
    where
        R: Read,
    {
        let mut init_buf = [0; 30];

        file.read_exact(&mut init_buf)?;

        let Some(lfh) = LocalFileHeader::new(init_buf) else {
            return Err(Error::UnzipLocalFileHeaderMissing);
        };

        if lfh.compression != 8 {
            return Err(Error::UnzipInvalidCompressionMethod(lfh.compression));
        }

        let mut file_name_buf = iter::repeat(0)
            .take(lfh.file_name_len.into())
            .collect::<Vec<_>>();

        let mut extra_field_buf = iter::repeat(0)
            .take(lfh.extra_field_len.into())
            .collect::<Vec<_>>();

        file.read_exact(&mut file_name_buf)?;
        file.read_exact(&mut extra_field_buf)?;

        Ok(Headers {
            lfh,
            file_name: file_name_buf,
            extra_fields: extra_field_buf,
            file,
        })
    }
    pub fn decompress_first_file<R>(
        &self,
        mut file: R,
    ) -> crate::Result<DeflateDecoder<CompressedReader<R>>>
    where
        R: Read,
    {
        let headers = Self::read_headers(file)?;

        Ok(DeflateDecoder::new(CompressedReader {
            file: headers.file,
            file_name: headers.file_name,
            extra_fields: headers.extra_fields,
            bytes_read: 0,
            unrestricted_length_reader: self.unrestricted_length_reader,
        }))
    }
    pub fn decompress_first_file_raw<R>(mut file: R) -> crate::Result<DeflateDecoder<R>>
    where
        R: Read,
    {
        let headers = Self::read_headers(file)?;

        Ok(DeflateDecoder::new(headers.file))
    }
}

// when done reader make sure we hit cendirectory.

pub struct CompressedReader<R> {
    file: R,
    file_name: Vec<u8>,
    extra_fields: Vec<u8>,
    bytes_read: usize,
    unrestricted_length_reader: Option<NonZeroUsize>,
}

impl<R> CompressedReader<R> {
    pub fn raw_file_name(&self) -> &[u8] {
        &self.file_name
    }
    pub fn raw_extra_fields(&self) -> &[u8] {
        &self.extra_fields
    }
    pub fn into_inner(self) -> R {
        self.file
    }
}
impl<R> Read for CompressedReader<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.file.read(buf)?;

        match self.unrestricted_length_reader {
            Some(max) if self.bytes_read + bytes_read > max.get() => {
                return Err(io::Error::new(
                    io::ErrorKind::FileTooLarge,
                    format!("Unzipped bytes exceeded the maximum of {max}"),
                ));
            }
            Some(_) => {
                self.bytes_read += bytes_read;
            }
            None => {
                // nothing
            }
        }

        Ok(bytes_read)
    }
}
