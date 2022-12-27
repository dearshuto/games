use crate::math::Mat3x2;
use crate::IMinoFactory;
use crate::Vec2;

pub enum Operation {
    Right,
    Left,
    Down,
    RotateClockwise,
    RotateCounterClockwise,
}

pub struct Engine<TFactory: IMinoFactory> {
    // 操作中のミノ
    current_mino_position: Option<[Vec2; 4]>,

    // 画面内に存在する四角形
    blocks: Vec<Vec2>,

    // ミノを生成器
    factory: TFactory,

    // ミノが落ちてからの経過フレーム
    tick_frame: u8,
}

impl<T: IMinoFactory> Engine<T> {
    pub fn new(factory: T) -> Self {
        Self {
            current_mino_position: None,
            blocks: Vec::new(),
            factory,
            tick_frame: 0,
        }
    }

    pub fn update(&mut self, operation: Option<Operation>) -> () {
        // もうミノを落とすことができなければ操作ミノをフリーズして行消し判定を実行
        // もし操作しているミノがなければ true が帰るのでこの条件には入らない
        if !self.can_drop() {
            // 操作ミノフリーズ
            for v_array in &self.current_mino_position {
                for v in v_array {
                    self.blocks.push(*v);
                }
            }

            // TODO: 横一列が埋まった要素の削除

            self.tick_frame = 0;
            self.current_mino_position = None;
            return;
        }

        // 操作中のミノがなければ、新しいミノを生成して更新処理は終了
        let Some(current_mino) = &mut self.current_mino_position else {
            let new_mino = self.factory.create();
            self.current_mino_position = Some(new_mino);
            return;
        };

        // 1 秒に 1 回ひとつマスを落とす
        // 現状では 1/60 で update() が呼ばれることを想定
        let current_tick_frame = self.tick_frame + 1;
        let mat = if 60 <= current_tick_frame {
            self.tick_frame = 0;
            Mat3x2::new_transform(&Vec2 { x: 0, y: -1 })
        } else {
            self.tick_frame = current_tick_frame;
            Mat3x2::new()
        };

        // 操作による移動
        let operation_matrix = if let Some(operation) = operation {
            match operation {
                Operation::Right => Mat3x2::new_transform(&Vec2 { x: 1, y: 0 }),
                Operation::Left => Mat3x2::new_transform(&Vec2 { x: -1, y: 0 }),
                Operation::Down => Mat3x2::new(),
                Operation::RotateClockwise => Mat3x2::new(),
                Operation::RotateCounterClockwise => Mat3x2::new(),
            }
        } else {
            Mat3x2::new()
        };

        // 最終的なミノの移動量
        let final_matrix = Mat3x2::multiply(&mat, &operation_matrix);

        // ミノを移動
        for mut current_mino_position in current_mino {
            let new_position = final_matrix.transform_vec2(&current_mino_position);
            current_mino_position.x = new_position.x;
            current_mino_position.y = new_position.y;
        }
    }

    pub fn get_block_translations(&self) -> &[Vec2] {
        &self.blocks
    }

    pub fn can_drop(&self) -> bool {
        // ブロックがなければ落とせるとみなす仕様
        let Some(current_blocks) = &self.current_mino_position else{
            return true;
        };

        // 操作中のミノをひとつ下に落としてみる
        // その状態で位置が重なるミノが存在したら落ちることができない
        let targets = current_blocks
            .iter()
            .map(|v| Vec2 { x: v.x, y: v.y + 1 })
            .collect::<Vec<Vec2>>();
        let is_match = self
            .blocks
            .iter()
            .any(|v| targets.iter().any(|t| t.x == v.x && t.y == v.y)); // n*n だけどまあいいか
        return !is_match;
    }
}
