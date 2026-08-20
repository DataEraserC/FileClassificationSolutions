// mod.rs
//! 业务服务层模块
//!
//! 该模块是应用程序的业务逻辑层，负责处理具体的业务需求，
//! 协调数据访问层进行数据操作，并实现业务规则和流程控制。
//! 所有业务逻辑都应该通过这个模块暴露给上层应用。

/// 为实体生成 6 个标准查询函数
///
/// 每个实体都有 filter/conditions 两种查询输入，各对应 limit/options/pagination 三个变体，
/// 且实现几乎一致，仅类型不同。本宏统一生成这 6 个函数：
/// - `filter_with_limit` / `filter_with_options` / `filter_with_pagination`：
///   以过滤条件为输入，其中 options/pagination 变体通过 `to_conditions` 转换为条件后查询
/// - `conditions_with_limit` / `conditions_with_options` / `conditions_with_pagination`：
///   以条件向量为输入，直接透传数据访问层
///
/// 参数:
/// - `dao`: 数据访问层模块（如 `files_dao`）
/// - 六个函数名: 生成的 6 个公开函数名
/// - `dto`: 实体 DTO 类型
/// - `filter`: 过滤条件类型
/// - `condition`: 查询条件类型
/// - `options`: 查询选项类型
/// - `to_conditions`: `fn(Filter) -> Vec<Condition>` 转换函数
macro_rules! select_functions {
  (
    dao: $dao:ident,
    filter_with_limit: $f_limit:ident,
    filter_with_options: $f_options:ident,
    filter_with_pagination: $f_pagination:ident,
    conditions_with_limit: $c_limit:ident,
    conditions_with_options: $c_options:ident,
    conditions_with_pagination: $c_pagination:ident,
    dto: $dto:ty,
    filter: $filter:ty,
    condition: $condition:ty,
    options: $options:ty,
    to_conditions: $to_conditions:path,
  ) => {
    /// 根据过滤条件查询记录列表
    ///
    /// 与 filter_with_options / filter_with_pagination 保持一致，统一经
    /// `to_conditions` 转换为条件后再查询，保证字符串字段的 LIKE 语义及
    /// description 等字段在所有 filter 变体间一致，避免各变体语义漂移
    pub fn $f_limit(
      conn: &mut $crate::utils::database::AnyConnection,
      search_input: $filter,
      limit: Option<i64>,
    ) -> Result<Vec<$dto>, $crate::service::AppError> {
      let conditions = $to_conditions(search_input);
      $dao::$c_limit(conn, conditions, limit).map_err($crate::service::AppError::from)
    }

    /// 根据过滤条件和选项查询记录列表
    pub fn $f_options(
      conn: &mut $crate::utils::database::AnyConnection,
      search_input: $filter,
      options: $options,
    ) -> Result<Vec<$dto>, $crate::service::AppError> {
      let conditions = $to_conditions(search_input);
      $dao::$c_options(conn, conditions, options).map_err($crate::service::AppError::from)
    }

    /// 根据过滤条件和选项查询记录列表（支持分页结果）
    pub fn $f_pagination(
      conn: &mut $crate::utils::database::AnyConnection,
      search_input: $filter,
      options: $options,
    ) -> Result<
      $crate::model::models::PaginationResult<$dto>,
      $crate::service::AppError,
    > {
      let conditions = $to_conditions(search_input);
      $dao::$c_pagination(conn, conditions, options).map_err($crate::service::AppError::from)
    }

    /// 根据条件查询记录列表
    pub fn $c_limit(
      conn: &mut $crate::utils::database::AnyConnection,
      condition: Vec<$condition>,
      limit: Option<i64>,
    ) -> Result<Vec<$dto>, $crate::service::AppError> {
      $dao::$c_limit(conn, condition, limit).map_err($crate::service::AppError::from)
    }

    /// 根据条件和选项查询记录列表
    pub fn $c_options(
      conn: &mut $crate::utils::database::AnyConnection,
      conditions: Vec<$condition>,
      options: $options,
    ) -> Result<Vec<$dto>, $crate::service::AppError> {
      $dao::$c_options(conn, conditions, options).map_err($crate::service::AppError::from)
    }

    /// 根据条件和选项查询记录列表（支持分页结果）
    pub fn $c_pagination(
      conn: &mut $crate::utils::database::AnyConnection,
      conditions: Vec<$condition>,
      options: $options,
    ) -> Result<
      $crate::model::models::PaginationResult<$dto>,
      $crate::service::AppError,
    > {
      $dao::$c_pagination(conn, conditions, options).map_err($crate::service::AppError::from)
    }
  };
}

/// 文件业务服务模块
///
/// 提供文件相关的业务逻辑处理，包括文件的创建、删除、查询、更新等操作，
/// 以及文件与其他实体（如分组、标签）之间的关联管理
pub mod files;

/// 分组业务服务模块
///
/// 提供分组相关的业务逻辑处理，包括分组的创建、删除、查询、更新等操作，
/// 以及分组与其他实体（如文件、标签）之间的关联管理
pub mod groups;

/// 标签业务服务模块
///
/// 提供标签相关的业务逻辑处理，包括标签的创建、删除、查询、更新等操作，
/// 以及标签与其他实体（如分组）之间的关联管理
pub mod tags;

/// 文件-分组关联业务服务模块
///
/// 提供文件与分组之间关联关系的业务逻辑处理，
/// 包括关联关系的创建、删除、查询等操作，以及相关的引用计数管理
pub mod file_group;

/// 分组-标签关联业务服务模块
///
/// 提供分组与标签之间关联关系的业务逻辑处理，
/// 包括关联关系的创建、删除、查询等操作，以及相关的引用计数管理
pub mod group_tag;

/// 组关系业务服务模块
///
/// 提供组与组之间关系的业务逻辑处理，
/// 包括关系的创建、删除、查询等操作，以及循环引用检测
pub mod group_relations;

// 导入数据库相关模块，供服务层使用
use super::utils::database;

// 导入错误处理模块，供服务层使用
use super::utils::errors::AppError;
