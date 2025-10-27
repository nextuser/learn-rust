use super::{Engine,SpecTransform};
use crate::pb;
use anyhow::Result;
use photon_rs::{
    native::open_image_from_bytes,
    effects,filters,multiple,transform,PhotonImage,
};
use lazy_static::lazy_static;
use std::convert::TryFrom;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use crate::pb::{filter, resize, Flipv, Watermark};

lazy_static!{
    static ref WATERMARK:PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark,64,64,transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon{
    type Error = anyhow::Error;
    fn try_from(data:Bytes)->Result<Self,Self::Error>{
        Ok(Self(open_image_from_bytes(&data)?))
    }
}

fn image_to_buf(img : PhotonImage, format:ImageOutputFormat) -> Vec<u8>{
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();
    let img_buffer = ImageBuffer::from_vec(width,height,raw_pixels).unwrap();
    let dyn_image = DynamicImage::ImageRgba8(img_buffer);
    let mut buffer = Vec::with_capacity(32768);
    dyn_image.write_to(&mut buffer,format).unwrap();
    buffer
}

impl Engine for Photon{
    fn apply(&mut self,specs: &[pb::Spec]){
        for spec in specs.iter() {
            match spec.data {
                Some(pb::spec::Data::Crop(ref v)) => self.transform(v),
                Some(pb::spec::Data::Contrast(ref v)) => self.transform(v),
                Some(pb::spec::Data::Filter(ref v )) => self.transform(v),
                Some(pb::spec::Data::Fliph(ref v)) => self.transform(v),
                Some(pb::spec::Data::Flipv(ref v )) => self.transform(v),
                Some(pb::spec::Data::Resize(ref v)) => self.transform(v),
                Some(pb::spec::Data::Watermark(ref v)) => self.transform(v),
                _ => {}
            }
        }
    }

    fn generate(self,format:ImageOutputFormat) -> Vec<u8>{
        image_to_buf(self.0,format)
    }
}

impl SpecTransform<&pb::Crop> for Photon{
    fn transform(&mut self,op : & pb::Crop) {
        let img = transform::crop(&mut self.0,op.x1,op.y1,op.x2,op.y2);
        self.0 = img;
    }
}

impl SpecTransform<&pb::Flipv> for Photon{
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0)
    }
}

impl SpecTransform<&pb::Fliph> for Photon{
    fn transform(&mut self, _op: &pb::Fliph) {
        transform::fliph(&mut self.0)
    }
}


impl SpecTransform<&pb::Filter> for Photon {
    fn transform(&mut self, op: &pb::Filter) {
        match filter::FilterType::from_i32(op.filter) {
            Some(filter::FilterType::Unspecified) => {}
            Some(f) => filters::filter(&mut self.0,f.to_str().unwrap()),
            _ => {}
        }
    }
}

impl SpecTransform<&pb::Contrast> for Photon {
    fn transform(&mut self, op: &pb::Contrast) {
        effects::adjust_contrast(&mut self.0,op.contrast);
    }
}

impl SpecTransform<&pb::Resize> for Photon {
    fn transform(&mut self, op: &pb::Resize) {
        let img = match resize::ResizeType::from_i32(op.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(
                &self.0,
                op.width,
                op.height,
                resize::SampleFilter::from_i32(op.filter).unwrap().into()
            ),
            resize::ResizeType::SeamCarve => transform::seam_carve(&self.0, op.width,op.height),
        };
        self.0 = img;
    }
}

impl SpecTransform<&Watermark> for Photon{
    fn transform(&mut self,op : &Watermark){
        multiple::watermark(&mut self.0 , &WATERMARK,op.x as i64, op.y as i64);
    }
}
