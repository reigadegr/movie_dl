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

use std::path::Path;
use futures::stream::{self, StreamExt};
use anyhow::{Context, Result};

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
    // 并发度：同时最多下载 N 个文件，可自行调整
    const CONCURRENT: usize = 8;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    stream::iter(URLS)
        .map(|url| {
            let client = &client;
            async move {
                let basename = url::Url::parse(url)
                    .ok()
                    .and_then(|u| u.path_segments()?.next_back().map(std::borrow::ToOwned::to_owned))
                    .unwrap_or_else(|| "unknown".into());

                if Path::new(&basename).exists() {
                    println!("跳过（已存在）：{basename}");
                    return Ok(());
                }

                println!("开始下载：{url} -> {basename}");

                let resp = client.get(*url).send().await
                    .with_context(|| format!("请求失败：{url}"))?;
                let mut stream = resp.bytes_stream();

                let mut file = tokio::fs::File::create(&basename).await
                    .with_context(|| format!("创建文件失败：{basename}"))?;

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk.with_context(|| format!("下载流中断：{url}"))?;
                    tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await
                        .with_context(|| format!("写入文件失败：{basename}"))?;
                }

                println!("完成：{basename}");
                Ok::<(), anyhow::Error>(())
            }
        })
        .buffer_unordered(CONCURRENT)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    println!("全部下载完成！");
    Ok(())
}
