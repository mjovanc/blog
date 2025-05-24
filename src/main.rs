use minijinja::{Environment, context};
use pulldown_cmark::{Parser, html};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // Initialize minijinja environment
    let mut env = Environment::new();
    env.add_template("base", include_str!("../templates/base.html"))
        .expect("Failed to load template");

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

    // Process each Markdown file
    for entry in fs::read_dir(content_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path)?;
            let mut parser = Parser::new(&content);
            let mut html_content = String::new();
            html::push_html(&mut html_content, &mut parser);

            // Extract title from first line (assuming it's a Markdown header)
            let title = content
                .lines()
                .next()
                .unwrap_or("Untitled")
                .trim_start_matches('#')
                .trim();

            // Render with template
            let template = env.get_template("base").expect("Template not found");
            let rendered = template
                .render(context! {
                    title => title,
                    content => html_content
                })
                .expect("Failed to render template");

            // Write to public directory
            let output_filename =
                path.file_stem().unwrap().to_string_lossy().into_owned() + ".html";
            let output_path = public_dir.join(output_filename);
            let mut file = File::create(&output_path)?;
            file.write_all(rendered.as_bytes())?;
            println!("Generated: {}", output_path.display());
        }
    }

    // Copy static assets (if any)
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
