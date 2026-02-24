use actix_web::{App, HttpServer, middleware::Logger, web};

use crate::handlers::file_groups;
use crate::handlers::files;
use crate::handlers::group_relations;
use crate::handlers::group_tags;
use crate::handlers::groups;
use crate::handlers::tags;
use crate::handlers::uploads;
use crate::utils::app_config;
use crate::utils::cors;
use crate::utils::database;
use crate::utils::static_files;

pub async fn start_server(config: app_config::AppConfig) -> std::io::Result<()> {
  let pool = database::establish_connection_pool();

  config.log_startup_info();

  let bind_info = config.bind_info();

  // 在HttpServer::new中添加新的路由
  HttpServer::new(move || {
    // 创建 CORS 中间件
    let cors_middleware = cors::create_cors();

    App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .wrap(cors_middleware)
            // API路由 - 放在静态文件服务之前以确保优先匹配
            // 文件相关路由
            .service(files::api_list_files_by_filter_with_limit)
            .service(files::api_list_files_by_filter_with_options)
            .service(files::api_list_files_by_filter_with_pagination)
            .service(files::api_list_files_by_conditions_with_limit)
            .service(files::api_list_files_by_conditions_with_options)
            .service(files::api_list_files_by_conditions_with_pagination)
            .service(files::api_list_files_by_group_id)
            .service(files::api_get_file_by_id)
            .service(files::api_create_file)
            .service(files::api_update_files_by_conditions)
            .service(files::api_update_file_by_id)
            .service(files::api_delete_file_by_id)
            .service(files::api_delete_files_by_ids)
            .service(files::api_delete_files_by_conditions)
            // 文件上传和下载路由
            .service(uploads::upload_file)
            .service(uploads::download_file)
            // 组相关路由
            .service(groups::api_list_groups_by_filter_with_limit)
            .service(groups::api_list_groups_by_filter_with_options)
            .service(groups::api_list_groups_by_filter_with_pagination)
            .service(groups::api_list_groups_by_conditions_with_limit)
            .service(groups::api_list_groups_by_conditions_with_options)
            .service(groups::api_list_groups_by_conditions_with_pagination)
            .service(groups::api_list_groups_by_file_id)
            .service(groups::api_list_groups_by_tag_id)
            .service(groups::api_get_group_by_id)
            .service(groups::api_create_group)
            .service(groups::api_update_groups_by_conditions)
            .service(groups::api_update_group_by_id)
            .service(groups::api_delete_group_by_id)
            .service(groups::api_delete_groups_by_ids)
            .service(groups::api_delete_groups_by_conditions)
            .service(groups::api_get_group_tree)
            // 标签相关路由
            .service(tags::api_list_tags_by_filter_with_limit)
            .service(tags::api_list_tags_by_filter_with_options)
            .service(tags::api_list_tags_by_filter_with_pagination)
            .service(tags::api_list_tags_by_conditions_with_limit)
            .service(tags::api_list_tags_by_conditions_with_options)
            .service(tags::api_list_tags_by_conditions_with_pagination)
            .service(tags::api_list_tags_by_group_id)
            .service(tags::api_get_tag_by_id)
            .service(tags::api_create_tag)
            .service(tags::api_update_tags_by_conditions)
            .service(tags::api_update_tag_by_id)
            .service(tags::api_delete_tag_by_id)
            .service(tags::api_delete_tags_by_ids)
            .service(tags::api_delete_tags_by_conditions)
            // 文件组关联路由
            .service(file_groups::api_list_file_groups_by_filter_with_limit)
            .service(file_groups::api_list_file_groups_by_filter_with_options)
            .service(file_groups::api_list_file_groups_by_filter_with_pagination)
            .service(file_groups::api_list_file_groups_by_conditions_with_limit)
            .service(file_groups::api_list_file_groups_by_conditions_with_options)
            .service(file_groups::api_list_file_groups_by_conditions_with_pagination)
            .service(file_groups::api_create_file_group)
            .service(file_groups::api_delete_file_group)
            .service(file_groups::api_delete_file_groups_by_dtos)
            .service(file_groups::api_delete_file_groups_by_conditions)
            // 组标签关联路由
            .service(group_tags::api_list_group_tags_by_filter_with_limit)
            .service(group_tags::api_list_group_tags_by_filter_with_options)
            .service(group_tags::api_list_group_tags_by_filter_with_pagination)
            .service(group_tags::api_list_group_tags_by_conditions_with_limit)
            .service(group_tags::api_list_group_tags_by_conditions_with_options)
            .service(group_tags::api_list_group_tags_by_conditions_with_pagination)
            .service(group_tags::api_create_group_tag)
            .service(group_tags::api_delete_group_tag)
            .service(group_tags::api_delete_group_tags_by_dtos)
            .service(group_tags::api_delete_group_tags_by_conditions)
            // 组关系路由
            .service(group_relations::api_list_group_relations_by_filter_with_limit)
            .service(group_relations::api_list_group_relations_by_filter_with_options)
            .service(group_relations::api_list_group_relations_by_filter_with_pagination)
            .service(group_relations::api_list_group_relations_by_conditions_with_limit)
            .service(group_relations::api_list_group_relations_by_conditions_with_options)
            .service(group_relations::api_list_group_relations_by_conditions_with_pagination)
            .service(group_relations::api_create_group_relation)
            .service(group_relations::api_delete_group_relation)
            .service(group_relations::api_delete_group_relations_by_dtos)
            .service(group_relations::api_delete_group_relations_by_conditions)
            // 静态文件服务 - 使用嵌入的资源
            .route("/", web::get().to(static_files::index_handler))
            .route("/{filename:.*}", web::get().to(static_files::static_handler))
  })
  .bind(&bind_info)?
  .run()
  .await
}
