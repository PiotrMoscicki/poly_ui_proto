// deps
use nalgebra::Point2;
use nalgebra::Vector2;

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Transform {
    pub pos: Point2<i32>,
    pub size: Vector2<u32>,
}

//************************************************************************************************
impl Transform {
    pub fn new(pos: &Point2<i32>, size: &Vector2<u32>) -> Self {
        Self {
            pos: *pos,
            size: *size,
        }
    }
}

//************************************************************************************************
impl Default for Transform {
    fn default() -> Self {
        Self {
            pos: Point2::<i32>::new(0, 0),
            size: Vector2::<u32>::new(0, 0),
        }
    }
}
