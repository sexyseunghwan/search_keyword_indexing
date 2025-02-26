use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct SchedulesInfo {
    pub schedule_name: String,
    pub index_alias_name: String,
    pub time: String,
}
