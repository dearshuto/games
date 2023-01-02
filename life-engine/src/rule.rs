// ライフゲームのルールを注入するトレイと
pub trait IRule {
    type Status: Clone + Copy + PartialEq + PartialOrd + Default;

    // ステータスの更新ロジック
    fn calculate_next_status(&self, cells: &[Self::Status; 9]) -> Self::Status;

    // 範囲外のステータス
    fn get_outside() -> Self::Status;
}
