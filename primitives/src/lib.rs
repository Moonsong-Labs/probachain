//! Custom host function
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use std::path::PathBuf;
#[cfg(feature = "std")]
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
#[cfg(feature = "std")]
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationConfig;
#[cfg(feature = "std")]
use rust_bert::pipelines::common::{ModelResource, ModelType};
#[cfg(feature = "std")]
use rust_bert::resources::{LocalResource};
#[cfg(feature = "std")]
use tch::Device;

// #[cfg(feature = "std")]
// sp_externalities::decl_extension! {
// 	pub struct CustomExt;
// }

// #[cfg(feature = "std")]
// impl CustomExt {
// 	pub fn ask_ai(&mut self) -> String {
// 		"Host function test".to_string()
// 	}
// }

#[cfg(feature = "std")]
struct CustomFunction {
	model: Option<ZeroShotClassificationModel>
}

#[cfg(feature = "std")]
impl CustomFunction {
	pub fn ask_ai(seed: u32) -> String {
		let model = Some(ZeroShotClassificationModel::new(Default::default()).unwrap());
		// let config_resource = LocalResource {
		// 	local_path: PathBuf::from("/root/projects/probachain/rust_model.ot"),
		// };

		// let sequence_classification_model = ZeroShotClassificationModel::new(ZeroShotClassificationConfig {
        //     model_type: ModelType::Bart,
        //     model_resource: ModelResource::Torch(Box::new(config_resource.clone())),
        //     config_resource: Box::new(config_resource.clone()),
        //     vocab_resource: Box::new(config_resource.clone()),
        //     merges_resource: Some(Box::new(config_resource.clone())),
        //     lower_case: false,
        //     strip_accents: None,
        //     add_prefix_space: None,
        //     device: Device::cuda_if_available(),
        //     kind: None,
        // }).unwrap();

		let input_sentence = format!("Using seed {}, pick a random number", seed).clone();
		let candidate_labels = &["23", "16", "56", "83"];
	
		let output = model.unwrap().predict_multilabel(
			&[input_sentence.as_str()],
			candidate_labels,
			None,
			128,
		);
		let mut top = output.unwrap()[0].clone();
		top.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
		log::info!("{} => {:?}", input_sentence, top);
		log::info!("Top => {:?}", top[0]);
		
		top[0].text.clone()
	}
}

/// Interface that provides access to the ai function.
#[runtime_interface]
pub trait Custom {
	fn ask_ai(&mut self, seed: u32) -> Vec<u8> {
		CustomFunction::ask_ai(seed).as_bytes().to_vec()
	}
}
