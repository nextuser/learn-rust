use crate::pb::Spec;
mod photon;
use image::ImageOutputFormat;
pub use photon::Photon;
pub trait Engine{
    //engine 按照specs进行一系列有序的处理
    fn apply(&mut self,specs : &[Spec]);
    // engine 中生成目标图片，注意这里用的是self，而不是self的引用
    fn generate(self,format:ImageOutputFormat) -> Vec<u8>;
}

//未来添加更多的spec，只需要实现它即可
pub trait SpecTransform<T>{
    //对图片使用op做transform
    fn transform(&mut self,op:T);
}
