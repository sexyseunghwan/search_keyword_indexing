use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone, new)]
pub struct KeywordPopularTbTest {
    pub keyword: String,
    pub qc: i32,
    pub reg_dt: String,
}
