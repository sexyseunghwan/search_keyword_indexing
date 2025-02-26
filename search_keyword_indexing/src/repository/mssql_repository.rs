use crate::common::*;

static MSSQL_CONNECTION: once_lazy<OnceCell<Arc<Mutex<Client<Compat<TcpStream>>>>>> =
    once_lazy::new(OnceCell::new);

pub struct MssqlSingleton;

impl MssqlSingleton {
    /* MSSQL 커넥션을 가져오는 함수 (싱글톤 유지) */
    pub async fn get_connection() -> Result<Arc<Mutex<Client<Compat<TcpStream>>>>> {
        MSSQL_CONNECTION
            .get_or_try_init(Self::create_connection)
            .await
            .map(Arc::clone)
    }

    /* MSSQL 연결 생성 (최초 실행 시 한 번만 실행됨) */
    async fn create_connection() -> Result<Arc<Mutex<Client<Compat<TcpStream>>>>> {
        dotenv::dotenv().ok(); // .env 파일 로드

        let database_url: String = env::var("DATABASE_URL").map_err(|e| {
            anyhow!(
                "[Error][create_connection()] DATABASE_URL not set in .env: {:?}",
                e
            )
        })?;

        let config: Config = Config::from_ado_string(&database_url)?;
        let tcp: TcpStream = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let client: Client<Compat<TcpStream>> = Client::connect(config, tcp.compat_write()).await?;
        info!("MSSQL Singleton Connection Established!");

        Ok(Arc::new(Mutex::new(client)))
    }
}
