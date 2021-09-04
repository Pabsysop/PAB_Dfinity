mod config;
mod end_point;
mod engine;
mod mapping_metadata;
mod summary;

use config::Config;
use mapping_metadata::MappingMetadata;
use engine::Engine;

static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";

pub struct OracleWorkshop {
    pub title: String,
    pub cover: String,
    config:     (),
    engine:     (),
    log:        ()
}

impl OracleWorkshop {
    fn new_oracle(config: Config, engine: Engine) -> Result<OracleWorkshop, String> {
        Ok(
            OracleWorkshop {
                title: "".to_string(),
                cover: COMMON_VIEW.parse().unwrap(),
                config: (),
                engine: (),
                log: ()
            }
        )
    }

    // Run starts the Oracle service
    fn run(&self) {
        self.update_oracle();
    }

    fn update_oracle(&self) {}

    fn update_meta(self, meta: MappingMetadata) {}
}
