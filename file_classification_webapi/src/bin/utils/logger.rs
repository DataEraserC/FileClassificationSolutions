use chrono;
use fern;
use file_classification_common::env_loader::load_env_file;
use log;
use std::env;

/// 初始化日志系统，同时输出到终端和文件
pub fn setup_logger() -> Result<(), fern::InitError> {
  // 加载环境变量文件
  if let Err(e) = load_env_file() {
    log::error!("加载环境变量文件失败: {}", e);
  }

  // 创建 logs 目录（如果不存在）
  std::fs::create_dir_all("logs")?;

  // 获取当前日期时间作为日志文件名
  let local_time = chrono::Local::now();
  let date_str = local_time.format("%Y-%m-%d").to_string();
  let log_file_path = format!("logs/{}.log", date_str);

  // 获取终端日志等级
  let console_log_level =
    env::var("RUST_LOG").ok().and_then(|s| s.parse().ok()).unwrap_or(log::LevelFilter::Info);

  // 文件日志等级默认为 Debug
  let file_log_level =
    env::var("RUST_LOG_FILE").ok().and_then(|s| s.parse().ok()).unwrap_or(log::LevelFilter::Debug);

  // 创建日志分发器
  let dispatch = fern::Dispatch::new().format(|out, message, record| {
    out.finish(format_args!(
      "[{}][{}][{}] {}",
      chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
      record.level(),
      record.target(),
      message
    ))
  });

  // 添加控制台输出
  let dispatch =
    dispatch.chain(fern::Dispatch::new().level(console_log_level).chain(std::io::stdout()));

  // 添加文件输出
  let dispatch = dispatch
    .chain(fern::Dispatch::new().level(file_log_level).chain(fern::log_file(log_file_path)?));

  dispatch.apply()?;

  Ok(())
}
