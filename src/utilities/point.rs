#[derive(Debug, Clone)]
pub struct Point<N> {
    pub x: N,
    pub y: N,
}

impl<N: Ord + Copy> Point<N> {
    pub fn from_tuple(source: (N, N)) -> Self {
        Point {
            x: source.0,
            y: source.1,
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Point {
            x: N::min(self.x, other.x),
            y: N::min(self.y, other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Point {
            x: N::max(self.x, other.x),
            y: N::max(self.y, other.y),
        }
    }
}
