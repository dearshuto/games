use std::collections::HashSet;

use crate::math::Mat3x2;
use crate::Blocks;
use crate::IBlockFactory;
use crate::Vec2;

pub enum Operation {
    Right,
    Left,
    Down,
    RotateClockwise,
    RotateCounterClockwise,
}

struct ControlBlocks {
    local_positions: Vec<Vec2>,
    translation_matrix: Mat3x2,
    rotation_matrix: Mat3x2,
}

impl ControlBlocks {
    pub fn calculate_global_positions(&self) -> Vec<Vec2> {
        self.local_positions
            .iter()
            .map(|v| {
                let rotated_v = self.rotation_matrix.transform_vec2(&v);
                let new_v = Vec2 {
                    x: (rotated_v.x + 1) / 2,
                    y: (rotated_v.y + 1) / 2,
                };
                let result = self.translation_matrix.transform_vec2(&new_v);
                return result;
            })
            .collect()
    }
}

// 10x25 のフィールドにブロックを落とす
pub struct Engine<TFactory: IBlockFactory> {
    // 操作中のブロック
    control_block: Option<ControlBlocks>,

    // 画面内に存在する四角形
    blocks: Vec<Vec2>,

    // ミノを生成器
    factory: TFactory,

    // ミノが落ちてからの経過フレーム
    tick_frame: u8,
}

impl<T: IBlockFactory> Engine<T> {
    pub fn new(factory: T) -> Self {
        Self::new_init_blocks(factory, &[])
    }

    pub fn new_init_blocks(factory: T, blocks: &[Vec2]) -> Self {
        Self {
            control_block: None,
            blocks: blocks.to_vec(),
            factory,
            tick_frame: 0,
        }
    }

    pub fn update(&mut self, operation: Option<Operation>) -> () {
        // もうミノを落とすことができなければ操作ミノをフリーズして行消し判定を実行
        // もし操作しているミノがなければ true が帰るのでこの条件には入らない
        if !self.can_drop() {
            // 操作ミノフリーズ
            let Some(control_blocks) = &self.control_block else {
                return;
            };
            for v in &control_blocks.calculate_global_positions() {
                self.blocks.push(*v);
            }

            // 横一列が埋まった要素の削除
            self.remove_line();

            self.tick_frame = 0;
            self.control_block = None;
            return;
        }

        // 操作中のミノがなければ、新しいミノを生成して更新処理は終了
        if self.control_block.is_none() {
            self.reset_control_block();
            return;
        }

        // 1 秒に 1 回ひとつマスを落とす
        // 現状では 1/60 で update() が呼ばれることを想定
        let current_tick_frame = self.tick_frame + 1;
        let mat = if 10 <= current_tick_frame {
            self.tick_frame = 0;
            Mat3x2::new_transform(&Vec2 { x: 0, y: -1 })
        } else {
            self.tick_frame = current_tick_frame;
            Mat3x2::new()
        };

        let operation_translation = if let Some(operation) = &operation {
            match operation {
                Operation::Right => {
                    if self.can_move_right() {
                        Mat3x2::new_transform(&Vec2 { x: 1, y: 0 })
                    } else {
                        Mat3x2::new()
                    }
                }
                Operation::Left => {
                    if self.can_move_left() {
                        Mat3x2::new_transform(&Vec2 { x: -1, y: 0 })
                    } else {
                        Mat3x2::new()
                    }
                }
                _ => Mat3x2::new(),
            }
        } else {
            Mat3x2::new()
        };

        // 操作による移動
        let operation_rotation = if let Some(operation) = &operation {
            match operation {
                Operation::RotateClockwise => Mat3x2::new_rotate_clockwise(),
                Operation::RotateCounterClockwise => Mat3x2::new_rotate_counter_clockwise(),
                _ => Mat3x2::new(),
            }
        } else {
            Mat3x2::new()
        };

        let Some(control_block) = &mut self.control_block else {
            return;
        };

        // 最終的な操作ブロックの行列を算出
        let rotation_matrix = Mat3x2::multiply(&operation_rotation, &control_block.rotation_matrix);
        let translation_matrix = Mat3x2::multiply(
            &mat,
            &Mat3x2::multiply(&operation_translation, &control_block.translation_matrix),
        );
        control_block.rotation_matrix = rotation_matrix;
        control_block.translation_matrix = translation_matrix;

        // 回転したら左にはみ出た
        if control_block
            .calculate_global_positions()
            .iter()
            .any(|v| v.x < 0)
        {
            control_block.translation_matrix = Mat3x2::multiply(
                &Mat3x2::new_transform(&Vec2 { x: 1, y: 0 }),
                &control_block.translation_matrix,
            );
        }

        // 回転したら右にはみ出た
        if control_block
            .calculate_global_positions()
            .iter()
            .any(|v| 9 < v.x)
        {
            control_block.translation_matrix = Mat3x2::multiply(
                &Mat3x2::new_transform(&Vec2 { x: -1, y: 0 }),
                &control_block.translation_matrix,
            );
        }
    }

    pub fn get_block_translations(&self) -> &[Vec2] {
        &self.blocks
    }

    pub fn get_current_block_translations(&self) -> Vec<Vec2> {
        if let Some(control_block) = &self.control_block {
            control_block.calculate_global_positions()
        } else {
            Vec::new()
        }
    }

    fn can_drop(&self) -> bool {
        // ブロックがなければ落とせるとみなす仕様
        let Some(control_block) = &self.control_block else {
            return true;
        };

        // 操作中のミノをひとつ下に落としてみる
        // その状態で位置が重なるミノが存在したら落ちることができない
        let global_positions = control_block.calculate_global_positions();
        let targets = global_positions
            .iter()
            .map(|v| Vec2 { x: v.x, y: v.y - 1 })
            .collect::<Vec<Vec2>>();
        let is_match = self.blocks.iter().any(|v| {
            targets.iter().any(|t| t.x == v.x && t.y == v.y) // n*n だけどまあいいか
        });
        let is_lowest = global_positions.iter().any(|v| v.y == 0);
        return !(is_match || is_lowest);
    }

    fn can_move_right(&self) -> bool {
        let Some(control_blocks) = &self.control_block else {
            return false;
        };

        let is_on_edge = control_blocks
            .calculate_global_positions()
            .iter()
            .any(|v| 9 <= v.x);
        !is_on_edge
    }

    fn can_move_left(&self) -> bool {
        let Some(control_blocks) = &self.control_block else {
            return false;
        };

        let is_on_edge = control_blocks
            .calculate_global_positions()
            .iter()
            .any(|v| v.x <= 0);
        !is_on_edge
    }

    fn remove_line(&mut self) {
        let mut blocks: [Vec<usize>; 20] = Default::default();
        for index in 0..self.blocks.len() {
            let block = &self.blocks[index];
            blocks[block.y as usize].push(index);
        }

        // 一列揃ってるやつは消す
        let mut hash = HashSet::new();
        for y in (0..blocks.len()).rev() {
            let block = &mut blocks[y];
            if block.len() != 10 {
                continue;
            }

            // 消すやつはソートが必要なので一旦保持
            for index in block {
                hash.insert(*index);
            }

            // 消したやつの上の行の段は一段ずつ下げる
            // メモ：消す予定のやつも生き残ってるから無駄な計算が走ってる
            for block in &mut self.blocks {
                if block.y < y as i32 {
                    continue;
                }

                block.y -= 1;
            }
        }

        // 消えるブロックたちを逆順で消していく
        // 昇順に消すとインデックスずれが起きるので注意
        let mut indices: Vec<usize> = hash.into_iter().collect();
        indices.sort();
        indices.reverse();
        for index in indices {
            self.blocks.remove(index);
        }
    }

    fn reset_control_block(&mut self) {
        let new_source = self.factory.create();
        let blocks = match new_source.block_type {
            crate::block_factory::BlockType::I => Blocks::i(),
            crate::block_factory::BlockType::O => Blocks::o(),
            crate::block_factory::BlockType::S => Blocks::s(),
            crate::block_factory::BlockType::Z => Blocks::z(),
            crate::block_factory::BlockType::J => Blocks::j(),
            crate::block_factory::BlockType::L => Blocks::l(),
            crate::block_factory::BlockType::T => Blocks::t(),
        };
        let rotation_matrix = match new_source.rotation {
            crate::block_factory::Rotation::C0 => Mat3x2::new(),
            crate::block_factory::Rotation::C90 => Mat3x2::new_rotate_counter_clockwise(),
            crate::block_factory::Rotation::C180 => Mat3x2::multiply(
                &Mat3x2::new_rotate_counter_clockwise(),
                &Mat3x2::new_rotate_counter_clockwise(),
            ),
            crate::block_factory::Rotation::C270 => Mat3x2::new_rotate_clockwise(),
        };
        let translation_matrix = Mat3x2::new_transform(&new_source.translation);

        self.control_block = Some(ControlBlocks {
            local_positions: blocks.to_vec(),
            translation_matrix,
            rotation_matrix,
        });
    }
}
