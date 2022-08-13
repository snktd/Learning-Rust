
struct MyMutableIterator<'a, T> {
    slice: &'a mut [T]
}

impl<'a, T> Iterator for MyMutableIterator<'a, T> {
    type Item = &'a mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice = std::mem::replace(&mut self.slice, &mut []);
        let (first, remaining) = slice.split_first_mut()?;

        self.slice = remaining;

        Some(first)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_works() {
        let mut coll = vec![1, 2, 3, 4, 5];
        let iter = MyMutableIterator {
            slice: &mut coll[..],
        };

        for (_, element) in iter.enumerate() {
            *element = *element + 10;
        }

        assert_eq!(coll.get(0), Some(&11));

    }
}