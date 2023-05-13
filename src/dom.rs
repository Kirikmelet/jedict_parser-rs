use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DOMDocument {
    pub depth: i32,
    pub version: i32,
    pub encoding: String,
    pub entities: EntityMapType,
}

use crate::{elements::JElementDataVariant, entities::EntityMapType};

pub type DictDOMWeakRef = Weak<RefCell<DictDOMItem>>;
pub type DictDOMRef = Rc<RefCell<DictDOMItem>>;
pub type DictDOMQueryFn = fn(&&DictDOMItem) -> bool;

#[derive(Debug, Clone, Default)]
pub struct DictDOMItem {
    pub object: JElementDataVariant,
    pub data: String,
    pub attributes: HashMap<String, String>,
    pub parent: Option<DictDOMWeakRef>,
    pub children: Vec<DictDOMRef>,
    pub id: Option<String>,
}

impl DictDOMItem {
    pub fn add_child(&mut self, child: DictDOMRef) {
        self.children.push(child);
    }
    pub fn get_parent(&self) -> Option<DictDOMRef> {
        match &self.parent {
            Some(x) => Weak::upgrade(x),
            None => None,
        }
    }
    pub fn query_children(&self, query: DictDOMQueryFn) -> Vec<DictDOMItem> {
        /* SEE NOTE ../notes/dom.md#Query Child */
        let mut results: Vec<DictDOMItem> = Vec::new();
        if query(&self) {
            results.push(self.clone());
        }
        for i in &self.children {
            results.extend(i.as_ref().borrow().query_children(query));
        }
        results
    }
    pub fn query_child(&self, query: DictDOMQueryFn) -> Box<Option<DictDOMItem>> {
        let query_results = self.query_children(query);
        Box::new(query_results.first().cloned())
    }
    pub fn new_empty_ref() -> DictDOMRef {
        Rc::new(RefCell::new(Self::default()))
    }
}

pub struct DictDomBuilder(DictDOMItem);

impl DictDomBuilder {
    pub fn create() -> Self {
        Self(DictDOMItem::default())
    }
    pub fn object(self, object: JElementDataVariant) -> Self {
        Self(DictDOMItem { object, ..self.0 })
    }
    pub fn parent(self, parent: &DictDOMRef) -> Self {
        Self(DictDOMItem {
            parent: Some(Rc::downgrade(parent)),
            ..self.0
        })
    }
    pub fn children(self, children: Vec<DictDOMRef>) -> Self {
        Self(DictDOMItem { children, ..self.0 })
    }
    pub fn id(self, id: String) -> Self {
        Self(DictDOMItem {
            id: Some(id),
            ..self.0
        })
    }
    pub fn data(self, data: String) -> Self {
        Self(DictDOMItem { data, ..self.0 })
    }
    pub fn attributes(self, attributes: HashMap<String, String>) -> Self {
        Self(DictDOMItem {
            attributes,
            ..self.0
        })
    }
    pub fn build(self) -> DictDOMRef {
        let item = Rc::new(RefCell::new(self.0));
        match &item.as_ref().borrow_mut().parent {
            Some(x) => {
                let parent = Weak::upgrade(x).expect("Should be a parent!");
                parent.as_ref().borrow_mut().add_child(Rc::clone(&item));
            }
            None => {}
        }
        item
    }
}
