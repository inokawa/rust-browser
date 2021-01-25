use std::collections::HashMap;

pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

enum NodeType {
    Element(ElementData),
    Text(String),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

pub fn elem(name: String, attr: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attr,
        }),
        children,
    }
}

pub fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        children: Vec::new(),
    }
}
