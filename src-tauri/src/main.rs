// 防止 Windows 发布版弹出额外控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    learnos_lib::run()
}
