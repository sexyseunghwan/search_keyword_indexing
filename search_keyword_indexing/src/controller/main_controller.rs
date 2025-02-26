use crate::common::*;

use crate::service::es_query_service::*;
use crate::service::query_service::*;

use crate::models::keyword_popular_tb_test::*;
use crate::models::schedules_info::*;

use crate::configuration::system_config::*;

use crate::utils_module::time_utils::*;

#[derive(Debug, new)]
pub struct MainController<Q: QueryService, E: EsQueryService> {
    query_service: Q,
    es_query_service: E,
}

impl<Q: QueryService, E: EsQueryService> MainController<Q, E> {
    #[doc = "메인 스케쥴러 함수"]
    /// # Arguments
    /// * `schedules_info` - 인덱스 스케쥴 정보
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_schedule_task(
        &self,
        schedules_info: SchedulesInfo,
    ) -> Result<(), anyhow::Error> {
        let schedule: Schedule =
            Schedule::from_str(&schedules_info.time).expect("Failed to parse CRON expression");

        let system_config: Arc<SystemConfig> = get_system_config();

        let mut interval: Interval = tokio::time::interval(tokio::time::Duration::from_millis(
            system_config.schedule_term,
        ));

        /* 한국 표준시 GMT + 9 */
        let kst_offset: FixedOffset = match FixedOffset::east_opt(9 * 3600) {
            Some(kst_offset) => kst_offset,
            None => {
                error!(
                    "[Error][main_schedule_task()] There was a problem initializing 'kst_offset'."
                );
                panic!(
                    "[Error][main_schedule_task()] There was a problem initializing 'kst_offset'."
                );
            }
        };

        loop {
            interval.tick().await;

            let now: DateTime<Utc> = Utc::now();
            let kst_now: DateTime<FixedOffset> = now.with_timezone(&kst_offset); /* Converting UTC Current Time to KST */

            if let Some(next) = schedule.upcoming(kst_offset).take(1).next() {
                if (next - kst_now).num_seconds() < 1 {
                    match self.main_task(schedules_info.clone()).await {
                        Ok(_) => (),
                        Err(e) => {
                            error!("[Error][main_schedule_task() -> main_task()] {:?}", e);
                        }
                    }
                }
            }
        }
    }

    #[doc = "메인작업 합수 -> 검색통계정보 색인진행 함수"]
    /// # Arguments
    /// * `schedules_info` - 스케쥴 정보
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_task(&self, schedules_info: SchedulesInfo) -> Result<(), anyhow::Error> {
        /*  현재기준 한국 날짜를 가져와준다. */
        let current_kor: NaiveDate = get_current_kor_naivedate();

        /* RDB 에서 데이터를 가져와준다. */
        let rdb_datas: Vec<KeywordPopularTbTest> = self
            .query_service
            .get_keyword_popular_tb_test_data(current_kor)
            .await?;

        /* RDB 에서 가져온 데이터를 Elasticsearch 로 보내준다. */ 
        self.es_query_service.post_indexing_data(schedules_info.index_alias_name(), &rdb_datas).await?;
        
        Ok(())
    }
}
