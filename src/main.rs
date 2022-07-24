use std::fmt;
#[warn(unused_variables)]
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

const NUL: u8 = 0x0;
const A1: u8 = 0xa1;
const A2: u8 = 0xa2;
const A3: u8 = 0xa3;
const A4: u8 = 0xa4;
const B1: u8 = 0xb1;
const B2: u8 = 0xb2;
const B3: u8 = 0xb3;
const B4: u8 = 0xb4;
const C1: u8 = 0xc1;
const C2: u8 = 0xc2;
const C3: u8 = 0xc3;
const C4: u8 = 0xc4;
const D1: u8 = 0xd1;
const D2: u8 = 0xd2;
const D3: u8 = 0xd3;
const D4: u8 = 0xd4;
const D5: u8 = 0xd5;
const D6: u8 = 0xd6;
const E1: u8 = 0xe1;
const E2: u8 = 0xe2;
const E3: u8 = 0xe3;
const E4: u8 = 0xe4;
const E5: u8 = 0xe5;
const E6: u8 = 0xe6;
const A1_INDEX: usize = 0;
const B1_INDEX: usize = 1;
const C1_INDEX: usize = 2;
const A2_INDEX: usize = 3;
const B2_INDEX: usize = 4;
const C2_INDEX: usize = 5;
const A3_INDEX: usize = 6;
const B3_INDEX: usize = 7;
const C3_INDEX: usize = 8;
const A4_INDEX: usize = 9;
const B4_INDEX: usize = 10;
const C4_INDEX: usize = 11;
const D1_INDEX: usize = 0;
const D2_INDEX: usize = 1;
const D3_INDEX: usize = 2;
const D4_INDEX: usize = 3;
const D5_INDEX: usize = 4;
const D6_INDEX: usize = 5;
const E1_INDEX: usize = 0;
const E2_INDEX: usize = 1;
const E3_INDEX: usize = 2;
const E4_INDEX: usize = 3;
const E5_INDEX: usize = 4;
const E6_INDEX: usize = 5;
const L1_CODE: u8 = 0x1a;
const L2_CODE: u8 = 0x21;
const E1_CODE: u8 = 0x1b;
const E2_CODE: u8 = 0x2b;
const G1_CODE: u8 = 0x1c;
const G2_CODE: u8 = 0x2c;
const C1_CODE: u8 = 0x1d;
const C2_CODE: u8 = 0x2d;
const H1_CODE: u8 = 0x1e;
const H2_CODE: u8 = 0x2e;

const PIECE_NUM: usize = 12;

fn main() {
    // ソケット通信
    let host = "localhost";
    let port = 4444;

    // サーバ接続
    let host_and_port = format!("{}:{}", host, port);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();

    // 直接Ipv4を指定
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        match TcpStream::connect(addr) {
            Err(_) => {
                println!("Connection NG.");
            }
            Ok(stream) => {
                println!("Connection Ok.");
                // ソケット通信での処理

                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);

                let mut turn_buffer: Vec<u8> = Vec::new();
                reader
                    .read_until(b'\n', &mut turn_buffer)
                    .expect("Receive failure.");
                // "You are Player1" or "You are Player2"

                let mut is_player1: bool = true;
                if turn_buffer[14] == b'1' {
                    println!("You are Player 1.");
                } else if turn_buffer[14] == b'2' {
                    println!("You are Player 2.");
                    is_player1 = !is_player1;
                }

                loop {
                    let mut buffer: Vec<u8> = Vec::new();
                    // 自分のターンを確認
                    write_socket(&mut writer, "turn");
                    reader
                        .read_until(b'\n', &mut buffer)
                        .expect("Receive failure.");

                    // 自分のターンの場合
                    if buffer[6] == turn_buffer[14] {
                        // 盤面情報を取得
                        let mut board_vec: Vec<u8> = Vec::new();
                        write_socket(&mut writer, "board");
                        reader
                            .read_until(b'\n', &mut board_vec)
                            .expect("Receive failure.");

                        //println!("{}", board);

                        let mut board: Board = make_board(&mut board_vec);

                        // println!("{}", board);

                        // let m: Vec<usize> = vec![B3_INDEX, B2_INDEX];

                        // let moved_board: &mut Board = make_moved_board(&mut board, m, is_player1);

                        // println!("{}", board);

                        let depth = 3;

                        let best_node: Node = nega_scout(&mut board, is_player1, depth);

                        let mut buffer: Vec<u8> = Vec::new();
                        // 自分のターンを確認
                        write_socket(&mut writer, "");
                        reader
                            .read_until(b'\n', &mut buffer)
                            .expect("Receive failure.");
                        //相手のターンが終わるまで待つ
                        loop {
                            let mut buffer: Vec<u8> = Vec::new();
                            write_socket(&mut writer, "turn");
                            reader
                                .read_until(b'\n', &mut buffer)
                                .expect("Receive failure.");
                            // 自分のターンが来たらループを抜ける
                            if buffer[6] == turn_buffer[14] {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn write_socket(writer: &mut BufWriter<&TcpStream>, msg: &str) {
        let buf = format!("{}\n", msg);
        writer.write(buf.as_bytes()).expect("Send failure.");
        let _ = writer.flush();
    }

    fn make_board(board_vec: &mut Vec<u8>) -> Board {
        let mut main_board: Vec<u8> = vec![0x0; 12];
        let mut player1_hand: Vec<u8> = vec![0x0; 6];
        let mut player2_hand: Vec<u8> = vec![0x0; 6];

        let board_str = std::str::from_utf8(&board_vec).unwrap();
        let v: Vec<&str> = board_str.split(",").collect();

        for (i, src) in v.iter().enumerate() {
            let temp: Vec<&str> = src.trim().split(" ").collect();
            if i < PIECE_NUM && temp[1] != "--" {
                match temp[0] {
                    "A1" => main_board[0] = str_to_u8(temp[1]),
                    "B1" => main_board[1] = str_to_u8(temp[1]),
                    "C1" => main_board[2] = str_to_u8(temp[1]),
                    "A2" => main_board[3] = str_to_u8(temp[1]),
                    "B2" => main_board[4] = str_to_u8(temp[1]),
                    "C2" => main_board[5] = str_to_u8(temp[1]),
                    "A3" => main_board[6] = str_to_u8(temp[1]),
                    "B3" => main_board[7] = str_to_u8(temp[1]),
                    "C3" => main_board[8] = str_to_u8(temp[1]),
                    "A4" => main_board[9] = str_to_u8(temp[1]),
                    "B4" => main_board[10] = str_to_u8(temp[1]),
                    "C4" => main_board[11] = str_to_u8(temp[1]),
                    "D1" => player1_hand[0] = str_to_u8(temp[1]),
                    "D2" => player1_hand[1] = str_to_u8(temp[1]),
                    "D3" => player1_hand[2] = str_to_u8(temp[1]),
                    "D4" => player1_hand[3] = str_to_u8(temp[1]),
                    "D5" => player1_hand[4] = str_to_u8(temp[1]),
                    "D6" => player1_hand[5] = str_to_u8(temp[1]),
                    "E1" => player2_hand[0] = str_to_u8(temp[1]),
                    "E2" => player2_hand[1] = str_to_u8(temp[1]),
                    "E3" => player2_hand[2] = str_to_u8(temp[1]),
                    "E4" => player2_hand[3] = str_to_u8(temp[1]),
                    "E5" => player2_hand[4] = str_to_u8(temp[1]),
                    "E6" => player2_hand[5] = str_to_u8(temp[1]),
                    _ => println!("make_board_error"),
                }
            }
        }
        Board {
            main_board,
            player1_hand,
            player2_hand,
        }
    }

    fn str_to_u8(s: &str) -> u8 {
        match s {
            "l1" => L1_CODE,
            "l2" => L2_CODE,
            "e1" => E1_CODE,
            "e2" => E2_CODE,
            "g1" => G1_CODE,
            "g2" => G2_CODE,
            "c1" => C1_CODE,
            "c2" => C2_CODE,
            "h1" => H1_CODE,
            "h2" => H2_CODE,
            _ => 0x00,
        }
    }

    fn make_moved_board(board: &mut Board, move_vec: Vec<usize>, is_player1: bool) -> &mut Board {
        let src = move_vec[0];
        let dst = move_vec[1];
        // ex. 0xb3 -> 0xb2
        let a = 4;
        board.main_board[dst] = board.main_board[src];
        board.main_board[src] = 0x00;
        board
    }

    fn next_move_list(board: &mut Board, is_player1: bool) -> Vec<(usize, Vec<usize>)> {
        let mut next_move_list: Vec<(usize, Vec<usize>)> = vec![];
        let player_num = if is_player1 { 0x1 } else { 0x2 };
        for src in 0..11 {
            let piece = board.main_board[src];
            if piece != NUL && piece / 0xf == player_num {
                let move_vec: Vec<usize> = move_list(src, piece);
                next_move_list.push((src, move_vec));
            }
        }
        next_move_list
    }

    fn move_list(src: usize, piece: u8) -> Vec<usize> {
        let t = (src, piece);
        match t {
            (A1_INDEX, L1_CODE) => vec![B1_INDEX, B2_INDEX, A2_INDEX],
            (A2_INDEX, L1_CODE) => vec![A1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, A3_INDEX],
            (A3_INDEX, L1_CODE) => vec![A2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, A4_INDEX],
            (A4_INDEX, L1_CODE) => vec![A3_INDEX, B3_INDEX, B4_INDEX],
            (B1_INDEX, L1_CODE) => vec![C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, B2_INDEX],
            (B2_INDEX, L1_CODE) => vec![
                B1_INDEX, C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, B3_INDEX,
            ],
            (B3_INDEX, L1_CODE) => vec![
                B2_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX, B4_INDEX,
            ],
            (B4_INDEX, L1_CODE) => vec![B3_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX],
            (C1_INDEX, L1_CODE) => vec![B1_INDEX, B2_INDEX, C2_INDEX],
            (C2_INDEX, L1_CODE) => vec![C1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, C3_INDEX],
            (C3_INDEX, L1_CODE) => vec![C2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, C4_INDEX],
            (C4_INDEX, L1_CODE) => vec![C3_INDEX, B3_INDEX, B4_INDEX],
            (A1_INDEX, L2_CODE) => vec![B1_INDEX, B2_INDEX, A2_INDEX],
            (A2_INDEX, L2_CODE) => vec![A1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, A3_INDEX],
            (A3_INDEX, L2_CODE) => vec![A2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, A4_INDEX],
            (A4_INDEX, L2_CODE) => vec![A3_INDEX, B3_INDEX, B4_INDEX],
            (B1_INDEX, L2_CODE) => vec![C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, B2_INDEX],
            (B2_INDEX, L2_CODE) => vec![
                B1_INDEX, C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, B3_INDEX,
            ],
            (B3_INDEX, L2_CODE) => vec![
                B2_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX, B4_INDEX,
            ],
            (B4_INDEX, L2_CODE) => vec![B3_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX],
            (C1_INDEX, L2_CODE) => vec![B1_INDEX, B2_INDEX, C2_INDEX],
            (C2_INDEX, L2_CODE) => vec![C1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, C3_INDEX],
            (C3_INDEX, L2_CODE) => vec![C2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, C4_INDEX],
            (C4_INDEX, L2_CODE) => vec![C3_INDEX, B3_INDEX, B4_INDEX],
            (A1_INDEX, C1_CODE) => vec![],
            (A2_INDEX, C1_CODE) => vec![A1_INDEX],
            (A3_INDEX, C1_CODE) => vec![A2_INDEX],
            (A4_INDEX, C1_CODE) => vec![A3_INDEX],
            (B1_INDEX, C1_CODE) => vec![],
            (B2_INDEX, C1_CODE) => vec![B1_INDEX],
            (B3_INDEX, C1_CODE) => vec![B2_INDEX],
            (B4_INDEX, C1_CODE) => vec![B3_INDEX],
            (C1_INDEX, C1_CODE) => vec![],
            (C2_INDEX, C1_CODE) => vec![C1_INDEX],
            (C3_INDEX, C1_CODE) => vec![C2_INDEX],
            (C4_INDEX, C1_CODE) => vec![C3_INDEX],
            (A1_INDEX, C2_CODE) => vec![A2_INDEX],
            (A2_INDEX, C2_CODE) => vec![A3_INDEX],
            (A3_INDEX, C2_CODE) => vec![A4_INDEX],
            (A4_INDEX, C2_CODE) => vec![],
            (B1_INDEX, C2_CODE) => vec![B2_INDEX],
            (B2_INDEX, C2_CODE) => vec![B3_INDEX],
            (B3_INDEX, C2_CODE) => vec![B4_INDEX],
            (B4_INDEX, C2_CODE) => vec![],
            (C1_INDEX, C2_CODE) => vec![C2_INDEX],
            (C2_INDEX, C2_CODE) => vec![C3_INDEX],
            (C3_INDEX, C2_CODE) => vec![C4_INDEX],
            (C4_INDEX, C2_CODE) => vec![],
            (A1_INDEX, G1_CODE) => vec![A2_INDEX, B1_INDEX],
            (A2_INDEX, G1_CODE) => vec![A3_INDEX, B2_INDEX, A1_INDEX],
            (A3_INDEX, G1_CODE) => vec![A4_INDEX, B3_INDEX, A2_INDEX],
            (A4_INDEX, G1_CODE) => vec![B4_INDEX, A3_INDEX],
            (B1_INDEX, G1_CODE) => vec![B2_INDEX, C1_INDEX, A1_INDEX],
            (B2_INDEX, G1_CODE) => vec![B3_INDEX, C2_INDEX, A2_INDEX, B1_INDEX],
            (B3_INDEX, G1_CODE) => vec![B4_INDEX, C3_INDEX, A3_INDEX, B2_INDEX],
            (B4_INDEX, G1_CODE) => vec![C4_INDEX, A4_INDEX, B3_INDEX],
            (C1_INDEX, G1_CODE) => vec![C2_INDEX, B1_INDEX],
            (C2_INDEX, G1_CODE) => vec![C3_INDEX, B2_INDEX, C1_INDEX],
            (C3_INDEX, G1_CODE) => vec![C4_INDEX, B3_INDEX, C2_INDEX],
            (C4_INDEX, G1_CODE) => vec![B4_INDEX, C3_INDEX],
            (A1_INDEX, G2_CODE) => vec![A2_INDEX, B1_INDEX],
            (A2_INDEX, G2_CODE) => vec![A3_INDEX, B2_INDEX, A1_INDEX],
            (A3_INDEX, G2_CODE) => vec![A4_INDEX, B3_INDEX, A2_INDEX],
            (A4_INDEX, G2_CODE) => vec![B4_INDEX, A3_INDEX],
            (B1_INDEX, G2_CODE) => vec![B2_INDEX, C1_INDEX, A1_INDEX],
            (B2_INDEX, G2_CODE) => vec![B3_INDEX, C2_INDEX, A2_INDEX, B1_INDEX],
            (B3_INDEX, G2_CODE) => vec![B4_INDEX, C3_INDEX, A3_INDEX, B2_INDEX],
            (B4_INDEX, G2_CODE) => vec![C4_INDEX, A4_INDEX, B3_INDEX],
            (C1_INDEX, G2_CODE) => vec![C2_INDEX, B1_INDEX],
            (C2_INDEX, G2_CODE) => vec![C3_INDEX, B2_INDEX, C1_INDEX],
            (C3_INDEX, G2_CODE) => vec![C4_INDEX, B3_INDEX, C2_INDEX],
            (C4_INDEX, G2_CODE) => vec![B4_INDEX, C3_INDEX],
            (A1_INDEX, E1_CODE) => vec![B2_INDEX],
            (A2_INDEX, E1_CODE) => vec![B3_INDEX, B1_INDEX],
            (A3_INDEX, E1_CODE) => vec![B4_INDEX, B2_INDEX],
            (A4_INDEX, E1_CODE) => vec![B3_INDEX],
            (B1_INDEX, E1_CODE) => vec![C2_INDEX, A2_INDEX],
            (B2_INDEX, E1_CODE) => vec![C3_INDEX, A3_INDEX, A1_INDEX, C1_INDEX],
            (B3_INDEX, E1_CODE) => vec![C4_INDEX, A4_INDEX, A2_INDEX, C2_INDEX],
            (B4_INDEX, E1_CODE) => vec![A3_INDEX, C3_INDEX],
            (C1_INDEX, E1_CODE) => vec![B2_INDEX],
            (C2_INDEX, E1_CODE) => vec![B3_INDEX, B1_INDEX],
            (C3_INDEX, E1_CODE) => vec![B4_INDEX, B2_INDEX],
            (C4_INDEX, E1_CODE) => vec![B3_INDEX],
            (A1_INDEX, E2_CODE) => vec![B2_INDEX],
            (A2_INDEX, E2_CODE) => vec![B3_INDEX, B1_INDEX],
            (A3_INDEX, E2_CODE) => vec![B4_INDEX, B2_INDEX],
            (A4_INDEX, E2_CODE) => vec![B3_INDEX],
            (B1_INDEX, E2_CODE) => vec![C2_INDEX, A2_INDEX],
            (B2_INDEX, E2_CODE) => vec![C3_INDEX, A3_INDEX, A1_INDEX, C1_INDEX],
            (B3_INDEX, E2_CODE) => vec![C4_INDEX, A4_INDEX, A2_INDEX, C2_INDEX],
            (B4_INDEX, E2_CODE) => vec![A3_INDEX, C3_INDEX],
            (C1_INDEX, E2_CODE) => vec![B2_INDEX],
            (C2_INDEX, E2_CODE) => vec![B3_INDEX, B1_INDEX],
            (C3_INDEX, E2_CODE) => vec![B4_INDEX, B2_INDEX],
            (C4_INDEX, E2_CODE) => vec![B3_INDEX],
            (A1_INDEX, H1_CODE) => vec![A2_INDEX, B1_INDEX],
            (A2_INDEX, H1_CODE) => vec![A3_INDEX, B1_INDEX, B2_INDEX, A1_INDEX],
            (A3_INDEX, H1_CODE) => vec![A4_INDEX, B2_INDEX, B3_INDEX, A2_INDEX],
            (A4_INDEX, H1_CODE) => vec![B3_INDEX, B4_INDEX, A3_INDEX],
            (B1_INDEX, H1_CODE) => vec![B2_INDEX, C1_INDEX, A1_INDEX],
            (B2_INDEX, H1_CODE) => vec![B3_INDEX, C1_INDEX, C2_INDEX, A1_INDEX, A2_INDEX, B1_INDEX],
            (B3_INDEX, H1_CODE) => vec![B4_INDEX, C2_INDEX, C3_INDEX, A2_INDEX, A3_INDEX, B2_INDEX],
            (B4_INDEX, H1_CODE) => vec![C3_INDEX, C4_INDEX, A3_INDEX, A4_INDEX, B3_INDEX],
            (C1_INDEX, H1_CODE) => vec![C2_INDEX, B1_INDEX],
            (C2_INDEX, H1_CODE) => vec![C3_INDEX, B1_INDEX, B2_INDEX, C1_INDEX],
            (C3_INDEX, H1_CODE) => vec![C4_INDEX, B2_INDEX, B3_INDEX, C2_INDEX],
            (C4_INDEX, H1_CODE) => vec![B3_INDEX, B4_INDEX, C3_INDEX],
            (A1_INDEX, H2_CODE) => vec![B1_INDEX, B2_INDEX, A2_INDEX],
            (A2_INDEX, H2_CODE) => vec![B2_INDEX, B3_INDEX, A1_INDEX, A3_INDEX],
            (A3_INDEX, H2_CODE) => vec![B3_INDEX, B4_INDEX, A2_INDEX, A4_INDEX],
            (A4_INDEX, H2_CODE) => vec![B4_INDEX, A3_INDEX],
            (B1_INDEX, H2_CODE) => vec![C1_INDEX, C2_INDEX, A2_INDEX, A1_INDEX, B2_INDEX],
            (B2_INDEX, H2_CODE) => vec![C2_INDEX, C3_INDEX, A3_INDEX, A2_INDEX, B1_INDEX, B3_INDEX],
            (B3_INDEX, H2_CODE) => vec![C3_INDEX, C4_INDEX, A4_INDEX, A3_INDEX, B2_INDEX, B4_INDEX],
            (B4_INDEX, H2_CODE) => vec![C4_INDEX, A4_INDEX, B3_INDEX],
            (C1_INDEX, H2_CODE) => vec![B2_INDEX, B1_INDEX, C2_INDEX],
            (C2_INDEX, H2_CODE) => vec![B3_INDEX, B2_INDEX, C1_INDEX, C3_INDEX],
            (C3_INDEX, H2_CODE) => vec![B4_INDEX, B3_INDEX, C2_INDEX, C4_INDEX],
            (C4_INDEX, H2_CODE) => vec![B4_INDEX, C3_INDEX],
            _ => vec![],
        }
    }

    fn nega_scout(board: &mut Board, is_player1: bool, depth: i32) -> Node {
        if depth == 0 {
            let move_vec: Vec<usize> = vec![B3_INDEX, B4_INDEX];
            let point: i16 = 10i16;
            Node { move_vec, point }
        } else {
            let next_move_list = next_move_list(board, is_player1);
            for next_move in next_move_list {
                //let next_board = make_moved_board(board, next_move, is_player1);
            }
            let node: Node = nega_scout(board, is_player1, depth - 1);
            node
        }
    }

    struct Node {
        move_vec: Vec<usize>,
        point: i16,
    }

    struct Board {
        // main board
        main_board: Vec<u8>, // main_board[0] = 0x2c
        player1_hand: Vec<u8>,
        player2_hand: Vec<u8>,
    }

    impl std::fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let _ = writeln!(f, "-----------------");
            for i in 0..6 {
                if self.player2_hand[i] != 0x0 {
                    let _ = write!(f, "{:x} ", self.player2_hand[i]);
                } else {
                    let _ = write!(f, "__ ");
                }
            }
            let _ = writeln!(f, "");
            for i in 0..4 {
                let _ = write!(f, "    ");
                for j in 0..3 {
                    if self.main_board[3 * i + j] != 0xff {
                        let _ = write!(f, "{:x} ", self.main_board[3 * i + j]);
                    } else {
                        let _ = write!(f, "__ ");
                    }
                }
                let _ = writeln!(f, "");
            }
            for i in 0..6 {
                if self.player1_hand[i] != 0x0 {
                    let _ = write!(f, "{:x} ", self.player1_hand[i]);
                } else {
                    let _ = write!(f, "__ ");
                }
            }
            let _ = writeln!(f, "");
            writeln!(f, "-----------------")
        }
    }
}

// struct Board {}
