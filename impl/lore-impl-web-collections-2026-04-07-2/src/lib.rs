//! 这个文件的 `Parser` 用于将给定的 Lore 数据实例转化成 `Info`
//! 
//! Config 可以由 Lore.toml 提供

use lore_web_collections_core::LineType;
use serde::Deserialize;

/// 这里提供的 `Info` 是目标结果。
/// 
/// 如果为范畴提供 `From<File>` 实现，可以被一个 bin crate 用在处理单个文件。
pub struct Info {
    pub content: String,
}

/// 解析以范畴为单位
/// `Config` 是针对范畴而言的
/// 范畴中的所有行共享同一个 `Config`
// 配置信息
#[derive(Deserialize)]
pub struct Config {
    pub link_base: String,
    pub css_url: String,
    pub from_lore_path: String,
    pub to_html_path: String,
}

pub struct Parser {
    pub config: Config,
}

impl Parser {
    pub fn parse_ir_node(&self, ir_node: &LineType) -> String {
        match ir_node {
            LineType::Empty => {
                "".to_string()
            },
            LineType::UrlLink(name, url) => {
                format!("{} | {}", name, url)
            },
            LineType::LoreLink(name, lore) => {
                format!("{} = {}{}", name, self.config.link_base, lore)
            },
            LineType::Placeholder => {
                "<Placeholder>".to_string()
            },
            LineType::Comment(comment) => {
                format!("# {}", comment)
            },
            LineType::DomainTitle(title) => {
                format!("+ {}", title)
            },
            LineType::Atom(atom) => {
                atom.to_string()
            }
        }
    }

    pub fn parse(&self, ir_nodes: &[LineType]) -> Info {
        let body: String = ir_nodes
            .iter()
            .map(|node| self.parse_ir_node(node))
            .collect::<Vec<_>>()
            .join("\n");
        
        let content = format!("[test]\n==========\n{}\n==========", body);
        
        Info { content }
    }
}
