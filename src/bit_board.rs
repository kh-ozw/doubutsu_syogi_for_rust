pub mod bit_board {
    use std::fmt;

    // '''
    // p = [32]
    //  bit :  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
    // board: A1 B1 C1 A2 B2 C2 A3 B3 C3 A4 B4 C4 D1 D2 D3 D4 D5 D6 E1 E2 E3 E4 E5 E6
    // '''
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

    impl std::fmt::Display for BitBoard {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let white_b_str = reverse_string(&format!("{:<024b}", &self.white_b).to_string());
            let black_b_str = reverse_string(&format!("{:<024b}", &self.black_b).to_string());
            let kb_str = reverse_string(&format!("{:<024b}", &self.kb).to_string());
            let rb_str = reverse_string(&format!("{:<024b}", &self.rb).to_string());
            let bb_str = reverse_string(&format!("{:<024b}", &self.bb).to_string());
            let pb_str = reverse_string(&format!("{:<024b}", &self.pb).to_string());
            let ppb_str = reverse_string(&format!("{:<024b}", &self.ppb).to_string());
            let board_list = [
                white_b_str,
                black_b_str,
                kb_str,
                rb_str,
                bb_str,
                pb_str,
                ppb_str,
            ];
            let _ = writeln!(f, "---------------------------------------------");
            let _ = writeln!(f, "white   black    kb      rb      bb      pb      ppb");
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

            // for i in 0..field_list.len() {
            //     let _ = writeln!(f, "{}", field_list[i]);
            //     // let b_str = format!("{:<012b}", board_list[i]).to_string();
            //     let b_str = format!("{:<024b}", board_list[i]).to_string();
            //     let _ = writeln!(f, "{}", &b_str[..6]);
            //     let _ = writeln!(f, "{}", &b_str[21..24]);
            //     let _ = writeln!(f, "{}", &b_str[18..21]);
            //     let _ = writeln!(f, "{}", &b_str[15..18]);
            //     let _ = writeln!(f, "{}", &b_str[12..15]);
            //     let _ = writeln!(f, "{}", &b_str[6..12]);
            // }
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
//white   black    kb      rb      bb      pb      ppb
//000000  000000  000000  000000  000000  000000  000000
// 000     000     000     000     000     000     000
// 000     000     000     000     000     000     000
// 000     000     000     000     000     000     000
// 000     000     000     000     000     000     000
//000000  000000  000000  000000  000000  000000  000000

// A1 1
// B1 2
// C1 4
// A2 8
// B2 16
// C2 32
// A3 64
// B3 128
// C3 256
// A4 512
// B4 1024
// C4 2048

//     A    B    C
// 1   1    2    4
// 2   8   16   32
// 3  64  128  256
// 4 512 1024 2048
