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
