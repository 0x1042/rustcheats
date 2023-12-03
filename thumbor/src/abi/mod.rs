pub mod abi;

use std::convert::TryFrom;

use abi::{
    filter::Filter, resize, resize::SampleFilter, spec::Data, Filter as SFilter, ImageSpec, Resize, Spec,
    Watermark,
};
use base64::{engine::general_purpose, Engine as _};
use photon_rs::transform::SamplingFilter;
use prost::Message;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// to string
impl From<&ImageSpec> for String {
    fn from(value: &ImageSpec) -> Self {
        let data = value.encode_to_vec();
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }
}

// 让 ImageSpec 可以通过一个字符串创建。比如 s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = general_purpose::URL_SAFE_NO_PAD.decode(value)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

impl Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            Filter::Unspecified => None,
            Filter::Oceanic => Some("oceanic"),
            Filter::Islands => Some("islands"),
            Filter::Marine => Some("marine"),
        }
    }
}

impl From<SampleFilter> for SamplingFilter {
    fn from(v: SampleFilter) -> Self {
        match v {
            SampleFilter::Undefined => SamplingFilter::Nearest,
            SampleFilter::Nearest => SamplingFilter::Nearest,
            SampleFilter::Triangle => SamplingFilter::Triangle,
            SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            SampleFilter::Gaussian => SamplingFilter::Gaussian,
            SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: Filter) -> Self {
        Self {
            data: Some(Data::Filter(SFilter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(Data::Watermark(Watermark { x, y })),
        }
    }
}
