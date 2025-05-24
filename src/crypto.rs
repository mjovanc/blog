use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn vigenere_encrypt(text: &str, key: &str) -> String {
    let key_chars: Vec<char> = key.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    if key_chars.is_empty() {
        return text.to_string();
    }
    let mut key_index = 0;
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let key_char = key_chars[key_index % key_chars.len()];
                let key_base = if key_char.is_ascii_uppercase() {
                    b'A'
                } else {
                    b'a'
                };
                let shift = (key_char as u8 - key_base) % 26;
                key_index += 1;
                let offset = (c as u8 - base + shift) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn generate_key(title: &str, date: &str) -> String {
    let mut hasher = DefaultHasher::new();
    (title.to_lowercase() + date).hash(&mut hasher);
    let hash = hasher.finish();
    let key = format!("key{}", hash % 10000); // Simple key for readability
    key.chars().filter(|c| c.is_ascii_alphabetic()).collect()
}

pub fn caesar_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = (c as u8 - base + shift) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}
