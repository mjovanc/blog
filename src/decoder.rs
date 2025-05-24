use minijinja::{Environment, context};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_decoder_page(env: &mut Environment, public_dir: &Path) -> std::io::Result<()> {
    env.add_template("decoder", include_str!("../templates/decoder.html"))
        .expect("Failed to load decoder template");

    let template = env
        .get_template("decoder")
        .expect("Decoder template not found");
    let rendered = template
        .render(context! {})
        .expect("Failed to render decoder page");

    let output_path = public_dir.join("decoder.html");
    let mut file = File::create(&output_path)?;
    file.write_all(rendered.as_bytes())?;
    println!("Generated: {}", output_path.display());

    Ok(())
}
