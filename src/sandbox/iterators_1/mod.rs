#![allow(unused_doc_comments)]
use std::iter::FromIterator;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn shoes_into_iter(shoes: MyVec<Vec<Shoe>>, shoe_size: u32) -> MyVec<Vec<Shoe>> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[derive(PartialEq, Debug, Clone)]
pub struct MyVec<T>(T);

struct MyVecIter<'a, T> {
    vector: &'a T,
    cur: usize,
}

impl MyVec<Vec<Shoe>> {
    pub fn new() -> Self {
        MyVec(Vec::<Shoe>::new())
    }

    pub fn into_iter(&self) -> MyVecIter<'_, Vec<Shoe>> {
        MyVecIter::<Vec<Shoe>> {
            vector: &self.0,
            cur: 0,
        }
    }

    fn add(&mut self, elem: Shoe) {
        self.0.push(elem);
    }
}

impl<'a> Iterator for MyVecIter<'a, Vec<Shoe>> {
    type Item = &'a Shoe;
    fn next(&mut self) -> Option<Self::Item> {
        let inner_vec = self.vector;
        if self.cur < inner_vec.len() {
            self.cur += 1;

            println!("cur: {:?}", self.cur);
            return Some(&inner_vec[self.cur - 1]);
        } else {
            return None;
        }
    }
}

// Implemented to call `filter`
impl<'a> FromIterator<&'a Shoe> for MyVec<Vec<Shoe>> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a Shoe>,
    {
        let mut c = MyVec::<Vec<Shoe>>::new();

        for i in iter {
            c.add(i.clone())
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size_wrapped() {
        let shoes = MyVec(vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]);

        let in_my_size = shoes_into_iter(shoes, 10);

        assert_eq!(
            in_my_size,
            MyVec(vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ])
        );
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
