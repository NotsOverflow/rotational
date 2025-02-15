/*
*                                                                              *
*               ╔╗╔┌─┐┌┬┐┌─┐╔═╗┬  ┬┌─┐┬─┐┌─┐┬  ┌─┐┬ ┬                          *
*               ║║║│ │ │ └─┐║ ║└┐┌┘├┤ ├┬┘├┤ │  │ ││││                          *
*               ╝╚╝└─┘ ┴ └─┘╚═╝ └┘ └─┘┴└─└  ┴─┘└─┘└┴┘                          *
*                                                                              *
*               virtkeyboard.rs created 2025/02/13                             *
*               by Richard JUAN (contact@richard-juan.com)                     *
*                                                                              *
*               Copyright © 2025 Richard JUAN. All rights reserved             *
*                                                                              *
*/

use std::env;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn show_virtual_keyboard() {
    if let Ok(common_files) = env::var("windir") {
        let osk_path = format!(r"{}\system32\osk.exe", common_files);
        Command::new("cmd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/C", osk_path.as_str()])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start osk.exe");
    } else {
        eprintln!("Failed to locate Windows directory.");
    }
}
