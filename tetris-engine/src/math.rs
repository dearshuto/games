#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub struct Mat3x2 {
    m: [i32; 6],
}

impl Mat3x2 {
    pub fn new() -> Self {
        // 単位行列を返す
        Self {
            m: [1, 0, 0, 0, 1, 0],
        }
    }

    pub fn new_transform(t: &Vec2) -> Self {
        Self {
            m: [1, 0, t.x, 0, 1, t.y],
        }
    }

    pub fn multiply(lfs: &Mat3x2, rfs: &Mat3x2) -> Mat3x2 {
        // 0 1 2    0 1 2
        // 3 4 5    3 4 5
        // 0 0 1    0 0 1  ⇦ ここは補間
        let new_m = [
            // 上の段
            lfs.m[0] * rfs.m[0] + lfs.m[1] * rfs.m[3],
            lfs.m[0] * rfs.m[1] + lfs.m[1] * rfs.m[4],
            lfs.m[0] * rfs.m[2] + lfs.m[1] * rfs.m[5] + lfs.m[2],
            // 下の段
            lfs.m[3] * rfs.m[0] + lfs.m[4] * rfs.m[3],
            lfs.m[3] * rfs.m[1] + lfs.m[4] * rfs.m[4],
            lfs.m[3] * rfs.m[2] + lfs.m[4] * rfs.m[5] + lfs.m[5],
        ];

        Self { m: new_m }
    }

    pub fn transform_vec2(&self, vec2: &Vec2) -> Vec2 {
        let new_x = self.m[0] * vec2.x + self.m[1] * vec2.y + self.m[2];
        let new_y = self.m[3] * vec2.x + self.m[4] * vec2.y + self.m[5];
        Vec2 { x: new_x, y: new_y }
    }
}
