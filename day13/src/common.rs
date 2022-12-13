use std::cmp::Ordering;
use std::str::Bytes;

enum Item<'a> {
    Singleton(usize),
    List(Items<'a>),
}

impl From<Item<'_>> for String {
    fn from(item: Item) -> Self {
        match item {
            Item::Singleton(v) => format!("{v}"),
            Item::List(items) => {
                format!("[{}]", String::from(items))
            }
        }
    }
}

struct Items<'a> {
    bytes: Option<&'a mut Bytes<'a>>,
}

impl From<Items<'_>> for String {
    fn from(items: Items) -> Self {
        let mut s = String::new();
        for (i, item) in items.enumerate() {
            if i != 0 {
                s.push(',')
            }
            s.push_str(&String::from(item));
        }
        s
    }
}

impl<'a> Items<'a> {
    fn new(bytes: &'a mut Bytes<'a>) -> Self {
        Self { bytes: Some(bytes) }
    }
}

impl<'a> Iterator for Items<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Item<'a>> {
        let bytes = &mut **self.bytes.as_mut()?;
        let mut num = None;
        loop {
            match bytes.next()? {
                b'[' => {
                    return Some(Item::List(Items {
                        // SAFETY: the `bytes` reference points at stack memory in `compare_items`
                        // so the pointer stays valid within that stack frame, which the items
                        // iterator never escapes from.
                        bytes: Some(unsafe { &mut *(bytes as *mut Bytes) }),
                    }));
                }
                b']' => {
                    self.bytes.take();
                    return num.map(Item::Singleton);
                }
                b',' => {
                    if let Some(num) = num {
                        return Some(Item::Singleton(num));
                    }
                }
                c if num.is_none() => num = Some((c - b'0') as usize),
                c => num = num.map(|d| d * 10 + (c - b'0') as usize),
            };
        }
    }
}

pub fn compare_items(left: &str, right: &str) -> Ordering {
    compare_sequence(
        Items::new(&mut left.bytes()),
        Items::new(&mut right.bytes()),
    )
}

fn compare_sequence<'a>(
    mut left: impl Iterator<Item = Item<'a>>,
    mut right: impl Iterator<Item = Item<'a>>,
) -> Ordering {
    loop {
        match (left.next(), right.next()) {
            (Some(l), Some(r)) => {
                let ordering = compare_item(l, r);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            _ => return Ordering::Equal,
        }
    }
}

macro_rules! singleton {
    ($i: ident) => {
        std::iter::once(Item::Singleton($i))
    };
}

fn compare_item<'a>(left: Item<'a>, right: Item<'a>) -> Ordering {
    match (left, right) {
        (Item::Singleton(l), Item::Singleton(r)) => l.cmp(&r),
        (Item::List(l), Item::List(r)) => compare_sequence(l, r),
        (Item::Singleton(l), Item::List(r)) => compare_sequence(singleton![l], r),
        (Item::List(l), Item::Singleton(r)) => compare_sequence(l, singleton![r]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "[[6,[[3,10],[],[],2,10],[[6,8,4,2]]],[]]";
        let output: String = Items::new(&mut input.bytes()).into();

        assert_eq!(input, output);
    }
}
