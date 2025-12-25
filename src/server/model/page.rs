pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: u64, page: u64, page_size: u64) -> Self {
        let total_pages = if page_size == 0 {
            0
        } else {
            (total + page_size - 1) / page_size
        };

        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}
