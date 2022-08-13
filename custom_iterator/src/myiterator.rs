struct MyIterator<'a, T> {
    slice: &'a[T]
}

impl<'a, T> Iterator for MyIterator<'a, T> {

    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let (first, remaining) = self.slice.split_first()?;
        self.slice = remaining;
        Some(first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_works() {
        let coll = vec![1, 2, 3, 4, 5];
        let iter = MyIterator {
            slice: &coll[..],
        };

        for (index, element) in iter.enumerate() {
            assert_eq!(*element, coll[index]);
        }

    }
}