/*
Author      : Seunghwan Shin
Create date : 2025-03-00
Description :

History     : 2025-03-00 Seunghwan Shin       # [v.1.0.0] first create
*/

mod common;
use common::*;

mod utils_module;
use utils_module::io_utils::*;
use utils_module::logger_utils::*;

mod controller;
use controller::main_controller::*;

mod service;
use service::es_query_service::*;
use service::query_service::*;

mod models;
use models::schedules_info::*;

mod repository;

mod configuration;
use configuration::common_variables::*;

#[tokio::main]
async fn main() {
    set_global_logger();
    dotenv().ok();

    info!("Search Keyword Batch Program Start");

    let query_service: QueryServicePub = QueryServicePub::new();
    let es_query_service: EsQueryServicePub = EsQueryServicePub::new();
    let controller: MainController<QueryServicePub, EsQueryServicePub> =
        MainController::new(query_service, es_query_service);

    let schedule_info: SchedulesInfo = match read_toml_from_file(&SCHEDULE_PATH) {
        Ok(schedule_info) => schedule_info,
        Err(e) => {
            error!("{:?}", e);
            panic!("{:?}", e);
        }
    };

    controller.main_task(schedule_info).await.unwrap();
    
    // tokio::spawn(async move {
    //     if let Err(e) = controller.main_schedule_task(schedule_info).await {
    //         error!("[Error][main_schedule_task] {:?}", e);
    //     }
    // });

    // /* 모두 서브테스크로 실행되므로 아래와 같이 메인 태스크를 계속 유지시켜줘야 한다. */
    // tokio::select! {
    //     _ = signal::ctrl_c() => {
    //         info!("Received Ctrl+C, shutting down...");
    //     }
    // }
}
