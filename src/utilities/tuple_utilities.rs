use std::iter::Map;

pub trait MapTupleElements<A, B, C, D>: Iterator<Item = (A, B)> + Sized {
    fn map_each_tuple_element<F, G>(self, map_first: F, map_second: G) -> Map<Self, fn((A, B)) -> (C, D)>
    where
        F: Fn(A) -> C,
        G: Fn(B) -> D;
}

impl<I, A, B, C, D> MapTupleElements<A, B, C, D> for I
where
    I: Iterator<Item = (A, B)>,
{
    fn map_each_tuple_element<F, G>(self, map_first: F, map_second: G) -> Map<Self, fn((A, B)) -> (C, D)>
    where
        F: Fn(A) -> C,
        G: Fn(B) -> D,
    {
        // Internal function to apply mapping to a single tuple
        //fn apply<A, B, C, D, F, G>((first, second): (A, B), map_first: F, map_second: G) -> (C, D)
        //where
        //    F: FnMut(A) -> C,
        //    G: FnMut(B) -> D,
        //{
        //    (map_first(first), map_second(second))
        //}

        let aa = map_first
        let test = self.map(|(first, second)| (map_first(first), map_second(second)));

        test
        // Apply the mapping functions to each tuple in the iterator
        // self.map(move |tuple| apply(tuple, map_first, map_second))
    }
}
