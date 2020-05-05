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
//! use chinese_segmenter::ChineseSegmenter;
//!
//! let segmenter = ChineseSegmenter::new();
//! let sentence: String = String::from("今天晚上想吃羊肉吗？");
//! let result: Vec<String> = segmenter.tokenize(sentence);
//! println!("{:?}", result); // --> ['今天', '晚上', '想', '吃', '羊肉', '吗']

extern crate bincode;
extern crate character_converter;

mod chinese_segmenter;
pub use self::chinese_segmenter::Segmenter as ChineseSegmenter;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tokenize() {
		let segmenter = ChineseSegmenter::new();
		let sentence: String = String::from("今天晚上想吃羊肉吗？");
		let result: Vec<String> = segmenter.tokenize(sentence);
		let expected: Vec<String> = vec!["今天".to_string(), "晚上".to_string(), "想".to_string(), "吃".to_string(), "羊肉".to_string(), "吗".to_string()];
		assert_eq!(expected, result);
	}
}
