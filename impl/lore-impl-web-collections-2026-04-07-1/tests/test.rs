use lore_web_collections_core::LineType;
use lore_impl_web_collections_2026_04_07_1::Parser;
use lore_impl_web_collections_2026_04_07_1::Config;

#[test]
fn test_models() {
    let config = Config { link_base: "https://www.example.com/".to_string() };
    let parser = Parser { config };

    let ir_nodes = [
        LineType::Comment("example comment".to_string()),
        LineType::Placeholder,
        LineType::DomainTitle("example domain title".to_string()),
        LineType::LoreLink("example lore link name".to_string(), "example lore link url".to_string())
    ];

    let target = parser.parse(&ir_nodes);
    
    println!("{}\n", target.content);
}