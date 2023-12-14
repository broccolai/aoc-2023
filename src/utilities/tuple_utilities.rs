// Define a trait with our custom map method for tuples
pub trait MapTuple: Iterator {
    fn map_tuple<A, B, C, D, F, G>(self, f: F, g: G) -> MapTupleAdapter<Self, F, G>
    where
        Self: Iterator<Item = (A, B)> + Sized,
        F: FnMut(A) -> C,
        G: FnMut(B) -> D,
    {
        MapTupleAdapter::new(self, f, g)
    }
}

// Implement the trait for all iterators that return a tuple
impl<I, A, B> MapTuple for I where I: Iterator<Item = (A, B)> {}

// A struct to hold the iterator and mapping functions
pub struct MapTupleAdapter<I, F, G> {
    iter: I,
    f: F,
    g: G,
}

impl<I, F, G, A, B, C, D> MapTupleAdapter<I, F, G>
where
    I: Iterator<Item = (A, B)>,
    F: FnMut(A) -> C,
    G: FnMut(B) -> D,
{
    fn new(iter: I, f: F, g: G) -> Self {
        Self { iter, f, g }
    }
}

// Implement Iterator for MapTupleAdapter
impl<I, F, G, A, B, C, D> Iterator for MapTupleAdapter<I, F, G>
where
    I: Iterator<Item = (A, B)>,
    F: FnMut(A) -> C,
    G: FnMut(B) -> D,
{
    type Item = (C, D);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(a, b)| ((self.f)(a), (self.g)(b)))
    }
}
