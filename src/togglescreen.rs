/*
*                                                                              *
*               ╔╗╔┌─┐┌┬┐┌─┐╔═╗┬  ┬┌─┐┬─┐┌─┐┬  ┌─┐┬ ┬                          *
*               ║║║│ │ │ └─┐║ ║└┐┌┘├┤ ├┬┘├┤ │  │ ││││                          *
*               ╝╚╝└─┘ ┴ └─┘╚═╝ └┘ └─┘┴└─└  ┴─┘└─┘└┴┘                          *
*                                                                              *
*               togglescreen.rs created 2025/02/13                             *
*               by Richard JUAN (contact@richard-juan.com)                     *
*                                                                              *
*               Copyright © 2025 Richard JUAN. All rights reserved             *
*                                                                              *
*/
use std::mem::MaybeUninit;

use windows::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsA, EnumDisplaySettingsA, DEVMODEA, DEVMODE_DISPLAY_ORIENTATION, DMDO_180,
    DMDO_270, DMDO_90, DMDO_DEFAULT, DM_DISPLAYORIENTATION, DM_PELSHEIGHT, DM_PELSWIDTH,
    ENUM_CURRENT_SETTINGS,
};

pub fn toggle_display(orientation: DEVMODE_DISPLAY_ORIENTATION) {
    let mut mode: MaybeUninit<DEVMODEA> = MaybeUninit::uninit();
    unsafe {
        EnumDisplaySettingsA(None, ENUM_CURRENT_SETTINGS, mode.as_mut_ptr());
    }
    let mut mode = unsafe { mode.assume_init() };

    // Ensure we're modifying the orientation field
    mode.dmFields |= DM_DISPLAYORIENTATION;

    let current_orientation = unsafe { mode.Anonymous1.Anonymous2.dmDisplayOrientation };
    // Set the desired orientation
    mode.Anonymous1.Anonymous2.dmDisplayOrientation = orientation;
    let mut swap: bool = false;
    // For 90° and 270°, swap width and height
    if (current_orientation == DMDO_DEFAULT || current_orientation == DMDO_180)
        && (orientation == DMDO_270 || orientation == DMDO_90)
    {
        swap = true;
    }
    if (current_orientation == DMDO_90 || current_orientation == DMDO_270)
        && (orientation == DMDO_180 || orientation == DMDO_DEFAULT)
    {
        swap = true;
    }
    if swap == true {
        let width = mode.dmPelsWidth;
        let height = mode.dmPelsHeight;
        mode.dmPelsWidth = height;
        mode.dmPelsHeight = width;

        // Indicate that we're changing width/height fields
        mode.dmFields |= DM_PELSWIDTH | DM_PELSHEIGHT;
    }

    unsafe {
        ChangeDisplaySettingsA(
            Some(&mode as *const _),
            windows::Win32::Graphics::Gdi::CDS_TYPE(0),
        )
    };
}
