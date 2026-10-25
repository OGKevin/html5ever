use markup5ever_rcdom::{RcDom, SerializableHandle};
use xml5ever::driver;
use xml5ever::serialize;
use xml5ever::tendril::TendrilSink;
use xml5ever::tree_builder::XmlTreeBuilderOpts;

#[test]
fn el_ns_serialize() {
    assert_eq_serialization(
        "<a:title xmlns:a=\"http://www.foo.org/\" value=\"test\">Test</a:title>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one("<a:title xmlns:a=\"http://www.foo.org/\" value=\"test\">Test</title>".as_bytes()),
    );
}

#[test]
fn nested_ns_serialize() {
    assert_eq_serialization("<a:x xmlns:a=\"http://www.foo.org/\" xmlns:b=\"http://www.bar.org/\" value=\"test\"><b:y/></a:x>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one("<a:x xmlns:a=\"http://www.foo.org/\" xmlns:b=\"http://www.bar.org/\" value=\"test\"><b:y/></a:x>".as_bytes()));
}

#[test]
fn def_ns_serialize() {
    assert_eq_serialization(
        "<table xmlns=\"html4\"><td></td></table>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one("<table xmlns=\"html4\"><td></td></table>".as_bytes()),
    );
}

#[test]
fn undefine_ns_serialize() {
    assert_eq_serialization(
        "<a:x xmlns:a=\"http://www.foo.org\"><a:y xmlns:a=\"\"><a:z/></a:y</a:x>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one(
                "<a:x xmlns:a=\"http://www.foo.org\"><a:y xmlns:a=\"\"><a:z/></a:y</a:x>"
                    .as_bytes(),
            ),
    );
}

#[test]
fn redefine_default_ns_serialize() {
    assert_eq_serialization(
        "<x xmlns=\"http://www.foo.org\"><y xmlns=\"\"><z/></y</x>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one("<x xmlns=\"http://www.foo.org\"><y xmlns=\"\"><z/></y</x>".as_bytes()),
    );
}

#[test]
fn attr_serialize() {
    assert_serialization(
        "<title value=\"test\">Test</title>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one("<title value='test'>Test".as_bytes()),
    );
}

#[test]
fn from_utf8() {
    assert_serialization(
        "<title>Test</title>",
        driver::parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .one("<title>Test".as_bytes()),
    );
}

fn document_element_names(dom: &RcDom) -> Vec<String> {
    dom.document
        .children
        .borrow()
        .iter()
        .filter_map(|node| match &node.data {
            markup5ever_rcdom::NodeData::Element { name, .. } => Some(name.local.to_string()),
            _ => None,
        })
        .collect()
}

fn parse_fragment(input: &str) -> RcDom {
    let opts = driver::XmlParseOpts {
        tree_builder: XmlTreeBuilderOpts { extra_roots: true },
        ..Default::default()
    };
    driver::parse_document(RcDom::default(), opts)
        .from_utf8()
        .one(input.as_bytes())
}

#[test]
fn extra_root_after_closed_element_is_kept() {
    assert_eq!(
        document_element_names(&parse_fragment("<h1></h1><p></p>")),
        ["h1", "p"]
    );
}

#[test]
fn extra_root_after_empty_element_is_kept() {
    assert_eq!(
        document_element_names(&parse_fragment("<br/><p></p>")),
        ["br", "p"]
    );
}

#[test]
fn extra_root_after_script_element_is_kept() {
    assert_eq!(
        document_element_names(&parse_fragment("<script></script><p></p>")),
        ["script", "p"]
    );
}

fn document_children(dom: &RcDom) -> Vec<String> {
    dom.document
        .children
        .borrow()
        .iter()
        .filter_map(|node| match &node.data {
            markup5ever_rcdom::NodeData::Text { contents } => {
                Some(format!("text:{}", contents.borrow()))
            },
            markup5ever_rcdom::NodeData::Element { name, .. } => {
                Some(format!("element:{}", name.local))
            },
            _ => None,
        })
        .collect()
}

#[test]
fn fragment_keeps_text_around_top_level_elements() {
    assert_eq!(
        document_children(&parse_fragment("definition <b>word</b> explanation")),
        ["text:definition ", "element:b", "text: explanation"]
    );
}

#[test]
fn fragment_round_trips_text_around_top_level_elements() {
    let mut serialized = Vec::new();
    let document: SerializableHandle = parse_fragment("definition <b>word</b> explanation")
        .document
        .clone()
        .into();
    serialize::serialize(&mut serialized, &document, Default::default()).unwrap();
    assert_eq!(
        String::from_utf8(serialized).unwrap(),
        "definition <b>word</b> explanation"
    );
}

#[test]
fn fragment_keeps_whitespace_between_top_level_elements() {
    assert_eq!(
        document_children(&parse_fragment("a <b>x</b> c")),
        ["text:a ", "element:b", "text: c"]
    );
    assert_eq!(
        document_children(&parse_fragment("<b>x</b> <b>y</b>")),
        ["element:b", "text: ", "element:b"]
    );
}

#[test]
fn document_mode_drops_text_outside_the_root() {
    let dom = driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one("definition <b>word</b> explanation".as_bytes());
    assert_eq!(document_children(&dom), ["element:b"]);
}

fn assert_eq_serialization(text: &'static str, dom: RcDom) {
    let mut serialized = Vec::new();
    let document: SerializableHandle = dom.document.clone().into();
    serialize::serialize(&mut serialized, &document, Default::default()).unwrap();

    let dom_from_text = driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one(text.as_bytes());

    let mut reserialized = Vec::new();
    let document: SerializableHandle = dom_from_text.document.clone().into();
    serialize::serialize(&mut reserialized, &document, Default::default()).unwrap();

    assert_eq!(
        String::from_utf8(serialized).unwrap(),
        String::from_utf8(reserialized).unwrap()
    );
}

fn assert_serialization(text: &'static str, dom: RcDom) {
    let mut serialized = Vec::new();
    let document: SerializableHandle = dom.document.clone().into();
    serialize::serialize(&mut serialized, &document, Default::default()).unwrap();
    assert_eq!(String::from_utf8(serialized).unwrap(), text);
}
