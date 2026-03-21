use regex::Regex;

fn main() {
    let text = "<script src=\"app.js\"></script>\n<link rel=\"stylesheet\" href='style.css'>";
    let html_re = Regex::new(r#"(?i)<(?:script[^>]+src|link[^>]+href)\s*=\s*['"]([^'"]+)['"]"#).unwrap();
    for cap in html_re.captures_iter(text) {
        println!("HTML Match: {}", cap.get(1).unwrap().as_str());
    }

    let text2 = "[link](doc.md)\n![img](pic.png)";
    let md_re = Regex::new(r#"\[[^\]]*\]\(([^)]+)\)"#).unwrap();
    for cap in md_re.captures_iter(text2) {
        println!("MD Match: {}", cap.get(1).unwrap().as_str());
    }
}
