use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_css(public_dir: &Path) -> std::io::Result<()> {
    let css_content = r#"
body {
    font-family: 'Courier New', monospace;
    background: #1a1a1a;
    color: #0f0;
    text-align: center;
    margin: 0;
    padding: 20px;
}
.container {
    max-width: 800px;
    margin: 50px auto;
    border: 2px solid #0f0;
    padding: 20px;
    box-shadow: 0 0 20px #0f0;
}
h1 {
    font-size: 2.5em;
    text-shadow: 0 0 10px #0f0;
}
select, input, button {
    padding: 10px;
    margin: 10px;
    font-size: 1em;
    background: #333;
    color: #0f0;
    border: 1px solid #0f0;
}
button:hover {
    background: #0f0;
    color: #1a1a1a;
}
#output {
    margin-top: 20px;
    font-weight: bold;
    background: #222;
    padding: 10px;
    border: 1px dashed #0f0;
}
.hint {
    font-style: italic;
    color: #0a0;
}
"#;
    let output_path = public_dir.join("decoder.css");
    let mut file = File::create(&output_path)?;
    file.write_all(css_content.as_bytes())?;
    println!("Generated: {}", output_path.display());
    Ok(())
}
