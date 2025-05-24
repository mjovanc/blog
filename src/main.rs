use minijinja::{Environment, context};
use pulldown_cmark::{Parser, html};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
mod crypto;
mod decoder;
mod easter_eggs;
mod style;

fn main() -> std::io::Result<()> {
    // Initialize minijinja environment
    let mut env = Environment::new();
    env.add_template("base", include_str!("../templates/base.html"))
        .expect("Failed to load base template");
    env.add_template("index", include_str!("../templates/index.html"))
        .expect("Failed to load index template");

    // Create output directory
    let public_dir = Path::new("public");
    if public_dir.exists() {
        fs::remove_dir_all(public_dir)?;
    }
    fs::create_dir_all(public_dir)?;

    // Read content directory
    let content_dir = Path::new("content");
    if !content_dir.exists() {
        fs::create_dir_all(content_dir)?;
        println!("Created content directory. Add your Markdown files there.");
        return Ok(());
    }

    // Collect posts for sorting
    let mut posts = BTreeMap::new();
    for entry in fs::read_dir(content_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path)?;
            let mut parser = Parser::new(&content);
            let mut html_content = String::new();
            html::push_html(&mut html_content, &mut parser);

            // Extract title from first line
            let title = content
                .lines()
                .next()
                .unwrap_or("Untitled")
                .trim_start_matches('#')
                .trim();

            // Extract date from filename (YYYY-MM-DD-title.md)
            let filename = path.file_stem().unwrap().to_string_lossy();
            let date = filename
                .splitn(2, '-')
                .next()
                .unwrap_or("1970-01-01")
                .to_string();
            let output_filename = filename
                .replace(&date, "")
                .trim_start_matches('-')
                .to_owned()
                + ".html";

            // Generate secret message and key
            let use_vigenere = rand::random::<bool>(); // Randomly choose cipher
            let secret_message = easter_eggs::get_random_message(title);
            let key = crypto::generate_key(title, &date);
            let cipher_type = if use_vigenere { "Vigenère" } else { "Caesar" };
            let encoded_message = if use_vigenere {
                crypto::vigenere_encrypt(&secret_message, &key)
            } else {
                crypto::caesar_encrypt(&secret_message, 3)
            };

            // Render post with secret message and cipher info
            let template = env.get_template("base").expect("Base template not found");
            let rendered = template
                .render(context! {
                    title => title,
                    content => html_content,
                    secret_message => encoded_message,
                    cipher_type => cipher_type,
                    cipher_key => if use_vigenere { Some(key) } else { None }
                })
                .expect("Failed to render post");

            // Write post to public directory
            let output_path = public_dir.join(&output_filename);
            let mut file = File::create(&output_path)?;
            file.write_all(rendered.as_bytes())?;
            println!("Generated: {}", output_path.display());

            // Store post info for index
            posts.insert(date, (title.to_string(), output_filename));
        }
    }

    // Generate index page (latest posts first)
    let template = env.get_template("index").expect("Index template not found");
    let posts: Vec<_> = posts
        .into_iter()
        .rev() // Reverse to get latest first
        .map(|(date, (title, filename))| {
            context! {
                date => date,
                title => title,
                url => filename
            }
        })
        .collect();
    let rendered_index = template
        .render(context! { posts => posts })
        .expect("Failed to render index");
    let index_path = public_dir.join("index.html");
    let mut index_file = File::create(&index_path)?;
    index_file.write_all(rendered_index.as_bytes())?;
    println!("Generated: {}", index_path.display());

    // Generate decoder page
    decoder::generate_decoder_page(&mut env, public_dir)?;

    // Generate CSS
    style::generate_css(public_dir)?;

    // Copy static assets
    let static_dir = Path::new("static");
    if static_dir.exists() {
        for entry in fs::read_dir(static_dir)? {
            let entry = entry?;
            let dest_path = public_dir.join(entry.file_name());
            fs::copy(entry.path(), dest_path)?;
        }
    }

    Ok(())
}
