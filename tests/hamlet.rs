use anyhow::Result;
use cvm;
use unicode_segmentation::UnicodeSegmentation;

const ERROR_MAX: f64 = 0.05;

#[test]
fn test() {
    match count() {
        Ok(count) => {
            let error = (count.0 as f64 - count.1 as f64).abs() / count.0 as f64;
            assert!(error < ERROR_MAX, "Error: {:.2} > {}", error, ERROR_MAX);
        }
        Err(e) => {
            panic!("ERROR {:#}", e);
        }
    }
}

fn count() -> Result<(usize, usize)> {
    let text = std::fs::read_to_string("tests/hamlet.txt")?;
    let words: Vec<&str> = text.unicode_words().collect();
    let mut cvm = cvm::CVM::new(100);
    let mut distinct = std::collections::HashSet::new();
    for word in words {
        cvm.add(word.to_string().to_lowercase());
        distinct.insert(word.to_string().to_lowercase());
    }
    Ok((distinct.len(), cvm.count()))
}
