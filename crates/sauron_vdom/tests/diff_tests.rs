#![deny(warnings)]
use sauron_vdom::*;
use std::{collections::BTreeMap,
          iter::FromIterator};

#[test]
fn test_replace_node() {
    let old: Node<&'static str, (), ()> =
        Node::Element(Element { tag: "div",
                                ..Default::default() });
    let new = Node::Element(Element { tag: "span",
                                      ..Default::default() });

    let diff = diff(&old, &new);
    assert_eq!(diff,
               vec![Patch::Replace(0, &new)],
               "Should replace the first node");
}

#[test]
fn test_simple_diff() {
    let old: Node<&'static str, (), ()> =
        Node::Element(Element { tag: "div",
                                attrs:
                                    BTreeMap::from_iter(vec![("id",
                                                          "some-id".into()),
                                                         ("class",
                                                          "some-class".into())]),
                                ..Default::default() });

    let new = Node::Element(Element { tag: "div",
                                      attrs: BTreeMap::from_iter(vec![
        ("id", "some-id".into()),
        ("class", "some-class".into()),
    ]),
                                      ..Default::default() });

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![])
}

#[test]
fn test_class_changed() {
    let old: Node<&'static str, (), ()> =
        Node::Element(Element { tag: "div",
                                attrs:
                                    BTreeMap::from_iter(vec![("id",
                                                          "some-id".into()),
                                                         ("class",
                                                          "some-class".into())]),
                                ..Default::default() });

    let new = Node::Element(Element { tag: "div",
                                      attrs: BTreeMap::from_iter(vec![
        ("id", "some-id".into()),
        ("class", "some-class2".into()),
    ]),
                                      ..Default::default() });

    let diff = diff(&old, &new);
    let class2 = Value::Str("some-class2");
    assert_eq!(diff,
               vec![Patch::AddAttributes(0,
                                         BTreeMap::from_iter(vec![("class",
                                                                   &class2),]))])
}

#[test]
fn test_class_removed() {
    let old: Node<&'static str, (), ()> =
        Node::Element(Element { tag: "div",
                                attrs:
                                    BTreeMap::from_iter(vec![("id",
                                                          "some-id".into()),
                                                         ("class",
                                                          "some-class".into())]),
                                ..Default::default() });

    let new = Node::Element(Element { tag: "div",
                                      attrs: BTreeMap::from_iter(vec![(
        "id",
        "some-id".into(),
    )]),
                                      ..Default::default() });

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![Patch::RemoveAttributes(0, vec!["class"])])
}

#[test]
fn no_change_event() {
    let func = |_| println!("Clicked!");
    let cb: Callback<(), ()> = func.into();
    let old: Node<&'static str, (), ()> =
        Node::Element(Element { tag: "div",
                                events: BTreeMap::from_iter(vec![("click",
                                                              cb.clone())]),
                                attrs: BTreeMap::new(),
                                children: vec![],
                                namespace: None });

    let new = Node::Element(Element { tag: "div",
                                      events:
                                          BTreeMap::from_iter(vec![("click",
                                                                    cb)]),
                                      attrs: BTreeMap::new(),
                                      children: vec![],
                                      namespace: None });

    let diff = diff(&old, &new);
    assert_eq!(diff, vec![])
}

#[test]
fn add_event() {
    let func = |_| println!("Clicked!");
    let cb: Callback<(), ()> = func.into();

    let old: Node<&'static str, (), ()> =
        Node::Element(Element { tag: "div",
                                attrs: BTreeMap::new(),
                                events: BTreeMap::new(),
                                children: vec![],
                                namespace: None });

    let new = Node::Element(Element { tag: "div",
                                      events:
                                          BTreeMap::from_iter(vec![("click",
                                                                cb.clone())]),
                                      attrs: BTreeMap::new(),
                                      children: vec![],
                                      namespace: None });

    let diff = diff(&old, &new);
    assert_eq!(diff,
               vec![Patch::AddEventListener(0,
                                            BTreeMap::from_iter(vec![("click",
                                                                  &cb)]))])
}
