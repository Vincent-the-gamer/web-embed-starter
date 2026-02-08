use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))) // 设置日志级别
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true) // 打印文件名
                .with_line_number(true) // 打印行号
                .with_thread_ids(true) // 打印线程ID
                .with_thread_names(true) // 打印线程名称
                .with_target(false), // 不打印target
        )
        .init();
}
