use std::ops::Add;

pub struct Triangle<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Triangle<T>
where
    T: From<u8> + PartialEq + PartialOrd + Add<Output = T> + Copy,
{
    fn is_valid(sides: [T; 3]) -> Option<[T; 3]> {
        let [x, y, z] = sides;
        if x == T::from(0_u8)
            || y == T::from(0_u8)
            || z == T::from(0_u8)
            || (x + y) < z
            || (x + z) < y
            || (z + y) < x
        {
            return None;
        }

        Some(sides)
    }

    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        Triangle::is_valid(sides).map(|[x, y, z]| Triangle { x, y, z })
    }

    pub fn is_equilateral(&self) -> bool {
        self.x == self.y && self.y == self.z
    }

    pub fn is_scalene(&self) -> bool {
        self.x != self.y && self.y != self.z && self.x != self.z
    }

    pub fn is_isosceles(&self) -> bool {
        (self.x == self.y || self.y == self.z || self.x == self.z) && !self.is_equilateral()
    }
}
