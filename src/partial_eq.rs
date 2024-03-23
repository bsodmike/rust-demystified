use anyhow::{Ok, Result};

#[derive(Debug, PartialEq, Default)]
pub struct List<T> {
    memory: Vec<T>,
}

impl<T> List<T>
where
    List<T>: PartialEq,
{
    pub fn new() -> Self {
        List { memory: Vec::new() }
    }
    // push() add to end of list
    pub fn push(&mut self, value: T) {
        self.memory.push(value);
    }

    pub fn is_equal_to(&mut self, other: &List<T>) -> bool {
        self == other
    }
}

pub fn runner() -> Result<()> {
    example1::run()?;

    Ok(())
}

pub mod example1 {
    use super::*;

    pub fn run() -> Result<()> {
        let mut list1 = List::<Option<usize>>::new();
        let mut list2 = List::<Option<usize>>::new();

        list1.push(Some(1));

        let are_they_equal = list1.is_equal_to(&mut list2);
        assert!(are_they_equal);

        Ok(())
    }
}
