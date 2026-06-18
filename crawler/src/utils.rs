use crate::{ApiDoc, ProgressRecord};
use reqwest;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use tokio;

// 从 apis.md 解析出的 API 列表
pub fn parse_api_list(apis_file: &Path) -> Result<Vec<ApiDoc>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(apis_file)?;
    let mut apis = Vec::new();
    let mut current_category = String::new();
    let mut current_subcategory = String::new();

    for line in content.lines() {
        let line = line.trim();

        // 跳过空行和分隔线
        if line.is_empty() || line.starts_with("|---") {
            continue;
        }

        // 检测一级分类标题 (## 开头)
        if line.starts_with("## ") {
            current_category = line.trim_start_matches("## ").trim().to_string();
            continue;
        }

        // 检测二级分类标题 (### 开头)
        if line.starts_with("### ") {
            current_subcategory = line.trim_start_matches("### ").trim().to_string();
            continue;
        }

        // 检测表格行 (包含 | 且不是表头)
        if line.starts_with('|') && !line.contains("名称") && !line.contains("URL") {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let name = parts[1].trim();
                let url = parts[2].trim();

                if !name.is_empty() && !url.is_empty() && url.starts_with('/') {
                    let full_url = format!("https://developers.weixin.qq.com{}", url);
                    apis.push(ApiDoc {
                        category: current_category.clone(),
                        subcategory: current_subcategory.clone(),
                        name: name.to_string(),
                        url: full_url,
                    });
                }
            }
        }
    }

    Ok(apis)
}

fn collect_text(elem: &ElementRef) -> String {
    elem.text().collect::<String>().trim().to_string()
}

fn push_header(md: &mut String, level: &str, text: &str) {
    let t = text.trim();
    if !t.is_empty() && !t.contains("header-anchor") {
        md.push_str(&format!("{} {}\n\n", level, t));
    }
}

fn render_paragraph(elem: &ElementRef) -> String {
    let mut out = String::new();

    for child in elem.children() {
        if let Some(child_elem) = ElementRef::wrap(child) {
            let tag = child_elem.value().name();

            if tag == "a" {
                if let Some(href) = child_elem.value().attr("href") {
                    let text = child_elem.text().collect::<String>();
                    out.push_str(&format!("[{}]({}) ", text.trim(), href));
                }
            } else if tag == "strong" {
                let text = child_elem.text().collect::<String>();
                out.push_str(&format!("**{}**", text.trim()));
            } else {
                out.push_str(&child_elem.text().collect::<String>());
            }
        } else if let Some(text) = child.value().as_text() {
            out.push_str(text);
        }
    }

    let out = out.trim().to_string();
    if out.is_empty() { String::new() } else { out }
}

fn render_blockquote(elem: &ElementRef) -> String {
    let mut out = String::new();

    for child in elem.children() {
        if let Some(child_elem) = ElementRef::wrap(child) {
            if child_elem.value().name() == "a" {
                if let Some(href) = child_elem.value().attr("href") {
                    let text = child_elem.text().collect::<String>();
                    out.push_str(&format!("[{}]({})", text.trim(), href));
                }
            } else {
                out.push_str(&child_elem.text().collect::<String>());
            }
        } else if let Some(text) = child.value().as_text() {
            out.push_str(text);
        }
    }

    out.trim().to_string()
}

fn render_list(elem: &ElementRef) -> String {
    let mut out = String::new();

    for li in elem.select(&Selector::parse("li").unwrap()) {
        let li = ElementRef::wrap(*li).unwrap();
        let mut line = String::from("- ");

        for child in li.children() {
            if let Some(child_elem) = ElementRef::wrap(child) {
                match child_elem.value().name() {
                    "a" => {
                        if let Some(href) = child_elem.value().attr("href") {
                            let text = child_elem.text().collect::<String>();
                            line.push_str(&format!("[{}]({})", text.trim(), href));
                        }
                    }
                    "code" => {
                        let text = child_elem.text().collect::<String>();
                        line.push_str(&format!("`{}`", text.trim()));
                    }
                    _ => {
                        line.push_str(&child_elem.text().collect::<String>());
                    }
                }
            } else if let Some(text) = child.value().as_text() {
                line.push_str(text);
            }
        }

        if line.len() > 2 {
            out.push_str(&line);
            out.push('\n');
        }
    }

    out
}

fn build_table_sep(len: usize) -> String {
    (0..len).map(|_| "---").collect::<Vec<_>>().join(" | ")
}

fn render_table(elem: &ElementRef) -> String {
    let mut rows = vec![];

    for tr in elem.select(&Selector::parse("tr").unwrap()) {
        let tr = ElementRef::wrap(*tr).unwrap();
        let mut row = vec![];

        let is_header = tr.select(&Selector::parse("th").unwrap()).next().is_some();
        let th = Selector::parse("th").unwrap();
        let td = Selector::parse("td").unwrap();
        let cells = if is_header {
            tr.select(&th)
        } else {
            tr.select(&td)
        };

        for cell in cells {
            row.push(cell.text().collect::<String>().trim().to_string());
        }

        if !row.is_empty() {
            rows.push(row);
        }
    }

    if rows.is_empty() {
        return String::new();
    }

    let mut out = String::new();

    let header = &rows[0];
    out.push_str(&format!("| {} |\n", header.join(" | ")));
    out.push_str(&format!("| {} |\n", build_table_sep(header.len())));

    for row in &rows[1..] {
        out.push_str(&format!("| {} |\n", row.join(" | ")));
    }

    out
}

fn detect_lang(code: &str) -> &'static str {
    if code.contains("fn ") || code.contains("let ") {
        "rust"
    } else if code.contains("def ") {
        "python"
    } else if code.contains("GET") || code.contains("POST") {
        "bash"
    } else if code.trim_start().starts_with('{') {
        "json"
    } else {
        ""
    }
}

// 从 HTML 中提取文档内容并转换为 Markdown
pub fn extract_content_to_markdown(html: &str, url: &str) -> String {
    let document = Html::parse_document(html);

    let content_selector = Selector::parse(".content.custom").unwrap();

    let mut markdown = String::new();
    markdown.push_str(&format!("<!-- 来源: {} -->\n\n", url));

    let Some(content) = document.select(&content_selector).next() else {
        return markdown;
    };

    for node in content.children() {
        let Some(elem) = ElementRef::wrap(node) else {
            continue;
        };

        let tag = elem.value().name();

        match tag {
            "h1" => {
                let text = collect_text(&elem);
                push_header(&mut markdown, "#", &text);
            }

            "h2" => {
                let text = collect_text(&elem);
                push_header(&mut markdown, "##", &text);
            }

            "h3" => {
                let text = collect_text(&elem);
                push_header(&mut markdown, "###", &text);
            }

            "p" => {
                let text = render_paragraph(&elem);
                if !text.is_empty() {
                    markdown.push_str(&format!("{}\n\n", text));
                }
            }

            "blockquote" => {
                let text = render_blockquote(&elem);
                if !text.is_empty() {
                    markdown.push_str(&format!("> {}\n\n", text));
                }
            }

            "ul" => {
                markdown.push_str(&render_list(&elem));
                markdown.push('\n');
            }

            "table" => {
                markdown.push_str(&render_table(&elem));
                markdown.push('\n');
            }

            "pre" => {
                if let Some(code) = elem.select(&Selector::parse("code").unwrap()).next() {
                    let text = code.text().collect::<String>();
                    let lang = detect_lang(&text);
                    markdown.push_str(&format!("```{}\n{}\n```\n\n", lang, text.trim()));
                }
            }

            _ => {}
        }
    }

    markdown
}

// 获取单个 API 文档
async fn fetch_api_doc(
    api: &ApiDoc,
    client: &reqwest::Client,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(&api.url).send().await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let html = response.text().await?;
    let markdown = extract_content_to_markdown(&html, &api.url);

    Ok(markdown)
}

// 获取所有 API 文档（支持断点续传）
pub async fn fetch_all_apis(
    apis: &[ApiDoc],
    output_dir: &Path,
    progress_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let total = apis.len();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36")
        .build()?;
    // 加载进度
    let mut progress = ProgressRecord::load(progress_file.to_str().unwrap());
    progress.total = total;

    // 统计已完成和待处理
    let completed = progress.fetched_urls.len();
    let pending = total - completed;

    if completed > 0 {
        println!("📊 检测到已有进度:");
        println!("   - 已完成: {} 个", completed);
        println!("   - 待处理: {} 个", pending);
        println!("   - 总计: {} 个", total);

        if pending == 0 {
            println!("✅ 所有文档已获取完成！");
            return Ok(());
        }
        println!("🔄 继续获取剩余文档...\n");
    }

    let mut success = 0;
    let mut failed = 0;

    for (_i, api) in apis.iter().enumerate() {
        // 检查是否已获取
        if progress.is_fetched(&api.url) {
            success += 1;
            continue;
        }

        let current = completed + success + failed + 1;
        println!(
            "[{}/{}] 正在获取: {} - {}",
            current, total, api.category, api.name
        );

        match fetch_api_doc(api, &client).await {
            Ok(markdown) => {
                // 构建保存路径
                let dir_path = output_dir.join(&api.category);
                if !dir_path.exists() {
                    if let Err(e) = fs::create_dir_all(&dir_path) {
                        eprintln!("   ❌ 创建目录失败: {}", e);
                        failed += 1;
                        continue;
                    }
                }

                // 文件名：清理非法字符
                let filename = format!("{}.md", api.name)
                    .replace('/', "、")
                    .replace('\\', "、")
                    .replace(':', "：")
                    .replace('*', "＊")
                    .replace('?', "？")
                    .replace('"', "'")
                    .replace('<', "＜")
                    .replace('>', "＞")
                    .replace('|', "｜");

                let file_path = dir_path.join(filename);

                if let Err(e) = fs::write(&file_path, markdown) {
                    eprintln!("   ❌ 保存文件失败: {}", e);
                    failed += 1;
                } else {
                    // 标记为已获取
                    progress.mark_fetched(&api.url);
                    // 保存进度
                    if let Err(e) = progress.save(progress_file.to_str().unwrap()) {
                        eprintln!("   ⚠️ 保存进度失败: {}", e);
                    }
                    success += 1;
                    println!("   ✅ 已保存: {}", file_path.display());
                }
            }
            Err(e) => {
                eprintln!("   ❌ 获取失败: {}", e);
                failed += 1;
            }
        }

        // 添加延迟避免请求过快
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

        // 每 10 个保存一次进度
        if (success + failed) % 10 == 0 {
            if let Err(e) = progress.save(progress_file.to_str().unwrap()) {
                eprintln!("⚠️ 保存进度失败: {}", e);
            }
        }
    }

    // 最终保存进度
    if let Err(e) = progress.save(progress_file.to_str().unwrap()) {
        eprintln!("⚠️ 保存最终进度失败: {}", e);
    }

    println!("\n{}", "=".repeat(50));
    println!("📊 任务完成统计:");
    println!("   ✅ 成功: {}", success);
    println!("   ❌ 失败: {}", failed);
    println!("   📁 总计: {}", total);
    println!("   💾 进度文件: {}", progress_file.display());

    if failed > 0 {
        println!(
            "\n💡 提示: 有 {} 个文档获取失败，重新运行脚本将自动重试",
            failed
        );
    }

    Ok(())
}

// 生成索引文件
pub fn generate_index(
    apis: &[ApiDoc],
    output_dir: &Path,
    progress: &ProgressRecord,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut index_content = String::from("# 微信服务号 API 文档索引\n\n");
    index_content.push_str(&format!(
        "> 📊 总共 {} 个 API，已获取 {} 个\n\n",
        apis.len(),
        progress.fetched_urls.len()
    ));

    let mut current_category = String::new();
    for api in apis {
        if api.category != current_category {
            current_category = api.category.clone();
            index_content.push_str(&format!("## {}\n\n", current_category));
        }

        // 构建相对路径
        let filename = format!("{}.md", api.name)
            .replace('/', "、")
            .replace('\\', "、")
            .replace(':', "：")
            .replace('*', "＊")
            .replace('?', "？")
            .replace('"', "'")
            .replace('<', "＜")
            .replace('>', "＞")
            .replace('|', "｜");

        let rel_path = format!("{}/{}", api.category, filename);
        let status = if progress.is_fetched(&api.url) {
            "✅"
        } else {
            "⏳"
        };
        index_content.push_str(&format!("- {} [{}]({})\n", status, api.name, rel_path));

        // 添加子分类信息
        if !api.subcategory.is_empty() {
            index_content.push_str(&format!("  - 分类: {}\n", api.subcategory));
        }
    }

    let index_path = output_dir.join("INDEX.md");
    fs::write(index_path, index_content)?;
    Ok(())
}

// 验证并修复进度文件
pub fn validate_progress(
    apis: &[ApiDoc],
    progress_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let progress = ProgressRecord::load(progress_file.to_str().unwrap());

    // 检查是否有无效的 URL（已获取但不在当前 API 列表中）
    let valid_urls: HashSet<String> = apis.iter().map(|api| api.url.clone()).collect();
    let invalid_urls: Vec<&String> = progress
        .fetched_urls
        .iter()
        .filter(|url| !valid_urls.contains(*url))
        .collect();

    if !invalid_urls.is_empty() {
        println!(
            "⚠️ 发现 {} 个无效的进度记录（可能 API 列表已变更）",
            invalid_urls.len()
        );
        for url in invalid_urls {
            println!("   - {}", url);
        }
        println!("💡 建议手动检查进度文件: {}", progress_file.display());
    }

    Ok(())
}
