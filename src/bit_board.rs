pub mod bit_board {
    use std::fmt;
    #[derive(Clone, Debug, PartialEq)]
    pub struct BitBoard {
        pub pb1: i32, // 先手
        pub pb2: i32, // 後手
        pub lb: i32,  // ライオン
        pub kb: i32,  // キリン
        pub zb: i32,  // ゾウ
        pub hb: i32,  // ヒヨコ
        pub nb: i32,  // ニワトリ
    }
    impl std::fmt::Display for BitBoard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let pb1_str = reverse_string(&format!("{:<024b}", &self.pb1).to_string());
            let pb2_str = reverse_string(&format!("{:<024b}", &self.pb2).to_string());
            let lb_str = reverse_string(&format!("{:<024b}", &self.lb).to_string());
            let kb_str = reverse_string(&format!("{:<024b}", &self.kb).to_string());
            let zb_str = reverse_string(&format!("{:<024b}", &self.zb).to_string());
            let hb_str = reverse_string(&format!("{:<024b}", &self.hb).to_string());
            let nb_str = reverse_string(&format!("{:<024b}", &self.nb).to_string());
            let board_list = [pb1_str, pb2_str, lb_str, kb_str, zb_str, hb_str, nb_str];
            let _ = writeln!(f, "---------------------------------------------");
            let _ = writeln!(f, "pb1     pb2      lb      kb      zb      hb      nb");
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
