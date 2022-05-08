/*
 * @author	:: Preston Wang-Stosur-Bassett <http://stosur.info>
 * @date	:: May 5, 2020
 * @description	:: Segment Chinese sentences into component words using a dictionary-driven largest first matching approach.
*/

//! ### About
//! Segment Chinese sentences into component words using a dictionary-driven largest first matching approach.
//!
//! ### Usage
//! ```rust
//! extern crate chinese_segmenter;
//!
//! use chinese_segmenter::{initialize, tokenize};
//!
//! let sentence = "今天晚上想吃羊肉吗？";
//! initialize(); // Optional initialization to load data
//! let result: Vec<&str> = tokenize(sentence);
//! println!("{:?}", result); // --> ['今天', '晚上', '想', '吃', '羊肉', '吗']

extern crate character_converter;

pub fn initialize() {
	character_converter::init()
}
pub fn tokenize(raw: &str) -> Vec<&str> {
	character_converter::tokenize(raw)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenize() {
		let sentence = "今天晚上想吃羊肉吗？";
		let result = tokenize(sentence);
		let expected = vec!["今天", "晚上", "想", "吃", "羊肉", "吗"];
		assert_eq!(expected, result);
	}
}
