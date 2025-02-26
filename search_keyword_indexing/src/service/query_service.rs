use crate::common::*;

use crate::repository::mssql_repository::*;

use crate::utils_module::time_utils::get_add_date_from_naivedate;

use crate::models::keyword_popular_tb_test::*;

#[async_trait]
pub trait QueryService {
    async fn execute_prepared_query(
        &self,
        query: &str,
        params: &[&(dyn ToSql)],
    ) -> Result<Vec<Row>, anyhow::Error>;
    async fn get_keyword_popular_tb_test_data(
        &self,
        date_time: NaiveDate,
    ) -> Result<Vec<KeywordPopularTbTest>, anyhow::Error>;
}

#[derive(Debug, new)]
pub struct QueryServicePub;

#[async_trait]
impl QueryService for QueryServicePub {

    #[doc = "Sql Server 에서 쿼리를 실행시켜주는 함수 - Prepared Statetment"]
    /// # Arguments
    /// * `query` - MSSQL 쿼리문
    /// * `params` - 쿼리에 맵핑될 변수 값
    /// 
    /// # Returns
    /// * Result<Vec<KeywordPopularTbTest>, anyhow::Error>
    async fn execute_prepared_query(
        &self,
        query: &str,
        params: &[&(dyn ToSql)], // Prepared Statement를 위한 바인딩 값
    ) -> Result<Vec<Row>, anyhow::Error> {
        let db_conn: Arc<Mutex<Client<Compat<TcpStream>>>> =
            MssqlSingleton::get_connection().await?;

        let mut client: MutexGuard<'_, Client<Compat<TcpStream>>> = db_conn.lock().await;
        let mut rows: tiberius::QueryStream<'_> = client.query(query, params).await?;
        let mut results: Vec<Row> = Vec::new();

        while let Some(item) = rows.try_next().await? {
            if let QueryItem::Row(row) = item {
                results.push(row);
            }
        }

        Ok(results)
    }

    #[doc = "keyword_popular_tb_test 테이블 데이터를 DB 로 부터 가져와주는 함수"]
    /// # Arguments
    /// * `date_time` - 날짜 데이터
    /// 
    /// # Returns
    /// * Result<Vec<KeywordPopularTbTest>, anyhow::Error>
    async fn get_keyword_popular_tb_test_data(
        &self,
        date_time: NaiveDate,
    ) -> Result<Vec<KeywordPopularTbTest>, anyhow::Error> {
        let next_date: NaiveDate = get_add_date_from_naivedate(date_time, 1)?;

        let cur_date_str: String = date_time.to_string();
        let next_date_str: String = next_date.to_string();

        let query_res: Vec<Row> = self
            .execute_prepared_query(
                "SELECT TOP(300)
                    keyword, 
                    qc, 
                    CONVERT(VARCHAR, regdt, 23) as regdt 
                FROM [JOBMAIN].[dbo].[keyword_popular_tb_test] WITH(NOLOCK)
                WHERE regdt BETWEEN @P1 AND @P2",
                &[&cur_date_str, &next_date_str],
            )
            .await?;

        let mut keyword_popular_list: Vec<KeywordPopularTbTest> = Vec::new();

        for elem in query_res {
            let keyword: String = elem.get::<&str, _>(0).ok_or_else(|| anyhow!("[Error][get_keyword_popular_tb_test_data()] 'keyword' value does not exist."))?.to_string();
            let qc: i32 = elem.get::<i32, _>(1).ok_or_else(|| {
                anyhow!("[Error][get_keyword_popular_tb_test_data()] 'qc' value does not exist.")
            })?;
            let reg_dt: String = elem.get::<&str, _>(2).ok_or_else(|| anyhow!("[Error][get_keyword_popular_tb_test_data()] 'reg_dt' value does not exist."))?.to_string();
            
            let keyword_info: KeywordPopularTbTest = KeywordPopularTbTest::new(keyword, qc, reg_dt);
            
            keyword_popular_list.push(keyword_info);
        }

        Ok(keyword_popular_list)
    }
}
