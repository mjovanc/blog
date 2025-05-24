use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_css_decoder(public_dir: &Path) -> std::io::Result<()> {
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

pub fn generate_css_main(public_dir: &Path) -> std::io::Result<()> {
    let css_content = r#"
@import url("https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;700&display=swap");
@import url("https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css");

/* Reset and Base Styles */
* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}

body {
    font-family: "Fira Code", monospace;
    background-color: #1a1a1a;
    color: #ffffff;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    line-height: 1.6;
    font-size: 16px;
}

/* Headings */
h1,
h2,
h3,
h4,
h5,
h6 {
    color: #ffffff;
    margin: 0.5em 0;
    font-weight: 700;
}

h1 {
    font-size: 2em;
}
h2 {
    font-size: 1.75em;
}
h3 {
    font-size: 1.5em;
}
h4 {
    font-size: 1.25em;
}
h5 {
    font-size: 1.1em;
}
h6 {
    font-size: 1em;
}

/* Text Elements */
p {
    margin: 1em 0;
}

strong {
    font-weight: 700;
    color: #ffffff;
}

em {
    font-style: italic;
    color: #e0e0e0;
}

del {
    color: #999999;
    text-decoration: line-through;
}

ins {
    color: #2e7d32;
    text-decoration: underline;
}

sub,
sup {
    font-size: 0.75em;
    line-height: 0;
    position: relative;
    vertical-align: baseline;
}

sup {
    top: -0.5em;
}
sub {
    bottom: -0.25em;
}

/* Links */
a {
    color: #2e7d32;
    text-decoration: none;
    transition: color 0.2s;
}

a:hover,
a:focus {
    color: #4caf50;
    text-decoration: underline;
}

a:focus {
    outline: 2px solid #2e7d32;
    outline-offset: 2px;
}

/* Lists */
ul,
ol {
    padding-left: 25px;
    margin: 1em 0;
}

ul {
    list-style: disc;
}

ol {
    list-style: decimal;
}

li {
    margin: 0.5em 0;
}

ul ul,
ol ol,
ul ol,
ol ul {
    margin-top: 0.5em;
    margin-bottom: 0.5em;
    padding-left: 20px;
}

/* Definition Lists */
dl {
    margin: 1em 0;
}

dt {
    font-weight: 700;
    color: #ffffff;
    margin-top: 0.5em;
}

dd {
    margin-left: 20px;
    color: #cccccc;
}

/* Tables */
table {
    width: 100%;
    border-collapse: collapse;
    margin: 1em 0;
    border: 1px solid #2e7d32;
}

th,
td {
    padding: 12px;
    border: 1px solid #2e7d32;
    text-align: left;
}

th {
    background-color: #2e7d32;
    color: #ffffff;
    font-weight: 700;
}

tr:nth-child(even) {
    background-color: #252525;
}

tr:nth-child(odd) {
    background-color: #1e1e1e;
}

/* Code */
code {
    font-family: "Fira Code", monospace;
    background-color: #252525;
    padding: 2px 6px;
    border-radius: 3px;
    color: #e0e0e0;
}

pre {
    background-color: #252525;
    padding: 15px;
    border-radius: 5px;
    overflow-x: auto;
    border: 1px solid #2e7d32;
    margin: 1em 0;
}

pre code {
    background-color: transparent;
    padding: 0;
    color: #ffffff;
}

/* Syntax Highlighting Placeholders */
.token.keyword {
    color: #2e7d32;
}
.token.string {
    color: #a5d6a7;
}
.token.comment {
    color: #999999;
}
.token.function {
    color: #4caf50;
}

/* Blockquotes */
blockquote {
    border-left: 4px solid #2e7d32;
    margin: 1em 0;
    padding: 0.5em 1em;
    background-color: #252525;
    color: #cccccc;
}

blockquote p {
    margin: 0;
}

/* Images and Figures */
img,
video,
iframe {
    max-width: 100%;
    height: auto;
    display: block;
    margin: 1em auto;
    border: 1px solid #2e7d32;
    border-radius: 5px;
}

figure {
    text-align: center;
    margin: 1em 0;
}

figcaption {
    color: #cccccc;
    font-size: 0.9em;
    margin-top: 0.5em;
}

/* Mathematical Symbols (KaTeX) */
.katex {
    font-family: "Fira Code", monospace;
    color: #ffffff;
    font-size: 1em;
}

/* Forms */
input,
textarea,
select,
button {
    font-family: "Fira Code", monospace;
    background-color: #252525;
    color: #ffffff;
    border: 1px solid #2e7d32;
    border-radius: 3px;
    padding: 8px;
    margin: 0.5em 0;
}

input:focus,
textarea:focus,
select:focus,
button:focus {
    outline: 2px solid #2e7d32;
    outline-offset: 2px;
}

button,
input[type="submit"],
input[type="button"] {
    background-color: #2e7d32;
    color: #ffffff;
    cursor: pointer;
    transition: background-color 0.2s;
}

button:hover,
input[type="submit"]:hover,
input[type="button"]:hover {
    background-color: #4caf50;
}

textarea {
    width: 100%;
    resize: vertical;
}

/* Navigation */
nav {
    margin: 1em 0;
}

nav a {
    margin: 0 10px;
    color: #2e7d32;
}

nav a:hover,
nav a:focus {
    color: #4caf50;
}

/* Header and Footer */
header {
    text-align: center;
    border-bottom: 1px solid #2e7d32;
    padding-bottom: 10px;
}

footer {
    text-align: center;
    color: #cccccc;
    margin-top: 2em;
    padding-top: 1em;
    border-top: 1px solid #2e7d32;
}

/* Horizontal Rules */
hr {
    border: none;
    border-top: 1px solid #2e7d32;
    margin: 2em 0;
}

/* Miscellaneous */
abbr {
    text-decoration: underline dotted;
    cursor: help;
}

kbd {
    background-color: #252525;
    border: 1px solid #2e7d32;
    border-radius: 3px;
    padding: 2px 4px;
    color: #ffffff;
}

mark {
    background-color: #2e7d32;
    color: #ffffff;
    padding: 2px 4px;
    border-radius: 3px;
}

progress {
    width: 100%;
    background-color: #252525;
    border: 1px solid #2e7d32;
    border-radius: 3px;
}

progress::-webkit-progress-bar {
    background-color: #252525;
}

progress::-webkit-progress-value {
    background-color: #2e7d32;
}

progress::-moz-progress-bar {
    background-color: #2e7d32;
}

/* Responsive Design */
@media (max-width: 600px) {
    body {
        padding: 10px;
        font-size: 14px;
    }

    h1 {
        font-size: 1.5em;
    }
    h2 {
        font-size: 1.25em;
    }
    h3 {
        font-size: 1.1em;
    }

    table {
        display: block;
        overflow-x: auto;
    }

    img,
    video,
    iframe {
        width: 100%;
    }
}

"#;
    let output_path = public_dir.join("styles.css");
    let mut file = File::create(&output_path)?;
    file.write_all(css_content.as_bytes())?;
    println!("Generated: {}", output_path.display());
    Ok(())
}
