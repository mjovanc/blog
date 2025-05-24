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

pub fn generate_secret_message(title: &str) -> String {
    let base_message = format!("Secret: {} is totally a ninja coder!", title);
    caesar_encrypt(&base_message, 3) // Shift by 3 for fun
}
