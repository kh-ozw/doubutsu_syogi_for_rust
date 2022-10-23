#[warn(unused_variables)]
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

mod Bitboard;

const A1_INDEX_DEC: i32 = 0;
const B1_INDEX_DEC: i32 = 1;
const C1_INDEX_DEC: i32 = 2;
const A2_INDEX_DEC: i32 = 3;
const B2_INDEX_DEC: i32 = 4;
const C2_INDEX_DEC: i32 = 5;
const A3_INDEX_DEC: i32 = 6;
const B3_INDEX_DEC: i32 = 7;
const C3_INDEX_DEC: i32 = 8;
const A4_INDEX_DEC: i32 = 9;
const B4_INDEX_DEC: i32 = 10;
const C4_INDEX_DEC: i32 = 11;
const D1_INDEX_DEC: i32 = 12;
const D2_INDEX_DEC: i32 = 13;
const D3_INDEX_DEC: i32 = 14;
const D4_INDEX_DEC: i32 = 15;
const D5_INDEX_DEC: i32 = 16;
const D6_INDEX_DEC: i32 = 17;
const E1_INDEX_DEC: i32 = 18;
const E2_INDEX_DEC: i32 = 19;
const E3_INDEX_DEC: i32 = 20;
const E4_INDEX_DEC: i32 = 21;
const E5_INDEX_DEC: i32 = 22;
const E6_INDEX_DEC: i32 = 23;

const A1_INDEX: i32 = 1 << 0;
const B1_INDEX: i32 = 1 << 1;
const C1_INDEX: i32 = 1 << 2;
const A2_INDEX: i32 = 1 << 3;
const B2_INDEX: i32 = 1 << 4;
const C2_INDEX: i32 = 1 << 5;
const A3_INDEX: i32 = 1 << 6;
const B3_INDEX: i32 = 1 << 7;
const C3_INDEX: i32 = 1 << 8;
const A4_INDEX: i32 = 1 << 9;
const B4_INDEX: i32 = 1 << 10;
const C4_INDEX: i32 = 1 << 11;
const D1_INDEX: i32 = 1 << 12;
const D2_INDEX: i32 = 1 << 13;
const D3_INDEX: i32 = 1 << 14;
const D4_INDEX: i32 = 1 << 15;
const D5_INDEX: i32 = 1 << 16;
const D6_INDEX: i32 = 1 << 17;
const E1_INDEX: i32 = 1 << 18;
const E2_INDEX: i32 = 1 << 19;
const E3_INDEX: i32 = 1 << 20;
const E4_INDEX: i32 = 1 << 21;
const E5_INDEX: i32 = 1 << 22;
const E6_INDEX: i32 = 1 << 23;

const PB_BOARD_POINT: i32 = 1;
const PB_HAND_POINT: i32 = 2;
const BB_BOARD_POINT: i32 = 6;
const BB_HAND_POINT: i32 = 8;
const RB_BOARD_POINT: i32 = 5;
const RB_HAND_POINT: i32 = 7;
const PPB_BOARD_POINT: i32 = 6;

const TRY_MASK1: i32 = 0b111;
const TRY_MASK4: i32 = 0b111 << 9;
const BOARD_MASK: i32 = 0b111_111_111_111;
const HAND_MASK: i32 = 0b111111_111111 << 12;
const D_HAND_MASK: i32 = 0b111111 << 12;
const E_HAND_MASK: i32 = 0b111111 << 18;

// 勝敗判定時のポイント
const WIN_POINT: i32 = 10000;
const LOSE_POINT: i32 = -10000;

// const EVAL_LIST: [[i32; 24]; 4] = [
//     //hiyoko
//     [
//         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, -3, -3, -3, -3, -3, -3,
//     ],
//     //zou
//     [
//         5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, -7, -7, -7, -7, -7, -7,
//     ],
//     //kirin
//     [
//         6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 8, 8, 8, 8, 8, 8, -8, -8, -8, -8, -8, -8,
//     ],
//     // niwatori
//     [
//         6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     ],
// ];

// const PIECE_NUM: usize = 12;
const DEPTH: i32 = 10;

fn main() {
    // test
    // let start = Instant::now();

    // let end = start.elapsed();
    // println!("{} :経過しました。", end.subsec_nanos());

    // let start = Instant::now();
    // for i in 0..100000 {
    //     let mut board = Bitboard::bitboard::Bitboard {
    //         black_b: 1 << 10 + 1,
    //         white_b: 1 << 3 + 1 << 3,
    //         kb: 1,
    //         rb: 1 << 3,
    //         bb: 1 << 5,
    //         pb: 1 << 10,
    //         ppb: 0,
    //     };
    //     let src = 1 << 3;
    //     let dst = 1 << 9;
    //     board.kb = board.kb & !src;
    //     board.rb = board.rb & !src;
    //     board.bb = board.bb & !src;
    //     board.pb = board.pb & !src;
    //     board.ppb = board.ppb & !src;

    //     board.white_b = board.white_b | dst;
    //     board.white_b = board.white_b | dst;
    //     board.white_b = board.white_b | dst;
    //     board.white_b = board.white_b | dst;
    //     board.white_b = board.white_b | dst;
    //     board.white_b = board.white_b | dst;
    // }

    // let end = start.elapsed();
    // println!("{}", end.subsec_nanos());

    // let start = Instant::now();
    // //for i in 0..100000 {
    // let mut board = Bitboard::bitboard::Bitboard {
    //     black_b: (1 << 3) + (1 << 5),
    //     white_b: (1 << 10) + 1,
    //     kb: 1,
    //     rb: 1 << 3,
    //     bb: 1 << 5,
    //     pb: 1 << 10,
    //     ppb: 0,
    // };
    // println!("{:b}", board.white_b);
    // println!("{:b}", board.black_b);
    // println!("{:b}", board.white_b & -board.white_b);
    // println!("{:b}", -1);
    // println!("{:b}", board.white_b - (board.white_b & -board.white_b));
    // println!("{:b}", 0b111 << 6);
    // println!("{}", if board.white_b != 0 { 10 } else { 11 });
    // println!("{:b}", !-board.bb);
    // println!("{}", EVAL_LIST[0][board.kb as usize]);

    // let src = 1 << 3;
    // let dst = 1 << 9;
    // if board.kb & !src != 0 {
    //     board.kb = board.kb & !src;
    //     board.white_b = board.white_b | dst;
    // } else if board.rb & !src != 0 {
    //     board.rb = board.rb & !src;
    //     board.white_b = board.white_b | dst;
    // } else if board.bb & !src != 0 {
    //     board.bb = board.bb & !src;
    //     board.white_b = board.white_b | dst;
    // } else if board.pb & !src != 0 {
    //     board.pb = board.pb & !src;
    //     board.white_b = board.white_b | dst;
    // } else if board.ppb & !src != 0 {
    //     board.ppb = board.ppb & !src;
    //     board.white_b = board.white_b | dst;
    // }
    //}

    // let end = start.elapsed();
    // println!("{}", end.subsec_nanos());

    // let start = Instant::now();
    // for i in 0..100000 {
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    // }

    // let end = start.elapsed();
    // println!("{}", end.subsec_nanos());

    // let mut a = 1 << 12;
    // let b = 1 << 10;
    // let start = Instant::now();
    // for i in 0..100000 {
    //     if a == 0 {
    //         a = a & !b;
    //     } else if a == 2 {
    //         a = a & !b;
    //     } else if a == 3 {
    //         a = a & !b;
    //     } else if a == 4 {
    //         a = a & !b;
    //     } else {
    //         a = a & !b;
    //     }
    // }

    // let end = start.elapsed();
    // println!("{}", end.subsec_nanos());

    // let start = Instant::now();
    // for i in 0..100000 {
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    //     a = a & !b;
    // }

    // let end = start.elapsed();
    // println!("{}", end.subsec_nanos());

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
                let mut bef_board = Bitboard::bitboard::Bitboard {
                    white_b: 0b000_000_000_010,
                    black_b: 0b010_000_000_000,
                    kb: 0b010_000_000_010,
                    rb: 0,
                    bb: 0,
                    pb: 0,
                    ppb: 0,
                };

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

                        let board = make_bitboard(&mut board_vec);
                        let clone_board = board.clone();
                        let depth = DEPTH;
                        // println!("{}", board);
                        let best_node: Node =
                            nega_scout(&board, &bef_board, is_player1, depth, -50000, 50000);

                        let move_str = String::from("mv ")
                            + &get_board_name(best_node.best_move.0)
                            + " "
                            + &get_board_name(best_node.best_move.1);

                        let mut buffer: Vec<u8> = Vec::new();
                        // 自分の手を送信
                        write_socket(&mut writer, &move_str);
                        reader
                            .read_until(b'\n', &mut buffer)
                            .expect("Receive failure.");

                        let next_board = make_moved_board(&board, best_node.best_move, is_player1);
                        //println!("{}", next_board);
                        let point = judge(&next_board, &clone_board, is_player1);
                        bef_board = clone_board;
                        if point == WIN_POINT {
                            println!("you win!");
                            break;
                        } else if point == LOSE_POINT {
                            println!("you lose!");
                            break;
                        }
                        //println!("{}", next_board);
                        //println!("{}, {}", move_str, best_node.point);
                        // println!("---");

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
}

pub fn write_socket(writer: &mut BufWriter<&TcpStream>, msg: &str) {
    let buf = format!("{}\n", msg);
    writer.write(buf.as_bytes()).expect("Send failure.");
    let _ = writer.flush();
}

pub fn make_bitboard(board_vec: &mut Vec<u8>) -> Bitboard::bitboard::Bitboard {
    let mut white_b: i32 = 0;
    let mut black_b: i32 = 0;
    let mut kb: i32 = 0;
    let mut rb: i32 = 0;
    let mut bb: i32 = 0;
    let mut pb: i32 = 0;
    let mut ppb: i32 = 0;
    let board: Vec<&str> = std::str::from_utf8(&board_vec[0..board_vec.len() - 3])
        .unwrap()
        .split(", ")
        .collect();

    for iter in board {
        let b_iter = iter.trim().as_bytes();
        if b_iter[3] != b'-' {
            //ex. "A1 g2"
            let p = piece_to_pos((b_iter[0], b_iter[1]));
            if b_iter[4] == b'1' {
                white_b |= 1 << p;
            } else if b_iter[4] == b'2' {
                black_b |= 1 << p;
            }
            match b_iter[3] {
                b'l' => kb |= 1 << p,
                b'g' => rb |= 1 << p,
                b'e' => bb |= 1 << p,
                b'c' => pb |= 1 << p,
                b'h' => ppb |= 1 << p,
                _ => (),
            }
        }
    }
    // println!("white_b:{:<012b}", white_b);
    // println!("black_b:{:<012b}", black_b);
    // println!("kb:{:<012b}", kb);
    // println!("rb:{:<012b}", rb);
    // println!("bb:{:<012b}", bb);
    // println!("pb:{:<012b}", pb);
    // println!("ppb:{:<012b}", ppb);

    Bitboard::bitboard::Bitboard {
        white_b,
        black_b,
        kb,
        rb,
        bb,
        pb,
        ppb,
    }
}

//pub fn make_board_map(board: &Bitboard::bitboard::Bitboard) -> &mut Vec<u8> {}

pub fn piece_to_pos(s: (u8, u8)) -> i32 {
    match s {
        (b'A', b'1') => A1_INDEX_DEC,
        (b'B', b'1') => B1_INDEX_DEC,
        (b'C', b'1') => C1_INDEX_DEC,
        (b'A', b'2') => A2_INDEX_DEC,
        (b'B', b'2') => B2_INDEX_DEC,
        (b'C', b'2') => C2_INDEX_DEC,
        (b'A', b'3') => A3_INDEX_DEC,
        (b'B', b'3') => B3_INDEX_DEC,
        (b'C', b'3') => C3_INDEX_DEC,
        (b'A', b'4') => A4_INDEX_DEC,
        (b'B', b'4') => B4_INDEX_DEC,
        (b'C', b'4') => C4_INDEX_DEC,
        (b'D', b'1') => D1_INDEX_DEC,
        (b'D', b'2') => D2_INDEX_DEC,
        (b'D', b'3') => D3_INDEX_DEC,
        (b'D', b'4') => D4_INDEX_DEC,
        (b'D', b'5') => D5_INDEX_DEC,
        (b'D', b'6') => D6_INDEX_DEC,
        (b'E', b'1') => E1_INDEX_DEC,
        (b'E', b'2') => E2_INDEX_DEC,
        (b'E', b'3') => E3_INDEX_DEC,
        (b'E', b'4') => E4_INDEX_DEC,
        (b'E', b'5') => E5_INDEX_DEC,
        (b'E', b'6') => E6_INDEX_DEC,
        _ => 0,
    }
}

pub fn pos_to_piece(s: i32) -> (u8, u8) {
    match s {
        A1_INDEX_DEC => (b'A', b'1'),
        B1_INDEX_DEC => (b'B', b'1'),
        C1_INDEX_DEC => (b'C', b'1'),
        A2_INDEX_DEC => (b'A', b'2'),
        B2_INDEX_DEC => (b'B', b'2'),
        C2_INDEX_DEC => (b'C', b'2'),
        A3_INDEX_DEC => (b'A', b'3'),
        B3_INDEX_DEC => (b'B', b'3'),
        C3_INDEX_DEC => (b'C', b'3'),
        A4_INDEX_DEC => (b'A', b'4'),
        B4_INDEX_DEC => (b'B', b'4'),
        C4_INDEX_DEC => (b'C', b'4'),
        D1_INDEX_DEC => (b'D', b'1'),
        D2_INDEX_DEC => (b'D', b'2'),
        D3_INDEX_DEC => (b'D', b'3'),
        D4_INDEX_DEC => (b'D', b'4'),
        D5_INDEX_DEC => (b'D', b'5'),
        D6_INDEX_DEC => (b'D', b'6'),
        E1_INDEX_DEC => (b'E', b'1'),
        E2_INDEX_DEC => (b'E', b'2'),
        E3_INDEX_DEC => (b'E', b'3'),
        E4_INDEX_DEC => (b'E', b'4'),
        E5_INDEX_DEC => (b'E', b'5'),
        E6_INDEX_DEC => (b'E', b'6'),
        _ => (0, 0),
    }
}

pub fn make_moved_board(
    bef_board: &Bitboard::bitboard::Bitboard,
    move_vec: (i32, i32),
    is_player1: bool,
) -> Bitboard::bitboard::Bitboard {
    let src = move_vec.0;
    let dst = move_vec.1;
    let mut board = bef_board.clone();

    // print_move(move_vec);
    // println!("{}", board);
    // プレイヤー1の場合
    if is_player1 {
        //println!("{}", board);
        // 移動先に相手のコマがある場合
        if board.black_b & dst != 0 {
            // 後手の盤面で取られる駒を削除
            board.black_b = board.black_b & !dst;
            //println!("{}", board);
            // 持ち駒に追加する場所に駒を追加
            let hand_posi = (board.white_b & D_HAND_MASK) + (1 << 12);
            board.white_b += hand_posi;
            //println!("{}", board);
            if board.kb & dst != 0 {
                // ライオンの盤面の駒を消し、取った駒を手持ちに加える
                board.kb = (board.kb & !dst) | hand_posi;
            } else if board.rb & dst != 0 {
                // キリンの盤面の駒を消し、取った駒を手持ちに加える
                board.rb = (board.rb & !dst) | hand_posi;
            } else if board.bb & dst != 0 {
                // ゾウの盤面の駒を消し、取った駒を手持ちに加える
                board.bb = (board.bb & !dst) | hand_posi;
            } else if board.pb & dst != 0 {
                // ヒヨコの盤面の駒を消し、取った駒を手持ちに加える
                board.pb = (board.pb & !dst) | hand_posi;
            } else if board.ppb & dst != 0 {
                // ニワトリの盤面の駒を消し、取った駒を手持ちに加える（ヒヨコとして）
                board.ppb = board.ppb & !dst;
                board.pb = board.pb | hand_posi;
            }
            //println!("{}", board);
        }
        // 先手の盤面を更新
        board.white_b = board.white_b & !src | dst;
        // ライオンの盤面を更新
        if board.kb & src != 0 {
            board.kb = board.kb & !src | dst;
        // キリンの盤面を更新
        } else if board.rb & src != 0 {
            board.rb = board.rb & !src | dst;
        // ゾウの盤面を更新
        } else if board.bb & src != 0 {
            board.bb = board.bb & !src | dst;
        // ヒヨコの盤面を更新
        } else if board.pb & src != 0 {
            if (src & D_HAND_MASK == 0) & (dst == A1_INDEX || dst == B1_INDEX || dst == C1_INDEX) {
                board.pb = board.pb & !src;
                board.ppb = board.ppb | dst;
            } else {
                board.pb = board.pb & !src | dst;
            }
        // ニワトリの盤面を更新
        } else if board.ppb & src != 0 {
            board.ppb = board.ppb & !src | dst;
        }
        // println!("{}", board);

        // 打った駒が手ごまの場合
        if src & D_HAND_MASK != 0 {
            let shift_bits = !(src - 1) & D_HAND_MASK;
            // 打った手駒のあった場所より右側に駒があった時、その駒たちをずらす（打った駒のD列の数字より大きい数字のマスに駒があるとき）
            if shift_bits & board.white_b != 0 {
                let non_shift_bits = (src - 1) & D_HAND_MASK;
                board.white_b = (board.white_b & (!D_HAND_MASK | non_shift_bits))
                    | ((board.white_b & shift_bits) >> 1);
                board.rb =
                    (board.rb & (!D_HAND_MASK | non_shift_bits)) | ((board.rb & shift_bits) >> 1);
                board.bb =
                    (board.bb & (!D_HAND_MASK | non_shift_bits)) | ((board.bb & shift_bits) >> 1);
                board.pb =
                    (board.pb & (!D_HAND_MASK | non_shift_bits)) | ((board.pb & shift_bits) >> 1);
            }
        }
    }
    // プレイヤー2の場合
    else {
        //println!("{}", board);
        // 移動先に相手のコマがある場合
        if board.white_b & dst != 0 {
            // 先手の盤面を更新
            board.white_b = board.white_b & !dst;
            //println!("{}", board);
            // 持ち駒に追加
            let hand_posi = (board.black_b & E_HAND_MASK) + (1 << 18);
            board.black_b += hand_posi;
            //println!("{}", board);
            if board.kb & dst != 0 {
                // ライオンの盤面の駒を消し、取った駒を手持ちに加える
                board.kb = (board.kb & !dst) | hand_posi;
            } else if board.rb & dst != 0 {
                // キリンの盤面の駒を消し、取った駒を手持ちに加える
                board.rb = (board.rb & !dst) | hand_posi;
            } else if board.bb & dst != 0 {
                // ゾウの盤面の駒を消し、取った駒を手持ちに加える
                board.bb = (board.bb & !dst) | hand_posi;
            } else if board.pb & dst != 0 {
                // ヒヨコの盤面の駒を消し、取った駒を手持ちに加える
                board.pb = (board.pb & !dst) | hand_posi;
            } else if board.ppb & dst != 0 {
                // ニワトリの盤面の駒を消し、取った駒を手持ちに加える（ヒヨコとして）
                board.ppb = board.ppb & !dst;
                board.pb = board.pb | hand_posi;
            }
            //println!("{}", board);
        }
        // 後手の盤面を更新
        board.black_b = board.black_b & !src | dst;
        //println!("{}", board);
        // ライオンの盤面を更新
        if board.kb & src != 0 {
            board.kb = board.kb & !src | dst;
        // キリンの盤面を更新
        } else if board.rb & src != 0 {
            board.rb = board.rb & !src | dst;
        // ゾウの盤面を更新
        } else if board.bb & src != 0 {
            board.bb = board.bb & !src | dst;
        // ヒヨコの盤面を更新
        } else if board.pb & src != 0 {
            if (src & E_HAND_MASK == 0) & (dst == A4_INDEX || dst == B4_INDEX || dst == C4_INDEX) {
                board.pb = board.pb & !src;
                board.ppb = board.ppb | dst;
            } else {
                board.pb = board.pb & !src | dst;
            }
        // ニワトリの盤面を更新
        } else if board.ppb & src != 0 {
            board.ppb = board.ppb & !src | dst;
        }
        //println!("{}", board);

        // 打った駒が手ごまの場合
        if src & E_HAND_MASK != 0 {
            let shift_bits = !(src - 1) & E_HAND_MASK;
            // 打った手駒のあった場所より右側に駒があった時、その駒たちをずらす（打った駒のD列の数字より大きい数字のマスに駒があるとき）
            if shift_bits & board.black_b != 0 {
                let non_shift_bits = (src - 1) & E_HAND_MASK;
                board.black_b = (board.black_b & (!E_HAND_MASK | non_shift_bits))
                    | ((board.black_b & shift_bits) >> 1);
                board.rb =
                    (board.rb & (!E_HAND_MASK | non_shift_bits)) | ((board.rb & shift_bits) >> 1);
                board.bb =
                    (board.bb & (!E_HAND_MASK | non_shift_bits)) | ((board.bb & shift_bits) >> 1);
                board.pb =
                    (board.pb & (!E_HAND_MASK | non_shift_bits)) | ((board.pb & shift_bits) >> 1);
            }
        }
    }
    board
}

pub fn next_move_list(board: &Bitboard::bitboard::Bitboard, is_player1: bool) -> Vec<(i32, i32)> {
    let mut next_move_list: Vec<(i32, i32)> = vec![];

    if is_player1 {
        let player_board = board.white_b;

        // 1pひよこの手探索
        let pb_board = player_board & board.pb;
        let target_board = pb_board & -pb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                // A1_INDEX => _,
                A2_INDEX => {
                    // 移動先に自分のコマがなければ、移動先に追加
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX))
                    }
                }
                A3_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX))
                    }
                }
                A4_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX))
                    }
                }
                //B1_INDEX => _,
                B2_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX))
                    }
                }
                B3_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX))
                    }
                }
                B4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX))
                    }
                }
                //C1_INDEX => _,
                C2_INDEX => {
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX))
                    }
                }
                C3_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX))
                    }
                }
                C4_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX))
                    }
                }
                _ => (),
            }
            let target_board = pb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    // A1_INDEX => _,
                    A2_INDEX => {
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A1_INDEX))
                        }
                    }
                    A3_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A2_INDEX))
                        }
                    }
                    A4_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, A3_INDEX))
                        }
                    }
                    //B1_INDEX => _,
                    B2_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX))
                        }
                    }
                    B3_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B2_INDEX))
                        }
                    }
                    B4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, B3_INDEX))
                        }
                    }
                    //C1_INDEX => _,
                    C2_INDEX => {
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C1_INDEX))
                        }
                    }
                    C3_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C2_INDEX))
                        }
                    }
                    C4_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, C3_INDEX))
                        }
                    }
                    _ => (),
                }
            }
        }

        // 1pゾウの手探索
        let bb_board = player_board & board.bb;
        let target_board = bb_board & -bb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX))
                    }
                }
                A2_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B3_INDEX))
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B1_INDEX))
                    }
                }
                A3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B4_INDEX))
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B2_INDEX))
                    }
                }
                A4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B3_INDEX))
                    }
                }
                B1_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C2_INDEX))
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A2_INDEX))
                    }
                }
                B2_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C3_INDEX))
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A3_INDEX))
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A1_INDEX))
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C1_INDEX))
                    }
                }
                B3_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C4_INDEX))
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A4_INDEX))
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A2_INDEX))
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C2_INDEX))
                    }
                }
                B4_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A3_INDEX))
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C3_INDEX))
                    }
                }
                C1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B2_INDEX))
                    }
                }
                C2_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B3_INDEX))
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B1_INDEX))
                    }
                }
                C3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B4_INDEX))
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B2_INDEX))
                    }
                }
                C4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B3_INDEX))
                    }
                }
                _ => (),
            }
            let target_board = bb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    A1_INDEX => {
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B2_INDEX))
                        }
                    }
                    A2_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B3_INDEX))
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B1_INDEX))
                        }
                    }
                    A3_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B4_INDEX))
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B2_INDEX))
                        }
                    }
                    A4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B3_INDEX))
                        }
                    }
                    B1_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C2_INDEX))
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A2_INDEX))
                        }
                    }
                    B2_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C3_INDEX))
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A3_INDEX))
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A1_INDEX))
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C1_INDEX))
                        }
                    }
                    B3_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C4_INDEX))
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A4_INDEX))
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A2_INDEX))
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C2_INDEX))
                        }
                    }
                    B4_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A3_INDEX))
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C3_INDEX))
                        }
                    }
                    C1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B2_INDEX))
                        }
                    }
                    C2_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B3_INDEX))
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B1_INDEX))
                        }
                    }
                    C3_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B4_INDEX))
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B2_INDEX))
                        }
                    }
                    C4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B3_INDEX))
                        }
                    }
                    _ => (),
                }
            }
        }

        // 1pキリンの手探索
        let rb_board = player_board & board.rb;
        let target_board = rb_board & -rb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B1_INDEX));
                    }
                }
                A2_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B2_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX));
                    }
                }
                A3_INDEX => {
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B3_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX));
                    }
                }
                A4_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B4_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX));
                    }
                }
                B1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A1_INDEX));
                    }
                }
                B2_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX));
                    }
                }
                B3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX));
                    }
                }
                B4_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX));
                    }
                }
                C1_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B1_INDEX));
                    }
                }
                C2_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX));
                    }
                }
                C3_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B3_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX));
                    }
                }
                C4_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX));
                    }
                }
                _ => (),
            }
            let target_board = rb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    A1_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B1_INDEX));
                        }
                    }
                    A2_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B2_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A1_INDEX));
                        }
                    }
                    A3_INDEX => {
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B3_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A2_INDEX));
                        }
                    }
                    A4_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B4_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, A3_INDEX));
                        }
                    }
                    B1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C1_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A1_INDEX));
                        }
                    }
                    B2_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B3_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C2_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX));
                        }
                    }
                    B3_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B1_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C3_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B2_INDEX));
                        }
                    }
                    B4_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C4_INDEX));
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, B3_INDEX));
                        }
                    }
                    C1_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, C2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B1_INDEX));
                        }
                    }
                    C2_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C1_INDEX));
                        }
                    }
                    C3_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B3_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C2_INDEX));
                        }
                    }
                    C4_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, C3_INDEX));
                        }
                    }
                    _ => (),
                }
            }
        }

        // 1pライオンの手探索
        let target_board = player_board & board.kb;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX));
                    }
                }
                A2_INDEX => {
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX));
                    }
                }
                A3_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX));
                    }
                }
                A4_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B4_INDEX));
                    }
                }
                B1_INDEX => {
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A1_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, B2_INDEX));
                    }
                }
                B2_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A1_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A2_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A3_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX));
                    }
                }
                B3_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A2_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A4_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX));
                    }
                }
                B4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A4_INDEX));
                    }
                }
                C1_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B2_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX));
                    }
                }
                C2_INDEX => {
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B3_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX));
                    }
                }
                C3_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B4_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX));
                    }
                }
                C4_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B4_INDEX));
                    }
                }
                _ => (),
            }
        }

        // 1pニワトリの手探索
        let ppb_board = player_board & board.ppb;
        let target_board = ppb_board & -ppb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B1_INDEX));
                    }
                }
                A2_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B2_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX));
                    }
                }
                A3_INDEX => {
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B3_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX));
                    }
                }
                A4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B4_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX));
                    }
                }
                B1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A1_INDEX));
                    }
                }
                B2_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C1_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C2_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A1_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX));
                    }
                }
                B3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C2_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C3_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A2_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX));
                    }
                }
                B4_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C4_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A3_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX));
                    }
                }
                C1_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B1_INDEX));
                    }
                }
                C2_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX));
                    }
                }
                C3_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B3_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX));
                    }
                }
                C4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX));
                    }
                }
                _ => (),
            }
            let target_board = ppb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    A1_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B1_INDEX));
                        }
                    }
                    A2_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A3_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B1_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B2_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A1_INDEX));
                        }
                    }
                    A3_INDEX => {
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A4_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B2_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B3_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A2_INDEX));
                        }
                    }
                    A4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B3_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B4_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, A3_INDEX));
                        }
                    }
                    B1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C1_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A1_INDEX));
                        }
                    }
                    B2_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B3_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C1_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C2_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A1_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX));
                        }
                    }
                    B3_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B4_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C2_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C3_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A2_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B2_INDEX));
                        }
                    }
                    B4_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C3_INDEX));
                        }
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C4_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A3_INDEX));
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, B3_INDEX));
                        }
                    }
                    C1_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, C2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B1_INDEX));
                        }
                    }
                    C2_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C3_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B1_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C1_INDEX));
                        }
                    }
                    C3_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C4_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B2_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B3_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C2_INDEX));
                        }
                    }
                    C4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B3_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, C3_INDEX));
                        }
                    }
                    _ => (),
                }
            }
        }

        // 持ち駒を打つ場合
        let target_board = !(board.black_b | board.white_b);

        // 1pヒヨコ
        if board.pb & D_HAND_MASK != 0 {
            // println!("{:b}", board.pb & D_HAND_MASK);
            // println!("{:b}", -(board.pb & D_HAND_MASK));
            // println!("{}", (board.pb & D_HAND_MASK) & -(board.pb & D_HAND_MASK));
            let hand_index = (board.pb & D_HAND_MASK) & -(board.pb & D_HAND_MASK);
            // println!("{}", board);
            // println!("{}", target_board & A1_INDEX);
            if target_board & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if target_board & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if target_board & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if target_board & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if target_board & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if target_board & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if target_board & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if target_board & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if target_board & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if target_board & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if target_board & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if target_board & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 1pゾウ
        if board.bb & D_HAND_MASK != 0 {
            let hand_index = (board.bb & D_HAND_MASK) & -(board.bb & D_HAND_MASK);
            if target_board & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if target_board & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if target_board & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if target_board & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if target_board & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if target_board & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if target_board & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if target_board & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if target_board & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if target_board & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if target_board & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if target_board & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 1pキリン
        if board.rb & D_HAND_MASK != 0 {
            let hand_index = (board.rb & D_HAND_MASK) & -(board.rb & D_HAND_MASK);
            if target_board & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if target_board & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if target_board & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if target_board & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if target_board & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if target_board & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if target_board & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if target_board & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if target_board & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if target_board & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if target_board & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if target_board & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
    } else {
        let player_board = board.black_b;
        // 2pひよこの手探索
        let pb_board = player_board & board.pb;
        let target_board = pb_board & -pb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX))
                    }
                }
                A2_INDEX => {
                    // 移動先に自分のコマがなければ、移動先に追加
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX))
                    }
                }
                A3_INDEX => {
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX))
                    }
                }
                //A4_INDEX => _
                B1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B2_INDEX))
                    }
                }
                B2_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX))
                    }
                }
                B3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX))
                    }
                }
                //B4_INDEX => _,
                C1_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX))
                    }
                }
                C2_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX))
                    }
                }
                C3_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX))
                    }
                }
                //C4_INDEX => _,
                _ => (),
            }
            let target_board = pb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    // A1_INDEX => _,
                    A2_INDEX => {
                        // 移動先に自分のコマがなければ、移動先に追加
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A1_INDEX))
                        }
                    }
                    A3_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A2_INDEX))
                        }
                    }
                    A4_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, A3_INDEX))
                        }
                    }
                    //B1_INDEX => _,
                    B2_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX))
                        }
                    }
                    B3_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B2_INDEX))
                        }
                    }
                    B4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, B3_INDEX))
                        }
                    }
                    //C1_INDEX => _,
                    C2_INDEX => {
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C1_INDEX))
                        }
                    }
                    C3_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C2_INDEX))
                        }
                    }
                    C4_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, C3_INDEX))
                        }
                    }
                    //C4_INDEX => _,
                    _ => (),
                }
            }
        }

        // 2pゾウの手探索
        let bb_board = player_board & board.bb;
        let target_board = bb_board & -bb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX))
                    }
                }
                A2_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B3_INDEX))
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B1_INDEX))
                    }
                }
                A3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B4_INDEX))
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B2_INDEX))
                    }
                }
                A4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B3_INDEX))
                    }
                }
                B1_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C2_INDEX))
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A2_INDEX))
                    }
                }
                B2_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C3_INDEX))
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A3_INDEX))
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A1_INDEX))
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C1_INDEX))
                    }
                }
                B3_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C4_INDEX))
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A4_INDEX))
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A2_INDEX))
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C2_INDEX))
                    }
                }
                B4_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A3_INDEX))
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C3_INDEX))
                    }
                }
                C1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B2_INDEX))
                    }
                }
                C2_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B3_INDEX))
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B1_INDEX))
                    }
                }
                C3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B4_INDEX))
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B2_INDEX))
                    }
                }
                C4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B3_INDEX))
                    }
                }
                _ => (),
            }
            let target_board = bb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    A1_INDEX => {
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B2_INDEX))
                        }
                    }
                    A2_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B3_INDEX))
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B1_INDEX))
                        }
                    }
                    A3_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B4_INDEX))
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B2_INDEX))
                        }
                    }
                    A4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B3_INDEX))
                        }
                    }
                    B1_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C2_INDEX))
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A2_INDEX))
                        }
                    }
                    B2_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C3_INDEX))
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A3_INDEX))
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A1_INDEX))
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C1_INDEX))
                        }
                    }
                    B3_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C4_INDEX))
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A4_INDEX))
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A2_INDEX))
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C2_INDEX))
                        }
                    }
                    B4_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A3_INDEX))
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C3_INDEX))
                        }
                    }
                    C1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B2_INDEX))
                        }
                    }
                    C2_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B3_INDEX))
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B1_INDEX))
                        }
                    }
                    C3_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B4_INDEX))
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B2_INDEX))
                        }
                    }
                    C4_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B3_INDEX))
                        }
                    }
                    _ => (),
                }
            }
        }

        // 2pキリンの手探索
        let rb_board = player_board & board.rb;
        let target_board = rb_board & -rb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B1_INDEX));
                    }
                }
                A2_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B2_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX));
                    }
                }
                A3_INDEX => {
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B3_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX));
                    }
                }
                A4_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B4_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX));
                    }
                }
                B1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A1_INDEX));
                    }
                }
                B2_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX));
                    }
                }
                B3_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX));
                    }
                }
                B4_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX));
                    }
                }
                C1_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B1_INDEX));
                    }
                }
                C2_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX));
                    }
                }
                C3_INDEX => {
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B3_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX));
                    }
                }
                C4_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX));
                    }
                }
                _ => (),
            }
            let target_board = rb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    A1_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B1_INDEX));
                        }
                    }
                    A2_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B2_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A1_INDEX));
                        }
                    }
                    A3_INDEX => {
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B3_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A2_INDEX));
                        }
                    }
                    A4_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B4_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, A3_INDEX));
                        }
                    }
                    B1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C1_INDEX));
                        }
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A1_INDEX));
                        }
                    }
                    B2_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B3_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C2_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX));
                        }
                    }
                    B3_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B1_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C3_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B2_INDEX));
                        }
                    }
                    B4_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C4_INDEX));
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, B3_INDEX));
                        }
                    }
                    C1_INDEX => {
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, C2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B1_INDEX));
                        }
                    }
                    C2_INDEX => {
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C1_INDEX));
                        }
                    }
                    C3_INDEX => {
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B3_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C2_INDEX));
                        }
                    }
                    C4_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, C3_INDEX));
                        }
                    }
                    _ => (),
                }
            }
        }

        // 2pライオンの手探索
        let target_board = player_board & board.kb;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX));
                    }
                }
                A2_INDEX => {
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX));
                    }
                }
                A3_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX));
                    }
                }
                A4_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B4_INDEX));
                    }
                }
                B1_INDEX => {
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A1_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, B2_INDEX));
                    }
                }
                B2_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C1_INDEX));
                    }
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A1_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A2_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A3_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX));
                    }
                }
                B3_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C2_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A2_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A4_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX));
                    }
                }
                B4_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C3_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C4_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A4_INDEX));
                    }
                }
                C1_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B2_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX));
                    }
                }
                C2_INDEX => {
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B3_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX));
                    }
                }
                C3_INDEX => {
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B4_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX));
                    }
                }
                C4_INDEX => {
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B4_INDEX));
                    }
                }
                _ => (),
            }
        }

        // 2pニワトリの手探索
        let ppb_board = player_board & board.ppb;
        let target_board = ppb_board & -ppb_board;
        if target_board != 0 {
            // 1つ目のコマの探索
            match target_board {
                A1_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX));
                    }
                }
                A2_INDEX => {
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A1_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, A3_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A2_INDEX, B3_INDEX));
                    }
                }
                A3_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A2_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A3_INDEX, B4_INDEX));
                    }
                }
                A4_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((A4_INDEX, A3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((A4_INDEX, B4_INDEX));
                    }
                }
                B1_INDEX => {
                    if player_board & A1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A1_INDEX));
                    }
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, A2_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, B2_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C1_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B1_INDEX, C2_INDEX));
                    }
                }
                B2_INDEX => {
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A2_INDEX));
                    }
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, A3_INDEX));
                    }
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B1_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, B3_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C2_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B2_INDEX, C3_INDEX));
                    }
                }
                B3_INDEX => {
                    if player_board & A3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A3_INDEX));
                    }
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, A4_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B2_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B3_INDEX, C4_INDEX));
                    }
                }
                B4_INDEX => {
                    if player_board & A4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, A4_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((B4_INDEX, B3_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((B4_INDEX, C4_INDEX));
                    }
                }
                C1_INDEX => {
                    if player_board & B1_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B1_INDEX));
                    }
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, B2_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C1_INDEX, C2_INDEX));
                    }
                }
                C2_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B2_INDEX));
                    }
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, B3_INDEX));
                    }
                    if player_board & C1_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C1_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C2_INDEX, C3_INDEX));
                    }
                }
                C3_INDEX => {
                    if player_board & B3_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B3_INDEX));
                    }
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, B4_INDEX));
                    }
                    if player_board & C2_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C2_INDEX));
                    }
                    if player_board & C4_INDEX == 0 {
                        next_move_list.push((C3_INDEX, C4_INDEX));
                    }
                }
                C4_INDEX => {
                    if player_board & B4_INDEX == 0 {
                        next_move_list.push((C4_INDEX, B4_INDEX));
                    }
                    if player_board & C3_INDEX == 0 {
                        next_move_list.push((C4_INDEX, C3_INDEX));
                    }
                }
                _ => (),
            }
            let target_board = ppb_board - target_board;
            if target_board != 0 {
                // 2つ目のコマの探索
                match target_board {
                    A1_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, A2_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B1_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B2_INDEX));
                        }
                    }
                    A2_INDEX => {
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A1_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A2_INDEX, A3_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B2_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A2_INDEX, B3_INDEX));
                        }
                    }
                    A3_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A2_INDEX));
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B3_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A3_INDEX, B4_INDEX));
                        }
                    }
                    A4_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((A4_INDEX, A3_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((A4_INDEX, B4_INDEX));
                        }
                    }
                    B1_INDEX => {
                        if player_board & A1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A1_INDEX));
                        }
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, A2_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, B2_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C1_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B1_INDEX, C2_INDEX));
                        }
                    }
                    B2_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A2_INDEX));
                        }
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, A3_INDEX));
                        }
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B1_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, B3_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C2_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B2_INDEX, C3_INDEX));
                        }
                    }
                    B3_INDEX => {
                        if player_board & A3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A3_INDEX));
                        }
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, A4_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B2_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C3_INDEX));
                        }
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B3_INDEX, C4_INDEX));
                        }
                    }
                    B4_INDEX => {
                        if player_board & A4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, A4_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((B4_INDEX, B3_INDEX));
                        }
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((B4_INDEX, C4_INDEX));
                        }
                    }
                    C1_INDEX => {
                        if player_board & B1_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B1_INDEX));
                        }
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, B2_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C1_INDEX, C2_INDEX));
                        }
                    }
                    C2_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B2_INDEX));
                        }
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, B3_INDEX));
                        }
                        if player_board & C1_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C1_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C2_INDEX, C3_INDEX));
                        }
                    }
                    C3_INDEX => {
                        if player_board & B3_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B3_INDEX));
                        }
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, B4_INDEX));
                        }
                        if player_board & C2_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C2_INDEX));
                        }
                        if player_board & C4_INDEX == 0 {
                            next_move_list.push((C3_INDEX, C4_INDEX));
                        }
                    }
                    C4_INDEX => {
                        if player_board & B4_INDEX == 0 {
                            next_move_list.push((C4_INDEX, B4_INDEX));
                        }
                        if player_board & C3_INDEX == 0 {
                            next_move_list.push((C4_INDEX, C3_INDEX));
                        }
                    }
                    _ => (),
                }
            }
        }

        // 持ち駒を打つ場合
        let target_board = !(board.black_b | board.white_b);

        // 2pヒヨコ
        if board.pb & E_HAND_MASK != 0 {
            let hand_index = (board.pb & E_HAND_MASK) & -(board.pb & E_HAND_MASK);
            if target_board & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if target_board & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if target_board & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if target_board & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if target_board & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if target_board & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if target_board & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if target_board & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if target_board & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if target_board & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if target_board & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if target_board & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 2pゾウ
        if board.bb & E_HAND_MASK != 0 {
            let hand_index = (board.bb & E_HAND_MASK) & -(board.bb & E_HAND_MASK);
            if target_board & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if target_board & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if target_board & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if target_board & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if target_board & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if target_board & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if target_board & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if target_board & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if target_board & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if target_board & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if target_board & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if target_board & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 2pキリン
        if board.rb & E_HAND_MASK != 0 {
            let hand_index = (board.rb & E_HAND_MASK) & -(board.rb & E_HAND_MASK);
            if target_board & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if target_board & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if target_board & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if target_board & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if target_board & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if target_board & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if target_board & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if target_board & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if target_board & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if target_board & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if target_board & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if target_board & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
    }
    next_move_list
}

//pub fn move_list(src: usize, piece: u8) -> Vec<usize> {
//     let t = (src, piece);
//     match t {
//         (A1_INDEX, L1_CODE) => vec![B1_INDEX, B2_INDEX, A2_INDEX],
//         (A2_INDEX, L1_CODE) => vec![A1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, A3_INDEX],
//         (A3_INDEX, L1_CODE) => vec![A2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, A4_INDEX],
//         (A4_INDEX, L1_CODE) => vec![A3_INDEX, B3_INDEX, B4_INDEX],
//         (B1_INDEX, L1_CODE) => vec![C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, B2_INDEX],
//         (B2_INDEX, L1_CODE) => vec![
//             B1_INDEX, C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, B3_INDEX,
//         ],
//         (B3_INDEX, L1_CODE) => vec![
//             B2_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX, B4_INDEX,
//         ],
//         (B4_INDEX, L1_CODE) => vec![B3_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX],
//         (C1_INDEX, L1_CODE) => vec![B1_INDEX, B2_INDEX, C2_INDEX],
//         (C2_INDEX, L1_CODE) => vec![C1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, C3_INDEX],
//         (C3_INDEX, L1_CODE) => vec![C2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, C4_INDEX],
//         (C4_INDEX, L1_CODE) => vec![C3_INDEX, B3_INDEX, B4_INDEX],
//         (A1_INDEX, L2_CODE) => vec![B1_INDEX, B2_INDEX, A2_INDEX],
//         (A2_INDEX, L2_CODE) => vec![A1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, A3_INDEX],
//         (A3_INDEX, L2_CODE) => vec![A2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, A4_INDEX],
//         (A4_INDEX, L2_CODE) => vec![A3_INDEX, B3_INDEX, B4_INDEX],
//         (B1_INDEX, L2_CODE) => vec![C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, B2_INDEX],
//         (B2_INDEX, L2_CODE) => vec![
//             B1_INDEX, C1_INDEX, A1_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, B3_INDEX,
//         ],
//         (B3_INDEX, L2_CODE) => vec![
//             B2_INDEX, C2_INDEX, A2_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX, B4_INDEX,
//         ],
//         (B4_INDEX, L2_CODE) => vec![B3_INDEX, C3_INDEX, A3_INDEX, C4_INDEX, A4_INDEX],
//         (C1_INDEX, L2_CODE) => vec![B1_INDEX, B2_INDEX, C2_INDEX],
//         (C2_INDEX, L2_CODE) => vec![C1_INDEX, B1_INDEX, B2_INDEX, B3_INDEX, C3_INDEX],
//         (C3_INDEX, L2_CODE) => vec![C2_INDEX, B2_INDEX, B3_INDEX, B4_INDEX, C4_INDEX],
//         (C4_INDEX, L2_CODE) => vec![C3_INDEX, B3_INDEX, B4_INDEX],
//         (A1_INDEX, C1_CODE) => vec![],
//         (A2_INDEX, C1_CODE) => vec![A1_INDEX],
//         (A3_INDEX, C1_CODE) => vec![A2_INDEX],
//         (A4_INDEX, C1_CODE) => vec![A3_INDEX],
//         (B1_INDEX, C1_CODE) => vec![],
//         (B2_INDEX, C1_CODE) => vec![B1_INDEX],
//         (B3_INDEX, C1_CODE) => vec![B2_INDEX],
//         (B4_INDEX, C1_CODE) => vec![B3_INDEX],
//         (C1_INDEX, C1_CODE) => vec![],
//         (C2_INDEX, C1_CODE) => vec![C1_INDEX],
//         (C3_INDEX, C1_CODE) => vec![C2_INDEX],
//         (C4_INDEX, C1_CODE) => vec![C3_INDEX],
//         (A1_INDEX, C2_CODE) => vec![A2_INDEX],
//         (A2_INDEX, C2_CODE) => vec![A3_INDEX],
//         (A3_INDEX, C2_CODE) => vec![A4_INDEX],
//         (A4_INDEX, C2_CODE) => vec![],
//         (B1_INDEX, C2_CODE) => vec![B2_INDEX],
//         (B2_INDEX, C2_CODE) => vec![B3_INDEX],
//         (B3_INDEX, C2_CODE) => vec![B4_INDEX],
//         (B4_INDEX, C2_CODE) => vec![],
//         (C1_INDEX, C2_CODE) => vec![C2_INDEX],
//         (C2_INDEX, C2_CODE) => vec![C3_INDEX],
//         (C3_INDEX, C2_CODE) => vec![C4_INDEX],
//         (C4_INDEX, C2_CODE) => vec![],
//         (A1_INDEX, G1_CODE) => vec![A2_INDEX, B1_INDEX],
//         (A2_INDEX, G1_CODE) => vec![A3_INDEX, B2_INDEX, A1_INDEX],
//         (A3_INDEX, G1_CODE) => vec![A4_INDEX, B3_INDEX, A2_INDEX],
//         (A4_INDEX, G1_CODE) => vec![B4_INDEX, A3_INDEX],
//         (B1_INDEX, G1_CODE) => vec![B2_INDEX, C1_INDEX, A1_INDEX],
//         (B2_INDEX, G1_CODE) => vec![B3_INDEX, C2_INDEX, A2_INDEX, B1_INDEX],
//         (B3_INDEX, G1_CODE) => vec![B4_INDEX, C3_INDEX, A3_INDEX, B2_INDEX],
//         (B4_INDEX, G1_CODE) => vec![C4_INDEX, A4_INDEX, B3_INDEX],
//         (C1_INDEX, G1_CODE) => vec![C2_INDEX, B1_INDEX],
//         (C2_INDEX, G1_CODE) => vec![C3_INDEX, B2_INDEX, C1_INDEX],
//         (C3_INDEX, G1_CODE) => vec![C4_INDEX, B3_INDEX, C2_INDEX],
//         (C4_INDEX, G1_CODE) => vec![B4_INDEX, C3_INDEX],
//         (A1_INDEX, G2_CODE) => vec![A2_INDEX, B1_INDEX],
//         (A2_INDEX, G2_CODE) => vec![A3_INDEX, B2_INDEX, A1_INDEX],
//         (A3_INDEX, G2_CODE) => vec![A4_INDEX, B3_INDEX, A2_INDEX],
//         (A4_INDEX, G2_CODE) => vec![B4_INDEX, A3_INDEX],
//         (B1_INDEX, G2_CODE) => vec![B2_INDEX, C1_INDEX, A1_INDEX],
//         (B2_INDEX, G2_CODE) => vec![B3_INDEX, C2_INDEX, A2_INDEX, B1_INDEX],
//         (B3_INDEX, G2_CODE) => vec![B4_INDEX, C3_INDEX, A3_INDEX, B2_INDEX],
//         (B4_INDEX, G2_CODE) => vec![C4_INDEX, A4_INDEX, B3_INDEX],
//         (C1_INDEX, G2_CODE) => vec![C2_INDEX, B1_INDEX],
//         (C2_INDEX, G2_CODE) => vec![C3_INDEX, B2_INDEX, C1_INDEX],
//         (C3_INDEX, G2_CODE) => vec![C4_INDEX, B3_INDEX, C2_INDEX],
//         (C4_INDEX, G2_CODE) => vec![B4_INDEX, C3_INDEX],
//         (A1_INDEX, E1_CODE) => vec![B2_INDEX],
//         (A2_INDEX, E1_CODE) => vec![B3_INDEX, B1_INDEX],
//         (A3_INDEX, E1_CODE) => vec![B4_INDEX, B2_INDEX],
//         (A4_INDEX, E1_CODE) => vec![B3_INDEX],
//         (B1_INDEX, E1_CODE) => vec![C2_INDEX, A2_INDEX],
//         (B2_INDEX, E1_CODE) => vec![C3_INDEX, A3_INDEX, A1_INDEX, C1_INDEX],
//         (B3_INDEX, E1_CODE) => vec![C4_INDEX, A4_INDEX, A2_INDEX, C2_INDEX],
//         (B4_INDEX, E1_CODE) => vec![A3_INDEX, C3_INDEX],
//         (C1_INDEX, E1_CODE) => vec![B2_INDEX],
//         (C2_INDEX, E1_CODE) => vec![B3_INDEX, B1_INDEX],
//         (C3_INDEX, E1_CODE) => vec![B4_INDEX, B2_INDEX],
//         (C4_INDEX, E1_CODE) => vec![B3_INDEX],
//         (A1_INDEX, E2_CODE) => vec![B2_INDEX],
//         (A2_INDEX, E2_CODE) => vec![B3_INDEX, B1_INDEX],
//         (A3_INDEX, E2_CODE) => vec![B4_INDEX, B2_INDEX],
//         (A4_INDEX, E2_CODE) => vec![B3_INDEX],
//         (B1_INDEX, E2_CODE) => vec![C2_INDEX, A2_INDEX],
//         (B2_INDEX, E2_CODE) => vec![C3_INDEX, A3_INDEX, A1_INDEX, C1_INDEX],
//         (B3_INDEX, E2_CODE) => vec![C4_INDEX, A4_INDEX, A2_INDEX, C2_INDEX],
//         (B4_INDEX, E2_CODE) => vec![A3_INDEX, C3_INDEX],
//         (C1_INDEX, E2_CODE) => vec![B2_INDEX],
//         (C2_INDEX, E2_CODE) => vec![B3_INDEX, B1_INDEX],
//         (C3_INDEX, E2_CODE) => vec![B4_INDEX, B2_INDEX],
//         (C4_INDEX, E2_CODE) => vec![B3_INDEX],
//         (A1_INDEX, H1_CODE) => vec![A2_INDEX, B1_INDEX],
//         (A2_INDEX, H1_CODE) => vec![A3_INDEX, B1_INDEX, B2_INDEX, A1_INDEX],
//         (A3_INDEX, H1_CODE) => vec![A4_INDEX, B2_INDEX, B3_INDEX, A2_INDEX],
//         (A4_INDEX, H1_CODE) => vec![B3_INDEX, B4_INDEX, A3_INDEX],
//         (B1_INDEX, H1_CODE) => vec![B2_INDEX, C1_INDEX, A1_INDEX],
//         (B2_INDEX, H1_CODE) => vec![B3_INDEX, C1_INDEX, C2_INDEX, A1_INDEX, A2_INDEX, B1_INDEX],
//         (B3_INDEX, H1_CODE) => vec![B4_INDEX, C2_INDEX, C3_INDEX, A2_INDEX, A3_INDEX, B2_INDEX],
//         (B4_INDEX, H1_CODE) => vec![C3_INDEX, C4_INDEX, A3_INDEX, A4_INDEX, B3_INDEX],
//         (C1_INDEX, H1_CODE) => vec![C2_INDEX, B1_INDEX],
//         (C2_INDEX, H1_CODE) => vec![C3_INDEX, B1_INDEX, B2_INDEX, C1_INDEX],
//         (C3_INDEX, H1_CODE) => vec![C4_INDEX, B2_INDEX, B3_INDEX, C2_INDEX],
//         (C4_INDEX, H1_CODE) => vec![B3_INDEX, B4_INDEX, C3_INDEX],
//         (A1_INDEX, H2_CODE) => vec![B1_INDEX, B2_INDEX, A2_INDEX],
//         (A2_INDEX, H2_CODE) => vec![B2_INDEX, B3_INDEX, A1_INDEX, A3_INDEX],
//         (A3_INDEX, H2_CODE) => vec![B3_INDEX, B4_INDEX, A2_INDEX, A4_INDEX],
//         (A4_INDEX, H2_CODE) => vec![B4_INDEX, A3_INDEX],
//         (B1_INDEX, H2_CODE) => vec![C1_INDEX, C2_INDEX, A2_INDEX, A1_INDEX, B2_INDEX],
//         (B2_INDEX, H2_CODE) => vec![C2_INDEX, C3_INDEX, A3_INDEX, A2_INDEX, B1_INDEX, B3_INDEX],
//         (B3_INDEX, H2_CODE) => vec![C3_INDEX, C4_INDEX, A4_INDEX, A3_INDEX, B2_INDEX, B4_INDEX],
//         (B4_INDEX, H2_CODE) => vec![C4_INDEX, A4_INDEX, B3_INDEX],
//         (C1_INDEX, H2_CODE) => vec![B2_INDEX, B1_INDEX, C2_INDEX],
//         (C2_INDEX, H2_CODE) => vec![B3_INDEX, B2_INDEX, C1_INDEX, C3_INDEX],
//         (C3_INDEX, H2_CODE) => vec![B4_INDEX, B3_INDEX, C2_INDEX, C4_INDEX],
//         (C4_INDEX, H2_CODE) => vec![B4_INDEX, C3_INDEX],
//         _ => vec![],
//     }
// }

pub fn judge(
    board: &Bitboard::bitboard::Bitboard,
    bef_board: &Bitboard::bitboard::Bitboard,
    is_player1: bool,
) -> i32 {
    // キャッチ判定
    // 1pがライオンをとった時
    if board.kb & board.black_b == 0 {
        return if is_player1 { WIN_POINT } else { LOSE_POINT };
    // 1pがライオンが取られた時
    } else if board.kb & board.white_b == 0 {
        return if is_player1 { LOSE_POINT } else { WIN_POINT };
    }
    // 1pトライ判定
    if board.kb & board.white_b & TRY_MASK1 != 0
        && bef_board.kb & bef_board.white_b & TRY_MASK1 != 0
    {
        return if is_player1 { WIN_POINT } else { LOSE_POINT };
    }
    // 2pトライ判定
    if board.kb & board.black_b & TRY_MASK4 != 0
        && bef_board.kb & bef_board.black_b & TRY_MASK4 != 0
    {
        return if is_player1 { LOSE_POINT } else { WIN_POINT };
    }
    // 勝敗がついていなければ0を返す
    0
}

pub fn eval_function(
    board: &Bitboard::bitboard::Bitboard,
    bef_board: &Bitboard::bitboard::Bitboard,
    is_player1: bool,
) -> i32 {
    // 勝敗がついていれば終了
    let mut point = judge(board, bef_board, is_player1);
    if point != 0 {
        return point;
    }

    // 拡張用
    /////////////////////////////////////////////
    // let pb_board = board.white_b & board.pb;
    // if pb_board != 0 {
    //     // 最上位ビットのみ取り出す
    //     let mut msb = pb_board & -pb_board;
    //     // 最上位ビット以下のビットを立てる
    //     let mut msb_count = msb + !-msb;
    //     // 立っているビット数をカウント
    //     msb_count = (msb_count & 0x55555555) + (msb_count >> 1 & 0x55555555);
    //     msb_count = (msb_count & 0x33333333) + (msb_count >> 2 & 0x33333333);
    //     msb_count = (msb_count & 0x0f0f0f0f) + (msb_count >> 4 & 0x0f0f0f0f);
    //     msb_count = (msb_count & 0x00ff00ff) + (msb_count >> 8 & 0x00ff00ff);
    //     msb_count = (msb_count & 0x0000ffff) + (msb_count >> 16 & 0x0000ffff);
    //
    //     point += EVAL_LIST[0][msb_count as usize];
    //
    //     // 2つ駒がある場合
    //     msb = pb_board - msb;
    //     if msb != 0 {
    //         // 最上位ビット以下のビットを立てる
    //         let mut msb_count = msb + !-msb;
    //         // 立っているビット数をカウント
    //         msb_count = (msb_count & 0x55555555) + (msb_count >> 1 & 0x55555555);
    //         msb_count = (msb_count & 0x33333333) + (msb_count >> 2 & 0x33333333);
    //         msb_count = (msb_count & 0x0f0f0f0f) + (msb_count >> 4 & 0x0f0f0f0f);
    //         msb_count = (msb_count & 0x00ff00ff) + (msb_count >> 8 & 0x00ff00ff);
    //         msb_count = (msb_count & 0x0000ffff) + (msb_count >> 16 & 0x0000ffff);
    //
    //         point += EVAL_LIST[0][msb_count as usize];
    //     }
    // }
    /////////////////////////////////////////////

    //勝敗がついていなければ盤面の点数を返す
    let white_board = board.white_b & BOARD_MASK;
    let black_board = board.black_b & BOARD_MASK;
    let white_hand = board.white_b & HAND_MASK;
    let black_hand = board.black_b & HAND_MASK;

    // ニワトリの個数で分岐
    if board.ppb == 0 {
        // ニワトリがいない場合
        // ヒヨコの得点
        point += if white_board & board.pb != 0 {
            PB_BOARD_POINT
        } else {
            -PB_BOARD_POINT
        };
        point += if black_board & board.pb != 0 {
            -PB_BOARD_POINT
        } else {
            PB_BOARD_POINT
        };
        point += if white_hand & board.pb != 0 {
            PB_HAND_POINT
        } else {
            -PB_HAND_POINT
        };
        point += if black_hand & board.pb != 0 {
            -PB_HAND_POINT
        } else {
            PB_HAND_POINT
        };
    } else {
        let ppb_board = board.ppb & -board.ppb;
        if board.ppb - ppb_board == 0 {
            // ニワトリが一つの場合
            // ヒヨコの得点
            // 盤面にヒヨコがいる場合
            if board.pb & BOARD_MASK != 0 {
                point += if white_board & board.pb != 0 {
                    PB_BOARD_POINT
                } else {
                    -PB_BOARD_POINT
                };
            } else {
                point += if white_hand & board.pb != 0 {
                    PB_HAND_POINT
                } else {
                    -PB_HAND_POINT
                };
            }
            // ニワトリの得点
            point += if white_board & board.ppb != 0 {
                PPB_BOARD_POINT
            } else {
                -PPB_BOARD_POINT
            };
        } else {
            // ニワトリが二つの場合
            // ニワトリの得点
            point += if white_board & board.ppb != 0 {
                PPB_BOARD_POINT
            } else {
                -PPB_BOARD_POINT
            };
            point += if black_board & board.ppb != 0 {
                -PPB_BOARD_POINT
            } else {
                PPB_BOARD_POINT
            };
        }
    }

    // ゾウの得点
    point += if white_board & board.bb != 0 {
        BB_BOARD_POINT
    } else {
        -BB_BOARD_POINT
    };
    point += if black_board & board.bb != 0 {
        -BB_BOARD_POINT
    } else {
        BB_BOARD_POINT
    };
    point += if white_hand & board.bb != 0 {
        BB_HAND_POINT
    } else {
        -BB_HAND_POINT
    };
    point += if black_hand & board.bb != 0 {
        -BB_HAND_POINT
    } else {
        BB_HAND_POINT
    };

    // キリンの得点
    point += if white_board & board.rb != 0 {
        RB_BOARD_POINT
    } else {
        -RB_BOARD_POINT
    };
    point += if black_board & board.rb != 0 {
        -RB_BOARD_POINT
    } else {
        RB_BOARD_POINT
    };
    point += if white_hand & board.rb != 0 {
        RB_HAND_POINT
    } else {
        -RB_HAND_POINT
    };
    point += if black_hand & board.rb != 0 {
        -RB_HAND_POINT
    } else {
        RB_HAND_POINT
    };
    point
}

pub fn nega_scout(
    board: &Bitboard::bitboard::Bitboard,
    bef_board: &Bitboard::bitboard::Bitboard,
    is_player1: bool,
    depth: i32,
    mut alpha: i32,
    beta: i32,
) -> Node {
    let mut best_move = (0, 0);
    // 根のノードの場合、静的評価
    if depth == 0 {
        let point: i32 = eval_function(board, bef_board, is_player1);
        print_nega(depth, point, best_move);
        return Node { best_move, point };
    }
    // 勝敗がついていれば終了
    let mut point = judge(board, bef_board, is_player1);
    if point != 0 {
        if point == WIN_POINT {
            point += depth;
        } else {
            point -= depth;
        }

        //print_nega(depth, point, best_move);
        return Node { best_move, point };
    }
    let next_move_list = next_move_list(board, is_player1);
    for next_move in next_move_list {
        // if depth == DEPTH {
        //     let a = 0;
        // }
        //print_nega(depth, 9999999, next_move);
        // if depth == DEPTH && next_move == (1024, 128) {
        //     if depth == DEPTH && next_move == (16, 256) {
        //         let a = 0;
        //     }
        //     let a = 0;
        //     print!("{}", board);
        // }
        let next_board = make_moved_board(board, next_move, is_player1);
        // println!("{}", next_board);
        // if depth == DEPTH && next_move == (128, 16) {
        //     let a = 0;
        //     print!("{}", next_board);
        // }
        let next_node = nega_scout(&next_board, &board, !is_player1, depth - 1, -beta, -alpha);
        point = -next_node.point;
        // if depth == DEPTH {
        //     println!(
        //         "{} {} {}",
        //         point,
        //         get_board_name(next_node.best_move.0),
        //         get_board_name(next_node.best_move.1)
        //     );
        // }
        //print_nega(depth, point, (-1, -1));
        if point > alpha {
            alpha = point;
            best_move = next_move;
        }
        if alpha >= beta {
            break;
        }
    }

    Node {
        best_move,
        point: alpha,
    }
}

pub struct Node {
    best_move: (i32, i32),
    point: i32,
}

pub fn get_board_name(i: i32) -> String {
    match i {
        A1_INDEX => "A1".to_string(),
        A2_INDEX => "A2".to_string(),
        A3_INDEX => "A3".to_string(),
        A4_INDEX => "A4".to_string(),
        B1_INDEX => "B1".to_string(),
        B2_INDEX => "B2".to_string(),
        B3_INDEX => "B3".to_string(),
        B4_INDEX => "B4".to_string(),
        C1_INDEX => "C1".to_string(),
        C2_INDEX => "C2".to_string(),
        C3_INDEX => "C3".to_string(),
        C4_INDEX => "C4".to_string(),
        D1_INDEX => "D1".to_string(),
        D2_INDEX => "D2".to_string(),
        D3_INDEX => "D3".to_string(),
        D4_INDEX => "D4".to_string(),
        D5_INDEX => "D5".to_string(),
        D6_INDEX => "D6".to_string(),
        E1_INDEX => "E1".to_string(),
        E2_INDEX => "E2".to_string(),
        E3_INDEX => "E3".to_string(),
        E4_INDEX => "E4".to_string(),
        E5_INDEX => "E5".to_string(),
        E6_INDEX => "E6".to_string(),
        _ => "".to_string(),
    }
}

pub fn print_move(move_vec: (i32, i32)) {
    println!("{:<024b}", move_vec.0);
    println!("{:<024b}", move_vec.1);
}

pub fn print_nega(depth: i32, point: i32, best_move: (i32, i32)) {
    if depth > 0 {
        for _ in 0..DEPTH - depth {
            print!("                ");
        }
        if depth == 0 {
            println!("{} ({})", point, depth,);
        } else if point == 9999999 {
            println!(
                "{}=>{}",
                get_board_name(best_move.0),
                get_board_name(best_move.1),
            );
        } else if best_move == (0, 0) {
            println!("{} ({})", point, depth,);
        } else if best_move == (-1, -1) {
            println!("{} ({})", point, depth,);
        } else {
            println!(
                "{} ({}) {}=>{}",
                point,
                depth,
                get_board_name(best_move.0),
                get_board_name(best_move.1),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test1_make_moved_board() {
        let mut board = Bitboard::bitboard::Bitboard {
            white_b: 0b_000000_000000_111_010_000_000,
            black_b: 0b_000000_000000_000_000_010_111,
            kb: 0b_000000_000000_010_000_000_010,
            rb: 0b_000000_000000_100_000_000_001,
            bb: 0b_000000_000000_001_000_000_100,
            pb: 0b_000000_000000_000_010_010_000,
            ppb: 0b_000000_000000_000_000_000_000,
        };
        let mut move_vec = (B3_INDEX, B2_INDEX);
        let mut is_player1 = true;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);

        move_vec = (C1_INDEX, B2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A4_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E1_INDEX, C3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (D1_INDEX, A2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C3_INDEX, C4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B3_INDEX, C4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A1_INDEX, A2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C4_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A2_INDEX, A3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (D1_INDEX, A4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A3_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B4_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E3_INDEX, C2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B3_INDEX, B4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E1_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B4_INDEX, C4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E1_INDEX, C3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C4_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C2_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        let moved_board = Bitboard::bitboard::Bitboard {
            white_b: 0b_000000_000011_001_000_000_000,
            black_b: 0b_000001_000000_000_110_010_010,
            kb: 0b_000001_000000_000_000_000_010,
            rb: 0b_000000_000011_000_000_000_000,
            bb: 0b_000000_000000_000_010_010_000,
            pb: 0b_000000_000000_001_100_000_000,
            ppb: 0b_000000_000000_000_000_000_000,
        };
        assert_eq!(moved_board, board);
    }

    #[test]
    fn test2_make_moved_board() {
        let mut board = Bitboard::bitboard::Bitboard {
            white_b: 0b_000000_000000_111_010_000_000,
            black_b: 0b_000000_000000_000_000_010_111,
            kb: 0b_000000_000000_010_000_000_010,
            rb: 0b_000000_000000_100_000_000_001,
            bb: 0b_000000_000000_001_000_000_100,
            pb: 0b_000000_000000_000_010_010_000,
            ppb: 0b_000000_000000_000_000_000_000,
        };
        let mut move_vec = (B3_INDEX, A1_INDEX);
        let mut is_player1 = true;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B2_INDEX, A2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A1_INDEX, A2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C1_INDEX, B2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A2_INDEX, A3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B2_INDEX, A3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B4_INDEX, A3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E1_INDEX, A2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A3_INDEX, A2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B1_INDEX, C1_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (D1_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C1_INDEX, B1_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (D1_INDEX, A3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);

        let moved_board = Bitboard::bitboard::Bitboard {
            white_b: 0b_000000_000011_101_011_001_000,
            black_b: 0b_000000_000000_000_000_000_010,
            kb: 0b_000000_000000_000_000_001_010,
            rb: 0b_000000_000000_100_010_000_000,
            bb: 0b_000000_000001_001_000_000_000,
            pb: 0b_000000_000010_000_001_000_000,
            ppb: 0b_000000_000000_000_000_000_000,
        };
        assert_eq!(moved_board, board);
    }

    #[test]
    fn test3_make_moved_board() {
        let mut board = Bitboard::bitboard::Bitboard {
            white_b: 0b_000000_000000_111_010_000_000,
            black_b: 0b_000000_000000_000_000_010_111,
            kb: 0b_000000_000000_010_000_000_010,
            rb: 0b_000000_000000_100_000_000_001,
            bb: 0b_000000_000000_001_000_000_100,
            pb: 0b_000000_000000_000_010_010_000,
            ppb: 0b_000000_000000_000_000_000_000,
        };
        let mut move_vec = (C4_INDEX, C3_INDEX);
        let mut is_player1 = true;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B2_INDEX, A4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C3_INDEX, C2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (A4_INDEX, C2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B3_INDEX, B2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C2_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B4_INDEX, C4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B3_INDEX, B2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (C4_INDEX, B2_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E2_INDEX, B4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (D1_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B1_INDEX, B3_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (B2_INDEX, B1_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        move_vec = (E2_INDEX, C4_INDEX);
        is_player1 = !is_player1;
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);

        let moved_board = Bitboard::bitboard::Bitboard {
            white_b: 0b_000000_000000_000_000_000_010,
            black_b: 0b_000011_000000_110_010_000_101,
            kb: 0b_000000_000000_000_010_000_010,
            rb: 0b_000000_000000_010_000_000_001,
            bb: 0b_000001_000000_000_000_000_100,
            pb: 0b_000010_000000_100_000_000_000,
            ppb: 0b_000000_000000_000_000_000_000,
        };
        assert_eq!(moved_board, board);
    }
}
