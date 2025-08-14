#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness
)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const URLS: &[&str] = &[
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z01",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z02",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z03",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z04",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z05",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z06",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z07",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z08",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z09",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z10",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z11",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z12",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z13",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z14",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z15",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z16",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z17",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z18",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z19",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z20",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z21",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z22",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z23",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z24",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z25",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z26",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z27",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z28",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z29",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z30",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z31",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z32",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z33",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z34",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z35",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z36",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z37",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z38",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z39",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z40",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z41",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z42",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z43",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z44",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z45",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z46",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z47",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z48",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z49",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z50",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z51",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z52",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z53",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z54",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z55",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z56",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.z57",
    "https://ghfast.top/https://github.com/reigadegr/movie_mp4/releases/download/v1.0.0/NeZha2.zip",
];

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3600)) // 1小时超时
        .connect_timeout(Duration::from_secs(30))
        .build()?;

    for url in URLS {
        // 从URL中提取文件名
        let file_name = url
            .split('/')
            .next_back()
            .with_context(|| format!("无法从URL提取文件名: {url}"))?;

        // 检查文件是否已存在
        if Path::new(file_name).exists() {
            println!("文件已存在，跳过: {file_name}");
            continue;
        }

        println!("开始下载: {file_name}");

        // 发送请求
        let response = client
            .get(*url)
            .send()
            .await
            .with_context(|| format!("请求失败: {url}"))?;

        // 检查HTTP状态
        if !response.status().is_success() {
            anyhow::bail!("下载失败: {} - 状态码: {}", file_name, response.status());
        }

        // 创建文件
        let mut file = File::create(file_name)
            .await
            .with_context(|| format!("无法创建文件: {file_name}"))?;

        // 流式下载并写入文件
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)
                .await
                .with_context(|| format!("写入文件失败: {file_name}"))?;
        }

        // 确保所有数据都写入磁盘
        file.sync_all()
            .await
            .with_context(|| format!("同步文件失败: {file_name}"))?;

        println!("下载完成: {file_name}");
    }

    println!("所有文件下载完成!");
    Ok(())
}
