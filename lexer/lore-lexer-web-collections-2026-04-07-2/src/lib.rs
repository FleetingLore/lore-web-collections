use lore_web_collections_core::LineType;

pub struct Parser;

impl Parser {
    pub fn parse_line(raw_line: &str) -> LineType {
        let content = raw_line;
    
        let line = if content.is_empty() {
            LineType::Empty
        } else if content.starts_with("#[url] ") {
            let content = content.strip_prefix("#[url] ").unwrap();
            let (name, url) = content.split_once(" = ").unwrap();
            let name = name.to_string();
            let url = url.to_string();
            LineType::UrlLink(
                name,
                url
            )
        } else if content.starts_with("#[lore] ") {
            let content = content.strip_prefix("#[lore] ").unwrap();
            let (name, category) = content.split_once(" = ").unwrap();
            let name = name.to_string();
            let category = category.to_string();
            LineType::LoreLink(
                name,
                category,
            )
        } else if content.starts_with("+ ") {
            let category = content
                .strip_prefix("+ ")
                .unwrap()
                .to_string();
            LineType::DomainTitle(
                category
            )
        } else if content.starts_with("# ") {
            let comment = content
                .strip_prefix("# ")
                .unwrap()
                .to_string();
            LineType::Comment(
                comment
            )
        } else if content == "---" {
            LineType::Placeholder
        } else {
            LineType::Atom(
                content.to_string()
            )
        };
    
        line
    }
}