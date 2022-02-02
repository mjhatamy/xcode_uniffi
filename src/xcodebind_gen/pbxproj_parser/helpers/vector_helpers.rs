pub trait VectorHelper<U> {
    fn remove_all(&mut self) -> Vec<U>;
}

impl<U> VectorHelper<U> for Vec<U>
where
    U: Eq,
{
    fn remove_all(&mut self) -> Vec<U> {
        let mut items: Vec<U> = vec![];
        while let Some(item) = self.pop() {
            items.push(item)
        }
        //       items.reverse();
        items
    }
}
