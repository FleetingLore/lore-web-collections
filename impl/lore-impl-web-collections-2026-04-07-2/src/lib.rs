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

/// 每个转为 html 格式的 lore 文件
pub struct LoreHtml {
    /// 目标文件的标题
    pub title: String,

    /// 样式表字面值
    pub css_path: String,

    /// 内容字面值
    pub html_content: String,
}

impl LoreHtml {
    pub fn new(
        title: String,
        css_path: String,
        html_content: String
    ) -> Self {
        LoreHtml { title, css_path, html_content }
    }
}

/// 导出为字符串
impl Into<Info> for LoreHtml {
    fn into(self) -> Info {
        let content: String = format!(
            r#"<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<link rel="stylesheet" href="{}">
</head>
<body><main>
{}
</main></body>
</html>"#,
            self.title,
            self.css_path,
            self.html_content,
        );

        Info { content }
    }
}