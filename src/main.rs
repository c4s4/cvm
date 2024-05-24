use anyhow::Result;
use cvm::CVM;
use unicode_segmentation::UnicodeSegmentation;

/// Count Hamlet words
fn main() {
    // run function
    match run() {
        Ok(_) => println!("OK"),
        Err(e) => {
            eprintln!("ERROR {:#}", e);
            std::process::exit(1);
        }
    }
}

/// Describe function here
fn run() -> Result<()> {
    let text = std::fs::read_to_string("tests/hamlet.txt")?;
    let words: Vec<&str> = text.unicode_words().collect();
    println!("Words: {}", words.len());
    let mut cvm = CVM::new(500);
    for word in words {
        cvm.add(word.to_string().to_lowercase());
    };
    println!("Count: {}", cvm.count());
    Ok(())
}
