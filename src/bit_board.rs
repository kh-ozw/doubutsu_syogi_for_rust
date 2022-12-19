pub mod bit_board {
    use std::fmt;
    #[derive(Clone, Debug, PartialEq)]
    pub struct BitBoard {
        pub pb1: i32, // 1pコマ
        pub hb1: i32, // 1pヒヨコ
        pub lb1: i32, // 1pライオン
        pub kb1: i32, // 1pキリン
        pub zb1: i32, // 1pゾウ
        pub nb1: i32, // 1pニワトリ
        pub pb2: i32, // 2pコマ
        pub hb2: i32, // 2pヒヨコ
        pub lb2: i32, // 2pライオン
        pub kb2: i32, // 2pキリン
        pub zb2: i32, // 2pゾウ
        pub nb2: i32, // 2pニワトリ
    }
    impl std::fmt::Display for BitBoard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let pb1_str = reverse_string(&format!("{:<024b}", &self.pb1).to_string());
            let lb1_str = reverse_string(&format!("{:<024b}", &self.lb1).to_string());
            let hb1_str = reverse_string(&format!("{:<024b}", &self.hb1).to_string());
            let kb1_str = reverse_string(&format!("{:<024b}", &self.kb1).to_string());
            let zb1_str = reverse_string(&format!("{:<024b}", &self.zb1).to_string());
            let nb1_str = reverse_string(&format!("{:<024b}", &self.nb1).to_string());
            let pb2_str = reverse_string(&format!("{:<024b}", &self.pb2).to_string());
            let lb2_str = reverse_string(&format!("{:<024b}", &self.lb2).to_string());
            let hb2_str = reverse_string(&format!("{:<024b}", &self.hb2).to_string());
            let kb2_str = reverse_string(&format!("{:<024b}", &self.kb2).to_string());
            let zb2_str = reverse_string(&format!("{:<024b}", &self.zb2).to_string());
            let nb2_str = reverse_string(&format!("{:<024b}", &self.nb2).to_string());

            let board_list = [
                pb1_str, lb1_str, hb1_str, kb1_str, zb1_str, nb1_str, pb2_str, lb2_str, hb2_str,
                kb2_str, zb2_str, nb2_str,
            ];
            let _ = writeln!(f, "---------------------------------------------");
            let _ = writeln!(f, " pb1     lb1     hb1     kb1     zb1     nb1     pb2     lb2     hb2     kb2     zb2     nb2");
            // 相手の持ち駒を表示
            for i in 0..board_list.len() {
                let _ = write!(f, "{}", &board_list[i][18..24]);
                let _ = write!(f, "  ");
            }
            let _ = writeln!(f, "");

            // 盤面のコマを表示
            for j in 0..4 {
                let _ = write!(f, " ");
                for i in 0..board_list.len() {
                    let _ = write!(f, "{}", &board_list[i][3 * j..3 * j + 3]);
                    let _ = write!(f, "     ");
                }
                let _ = writeln!(f, "");
            }

            // 自分の持ち駒を表示
            for i in 0..board_list.len() {
                let _ = write!(f, "{}", &board_list[i][12..18]);
                let _ = write!(f, "  ");
            }
            let _ = writeln!(f, "");

            writeln!(f, "---------------------------------")
        }
    }
    fn reverse_string(input: &String) -> String {
        let mut reversed = String::new();
        let mut chars: Vec<char> = Vec::new();

        for c in input.chars().into_iter() {
            chars.push(c);
        }

        for i in (0..chars.len()).rev() {
            reversed += &chars[i].to_string();
        }

        return reversed;
    }
}
