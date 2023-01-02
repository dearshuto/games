use crate::Vec2;

pub struct Blocks;

impl Blocks {
    pub fn i() -> [Vec2; 4] {
        [
            Vec2 { x: -3, y: -1 },
            Vec2 { x: -1, y: -1 },
            Vec2 { x: 1, y: -1 },
            Vec2 { x: 3, y: -1 },
        ]
    }

    pub fn o() -> [Vec2; 4] {
        [
            Vec2 { x: 1, y: 1 },
            Vec2 { x: -1, y: 1 },
            Vec2 { x: -1, y: -1 },
            Vec2 { x: 1, y: -1 },
        ]
    }

    pub fn s() -> [Vec2; 4] {
        [
            Vec2 { x: -1, y: -1 },
            Vec2 { x: 1, y: -1 },
            Vec2 { x: 1, y: 1 },
            Vec2 { x: 3, y: 1 },
        ]
    }

    pub fn z() -> [Vec2; 4] {
        [
            Vec2 { x: -1, y: 1 },
            Vec2 { x: 1, y: 1 },
            Vec2 { x: 1, y: -1 },
            Vec2 { x: 3, y: -1 },
        ]
    }

    pub fn j() -> [Vec2; 4] {
        [
            Vec2 { x: -1, y: 1 },
            Vec2 { x: -1, y: -1 },
            Vec2 { x: 1, y: -1 },
            Vec2 { x: 3, y: -1 },
        ]
    }

    pub fn l() -> [Vec2; 4] {
        [
            Vec2 { x: -1, y: -1 },
            Vec2 { x: 1, y: -1 },
            Vec2 { x: 3, y: -1 },
            Vec2 { x: 3, y: 1 },
        ]
    }

    pub fn t() -> [Vec2; 4] {
        [
            Vec2 { x: -1, y: -1 },
            Vec2 { x: 1, y: -1 },
            Vec2 { x: 1, y: 1 },
            Vec2 { x: 3, y: -1 },
        ]
    }
}
