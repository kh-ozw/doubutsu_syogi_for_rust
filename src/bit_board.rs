pub mod bit_board {
    #[derive(Clone, Debug, PartialEq)]
    pub struct BitBoard {
        pub hb1: i32, // 1pヒヨコ
        pub lb1: i32, // 1pライオン
        pub kb1: i32, // 1pキリン
        pub zb1: i32, // 1pゾウ
        pub nb1: i32, // 1pニワトリ
        pub hb2: i32, // 2pヒヨコ
        pub lb2: i32, // 2pライオン
        pub kb2: i32, // 2pキリン
        pub zb2: i32, // 2pゾウ
        pub nb2: i32, // 2pニワトリ
    }
}
