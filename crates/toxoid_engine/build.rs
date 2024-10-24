use tera::{Tera, Context};
use std::fs;

fn main() {
    // Run build.rs if the template has changed
    println!("cargo:rerun-if-changed=wit/template/world.wit");
    
    // Initialize Tera with the path to your templates
    let tera = Tera::new("wit/template/*").unwrap();

    // Create a context and add variables if needed
    let mut context = Context::new();
    context.insert("wasm", &false);

    // Render the template
    let rendered = tera.render("world.wit", &context).unwrap();

    // Write the rendered content to a file
    fs::write("wit/world.wit", rendered).unwrap();
}