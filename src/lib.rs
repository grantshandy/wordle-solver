use js_sys::JsString;
use wasm_bindgen::prelude::*;

const WORD_TEXT: &'static str = include_str!("word_list.txt");

#[wasm_bindgen]
pub fn wordle(
    known: Vec<JsValue>,
    not_in_word: &str,
    in_word: &str,
) -> Vec<JsString> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let not_in_word = not_in_word.chars().map(|x| { x.to_ascii_lowercase() }).collect::<Vec<char>>();
    let in_word = in_word.chars().map(|x| { x.to_ascii_lowercase() }).collect::<Vec<char>>();

    let mut res = Vec::new();

    // go through all words in the word list
    'word_loop: for word in WORD_TEXT.lines() {
        // skip word if word contains a char that's not supposed to be in there
        for c in not_in_word.iter() {
            if word.contains(*c) {
                continue 'word_loop;
            }
        }

        // skip word if word doesn't contain one of the chars it needs
        for c in in_word.iter() {
            if !word.contains(*c) {
                continue 'word_loop;
            }
        }

        // get a list of all the characters in the word
        let chars: Vec<char> = word.chars().collect();

        // go through all the known chars, continue if we don't match
        for (n, k) in known.iter().enumerate() {
            let first = match k.as_string() {
                Some(d) => match d.chars().min() {
                    Some(d) => d,
                    None => continue,
                },
                None => continue,
            }.to_ascii_lowercase();

            if chars[n] != first {
                continue 'word_loop;
            }
        }

        res.push(word.into());
    }

    return res;
}
