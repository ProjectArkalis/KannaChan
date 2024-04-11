use std::{
    path::Path,
    process::{Command, Stdio},
};

pub async fn encode(path: String, output: Option<String>) -> anyhow::Result<()> {
    //ffmpeg -hwaccel cuda -hwaccel_output_format cuda -i input.mkv -c:v av1_nvenc -c:a libopus -b:v 8M output.webm
    // Isso ta quebardo no momento

    let output = output.unwrap_or(
        Path::new(&path)
            .with_extension("webm")
            .to_str()
            .unwrap()
            .to_string(),
    );

    let mut cmd = Command::new("ffmpeg")
        .args([
            "-hwaccel",
            "cuda",
            "-hwaccel_output_format",
            "cuda",
            "-i",
            &path,
            "-c:v",
            "av1_nvenc",
            "-c:a",
            "libopus",
            "-b:v",
            "8M",
            &output,
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    cmd.wait().unwrap();

    Ok(())
}
