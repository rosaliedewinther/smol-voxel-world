use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
};

use lz4_flex::{compress_prepend_size, decompress_size_prepended};

pub fn write_and_compress_to_file<T: Serialize>(data: &T, filename: &str) -> Result<()> {
    let data = serde_cbor::to_vec(data).with_context(|| format!("could not encode to cbor"))?;
    let data = compress_prepend_size(&data);
    let mut file = File::create(filename).with_context(|| format!("could not create file: {}", filename))?;
    file.write_all(&data).with_context(|| format!("could not write to file: {}", filename))?;
    Ok(())
}

pub fn write_to_file(bytes: &[u8], filename: &str) -> Result<()> {
    let mut file = File::create(filename).with_context(|| format!("could not create file: {}", filename))?;
    file.write_all(&bytes).with_context(|| format!("could not write to file: {}", filename))?;
    Ok(())
}

pub fn read_and_decompress_file<T: DeserializeOwned>(filename: &str) -> Result<T> {
    let data = std::fs::read(filename).with_context(|| format!("could not read from file: {}", filename))?;
    let data = decompress_size_prepended(&data).with_context(|| format!("could not decompress data from file: {}", filename))?;
    Ok(serde_cbor::from_slice(&data[..]).with_context(|| format!("could not decode to cbor: {}", filename))?)
}

pub fn read_file(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename).with_context(|| format!("could not open file: {}", filename))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).with_context(|| format!("could not read file: {}", filename))?;
    Ok(buf)
}

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>()) }
}
pub fn u8_as_obj<T: Sized>(bytes: &[u8]) -> &T {
    unsafe { &*(bytes.as_ptr() as *const T) }
}
