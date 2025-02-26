use crate::common::*;

use crate::utils_module::time_utils::*;

use crate::repository::es_repository::*;

#[async_trait]
pub trait EsQueryService {
    async fn post_indexing_data<T: Serialize + Send + Sync + Debug>(&self, index_alias_name:&str, data: &Vec<T>) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct EsQueryServicePub;

#[async_trait]
impl EsQueryService for EsQueryServicePub {
    
    #[doc = ""]
    /// # Arguments
    /// * `index_name` - 스케쥴 정보
    /// * `data` - 스케쥴 정보
    ///
    /// # Returns
    /// * Result<(), anyhow::Error> 
    async fn post_indexing_data<T: Serialize + Send + Sync + Debug>(&self, index_alias_name:&str, data: &Vec<T>) -> Result<(), anyhow::Error> {

        /* Put today's date time on the index you want to create. */
        let curr_time: String = get_current_kor_naive_datetime()
            .format("%Y%m%d%H%M%S")
            .to_string();

        let new_index_name: String = format!("{}-{}", index_alias_name, curr_time);

        let es_conn: ElasticConnGuard = get_elastic_guard_conn().await?;

        /* Bulk post the data to the index above at once. */
        es_conn
            .bulk_indexing_query(&new_index_name, data, 1000)
            .await?;
        
        /* 해당 인덱스가 있는지 없는지 확인해준다. */
        let index_exists_yn: bool = match es_conn.check_index_exist(index_alias_name).await {
            Ok(_index_exists_yn) => true,
            Err(e) => {
                error!("[Error][post_indexing_data()] An index starting with that name does not exist.: {}, {:?}", index_alias_name, e);
                false
            }
        };

        if index_exists_yn {
            /* 기존 인덱스가 존재하는 경우 */
            let alias_resp: Value = es_conn
                .get_indexes_mapping_by_alias(index_alias_name)
                .await?;

            let old_index_name: String;

            if let Some(first_key) = alias_resp.as_object().and_then(|map| map.keys().next()) {
                old_index_name = first_key.to_string();
            } else {
                return Err(anyhow!("[Error][post_indexing_data_by_bulk()] Failed to extract index name within 'index-alias'"));
            }

            es_conn
                .update_index_alias(index_alias_name, &new_index_name, &old_index_name)
                .await?;

        } else {
            /* 기존 인덱스가 존재하지 않는 경우: 새로운 인덱스를 생성해준다. */
            es_conn
                .create_index_alias(index_alias_name, &new_index_name)
                .await?;
        }
        
        /* Functions to enable search immediately after index */
        es_conn.refresh_index(index_alias_name).await?;

        Ok(())
    }
}
