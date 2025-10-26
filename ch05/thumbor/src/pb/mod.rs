use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
mod abi;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        //protobuf object => bytes
        let data = image_spec.encode_to_vec();
        //bytes => base64 string
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

use anyhow::Result;

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        //base64 => byte array
        let data = decode_config(value, base64::URL_SAFE_NO_PAD)?;
        //byte array => protobuf object
        Ok(ImageSpec::decode(&data[..])?)
    }
}

impl abi::filter::FilterType {
    pub fn to_str(self) -> Option<&'static str> {
        match self {
            Self::Unspecified => None,
            Self::Oceanic => Some("linear"),
            Self::Islands => Some("islands"),
            Self::Marine => Some("marine"),
        }
    }
}

impl From<resize::SampleFilter> for SamplingFilter {
    //从通信的protobuf类型转换成 proton_rs 图像操作 能接受的filter 类型
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(abi::Resize {
                width,
                height,
                filter: resize::SampleFilter::Undefined as i32,
                rtype: resize::ResizeType::SeamCarve as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Spec {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                filter: filter as i32,
                rtype: resize::ResizeType::Normal as i32,
            })),
        }
    }
    pub fn new_filter(f: filter::FilterType) -> Self {
        Self {
            data: Some(spec::Data::Filter(abi::Filter { filter: f as i32 })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(abi::Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    #[test]
    fn encode_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(100, 100, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::FilterType::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        println!("spec s: {}", &s);
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
