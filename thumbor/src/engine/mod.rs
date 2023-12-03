pub mod photon;

use image::ImageOutputFormat;
use crate::abi::abi::Spec;

pub trait Engine {
	fn apply(&mut self,specs: &[Spec]);

	fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}


pub trait SpecTransform<T> {
	// 对图片使用 op 做 transform
	fn transform(&mut self, op: T);
}