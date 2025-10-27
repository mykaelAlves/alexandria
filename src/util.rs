use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
	pub page: usize,
	pub per_page: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Paginated<T>
where
	T: Serialize,
{
	pub items: Vec<T>,
	pub total: i64,
	pub page: usize,
	pub per_page: usize,
}

pub fn capitalize_words(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut capitalize_next = true;

	for c in s.chars() {
		if c.is_whitespace() {
			capitalize_next = true;
			result.push(c);
		} else if capitalize_next {
			for upper_c in c.to_uppercase() {
				result.push(upper_c);
			}
			capitalize_next = false;
		} else {
			for lower_c in c.to_lowercase() {
				result.push(lower_c);
			}
		}
	}

	result
}
