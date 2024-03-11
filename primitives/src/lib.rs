//! Custom host function
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
pub use model::AIModel;
#[cfg(feature = "std")]
use std::sync::Mutex;

#[cfg(feature = "std")]
pub mod model {
	use super::*;
	use once_cell::sync::OnceCell;
	use rust_bert::pipelines::conversation::{
		ConversationConfig, ConversationManager, ConversationModel,
	};

	pub static AI_MODEL: OnceCell<Mutex<AIModel>> = OnceCell::new();

	pub struct AIModel {
		pub conversation_model: ConversationModel,
		pub conversation_manager: ConversationManager,
	}

	impl AIModel {
		pub fn new() -> Self {
			let config = ConversationConfig {
				do_sample: true,
				max_length: Some(512),
				num_beams: 5,
				num_return_sequences: 1,
				..Default::default()
			};
			let conversation_model = ConversationModel::new(config).unwrap();
			let conversation_manager = ConversationManager::new();
			AIModel { conversation_model, conversation_manager }
		}

		pub fn ask_ai(&mut self, _seed: u32, question: String) -> String {
			// let input_sentence = format!("Using seed {}, pick a random number", seed).clone();
			// let candidate_labels = &["23", "16", "56", "83"];

			// let output = model.unwrap().predict_multilabel(
			// 	&[input_sentence.as_str()],
			// 	candidate_labels,
			// 	None,
			// 	128,
			// );
			// let mut top = output.unwrap()[0].clone();
			// top.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
			// log::info!("{} => {:?}", input_sentence, top);
			// log::info!("Top => {:?}", top[0]);

			let _conversation_1_id =
				self.conversation_manager.create(format!("{}", question.clone()).as_str());

			// let context = format!("This is a blockchain. Also use seed {}", seed).clone();
			// let qa_input = QaInput {
			// 	question: question.clone().try_into().unwrap(),
			// 	context: context.clone(),
			// };
			// let answers = model.predict(&[qa_input], 1, 2048);
			let output = self.conversation_model.generate_responses(&mut self.conversation_manager);

			log::info!("question: {:?}", question);
			log::info!("{:?}", output);

			output.unwrap().values().cloned().collect::<Vec<_>>().join(" ")
		}
	}
}

/// Interface that provides access to the ai function.
#[runtime_interface]
pub trait Custom {
	fn ask_ai(seed: u32, question: Vec<u8>) -> Vec<u8> {
		model::AI_MODEL
			.get_or_init(|| Mutex::new(model::AIModel::new()))
			.lock()
			.unwrap()
			.ask_ai(seed, String::from_utf8_lossy(&question).to_string())
			.as_bytes()
			.to_vec()
	}
}
