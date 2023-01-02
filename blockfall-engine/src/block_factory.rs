use crate::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    C0,
    C90,
    C180,
    C270,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

pub struct FactoryResult {
    pub block_type: BlockType,
    pub translation: Vec2,
    pub rotation: Rotation,
}

pub trait IBlockFactory {
    fn create(&mut self) -> FactoryResult;
}
