
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;
// use tauri::Manager;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! 你已经打开了新世界的大门!", name)
}

#[tauri::command]
async fn run_powershell_command(command: String) -> Result<String, String> {
    let output = std::process::Command::new("powershell.exe")
        .args(&["-Command", "chcp 65001 ;", &command])
        .creation_flags(0x08000000) // 设置不显示命令弹窗
        .output()
        .expect("failed to execute process");
    // println!("结果: {:?}", output);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        return Err(stderr.to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn main() {    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, run_powershell_command])
        .run(tauri::generate_context!())
        .expect("程序运行错误");
    
}
