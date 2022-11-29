pub mod bit_board {
    #[derive(Clone, Debug, PartialEq)]
    pub struct BitBoard {
        pub white_b: i32, // 先手
        pub black_b: i32, // 後手
        pub kb: i32,      // ライオン
        pub rb: i32,      // キリン
        pub bb: i32,      // ゾウ
        pub pb: i32,      // ヒヨコ
        pub ppb: i32,     // ニワトリ
    }
}
