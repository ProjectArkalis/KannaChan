use std::{
    path::Path,
    process::{Command, Stdio},
};

use glob::glob;
use tokio::fs;

pub async fn encode(path: String, output: Option<String>) -> anyhow::Result<()> {
    //ffmpeg -hwaccel cuda -hwaccel_output_format cuda -i input.mkv -c:v av1_nvenc -c:a libopus -b:v 8M output.webm
    // Isso ta quebardo no

    if Path::new(&path).is_dir() {
        for file in glob(&format!("{}/*", path))? {
            let file = file?;

            let output = file.with_extension("webm").to_str().unwrap().to_string();

            ffmpeg(file.to_str().unwrap().to_owned(), output)?;

            let old = format!("{}/old", path);
            fs::create_dir_all(&old).await?;
            fs::rename(
                &file,
                format!("{}/{}", old, file.file_name().unwrap().to_str().unwrap()),
            )
            .await?;
        }
    } else {
        let output = output.unwrap_or(
            Path::new(&path)
                .with_extension("webm")
                .to_str()
                .unwrap()
                .to_string(),
        );

        ffmpeg(path, output)?;
    }

    Ok(())
}

fn ffmpeg(file: String, out: String) -> anyhow::Result<()> {
    let mut cmd = Command::new("ffmpeg")
        .args([
            "-hwaccel",
            "cuda",
            "-hwaccel_output_format",
            "cuda",
            "-i",
            &file,
            "-c:v",
            "av1_nvenc",
            "-c:a",
            "libopus",
            "-b:v",
            "8M",
            &out,
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    cmd.wait().unwrap();

    Ok(())
}
