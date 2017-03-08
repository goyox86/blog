
#[derive(FromForm)]
pub struct Pagination {
    pub per_page: i64,
    pub page: i64
}
