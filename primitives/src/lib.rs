//! Custom host function
#![cfg_attr(not(feature = "std"), no_std)]

use serde::{Deserialize, Serialize};
use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use jsonrpsee::{
	core::{client::ClientT, params::ArrayParams, traits::ToRpcParams, JsonRawValue, Error},
	http_client::{HttpClient, HttpClientBuilder},
	rpc_params,
};
#[cfg(feature = "std")]
pub use serde_json::value::to_raw_value;
#[cfg(feature = "std")]
use std::time::{Duration};

#[cfg(feature = "std")]
pub use model::AiRequester;
#[cfg(feature = "std")]
use std::sync::Mutex;

#[cfg(feature = "std")]
pub mod model {
	use super::*;
	use once_cell::sync::OnceCell;

	pub static AI_MODEL: OnceCell<Mutex<AiRequester>> = OnceCell::new();

	#[derive(Debug, Deserialize ,Serialize)]
	pub struct Query {
		pub message: String,
	}
	#[derive(Debug, Deserialize ,Serialize)]
	pub struct QueryResponse {
		pub message: String,
		pub response: String,
	}

	impl ToRpcParams for Query {
		fn to_rpc_params(self) -> Result<Option<Box<JsonRawValue>>, Error> {
			log::info!("Query: {:?}", self.message.clone());
			Ok(to_raw_value(&self).unwrap()).map(Some)
		}
	}

	pub struct AiRequester {
		pub client: HttpClient,
	}

	impl AiRequester {
		pub fn new() -> Self {
			AiRequester {
				client: HttpClientBuilder::default().request_timeout(Duration::from_secs(360))
					.build("http://gpu4.sequitur.kaki.dev:9000/api/v1/generate")
					.unwrap(),
			}
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

			log::info!("question: {:?}", question.clone());
			let response: Result<QueryResponse, _> = futures::executor::block_on(
				self.client.request("generate", Query { message: question }),
			);
			let response = response.unwrap().response;

			log::info!("{:?}", response);

			response
		}
	}
}

/// Interface that provides access to the ai function.
#[runtime_interface]
pub trait Custom {
	fn ask_ai(seed: u32, question: Vec<u8>) -> Vec<u8> {
		model::AI_MODEL
			.get_or_init(|| Mutex::new(model::AiRequester::new()))
			.lock()
			.unwrap()
			.ask_ai(seed, String::from_utf8_lossy(&question).to_string())
			.as_bytes()
			.to_vec()
	}
}
