use std::cmp::Ordering;

#[derive(Debug)]
pub enum Item {
    Singleton(usize),
    List(Vec<Item>),
}

pub fn parse_items(line: &str) -> Vec<Item> {
    let mut parents = Vec::new();
    let mut items = Vec::new();
    let mut digit = None;
    for b in line.bytes() {
        match b {
            b'[' => {
                parents.push(items);
                items = Vec::new();
            }
            b']' => {
                if let Some(d) = digit {
                    items.push(Item::Singleton(d));
                    digit = None;
                }
                let mut parent = parents.pop().unwrap();
                parent.push(Item::List(items));
                items = parent;
            }
            b',' => {
                if let Some(d) = digit {
                    items.push(Item::Singleton(d));
                    digit = None;
                }
            }
            c if digit.is_none() => digit = Some((c - b'0') as usize),
            c => digit = digit.map(|d| d * 10 + (c - b'0') as usize),
        };
    }

    items
}

pub fn compare_items(left: &Vec<Item>, right: &Vec<Item>) -> Ordering {
    for (l, r) in left.iter().zip(right) {
        let ordering = compare_item(l, r);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    left.len().cmp(&right.len())
}

fn compare_singular_item(left: usize, right: &Vec<Item>) -> Ordering {
    if right.is_empty() {
        return Ordering::Greater;
    }

    match compare_item(&Item::Singleton(left), &right[0]) {
        Ordering::Equal => Ordering::Less,
        c => c,
    }
}

fn compare_item(left: &Item, right: &Item) -> Ordering {
    match (left, right) {
        (Item::Singleton(l), Item::Singleton(r)) => l.cmp(r),
        (Item::List(l), Item::List(r)) => compare_items(l, r),
        (Item::Singleton(l), Item::List(r)) => compare_singular_item(*l, r),
        (Item::List(l), Item::Singleton(r)) => compare_singular_item(*r, l).reverse(),
    }
}
