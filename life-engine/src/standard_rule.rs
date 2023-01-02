use crate::IRule;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum Status {
    Alive,

    #[default]
    Dead,
}

pub struct StandardRule;

impl StandardRule {
    pub fn new() -> Self {
        Self {}
    }
}

impl IRule for StandardRule {
    type Status = Status;

    fn get_outside() -> Self::Status {
        Status::Dead
    }

    fn calculate_next_status(&self, status_array: &[Self::Status; 9]) -> Self::Status {
        let alive_count = status_array
            .into_iter()
            .filter(|s| **s == Status::Alive)
            .count();
        let current_status = status_array[4];
        let next_status = if current_status == Status::Alive {
            // 生きてるセルの周囲に生きてるセルが 2or3 のときに生き残る
            // それ以外の場合は死ぬ
            match alive_count - 1 /*自身を抜く */ {
                2 => Status::Alive,
                3 => Status::Alive,
                _ => Status::Dead,
            }
        } else {
            // 死んでいるセルの周囲に 3 つの生きているセルが存在したら誕生
            if alive_count == 3 {
                Status::Alive
            } else {
                Status::Dead
            }
        };

        return next_status;
    }
}
