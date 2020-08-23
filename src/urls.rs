use seed::{prelude::*, *};

use super::model::*;

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn active(self) -> Url {
        self.base_url().add_hash_path_part(ACTIVE)
    }
    pub fn completed(self) -> Url {
        self.base_url().add_hash_path_part(COMPLETED)
    }
}
