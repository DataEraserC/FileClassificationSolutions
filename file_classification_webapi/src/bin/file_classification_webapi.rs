use actix_web::{web, App, HttpServer, middleware::Logger};
mod handlers;
mod utils;

use utils::database::establish_connection_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    println!("正在启动文件分类 Web API...");

    let pool = establish_connection_pool();

    // 在HttpServer::new中添加新的路由
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            // 文件相关路由
            .service(handlers::files::api_list_files_by_filter)
            .service(handlers::files::api_get_file_by_id)
            .service(handlers::files::api_list_files_by_conditions)
            .service(handlers::files::api_list_files_by_conditions_with_options)
            .service(handlers::files::api_list_files_by_group_id)
            .service(handlers::files::api_create_file)
            .service(handlers::files::api_update_files_by_conditions)
            .service(handlers::files::api_update_file_by_id)
            .service(handlers::files::api_delete_file_by_id)
            .service(handlers::files::api_delete_files_by_conditions)
            // 组相关路由
            .service(handlers::groups::api_list_groups_by_filter)
            .service(handlers::groups::api_get_group_by_id)
            .service(handlers::groups::api_list_groups_by_conditions)
            .service(handlers::groups::api_list_groups_by_conditions_with_options)
            .service(handlers::groups::api_list_groups_by_file_id)
            .service(handlers::groups::api_list_groups_by_tag_id)
            .service(handlers::groups::api_create_group)
            .service(handlers::groups::api_update_groups_by_conditions)
            .service(handlers::groups::api_update_group_by_id)
            .service(handlers::groups::api_delete_group_by_id)
            .service(handlers::groups::api_delete_groups_by_conditions)
            // 标签相关路由
            .service(handlers::tags::api_list_tags_by_filter)
            .service(handlers::tags::api_get_tag_by_id)
            .service(handlers::tags::api_list_tags_by_conditions)
            .service(handlers::tags::api_list_tags_by_conditions_with_options)
            .service(handlers::tags::api_list_tags_by_group_id)
            .service(handlers::tags::api_create_tag)
            .service(handlers::tags::api_update_tags_by_conditions)
            .service(handlers::tags::api_update_tag_by_id)
            .service(handlers::tags::api_delete_tag_by_id)
            .service(handlers::tags::api_delete_tags_by_conditions)
            // 文件组关联路由
            .service(handlers::file_groups::api_list_file_groups_by_conditions)
            .service(handlers::file_groups::api_list_file_groups_by_conditions_with_options)
            .service(handlers::file_groups::api_create_file_group)
            .service(handlers::file_groups::api_delete_file_group)
            .service(handlers::file_groups::api_delete_file_groups_by_conditions)
            // 组标签关联路由
            .service(handlers::group_tags::api_list_group_tags_by_conditions)
            .service(handlers::group_tags::api_list_group_tags_by_conditions_with_options)
            .service(handlers::group_tags::api_create_group_tag)
            .service(handlers::group_tags::api_delete_group_tag)
            .service(handlers::group_tags::api_delete_group_tags_by_conditions)
    })


    .bind("127.0.0.1:8082")?
    .run()
    .await
}
