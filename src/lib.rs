use std::collections::hash_map::{HashMap, Entry};
use node_bindgen::derive::node_bindgen;

fn idx() -> i64 {
    static mut IDX: i64 = 0;
    let i: i64 = unsafe {
        // todo instead of reset the value we should panic
        if IDX == i64::MAX { IDX = 0 };
        IDX += 1;
        IDX
    };
    i - 1
}

#[derive(Debug)]
pub struct Finder {
    pub id: i64,
    pub children: HashMap<String, Finder>,
}

impl Default for Finder {
    fn default() -> Self {
        Self::new()
    }
}

#[node_bindgen]
impl Finder {
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            id: idx(),
            children: HashMap::new(),
        }
    }


    #[node_bindgen]
    pub fn display(&self) {
        println!("{:#?}", &self);
    }

    #[node_bindgen]
    pub fn create_path(&mut self, path: String) -> Vec<i64> {
        let mut ids = vec![];
        let last: &mut Finder = path.split('/').fold(self, |current_node, part| {
            ids.push(current_node.id);
            match current_node.children.entry(part.to_string()) {
                Entry::Vacant(slot) => slot.insert(Finder::new()),
                Entry::Occupied(slot) => slot.into_mut()
            }
        });
        ids.push(last.id);
        ids
    }

    #[node_bindgen]
    pub fn find(&self, path: String) -> Vec<(i64, bool, String)> {
        // todo get rid of this vector
        let mut node_path = vec![self];
        let mut res = vec![(self.id, false, String::new())];

        for part in path.split('/') {
            match node_path.last().unwrap().children.get(part) {
                Some(leaf) => {
                    node_path.push(leaf);
                    res.push((leaf.id, false, String::new()));
                },
                None => match node_path.last().unwrap().children.get(":") {
                    Some(leaf) => {
                        node_path.push(leaf);
                        res.push((leaf.id, true, String::from(part)));
                    },
                    None => {
                        res = vec![];
                        break;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_finder() {
        let finder = Finder::new();
        assert_eq!(finder.id, 0);
        assert_eq!(finder.children.len(), 0);
    }

}
