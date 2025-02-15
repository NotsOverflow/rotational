/*
*                                                                              *
*               ╔╗╔┌─┐┌┬┐┌─┐╔═╗┬  ┬┌─┐┬─┐┌─┐┬  ┌─┐┬ ┬                          *
*               ║║║│ │ │ └─┐║ ║└┐┌┘├┤ ├┬┘├┤ │  │ ││││                          *
*               ╝╚╝└─┘ ┴ └─┘╚═╝ └┘ └─┘┴└─└  ┴─┘└─┘└┴┘                          *
*                                                                              *
*               main.rs created 2025/02/11                                     *
*               by Richard JUAN (contact@richard-juan.com)                     *
*                                                                              *
*               Copyright © 2025 Richard JUAN. All rights reserved             *
*                                                                              *
*/
#![windows_subsystem = "windows"]

use tray_item::{IconSource, TrayItem};
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};
mod togglescreen;
mod virtkeyboard;
use windows::Win32::Graphics::Gdi::{DMDO_180, DMDO_270, DMDO_90, DMDO_DEFAULT};

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::Left, &[ModKey::Alt], || {
        togglescreen::toggle_display(DMDO_270);
    })
    .unwrap();
    hkm.register(VKey::Right, &[ModKey::Alt], || {
        togglescreen::toggle_display(DMDO_90);
    })
    .unwrap();
    hkm.register(VKey::Up, &[ModKey::Alt], || {
        togglescreen::toggle_display(DMDO_180);
    })
    .unwrap();
    hkm.register(VKey::Down, &[ModKey::Alt], || {
        togglescreen::toggle_display(DMDO_DEFAULT);
    })
    .unwrap();

    let mut tray = TrayItem::new("Rotational", IconSource::Resource("icon")).unwrap();

    tray.add_menu_item("Show Keyboard", || {
        virtkeyboard::show_virtual_keyboard();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    tray.add_menu_item("Portrait", move || {
        togglescreen::toggle_display(DMDO_DEFAULT);
    })
    .unwrap();
    tray.add_menu_item("Landsacpe", move || {
        togglescreen::toggle_display(DMDO_90);
    })
    .unwrap();
    tray.add_menu_item("Portrait (fliped)", move || {
        togglescreen::toggle_display(DMDO_180);
    })
    .unwrap();
    tray.add_menu_item("Landsacpe (fliped)", move || {
        togglescreen::toggle_display(DMDO_270);
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    tray.add_menu_item("Quit", move || std::process::exit(0))
        .unwrap();

    hkm.event_loop();
}
