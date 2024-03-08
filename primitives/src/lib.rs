//! Custom host function
#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;
use sp_std::vec::Vec;

// #[cfg(feature = "std")]
// sp_externalities::decl_extension! {
// 	pub struct CustomExt;
// }

// #[cfg(feature = "std")]
// impl CustomExt {
// 	pub fn do_something(&mut self) -> String {
// 		"Host function test".to_string()
// 	}
// }

#[cfg(feature = "std")]
struct CustomFunction;

#[cfg(feature = "std")]
impl CustomFunction {
	pub fn do_something() -> String {
		"Host function test".to_string()
	}
}

/// Interface that provides access to the ai function.
#[runtime_interface]
pub trait Custom {
	fn do_something(&mut self) -> Vec<u8> {
		//self.extension::<CustomExt>().expect("CustomExt is not registered").do_something().into_bytes()
		CustomFunction::do_something().into_bytes()
	}
}
