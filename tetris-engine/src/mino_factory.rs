use crate::Vec2;

pub trait IMinoFactory {
    fn create(&mut self) -> [Vec2; 4];
}

pub struct StandardMinoFactory {
    factory_queue: [[Vec2; 4]; 7],
    current_index: u8,
}

impl StandardMinoFactory {
    pub fn new() -> Self {
        let factory_queue = Self::generate_mino_queue();
        Self {
            factory_queue,
            current_index: 0,
        }
    }

    fn generate_mino_queue() -> [[Vec2; 4]; 7] {
        let factory_queue = [
            [
                Vec2 { x: -2, y: 0 },
                Vec2 { x: -1, y: 0 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 2, y: 0 },
            ], // I
            [
                Vec2 { x: -1, y: 1 },
                Vec2 { x: -1, y: -1 },
                Vec2 { x: 1, y: 1 },
                Vec2 { x: 1, y: -1 },
            ], // O
            [
                Vec2 { x: -1, y: -1 },
                Vec2 { x: 0, y: -1 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 1 },
            ], // S
            [
                Vec2 { x: -1, y: 1 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 0, y: -1 },
                Vec2 { x: 1, y: -1 },
            ], // Z
            [
                Vec2 { x: -1, y: 1 },
                Vec2 { x: -1, y: 0 },
                Vec2 { x: 1, y: -1 },
                Vec2 { x: 2, y: -1 },
            ], // J
            [
                Vec2 { x: -1, y: -1 },
                Vec2 { x: 0, y: -1 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 2, y: 1 },
            ], // L
            [
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: -1 },
                Vec2 { x: -1, y: -1 },
            ], // T
        ];
        factory_queue
    }
}

impl IMinoFactory for StandardMinoFactory {
    fn create(&mut self) -> [Vec2; 4] {
        let current_index = self.current_index;
        self.current_index += 1;

        // 全部使い切ったら再生成
        if self.current_index >= self.factory_queue.len() as u8 {
            self.factory_queue = Self::generate_mino_queue();
        }

        self.factory_queue[current_index as usize].clone()
    }
}
