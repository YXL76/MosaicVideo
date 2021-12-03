#![feature(int_log)]
#![feature(map_first_last)]

extern crate ffmpeg_next as ffmpeg;

mod process;
mod utils;

use {
    ffmpeg::{
        format::{input, Pixel},
        media,
        util::frame::video::Video,
    },
    std::path::Path,
};

pub use {ffmpeg::Error, process::*, utils::*};

pub fn init() {
    ffmpeg::init().unwrap();
    ffmpeg::log::set_level(ffmpeg::log::Level::Quiet);
}

pub fn first_frame(filename: &Path) -> Result<(u32, u32, Vec<u8>), ffmpeg::Error> {
    let mut ictx = input(&filename)?;
    let input = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;

    let stream_idx = input.index();
    let mut decoder = input.codec().decoder().video()?;
    let mut converter = decoder.converter(Pixel::BGRA)?;

    for (stream, packet) in ictx.packets() {
        if stream.index() == stream_idx {
            decoder.send_packet(&packet)?;
            let mut decoded = Video::empty();
            if decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                converter.run(&decoded, &mut rgb_frame)?;
                return Ok((
                    decoder.width(),
                    decoder.height(),
                    rgb_frame.data(0).to_owned(),
                ));
            }
        }
    }

    Err(ffmpeg::Error::InvalidData)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CalculationUnit {
    Average,
    Pixel,
    KMeans,
}

impl Default for CalculationUnit {
    fn default() -> Self {
        Self::Average
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorSpace {
    RGB,
    HSV,
    CIELAB,
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::RGB
    }
}

impl From<ColorSpace> for String {
    fn from(cs: ColorSpace) -> Self {
        Self::from(match cs {
            ColorSpace::RGB => "RGB",
            ColorSpace::HSV => "HSV",
            ColorSpace::CIELAB => "CIE L*a*b*",
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DistanceAlgorithm {
    Euclidean,
    CIEDE2000,
}

impl Default for DistanceAlgorithm {
    fn default() -> Self {
        Self::Euclidean
    }
}

impl From<DistanceAlgorithm> for String {
    fn from(da: DistanceAlgorithm) -> Self {
        Self::from(match da {
            DistanceAlgorithm::Euclidean => "Euclidean",
            DistanceAlgorithm::CIEDE2000 => "CIEDE2000",
        })
    }
}
