use js_sys::JsString;
use wasm_bindgen::prelude::*;
use log::{Level, debug};

const WORD_TEXT: &'static str = include_str!("word_list.txt");

#[wasm_bindgen]
pub fn wordle(
    known_1: &str,
    known_2: &str,
    known_3: &str,
    known_4: &str,
    known_5: &str,
    not_in_word: &str,
    in_word: &str,
) -> Vec<JsString> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // console_log::init_with_level(Level::Debug).unwrap();

    let known_1 = known_1.to_lowercase().chars().min();
    let known_2 = known_2.to_lowercase().chars().min();
    let known_3 = known_3.to_lowercase().chars().min();
    let known_4 = known_4.to_lowercase().chars().min();
    let known_5 = known_5.to_lowercase().chars().min();

    let not_in_word = not_in_word.chars().collect::<Vec<char>>();
    let in_word = in_word.chars().collect::<Vec<char>>();

    let mut res = Vec::new();

    let word_list = WORD_TEXT.lines();

    'word_loop: for word in word_list {
        let chars: Vec<char> = word.chars().collect();

        for c in not_in_word.iter() {
            if word.contains(*c) {
                continue 'word_loop;
            }
        }

        for c in in_word.iter() {
            if !word.contains(*c) {
                continue 'word_loop;
            }
        }

        if let Some(k) = known_1 {
            if chars[0] != k {
                continue;
            }
        }

        if let Some(k) = known_2 {
            if chars[1] != k {
                continue;
            }
        }

        if let Some(k) = known_3 {
            if chars[2] != k {
                continue;
            }
        }

        if let Some(k) = known_4 {
            if chars[3] != k {
                continue;
            }
        }

        if let Some(k) = known_5 {
            if chars[4] != k {
                continue;
            }
        }

        res.push(word.into());
    }

    return res;
}
