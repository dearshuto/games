use crate::{BlockType, FactoryResult, IBlockFactory, Rotation, Vec2};
use rand::seq::SliceRandom;

pub struct StandardBlockFactory {
    factory_queue: [BlockType; 7],
    current_index: u8,
}

impl StandardBlockFactory {
    pub fn new() -> Self {
        let factory_queue = Self::generate_block_queue();
        Self {
            factory_queue,
            current_index: 0,
        }
    }

    fn generate_block_queue() -> [BlockType; 7] {
        let mut factory_queue = [
            BlockType::I,
            BlockType::O,
            BlockType::S,
            BlockType::Z,
            BlockType::J,
            BlockType::L,
            BlockType::T,
        ];
        let mut rng = rand::thread_rng();
        factory_queue.shuffle(&mut rng);
        factory_queue
    }
}

impl IBlockFactory for StandardBlockFactory {
    fn create(&mut self) -> FactoryResult {
        // 全部使い切ったら再生成
        if self.current_index >= self.factory_queue.len() as u8 {
            self.factory_queue = Self::generate_block_queue();
            self.current_index = 0;
        }

        let next_block_type = self.factory_queue[self.current_index as usize];
        let offset = Vec2 { x: 5, y: 50 };

        self.current_index += 1;
        FactoryResult {
            block_type: next_block_type,
            translation: offset,
            rotation: Rotation::C0,
        }
    }
}
