use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub size: u32,
    pub page: u32,
    pub pages: u64,
    pub total: u64,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, size: u32, page: u32, total: u64) -> Self {
        let page_float = page as f64;
        let total_float = total as f64;

        let pages = (total_float / page_float).ceil() as u64;

        Page {
            items,
            size,
            page,
            pages,
            total,
        }
    }

    pub fn map<F, U>(self, items: F) -> Page<U>
    where
        F: FnMut(T) -> U,
    {
        Page {
            items: self.items.into_iter().map(items).collect(),
            size: self.size,
            page: self.page,
            pages: self.pages,
            total: self.total,
        }
    }
}

#[derive(Deserialize, Debug, IntoParams)]
#[serde(default)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}

impl Default for Pagination {
    fn default() -> Pagination {
        Pagination { page: 1, size: 50 }
    }
}
