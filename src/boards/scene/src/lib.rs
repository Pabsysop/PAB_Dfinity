mod common_scene;

trait Scene {
    fn default(owner: &str);
    fn open(&self) -> Self;
    fn name(&self) -> &'static str;
    fn get_event_cover(&self) -> String;
}
