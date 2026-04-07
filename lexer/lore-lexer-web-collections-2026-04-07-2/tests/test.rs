use std::fs;
use lore_lexer_web_collections_2026_04_07_2::Parser;
use lore_web_collections_core::LineType;

#[test]
fn test_parser() {
    let raw = fs::read_to_string("./examples/examples.lore").unwrap();
        
    let lines = raw.lines();
    
    for (n, line) in lines.enumerate() {
        let raw_line = line.trim().to_string();
    
        let line = Parser::parse_line(&raw_line);
    
        display_line(n, line);
    }
}

fn display_line(n: usize, line: LineType) {
    let content = match line {
        LineType::Empty => "<Empty>".to_string(),
        LineType::Atom(atom) => format!("atom {}", atom),
        LineType::UrlLink(name, url) => format!("#[url] {} = {}", name, url),
        LineType::LoreLink(name, lore) => format!("#[lore] {} = {}", name, lore),
        LineType::Placeholder => "<Placeholder>".to_string(),
        LineType::Comment(comment) => format!("# {}", comment),
        LineType::DomainTitle(title) => format!("+ {}", title),
    };

    println!("[{}] {}", n, content);
}