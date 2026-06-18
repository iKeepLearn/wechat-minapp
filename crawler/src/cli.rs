use clap::Parser;
use std::path::PathBuf;

// 定义命令行参数
#[derive(Parser, Debug)]
#[command(name = "wechat_api_crawler")]
#[command(about = "微信服务号 API 文档爬虫 - 支持断点续传", version = "2.0")]
pub struct Cli {
    /// API 列表文件路径 (默认为 ./apis.md)
    #[arg(short, long, default_value = "apis.md")]
    pub apis_file: PathBuf,

    /// 输出目录路径 (默认为 ./wechat_api_docs)
    #[arg(short, long, default_value = "wechat_api_docs")]
    pub output_dir: PathBuf,

    /// 进度文件路径 (默认为 ./progress.json)
    #[arg(long, default_value = "progress.json")]
    pub progress_file: PathBuf,

    /// 是否强制重新获取所有文档 (忽略进度文件)
    #[arg(short, long, default_value_t = false)]
    pub force: bool,
}
