use crate::Scene;
use url::Url;

pub struct CommonScene {
    pub title: String,
    pub cover: Url
}

impl Scene for CommonScene {
    fn default(_owner: &str) {
        todo!()
    }

    fn open(&self) -> Self {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }

    fn get_event_cover(&self) -> String {
        todo!()
    }
}
