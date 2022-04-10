pub trait QueryParameters {
    type ItemFetch;
    fn fetch(world: &mut World) -> Self::ItemFetch;
}
impl<A: 'static> QueryParameters for (A,) {
    type ItemFetch = Vec<(usize, (&'a A,))>;
    fn fetch(world: &mut World) -> Self::ItemFetch {
        let data = world.borrow_component_vec::<A>().unwrap();
        let result = data
            .iter()
            .enumerate()
            .filter_map(|(i, f)| Some((i, (f.as_ref()?,))))
            .collect();

        result
    }
}
impl<A: 'static, B: 'static> QueryParameters for (A, B) {
    type ItemFetch = Vec<(usize, (A,))>;
    fn fetch(world: &mut World) -> Self::ItemFetch {
        todo!()
    }
}
impl<A: 'static, B: 'static, C: 'static> QueryParameters for (A, B, C) {
    type ItemFetch = Vec<(usize, (A,))>;
    fn fetch(world: &mut World) -> Self::ItemFetch {
        todo!()
    }
}

pub struct QueryIterator<Q: QueryParameters> {
    x: Q,
}

// impl<Q> QueryIterator<Q> {

//     pub fn iter(&mut self) -> QueryIter<'_, Q> {
//         self.borrow();
//         unsafe { QueryIter::new(self.meta, self.archetypes.iter()) }
//     }
// }

use super::world::World;

impl<Q: QueryParameters> Iterator for QueryIterator<Q> {
    type Item = (u32, Q);

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}
