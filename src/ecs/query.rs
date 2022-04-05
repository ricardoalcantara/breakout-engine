pub trait QueryParameters {}
impl<A> QueryParameters for (A,) {}
impl<A, B> QueryParameters for (A, B) {}
impl<A, B, C> QueryParameters for (A, B, C) {}

pub struct QueryIterator<Q: QueryParameters> {
    x: Q,
}

// impl<Q> QueryIterator<Q> {

//     pub fn iter(&mut self) -> QueryIter<'_, Q> {
//         self.borrow();
//         unsafe { QueryIter::new(self.meta, self.archetypes.iter()) }
//     }
// }

impl<Q: QueryParameters> Iterator for QueryIterator<Q> {
    type Item = (u32, Q);

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
