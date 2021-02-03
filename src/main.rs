use rust_browser::{css, html, style};

fn main() {
    let html_source: String = "foo".to_string();
    let css_source: String = "bar".to_string();
    let root_node = html::parse(html_source);
    let stylesheet = css::parse(css_source);
    let style_root = style::style_tree(&root_node, &stylesheet);
}
