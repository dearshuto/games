use crate::IRule;

pub struct Request<T> {
    pub x: u8,
    pub y: u8,
    pub status: T,
}

// 256x256 のライフゲームロジックを提供
pub struct Engine<TRule: IRule> {
    status: [Vec<TRule::Status>; 2],
    current_buffer_index: usize,
    rule: TRule,
}

impl<TRule: IRule> Engine<TRule> {
    pub fn new(rule: TRule) -> Self {
        Self::new_init(rule, &[])
    }

    pub fn new_init(rule: TRule, requests: &[Request<TRule::Status>]) -> Self {
        let mut status_vec = vec![Default::default(); 256 * 256];
        // リクエストの反映
        for request in requests {
            let index = (request.x as usize) + 256 * (request.y as usize);
            status_vec[index] = request.status;
        }

        Self {
            status: [status_vec, vec![Default::default(); 256 * 256]],
            current_buffer_index: 0,
            rule,
        }
    }

    pub fn update(&mut self, requests: &[Request<TRule::Status>]) {
        // リクエストの反映
        for request in requests {
            self.set_current_status(request.x as i32, request.y as i32, request.status);
        }

        for y in 0..256 {
            for x in 0..256 {
                let status_array = [
                    self.get_status(x - 1, y - 1),
                    self.get_status(x, y - 1),
                    self.get_status(x + 1, y - 1),
                    self.get_status(x - 1, y),
                    self.get_status(x, y),
                    self.get_status(x + 1, y),
                    self.get_status(x - 1, y + 1),
                    self.get_status(x, y + 1),
                    self.get_status(x + 1, y + 1),
                ];
                let next_status = self.rule.calculate_next_status(&status_array);
                self.set_next_status(x, y, next_status);
            }
        }

        self.current_buffer_index = self.get_next_buffer_index();
    }

    pub fn get_status(&self, x: i32, y: i32) -> TRule::Status {
        if let Some(index) = self.to_index(x, y) {
            (self.status[self.current_buffer_index])[index]
        } else {
            TRule::get_outside()
        }
    }

    // 現在使用しているバッファに情報を書き込みます
    fn set_current_status(&mut self, x: i32, y: i32, status: TRule::Status) {
        if let Some(index) = self.to_index(x, y) {
            (self.status[self.current_buffer_index])[index] = status;
        }
    }

    // 次に使用するバッファに情報を書き込みます
    fn set_next_status(&mut self, x: i32, y: i32, status: TRule::Status) {
        if let Some(index) = self.to_index(x, y) {
            let next_buffer_index = self.get_next_buffer_index();
            (self.status[next_buffer_index])[index] = status;
        }
    }

    fn get_next_buffer_index(&self) -> usize {
        let next_buffer_index = (self.current_buffer_index + 1) % 2;
        next_buffer_index
    }

    // x-y 座標を一次元配列のインデクスに変換
    fn to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || 256 <= x {
            return None;
        }
        if y < 0 || 256 <= y {
            return None;
        }

        Some((x + 256 * y) as usize)
    }
}
