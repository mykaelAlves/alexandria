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
	s.split_whitespace()
		.map(|word| {
			let mut chars = word.chars();
			match chars.next() {
				Some(first_char) => {
					let first_upper = first_char.to_uppercase().to_string();

					let rest_lower = chars.as_str().to_lowercase();

					format!("{}{}", first_upper, rest_lower)
				}

				None => String::new(),
			}
		})
		.collect::<Vec<String>>()
		.join(" ")
}
