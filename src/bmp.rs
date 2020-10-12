extern crate byteorder;

use byteorder::*;
use std::convert::*;
use std::io::{Cursor, Error, Read, Seek, SeekFrom};
use std::ops::Index;

pub struct Bitmap {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum BMPError {
    WrongSignature,
    UnsupportedBitsPerPixel,
    IOError,
}

struct BitmapHeader {
    pub file_size: u32,
    pub reserved: u32,
    pub bitmap_offset: u32,
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bits_per_pixel: u16,
}

impl From<Error> for BMPError {
    fn from(_error: Error) -> Self {
        return BMPError::IOError;
    }
}

impl Index<usize> for Bitmap {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

const BMP_SIGNATURE: u16 = 0x424d;
const NUM_COLORS: u8 = 5;

pub fn bmp_load(data: &[u8]) -> Result<Bitmap, BMPError> {
    let mut pointer = Cursor::new(data);

    if pointer.read_u16::<BigEndian>()? != BMP_SIGNATURE {
        return Err(BMPError::WrongSignature);
    }

    let header = BitmapHeader {
        file_size: pointer.read_u32::<LittleEndian>()?,
        reserved: pointer.read_u32::<LittleEndian>()?,
        bitmap_offset: pointer.read_u32::<LittleEndian>()?,
        size: pointer.read_u32::<LittleEndian>()?,
        width: pointer.read_i32::<LittleEndian>()?,
        height: pointer.read_i32::<LittleEndian>()?,
        planes: pointer.read_u16::<LittleEndian>()?,
        bits_per_pixel: pointer.read_u16::<LittleEndian>()?,
    };

    if header.bits_per_pixel != 32 {
        return Err(BMPError::UnsupportedBitsPerPixel);
    }

    let mut px = [0; 4];
    let mut data = vec![0; (header.width * header.height) as usize];

    pointer.seek(SeekFrom::Start(header.bitmap_offset as u64))?;
    for y in 0..header.height {
        for x in 0..header.width {
            pointer.read(&mut px)?;
            data[(x + (header.height - y - 1) * header.width) as usize] = px[0] / (255 / NUM_COLORS);
        }
    }

    return Ok(Bitmap { width: header.width, height: header.height, data: data });
}
