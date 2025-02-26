use crate::common::*;

#[doc = "Function to globally initialize the 'SCHEDULE_PATH' variable"]
pub static SCHEDULE_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("SCHEDULE_PATH").expect("[ENV file read Error] 'SCHEDULE_PATH' must be set")
});
