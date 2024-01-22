use std::time::Instant;

use crate::{
    constant::{
        ABOUT_TEXT, ANIME_TIME_160, DIALOG_BOTTOM_TOP, DIALOG_CANCEL_TEXT, DIALOG_CONFIRM_TEXT,
        DIALOG_HEIGHT, DIALOG_WIDTH, INVALID_EAT_PANCAKE, SCREEN_HEIGHT, SCREEN_WIDTH,
    },
    utils::ease_out_expo,
    vita2d::{
        is_button, rgba, vita2d_ctrl_peek_positive, vita2d_draw_rect, vita2d_draw_text,
        vita2d_draw_texture, vita2d_drawing, vita2d_get_screenshot, vita2d_line,
        vita2d_load_png_buf, vita2d_present, vita2d_text_height, vita2d_text_width, SceCtrlButtons,
    },
};

pub struct UIDialog;

impl UIDialog {
    fn draw(text: &str, is_qrcode: bool, is_about: bool) -> bool {
        // qrcode
        let qr_code = if is_qrcode {
            let buf = qrcode_generator::to_png_to_vec(text, qrcode_generator::QrCodeEcc::Low, 150)
                .unwrap();
            Some(vita2d_load_png_buf(&buf))
        } else {
            None
        };
        // get screenshot
        let screenshot = vita2d_get_screenshot();
        let mut open = true;
        let mut toggle_at = Instant::now();
        let mut result: Option<bool> = None;
        'dialog_loop: loop {
            // state
            if open && Instant::now() - toggle_at > ANIME_TIME_160 {
                let buttons = vita2d_ctrl_peek_positive();
                result = match buttons {
                    _ if is_button(buttons, SceCtrlButtons::SceCtrlCircle)
                        && is_button(buttons, SceCtrlButtons::SceCtrlCross) =>
                    {
                        None
                    }
                    _ if is_button(buttons, SceCtrlButtons::SceCtrlCircle) => Some(true),
                    _ if is_button(buttons, SceCtrlButtons::SceCtrlCross) => Some(false),
                    _ => None,
                };
                if result.is_some() {
                    open = false;
                    toggle_at = Instant::now();
                }
            }
            if !open && Instant::now() - toggle_at > ANIME_TIME_160 {
                break 'dialog_loop;
            }

            let (start, end) = if open {
                (
                    SCREEN_HEIGHT as f32,
                    ((SCREEN_HEIGHT - DIALOG_HEIGHT) / 2) as f32,
                )
            } else {
                (
                    ((SCREEN_HEIGHT - DIALOG_HEIGHT) / 2) as f32,
                    SCREEN_HEIGHT as f32,
                )
            };
            let top = ease_out_expo(
                Instant::now().duration_since(toggle_at),
                ANIME_TIME_160,
                start,
                end,
            );

            let left = ((SCREEN_WIDTH - DIALOG_WIDTH) / 2) as f32;
            // draw
            vita2d_drawing();
            // screen background
            vita2d_draw_texture(&screenshot, 0.0, 0.0);
            // dialog bg
            vita2d_draw_rect(
                left,
                top,
                DIALOG_WIDTH as f32,
                DIALOG_HEIGHT as f32,
                rgba(0x44, 0x44, 0x44, 0xff),
            );
            // line
            vita2d_line(
                left,
                top + DIALOG_BOTTOM_TOP as f32,
                SCREEN_WIDTH as f32 - left,
                top + DIALOG_BOTTOM_TOP as f32,
                rgba(0x18, 0x18, 0x18, 0xff),
            );
            vita2d_line(
                (SCREEN_WIDTH / 2) as f32,
                top + DIALOG_BOTTOM_TOP as f32,
                (SCREEN_WIDTH / 2) as f32,
                top + DIALOG_HEIGHT as f32,
                rgba(0x18, 0x18, 0x18, 0xff),
            );
            // buttons
            vita2d_draw_text(
                left as i32 + (DIALOG_WIDTH / 2 - vita2d_text_width(1.0, DIALOG_CANCEL_TEXT)) / 2,
                top as i32 + DIALOG_BOTTOM_TOP + 38
                    - (40 - vita2d_text_height(1.0, DIALOG_CANCEL_TEXT)) / 2,
                rgba(0xff, 0xff, 0xff, 0xff),
                1.0,
                DIALOG_CANCEL_TEXT,
            );
            vita2d_draw_text(
                SCREEN_WIDTH / 2
                    + (DIALOG_WIDTH / 2 - vita2d_text_width(1.0, DIALOG_CONFIRM_TEXT)) / 2,
                top as i32 + DIALOG_BOTTOM_TOP + 38
                    - (40 - vita2d_text_height(1.0, DIALOG_CONFIRM_TEXT)) / 2,
                rgba(0xff, 0xff, 0xff, 0xff),
                1.0,
                DIALOG_CONFIRM_TEXT,
            );
            if let Some(qr_code) = &qr_code {
                let text = if is_about {
                    ABOUT_TEXT
                } else {
                    INVALID_EAT_PANCAKE
                };
                vita2d_draw_text(
                    left as i32 + (DIALOG_WIDTH - vita2d_text_width(1.0, text)) / 2,
                    top as i32 + 30,
                    rgba(0xff, 0xff, 0xff, 0xff),
                    1.0,
                    text,
                );
                vita2d_draw_texture(
                    qr_code,
                    left + ((DIALOG_WIDTH - 150) / 2) as f32,
                    top + 50.0,
                );
            } else {
                // text
                vita2d_draw_text(
                    left as i32 + (DIALOG_WIDTH - vita2d_text_width(1.0, text)) / 2,
                    top as i32 + DIALOG_HEIGHT / 3,
                    rgba(0xff, 0xff, 0xff, 0xff),
                    1.0,
                    text,
                );
            }
            vita2d_present();
        }

        result.unwrap_or(false)
    }

    pub fn present(text: &str) -> bool {
        UIDialog::draw(text, false, false)
    }

    pub fn present_qrcode(text: &str) -> bool {
        UIDialog::draw(text, true, false)
    }

    pub fn present_about(text: &str) -> bool {
        UIDialog::draw(text, true, true)
    }
}
