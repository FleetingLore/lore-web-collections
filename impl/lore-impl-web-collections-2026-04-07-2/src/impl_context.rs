/// 解析以范畴为单位
/// `ImplContext` 是针对范畴而言的
/// 范畴中的所有行共享同一个 `ImplContext`
// 配置信息
pub struct ImplContext<'a> {
    pub title: &'a str,
    pub link_base: &'a str,
    pub css_url: &'a str,
}
