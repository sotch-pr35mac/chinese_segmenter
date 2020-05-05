/*
 * @author		:: Preston Wang-Stosur-Bassett
 * @date 		:: January 26, 2020
 * @description		:: This package segments chinese text into valid chinese words based on CC-CEDICT data
 */

use bincode::deserialize_from;
use character_converter::CharacterConverter;
use std::collections::HashSet;

static TRADITIONAL_DATA: &'static [u8] = include_bytes!("../data/traditional.profile");
static SIMPLIFIED_DATA: &'static [u8] = include_bytes!("../data/simplified.profile");

pub struct Segmenter {
	traditional: HashSet<String>,
	simplified: HashSet<String>,
	converter: CharacterConverter
}

impl Segmenter {
	pub fn new() -> Segmenter {
		return Segmenter {
			traditional: deserialize_from(TRADITIONAL_DATA).unwrap(),
			simplified: deserialize_from(SIMPLIFIED_DATA).unwrap(),
			converter: CharacterConverter::new()
		}
	}

	pub fn tokenize(&self, raw: String) -> Vec<String> {
		let mut tokens: Vec<String> = Vec::new();
		let default_take = if raw.len() < 20 { raw.len() } else { 20 };
		let mut skip = 0;
		let mut take = default_take;
		let dictionary = if self.converter.is_traditional(raw.to_string()) { &self.traditional } else { &self.simplified };

		while skip < raw.chars().count() {
			let substring: String = raw.chars().skip(skip).take(take).collect();
			if !dictionary.contains(&substring) {
				if take > 1 {
					take -= 1;
				} else {
					skip += 1;
					take = default_take;
				}
			} else {
				tokens.push(substring);
				skip += take;
				take = default_take;
			}

		}

		return tokens;
	}
}
