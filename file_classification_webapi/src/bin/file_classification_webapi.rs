use actix_web::{web, App, HttpServer, middleware::Logger};
use file_classification_core::utils::database::establish_connection;
mod handlers;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("正在启动文件分类 Web API...");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(establish_connection()))
            .wrap(Logger::default())
            .service(handlers::files::api_list_files)
            .service(handlers::files::api_list_files_by_conditions)
            // .service(handlers::files::api_create_file)
            .service(handlers::files::api_update_files_by_conditions)
            .service(handlers::files::api_delete_file)
            .service(handlers::groups::api_list_groups)
            .service(handlers::groups::api_list_groups_by_conditions)
            .service(handlers::groups::api_create_group)
            .service(handlers::groups::api_update_groups_by_conditions)
            .service(handlers::groups::api_delete_group)
            .service(handlers::tags::api_list_tags)
            .service(handlers::tags::api_list_tags_by_conditions)
            .service(handlers::tags::api_create_tag)
            .service(handlers::tags::api_update_tags_by_conditions)
            .service(handlers::tags::api_delete_tag)
            .service(handlers::file_groups::api_list_file_groups_by_conditions)
            .service(handlers::file_groups::api_create_file_group)
            .service(handlers::file_groups::api_delete_file_group)
            .service(handlers::group_tags::api_list_group_tags_by_conditions)
            .service(handlers::group_tags::api_create_group_tag)
            .service(handlers::group_tags::api_delete_group_tag)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
