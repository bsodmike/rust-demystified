#![allow(unused_doc_comments)]
use std::iter::FromIterator;
use std::vec::Vec;


#[derive(Debug, PartialEq, Copy, Clone)]
struct Shoe {
    size: u32,
    id: u32
}

#[derive(Debug, PartialEq)]
struct NormalShoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<NormalShoe>, shoe_size: u32) -> Vec<NormalShoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn shoes_in_size_iterable(shoes: MyVec<Vec<Shoe>>, shoe_size: u32) -> MyVec<Vec<Shoe>> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}

// fn shoes_into_iter(shoes: MyVec<Vec<Shoe>>, shoe_size: u32) -> MyVec<Vec<Shoe>> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }

#[derive(PartialEq, Debug, Copy, Clone)]
struct MyVec<T>(T);

struct MyVecIter<'a, T> {
    vector: &'a T,
    cur: usize,
}

impl MyVec<Vec<Shoe>> {
    pub fn new() -> Self {
        MyVec(Vec::<Shoe>::new())
    }

    pub fn iter(&self) -> MyVecIter<'_, Vec<Shoe>> {
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
            return None
        }
    }
}

// impl IntoIterator for MyVec<Vec<Shoe>> {
//     type Item = Shoe;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         let inner_vec = self.0;

//         return inner_vec.into_iter();
//     }
// }

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

    // #[test]
    // fn add_into_iter() {
    //     let shoes = MyVec(vec![
    //         Shoe {
    //             size: 10,
    //             id: 0
    //         },
    //         Shoe {
    //             size: 13,
    //             id: 1,
    //         },
    //         Shoe {
    //             size: 10,
    //             id: 2,
    //         },
    //     ]);

    //     let in_my_size = shoes_into_iter(shoes, 10);

    //     assert_eq!(
    //         in_my_size,
    //         MyVec(vec![
    //             Shoe {
    //                 size: 10,
    //                 id: 0,
    //             },
    //             Shoe {
    //                 size: 10,
    //                 id: 2,
    //             },
    //         ])
    //     );
    // }

    #[test]
    fn filters_by_size_wrapped_iter() {
        let shoes = MyVec(vec![
            Shoe {
                size: 10,
                id: 100
            },
            Shoe {
                size: 13,
                id: 101,
            },
            Shoe {
                size: 10,
                id: 102,
            },
        ]);

        let in_my_size = shoes_in_size_iterable(shoes, 10);

        assert_eq!(
            in_my_size,
            MyVec(vec![
                Shoe {
                    size: 10,
                    id: 100,
                },
                Shoe {
                    size: 10,
                    id: 102,
                },
            ])
        );
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            NormalShoe {
                size: 10,
                style: String::from("sneaker"),
            },
            NormalShoe {
                size: 13,
                style: String::from("sandal"),
            },
            NormalShoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                NormalShoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                NormalShoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
