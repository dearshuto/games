use crate::Request;
use std::marker::PhantomData;

// パターンの生成器
pub struct Template<T> {
    _marker: PhantomData<T>,
}

impl<T: Copy> Template<T> {
    // ビーコン。周期 1
    pub fn beacon(status: T) -> Vec<Request<T>> {
        vec![
            Request { x: 0, y: 0, status },
            Request { x: 1, y: 0, status },
            Request { x: 0, y: 1, status },
            Request { x: 1, y: 1, status },
            Request { x: 2, y: 2, status },
            Request { x: 3, y: 2, status },
            Request { x: 2, y: 3, status },
            Request { x: 3, y: 3, status },
        ]
    }

    // ヒキガエル。周期 1
    pub fn toad(status: T) -> Vec<Request<T>> {
        vec![
            Request { x: 0, y: 0, status },
            Request { x: 0, y: 1, status },
            Request { x: 0, y: 2, status },
            Request { x: 1, y: 1, status },
            Request { x: 1, y: 2, status },
            Request { x: 1, y: 3, status },
        ]
    }

    // 八角形。周期 3
    pub fn octagon(status: T) -> Vec<Request<T>> {
        vec![
            Request { x: 3, y: 0, status },
            Request { x: 2, y: 1, status },
            Request { x: 1, y: 2, status },
            Request { x: 0, y: 3, status },
            Request { x: 4, y: 0, status },
            Request { x: 5, y: 1, status },
            Request { x: 6, y: 2, status },
            Request { x: 7, y: 3, status },
            Request { x: 3, y: 7, status },
            Request { x: 2, y: 6, status },
            Request { x: 1, y: 5, status },
            Request { x: 0, y: 4, status },
            Request { x: 4, y: 7, status },
            Request { x: 5, y: 6, status },
            Request { x: 6, y: 5, status },
            Request { x: 7, y: 4, status },
        ]
    }
}
