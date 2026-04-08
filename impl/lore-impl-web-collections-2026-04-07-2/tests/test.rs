use lore_web_collections_core::LineType;
use lore_impl_web_collections_2026_04_07_2::lore_html::LoreHtml;
use lore_impl_web_collections_2026_04_07_2::parser::Parser;
use lore_impl_web_collections_2026_04_07_2::impl_context::ImplContext;

#[test]
fn test_models() {
    let impl_context = ImplContext {
        title: "Example Web Collections",
        link_base: "https://www.example.com/",
        css_url: "https://www.example.css",
    };
    let parser = Parser { impl_context: &impl_context };

    let ir_nodes = [
        LineType::Comment("example comment".to_string()),
        LineType::Placeholder,
        LineType::DomainTitle("example domain title".to_string()),
        LineType::LoreLink(
            "example lore link name".to_string(),
            "example lore link url".to_string()
        )
    ];

    let target: LoreHtml = parser.parse(&ir_nodes, &impl_context);
    let target: String = target.into();
    
    println!("{}\n", target);
}