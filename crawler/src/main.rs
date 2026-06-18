use clap::Parser;
use std::fs;
use wechat_api_crawler::ProgressRecord;
use wechat_api_crawler::cli::Cli;
use wechat_api_crawler::utils::{
    fetch_all_apis, generate_index, parse_api_list, validate_progress,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 添加依赖检查
    println!("🚀 微信服务号 API 文档爬虫 (支持断点续传)");
    println!("{}", "=".repeat(50));

    // 显示配置信息
    println!("📋 配置信息:");
    println!("   📄 API 列表文件: {}", cli.apis_file.display());
    println!("   📁 输出目录: {}", cli.output_dir.display());
    println!("   💾 进度文件: {}", cli.progress_file.display());
    if cli.force {
        println!("   🔄 强制重新获取: 是 (忽略已有进度)");
    }
    println!();

    // 检查 API 列表文件是否存在
    if !cli.apis_file.exists() {
        eprintln!("❌ API 列表文件不存在: {}", cli.apis_file.display());
        eprintln!("💡 请确保文件存在，或使用 --apis-file 指定正确路径");
        return Ok(());
    }

    println!("📖 解析 API 列表...");
    let apis = parse_api_list(&cli.apis_file)?;
    println!("✅ 共解析到 {} 个 API 接口", apis.len());

    if apis.is_empty() {
        eprintln!(
            "❌ 未解析到任何 API，请检查 {} 文件格式",
            cli.apis_file.display()
        );
        return Ok(());
    }

    // 创建输出目录
    if !cli.output_dir.exists() {
        fs::create_dir_all(&cli.output_dir)?;
        println!("📁 创建输出目录: {}", cli.output_dir.display());
    }

    // 如果是强制重新获取，删除旧的进度文件
    if cli.force && cli.progress_file.exists() {
        fs::remove_file(&cli.progress_file)?;
        println!("🔄 已删除旧进度文件，将重新获取所有文档");
    }

    // 验证进度文件
    validate_progress(&apis, &cli.progress_file)?;

    // 加载进度
    let progress = ProgressRecord::load(cli.progress_file.to_str().unwrap());

    // 生成索引文件
    generate_index(&apis, &cli.output_dir, &progress)?;
    println!("📄 已生成索引文件: {}/INDEX.md", cli.output_dir.display());

    // 获取所有文档
    println!("\n🔄 开始获取 API 文档...");
    fetch_all_apis(&apis, &cli.output_dir, &cli.progress_file).await?;

    // 最后更新索引
    let final_progress = ProgressRecord::load(cli.progress_file.to_str().unwrap());
    generate_index(&apis, &cli.output_dir, &final_progress)?;

    println!("\n✨ 全部完成！");
    println!("📁 文档保存在: {}/", cli.output_dir.display());
    println!("💾 进度文件: {}", cli.progress_file.display());

    Ok(())
}
