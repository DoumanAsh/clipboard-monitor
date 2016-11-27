//! KiriKiriZ Text Corrector

extern crate clipboard_win;
use clipboard_win::{set_clipboard};
use ::utils;

pub fn handler_clip_text(text: &String) {
    if !utils::is_jp(text) { return; }

    let text_len = text.len();

    let chars = text.chars().collect::<Vec<_>>();
    let mut result = String::with_capacity(text_len);

    result.push(chars[0]);
    for idx in 1..chars.len() {
        if chars[idx-1] != chars[idx] {
            result.push(chars[idx]);
        }
    }

    if text_len == result.len() {
        return;
    }

    if set_clipboard(&result).is_err() {
        println!("Hmph... failed to update clipboard");
    }
    else {
        println!("Repetitions are removed!");
    }
}
