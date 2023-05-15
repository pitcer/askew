use num_traits::Num;

#[derive(Debug, Copy, Clone)]
pub struct Vector<T> {
    horizontal: T,
    vertical: T,
}

impl<T> Vector<T> {
    pub fn new(horizontal: T, vertical: T) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

impl<T> Vector<T>
where
    T: Copy,
{
    pub fn horizontal(&self) -> T {
        self.horizontal
    }

    pub fn vertical(&self) -> T {
        self.vertical
    }
}

impl<T> Vector<T>
where
    T: Copy + Num,
{
    pub fn cross_product_magnitude(&self, other: Self) -> T {
        self.horizontal * other.vertical - self.vertical * other.horizontal
    }
}
