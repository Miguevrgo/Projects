use crate::table::Page;
const PAGES_IN_CACHE: usize = 5;

pub struct Cache {
    pages: [(Option<usize>, Option<Page>); PAGES_IN_CACHE],
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            pages: [(None, None); PAGES_IN_CACHE],
        }
    }

    pub fn get(&self, page_num: usize) -> Option<Page> {
        for page in self.pages.iter() {
            if let Some(stored_page_num) = page.0 {
                if page_num == stored_page_num {
                    return Some(page.1)?;
                }
            }
        }
        None
    }
}
