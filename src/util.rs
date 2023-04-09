#[derive(Copy, Clone)]
pub struct Clock {}

impl Clock {
    pub fn get_current_time(&self) -> String {
        format!("{:?}", std::time::Instant::now())
    }
}
