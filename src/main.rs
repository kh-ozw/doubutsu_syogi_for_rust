#[warn(unused_variables)]
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Instant,
};

mod bit_board;

// 盤面定数
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

// ビットマスク
const BOARD_MASK: i32 = 0b111_111_111_111;
const HAND_MASK: i32 = 0b111111_111111 << 12;
const D_TRY_MASK: i32 = 0b111;
const E_TRY_MASK: i32 = 0b111 << 9;
const D_HAND_MASK: i32 = 0b111111 << 12;
const E_HAND_MASK: i32 = 0b111111 << 18;

// コマの得点
const H_BOARD_POINT: i32 = 1;
const H_HAND_POINT: i32 = 2;
const Z_BOARD_POINT: i32 = 6;
const Z_HAND_POINT: i32 = 8;
const K_BOARD_POINT: i32 = 5;
const K_HAND_POINT: i32 = 7;
const N_BOARD_POINT: i32 = 6;

// 勝敗判定時のポイント
const WIN_POINT: i32 = 10000;
const LOSE_POINT: i32 = -10000;

// パラメータ
const DEPTH: i32 = 9;
const SHALLOW_DEPTH: i32 = 3;
const HOST_NAME: &str = "localhost";
//const HOST_NAME: &str = "192.168.11.8";
const PORT_NUM: i32 = 4444;

fn main() {
    // let start = Instant::now();
    // let end = start.elapsed();
    // println!("{} :経過しました。", end.subsec_nanos());

    // サーバ接続
    let host_and_port = format!("{}:{}", HOST_NAME, PORT_NUM);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();

    // 直接Ipv4を指定
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        match TcpStream::connect(addr) {
            Err(e) => {
                println!("{}", e);
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
                let mut bef_board = bit_board::bit_board::BitBoard {
                    pb1: 0b000_000_000_010,
                    pb2: 0b010_000_000_000,
                    lb: 0b010_000_000_010,
                    kb: 0,
                    zb: 0,
                    hb: 0,
                    nb: 0,
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

                        // bitboardに変換
                        let board = make_bit_board(&mut board_vec);
                        let clone_board = board.clone();
                        let best_node;
                        // println!("{:#?}", board);

                        // 持ち駒と最初の探索数によって深さを変える
                        let mut depth: i32 = DEPTH;
                        let p1_hand_count: u32 = (&board.pb1 & D_HAND_MASK).count_ones();
                        let p2_hand_count: u32 = (&board.pb2 & E_HAND_MASK).count_ones();
                        if p1_hand_count + p2_hand_count <= 2 {
                            depth = depth;
                        } else if p1_hand_count + p2_hand_count <= 4 {
                            depth = depth - 1;
                        } else {
                            depth = depth - 2;
                        }

                        // 探索
                        let start = Instant::now();
                        best_node =
                            nega_scout(&board, &bef_board, is_player1, depth, -50000, 50000);
                        let end = start.elapsed();

                        // 自分の手を送信
                        let move_str = String::from("mv ")
                            + &get_board_name(best_node.best_move.0)
                            + " "
                            + &get_board_name(best_node.best_move.1);
                        let mut buffer: Vec<u8> = Vec::new();
                        write_socket(&mut writer, &move_str);
                        reader
                            .read_until(b'\n', &mut buffer)
                            .expect("Receive failure.");

                        // 勝敗がついているか確認
                        let next_board = make_moved_board(&board, best_node.best_move, is_player1);
                        let point = judge(&next_board, &clone_board, is_player1);
                        bef_board = clone_board;
                        if point == WIN_POINT {
                            println!("you win!");
                            break;
                        } else if point == LOSE_POINT {
                            println!("you lose!");
                            break;
                        }

                        println!(
                            "{}, point:{:>05}, d:{}, time:{}.{}s ({}), hand count:{:>01}+{:>01}={:>01}",
                            move_str,
                            best_node.point,
                            depth,
                            end.as_nanos() / 1000000000,
                            end.as_nanos() / 1000000 - end.as_nanos() / 1000000000,
                            end.as_nanos() ,
                            p1_hand_count,
                            p2_hand_count,
                            p1_hand_count + p2_hand_count,
                        );
                        //println!("-------------------");

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

pub fn make_bit_board(board_vec: &mut Vec<u8>) -> bit_board::bit_board::BitBoard {
    let mut pb1: i32 = 0;
    let mut pb2: i32 = 0;
    let mut lb: i32 = 0;
    let mut kb: i32 = 0;
    let mut zb: i32 = 0;
    let mut hb: i32 = 0;
    let mut nb: i32 = 0;
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
                pb1 |= 1 << p;
            } else if b_iter[4] == b'2' {
                pb2 |= 1 << p;
            }
            match b_iter[3] {
                b'l' => lb |= 1 << p,
                b'g' => kb |= 1 << p,
                b'e' => zb |= 1 << p,
                b'c' => hb |= 1 << p,
                b'h' => nb |= 1 << p,
                _ => (),
            }
        }
    }

    bit_board::bit_board::BitBoard {
        pb1,
        pb2,
        lb,
        kb,
        zb,
        hb,
        nb,
    }
}

//pub fn make_board_map(board: &bit_board::bit_board::BitBoard) -> &mut Vec<u8> {}

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

// #[inline(always)]
pub fn make_moved_board(
    bef_board: &bit_board::bit_board::BitBoard,
    move_vec: (i32, i32),
    is_player1: bool,
) -> bit_board::bit_board::BitBoard {
    let src: i32 = move_vec.0;
    let dst: i32 = move_vec.1;
    let mut board = bef_board.clone();

    // プレイヤー1の場合
    if is_player1 {
        // 打った駒が手ごまの場合
        if src & D_HAND_MASK != 0 {
            // 先手の盤面を更新
            board.pb1 = board.pb1 & !src | dst;
            if board.hb & src != 0 {
                // ヒヨコの盤面を更新
                if (src & D_HAND_MASK == 0) && (dst & D_TRY_MASK != 0) {
                    // ニワトリに進化
                    board.hb = board.hb & !src;
                    board.nb = board.nb | dst;
                } else {
                    board.hb = board.hb & !src | dst;
                }
            } else if board.lb & src != 0 {
                // ライオンの盤面を更新
                board.lb = board.lb & !src | dst;
            } else if board.kb & src != 0 {
                // キリンの盤面を更新
                board.kb = board.kb & !src | dst;
            } else if board.zb & src != 0 {
                // ゾウの盤面を更新
                board.zb = board.zb & !src | dst;
            } else if board.nb & src != 0 {
                // ニワトリの盤面を更新
                board.nb = board.nb & !src | dst;
            }
            // 打った手駒のあった場所より右側に駒があった時、その駒たちをずらす（打った駒のD列の数字より大きい数字のマスに駒があるとき）
            let shift_bits: i32 = !(src - 1) & D_HAND_MASK;
            if shift_bits & board.pb1 != 0 {
                let non_shift_bits: i32 = (src - 1) & D_HAND_MASK | !D_HAND_MASK;
                board.pb1 = (board.pb1 & non_shift_bits) | ((board.pb1 & shift_bits) >> 1);
                board.kb = (board.kb & non_shift_bits) | ((board.kb & shift_bits) >> 1);
                board.zb = (board.zb & non_shift_bits) | ((board.zb & shift_bits) >> 1);
                board.hb = (board.hb & non_shift_bits) | ((board.hb & shift_bits) >> 1);
            }
        } else {
            // 移動先に相手のコマがある場合
            if board.pb2 & dst != 0 {
                // 後手の盤面で取られる駒を削除
                board.pb2 &= !dst;
                // 持ち駒に加える位置
                let hand_pos = (board.pb1 & D_HAND_MASK) + (1 << 12);
                board.pb1 |= hand_pos;
                if board.hb & dst != 0 {
                    // ヒヨコの盤面の駒を消し、取った駒を手持ちに加える
                    board.hb = (board.hb & !dst) | hand_pos;
                } else if board.lb & dst != 0 {
                    // ライオンの盤面の駒を消し、取った駒を手持ちに加える
                    board.lb = (board.lb & !dst) | hand_pos;
                } else if board.kb & dst != 0 {
                    // キリンの盤面の駒を消し、取った駒を手持ちに加える
                    board.kb = (board.kb & !dst) | hand_pos;
                } else if board.zb & dst != 0 {
                    // ゾウの盤面の駒を消し、取った駒を手持ちに加える
                    board.zb = (board.zb & !dst) | hand_pos;
                } else if board.nb & dst != 0 {
                    // ニワトリの盤面の駒を消し、取った駒を手持ちに加える（ヒヨコとして）
                    board.nb = board.nb & !dst;
                    board.hb = board.hb | hand_pos;
                }
            }
            // 先手の盤面を更新
            board.pb1 = board.pb1 & !src | dst;
            if board.hb & src != 0 {
                // ヒヨコの盤面を更新
                if (src & D_HAND_MASK == 0) && (dst & D_TRY_MASK != 0) {
                    // ニワトリに進化
                    board.hb = board.hb & !src;
                    board.nb = board.nb | dst;
                } else {
                    board.hb = board.hb & !src | dst;
                }
            } else if board.lb & src != 0 {
                // ライオンの盤面を更新
                board.lb = board.lb & !src | dst;
            } else if board.kb & src != 0 {
                // キリンの盤面を更新
                board.kb = board.kb & !src | dst;
            } else if board.zb & src != 0 {
                // ゾウの盤面を更新
                board.zb = board.zb & !src | dst;
            } else if board.nb & src != 0 {
                // ニワトリの盤面を更新
                board.nb = board.nb & !src | dst;
            }
        }
    }
    // プレイヤー2の場合
    else {
        // 打った駒が手ごまの場合
        if src & E_HAND_MASK != 0 {
            // 後手の盤面を更新
            board.pb2 = board.pb2 & !src | dst;
            if board.hb & src != 0 {
                // ヒヨコの盤面を更新
                if (src & E_HAND_MASK == 0) && (dst & E_TRY_MASK != 0) {
                    // ニワトリに進化
                    board.hb = board.hb & !src;
                    board.nb = board.nb | dst;
                } else {
                    board.hb = board.hb & !src | dst;
                }
            } else if board.lb & src != 0 {
                // ライオンの盤面を更新
                board.lb = board.lb & !src | dst;
            } else if board.kb & src != 0 {
                // キリンの盤面を更新
                board.kb = board.kb & !src | dst;
            } else if board.zb & src != 0 {
                // ゾウの盤面を更新
                board.zb = board.zb & !src | dst;
            } else if board.nb & src != 0 {
                // ニワトリの盤面を更新
                board.nb = board.nb & !src | dst;
            }
            // 打った手駒のあった場所より右側に駒があった時、その駒たちをずらす（打った駒のE列の数字より大きい数字のマスに駒があるとき）
            let shift_bits: i32 = !(src - 1) & E_HAND_MASK;
            if shift_bits & board.pb2 != 0 {
                let non_shift_bits: i32 = (src - 1) & E_HAND_MASK | !E_HAND_MASK;
                board.pb2 = (board.pb2 & non_shift_bits) | ((board.pb2 & shift_bits) >> 1);
                board.kb = (board.kb & non_shift_bits) | ((board.kb & shift_bits) >> 1);
                board.zb = (board.zb & non_shift_bits) | ((board.zb & shift_bits) >> 1);
                board.hb = (board.hb & non_shift_bits) | ((board.hb & shift_bits) >> 1);
            }
        } else {
            // 移動先に相手のコマがある場合
            if board.pb1 & dst != 0 {
                // 先手の盤面で取られる駒を削除
                board.pb1 &= !dst;
                // 持ち駒に加える位置
                let hand_pos = (board.pb2 & E_HAND_MASK) + (1 << 18);
                board.pb2 |= hand_pos;
                if board.hb & dst != 0 {
                    // ヒヨコの盤面の駒を消し、取った駒を手持ちに加える
                    board.hb = (board.hb & !dst) | hand_pos;
                } else if board.lb & dst != 0 {
                    // ライオンの盤面の駒を消し、取った駒を手持ちに加える
                    board.lb = (board.lb & !dst) | hand_pos;
                } else if board.kb & dst != 0 {
                    // キリンの盤面の駒を消し、取った駒を手持ちに加える
                    board.kb = (board.kb & !dst) | hand_pos;
                } else if board.zb & dst != 0 {
                    // ゾウの盤面の駒を消し、取った駒を手持ちに加える
                    board.zb = (board.zb & !dst) | hand_pos;
                } else if board.nb & dst != 0 {
                    // ニワトリの盤面の駒を消し、取った駒を手持ちに加える（ヒヨコとして）
                    board.nb = board.nb & !dst;
                    board.hb = board.hb | hand_pos;
                }
            }
            // 後手の盤面を更新
            board.pb2 = board.pb2 & !src | dst;
            if board.hb & src != 0 {
                // ヒヨコの盤面を更新
                if (src & E_HAND_MASK == 0) && (dst & E_TRY_MASK != 0) {
                    // ニワトリに進化
                    board.hb = board.hb & !src;
                    board.nb = board.nb | dst;
                } else {
                    board.hb = board.hb & !src | dst;
                }
            } else if board.lb & src != 0 {
                // ライオンの盤面を更新
                board.lb = board.lb & !src | dst;
            } else if board.kb & src != 0 {
                // キリンの盤面を更新
                board.kb = board.kb & !src | dst;
            } else if board.zb & src != 0 {
                // ゾウの盤面を更新
                board.zb = board.zb & !src | dst;
            } else if board.nb & src != 0 {
                // ニワトリの盤面を更新
                board.nb = board.nb & !src | dst;
            }
        }
    }
    board
}

// #[inline(always)]
pub fn next_move_list(board: &bit_board::bit_board::BitBoard, is_player1: bool) -> Vec<(i32, i32)> {
    let mut next_move_list: Vec<(i32, i32)> = vec![];

    if is_player1 {
        // 1pの手の探索
        let player_board: i32 = board.pb1;

        // 1pひよこの手探索
        // hb_boardの1となる下位ビットを取得
        let hb_board: i32 = player_board & board.hb;
        let mut target_bit: i32 = hb_board & -hb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
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
            // hb_boardの1となる上位ビットを取得
            target_bit ^= hb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
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
        let zb_board: i32 = player_board & board.zb;
        target_bit = zb_board & -zb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
                A1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX))
                    }
                }
                A2_INDEX => {
                    if player_board & B3_INDEX == 0 {
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
            target_bit ^= zb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
                    A1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B2_INDEX))
                        }
                    }
                    A2_INDEX => {
                        if player_board & B3_INDEX == 0 {
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
        let kb_board: i32 = player_board & board.kb;
        target_bit = kb_board & -kb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
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
            target_bit = kb_board - target_bit;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
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
            }
        }

        // 1pライオンの手探索
        target_bit = player_board & board.lb;
        match target_bit {
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

        // 1pニワトリの手探索
        let nb_board: i32 = player_board & board.nb;
        target_bit = nb_board & -nb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
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
            target_bit ^= nb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
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
        let empty_bit: i32 = !(board.pb1 | board.pb2);

        // 1pヒヨコ
        if board.hb & D_HAND_MASK != 0 {
            let hand_index: i32 = (board.hb & D_HAND_MASK) & -(board.hb & D_HAND_MASK);
            if empty_bit & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if empty_bit & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if empty_bit & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if empty_bit & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if empty_bit & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if empty_bit & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if empty_bit & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if empty_bit & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if empty_bit & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if empty_bit & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if empty_bit & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if empty_bit & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 1pゾウ
        if board.zb & D_HAND_MASK != 0 {
            let hand_index: i32 = (board.zb & D_HAND_MASK) & -(board.zb & D_HAND_MASK);
            if empty_bit & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if empty_bit & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if empty_bit & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if empty_bit & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if empty_bit & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if empty_bit & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if empty_bit & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if empty_bit & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if empty_bit & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if empty_bit & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if empty_bit & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if empty_bit & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 1pキリン
        if board.kb & D_HAND_MASK != 0 {
            let hand_index: i32 = (board.kb & D_HAND_MASK) & -(board.kb & D_HAND_MASK);
            if empty_bit & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if empty_bit & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if empty_bit & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if empty_bit & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if empty_bit & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if empty_bit & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if empty_bit & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if empty_bit & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if empty_bit & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if empty_bit & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if empty_bit & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if empty_bit & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
    } else {
        // 2pの手の探索
        let player_board: i32 = board.pb2;

        // 2pひよこの手探索
        let hb_board: i32 = player_board & board.hb;
        let mut target_bit: i32 = hb_board & -hb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
                A1_INDEX => {
                    // 移動先に自分のコマがなければ、移動先に追加
                    if player_board & A2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, A2_INDEX))
                    }
                }
                A2_INDEX => {
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
                        next_move_list.push((B1_INDEX, B2_INDEX))
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
            target_bit ^= hb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
                    A1_INDEX => {
                        if player_board & A2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, A2_INDEX))
                        }
                    }
                    A2_INDEX => {
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
                            next_move_list.push((B1_INDEX, B2_INDEX))
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
            }
        }

        // 2pゾウの手探索
        let zb_board: i32 = player_board & board.zb;
        target_bit = zb_board & -zb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
                A1_INDEX => {
                    if player_board & B2_INDEX == 0 {
                        next_move_list.push((A1_INDEX, B2_INDEX))
                    }
                }
                A2_INDEX => {
                    if player_board & B3_INDEX == 0 {
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
            target_bit ^= zb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
                    A1_INDEX => {
                        if player_board & B2_INDEX == 0 {
                            next_move_list.push((A1_INDEX, B2_INDEX))
                        }
                    }
                    A2_INDEX => {
                        if player_board & B3_INDEX == 0 {
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
        let kb_board: i32 = player_board & board.kb;
        target_bit = kb_board & -kb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
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
            target_bit ^= kb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
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
            }
        }

        // 2pライオンの手探索
        target_bit = player_board & board.lb;
        match target_bit {
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

        // 2pニワトリの手探索
        let nb_board: i32 = player_board & board.nb;
        target_bit = nb_board & -nb_board;
        if target_bit != 0 {
            // 1つ目のコマの探索
            match target_bit {
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
            target_bit ^= nb_board;
            if target_bit != 0 {
                // 2つ目のコマの探索
                match target_bit {
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
        let empty_bit: i32 = !(board.pb1 | board.pb2);

        // 2pヒヨコ
        if board.hb & E_HAND_MASK != 0 {
            let hand_index: i32 = (board.hb & E_HAND_MASK) & -(board.hb & E_HAND_MASK);
            if empty_bit & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if empty_bit & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if empty_bit & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if empty_bit & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if empty_bit & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if empty_bit & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if empty_bit & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if empty_bit & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if empty_bit & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if empty_bit & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if empty_bit & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if empty_bit & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 2pゾウ
        if board.zb & E_HAND_MASK != 0 {
            let hand_index: i32 = (board.zb & E_HAND_MASK) & -(board.zb & E_HAND_MASK);
            if empty_bit & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if empty_bit & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if empty_bit & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if empty_bit & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if empty_bit & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if empty_bit & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if empty_bit & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if empty_bit & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if empty_bit & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if empty_bit & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if empty_bit & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if empty_bit & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
        // 2pキリン
        if board.kb & E_HAND_MASK != 0 {
            let hand_index: i32 = (board.kb & E_HAND_MASK) & -(board.kb & E_HAND_MASK);
            if empty_bit & A1_INDEX != 0 {
                next_move_list.push((hand_index, A1_INDEX));
            }
            if empty_bit & A2_INDEX != 0 {
                next_move_list.push((hand_index, A2_INDEX));
            }
            if empty_bit & A3_INDEX != 0 {
                next_move_list.push((hand_index, A3_INDEX));
            }
            if empty_bit & A4_INDEX != 0 {
                next_move_list.push((hand_index, A4_INDEX));
            }
            if empty_bit & B1_INDEX != 0 {
                next_move_list.push((hand_index, B1_INDEX));
            }
            if empty_bit & B2_INDEX != 0 {
                next_move_list.push((hand_index, B2_INDEX));
            }
            if empty_bit & B3_INDEX != 0 {
                next_move_list.push((hand_index, B3_INDEX));
            }
            if empty_bit & B4_INDEX != 0 {
                next_move_list.push((hand_index, B4_INDEX));
            }
            if empty_bit & C1_INDEX != 0 {
                next_move_list.push((hand_index, C1_INDEX));
            }
            if empty_bit & C2_INDEX != 0 {
                next_move_list.push((hand_index, C2_INDEX));
            }
            if empty_bit & C3_INDEX != 0 {
                next_move_list.push((hand_index, C3_INDEX));
            }
            if empty_bit & C4_INDEX != 0 {
                next_move_list.push((hand_index, C4_INDEX));
            }
        }
    }
    next_move_list
}

// #[inline(always)]
pub fn judge(
    board: &bit_board::bit_board::BitBoard,
    bef_board: &bit_board::bit_board::BitBoard,
    is_player1: bool,
) -> i32 {
    // キャッチ判定
    // 1pがライオンをとった時
    if board.lb & board.pb2 == 0 {
        return if is_player1 { WIN_POINT } else { LOSE_POINT };
    // 1pがライオンが取られた時
    } else if board.lb & board.pb1 == 0 {
        return if is_player1 { LOSE_POINT } else { WIN_POINT };
    }
    // 1pトライ判定
    if board.lb & board.pb1 & D_TRY_MASK != 0 && bef_board.lb & bef_board.pb1 & D_TRY_MASK != 0 {
        return if is_player1 { WIN_POINT } else { LOSE_POINT };
    }
    // 2pトライ判定
    if board.lb & board.pb2 & E_TRY_MASK != 0 && bef_board.lb & bef_board.pb2 & E_TRY_MASK != 0 {
        return if is_player1 { LOSE_POINT } else { WIN_POINT };
    }
    // 勝敗がついていなければ0を返す
    0
}

// #[inline(always)]
pub fn eval_function(
    board: &bit_board::bit_board::BitBoard,
    bef_board: &bit_board::bit_board::BitBoard,
    is_player1: bool,
) -> i32 {
    // 勝敗がついていれば終了
    let mut point = judge(board, bef_board, is_player1);
    if point != 0 {
        return point;
    }
    //勝敗がついていなければ盤面の点数を返す
    let pb1_board: i32 = board.pb1 & BOARD_MASK;
    let pb2_board: i32 = board.pb2 & BOARD_MASK;
    let pb1_hand: i32 = board.pb1 & HAND_MASK;
    let pb2_hand: i32 = board.pb2 & HAND_MASK;

    // ニワトリ0,ヒヨコ2の場合
    if board.nb == 0 {
        // ヒヨコの得点
        point += if board.hb & pb1_board != 0 {
            H_BOARD_POINT
        } else {
            -H_BOARD_POINT
        };
        point += if board.hb & pb2_board != 0 {
            -H_BOARD_POINT
        } else {
            H_BOARD_POINT
        };
        point += if board.hb & pb1_hand != 0 {
            H_HAND_POINT
        } else {
            -H_HAND_POINT
        };
        point += if board.hb & pb2_hand != 0 {
            -H_HAND_POINT
        } else {
            H_HAND_POINT
        };
    } else {
        // ニワトリ2,ヒヨコ0の場合
        if board.hb == 0 {
            // ニワトリの得点
            point += if board.nb & pb1_board != 0 {
                N_BOARD_POINT
            } else {
                -N_BOARD_POINT
            };
            point += if board.nb & pb2_board != 0 {
                -N_BOARD_POINT
            } else {
                N_BOARD_POINT
            };
        } else {
            // ニワトリ1,ヒヨコ1の場合
            // ヒヨコの得点
            point += if board.hb & pb1_board != 0 {
                H_BOARD_POINT
            } else {
                -H_BOARD_POINT
            };
            point += if board.hb & pb1_hand != 0 {
                H_HAND_POINT
            } else {
                -H_HAND_POINT
            };
            // ニワトリの得点
            point += if board.hb & pb1_board != 0 {
                N_BOARD_POINT
            } else {
                -N_BOARD_POINT
            };
        }
    }

    // ゾウの得点
    point += if board.zb & pb1_board != 0 {
        Z_BOARD_POINT
    } else {
        -Z_BOARD_POINT
    };
    point += if board.zb & pb2_board != 0 {
        -Z_BOARD_POINT
    } else {
        Z_BOARD_POINT
    };
    point += if board.zb & pb1_hand != 0 {
        Z_HAND_POINT
    } else {
        -Z_HAND_POINT
    };
    point += if board.zb & pb2_hand != 0 {
        -Z_HAND_POINT
    } else {
        Z_HAND_POINT
    };

    // キリンの得点
    point += if board.kb & pb1_board != 0 {
        K_BOARD_POINT
    } else {
        -K_BOARD_POINT
    };
    point += if board.kb & pb2_board != 0 {
        -K_BOARD_POINT
    } else {
        K_BOARD_POINT
    };
    point += if board.kb & pb1_hand != 0 {
        K_HAND_POINT
    } else {
        -K_HAND_POINT
    };
    point += if board.kb & pb2_hand != 0 {
        -K_HAND_POINT
    } else {
        K_HAND_POINT
    };
    point
}

// #[inline(always)]
pub fn nega_alpha(
    board: &bit_board::bit_board::BitBoard,
    bef_board: &bit_board::bit_board::BitBoard,
    is_player1: bool,
    depth: i32,
    mut alpha: i32,
    beta: i32,
) -> Node {
    let mut best_move = (0, 0);
    // 根のノードの場合、静的評価
    if depth == 0 {
        let point: i32 = eval_function(board, bef_board, is_player1);
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
        return Node { best_move, point };
    }
    let next_move_list = next_move_list(board, is_player1);
    for next_move in next_move_list {
        let next_board = make_moved_board(board, next_move, is_player1);
        let next_node = nega_alpha(&next_board, &board, !is_player1, depth - 1, -beta, -alpha);
        point = -next_node.point;
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

// #[inline(always)]
pub fn nega_scout(
    board: &bit_board::bit_board::BitBoard,
    bef_board: &bit_board::bit_board::BitBoard,
    is_player1: bool,
    depth: i32,
    mut alpha: i32,
    beta: i32,
) -> Node {
    let mut best_move: (i32, i32) = (0, 0);
    // 根のノードの場合、静的評価
    if depth == 0 {
        let point: i32 = eval_function(board, bef_board, is_player1);
        return Node { best_move, point };
    }
    // 勝敗がついていれば終了
    let mut point: i32 = judge(board, bef_board, is_player1);
    if point != 0 {
        if point == WIN_POINT {
            point += depth;
        } else {
            point -= depth;
        }
        return Node { best_move, point };
    }

    // 次の打てる手の取得
    let mut next_move_list: Vec<(i32, i32)> = next_move_list(board, is_player1);
    let move_cnt = next_move_list.len();
    if move_cnt == 0 {
        print!("{}", board);
    }
    let mut sorted_next_move_list: Vec<(i32, i32)> = Vec::with_capacity(move_cnt);
    let mut max_point: i32 = LOSE_POINT;
    // 浅い探索で最もよさそうな手を選択（negaalphaの5手読み）
    // 最初の手の探索
    let first_move = next_move_list[move_cnt - 1];
    let first_board = make_moved_board(board, first_move, is_player1);
    let first_node = nega_alpha(
        &first_board,
        &board,
        !is_player1,
        SHALLOW_DEPTH,
        -beta,
        -alpha,
    );
    point = -first_node.point;
    if max_point < point {
        max_point = point;
        best_move = first_move;
    }
    // 2手目以降の探索
    next_move_list.remove(next_move_list.len() - 1);
    for next_move in next_move_list {
        let next_board = make_moved_board(board, next_move, is_player1);
        let next_node = nega_alpha(
            &next_board,
            &board,
            !is_player1,
            SHALLOW_DEPTH,
            -beta,
            -alpha,
        );
        point = -next_node.point;
        if max_point < point {
            sorted_next_move_list.push(best_move);
            max_point = point;
            best_move = next_move;
        } else {
            sorted_next_move_list.push(next_move);
        }
    }
    // negascout
    // 最初のみ普通に探索
    let next_board = make_moved_board(board, best_move, is_player1);
    let next_node = nega_scout(&next_board, &board, !is_player1, depth - 1, -beta, -alpha);
    point = -next_node.point;

    if beta <= point {
        //return next_node;
    }

    if point > alpha {
        alpha = point;
    }
    let mut max_point = point;

    // ２つ目以降の手はnullwindowsearchで確認のみ行う
    for next_move in sorted_next_move_list {
        let next_board = make_moved_board(board, next_move, is_player1);
        let next_node = nega_scout(
            &next_board,
            &board,
            !is_player1,
            depth - 1,
            -alpha - 1,
            -alpha,
        );
        point = -next_node.point;

        if beta <= point {
            //return next_node;
            break;
        }
        // failed highの場合再探索
        if alpha < point {
            alpha = point;
            let next_node = nega_scout(&next_board, &board, !is_player1, depth - 1, -beta, -alpha);
            point = -next_node.point;

            if beta <= point {
                //return next_node;
                break;
            }
            if alpha < point {
                alpha = point;
            }
        }
        if max_point < point {
            max_point = point;
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

// #[inline(always)]
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

#[test]
fn test1_make_moved_board() {
    let mut board = bit_board::bit_board::BitBoard {
        pb1: 0b_000000_000000_111_010_000_000,
        lb: 0b_000000_000000_010_000_000_010,
        kb: 0b_000000_000000_100_000_000_001,
        zb: 0b_000000_000000_001_000_000_100,
        hb: 0b_000000_000000_000_010_010_000,
        nb: 0b_000000_000000_000_000_000_000,
        pb2: 0b_000000_000000_000_000_010_111,
    };
    let move_vec_list = [
        (B3_INDEX, B2_INDEX),
        (C1_INDEX, B2_INDEX),
        (A4_INDEX, B3_INDEX),
        (E1_INDEX, C3_INDEX),
        (D1_INDEX, A2_INDEX),
        (C3_INDEX, C4_INDEX),
        (B3_INDEX, C4_INDEX),
        (A1_INDEX, A2_INDEX),
        (C4_INDEX, B3_INDEX),
        (A2_INDEX, A3_INDEX),
        (D1_INDEX, A4_INDEX),
        (A3_INDEX, B3_INDEX),
        (B4_INDEX, B3_INDEX),
        (E3_INDEX, C2_INDEX),
        (B3_INDEX, B4_INDEX),
        (E1_INDEX, B3_INDEX),
        (B4_INDEX, C4_INDEX),
        (E1_INDEX, C3_INDEX),
        (C4_INDEX, B3_INDEX),
        (C2_INDEX, B3_INDEX),
    ];
    println!("{}", board);
    let mut is_player1 = true;
    for move_vec in move_vec_list {
        print!("{:?}", move_vec);
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{}", board);
        is_player1 = !is_player1;
    }

    let moved_board = bit_board::bit_board::BitBoard {
        pb1: 0b_000000_000011_001_000_000_000,
        lb: 0b_000001_000000_000_000_000_010,
        kb: 0b_000000_000011_000_000_000_000,
        zb: 0b_000000_000000_000_010_010_000,
        hb: 0b_000000_000000_001_100_000_000,
        nb: 0b_000000_000000_000_000_000_000,
        pb2: 0b_000001_000000_000_110_010_010,
    };
    assert_eq!(moved_board, board);
}

#[test]
fn test2_make_moved_board() {
    let mut board = bit_board::bit_board::BitBoard {
        pb1: 0b_000000_000000_111_010_000_000,
        lb: 0b_000000_000000_010_000_000_010,
        kb: 0b_000000_000000_100_000_000_001,
        zb: 0b_000000_000000_001_000_000_100,
        hb: 0b_000000_000000_000_010_000_000,
        nb: 0b_000000_000000_000_000_010_000,
        pb2: 0b_000000_000000_000_000_010_111,
    };
    let move_vec_list = [
        (B3_INDEX, A1_INDEX),
        (B2_INDEX, A2_INDEX),
        (A1_INDEX, A2_INDEX),
        (C1_INDEX, B2_INDEX),
        (A2_INDEX, A3_INDEX),
        (B2_INDEX, A3_INDEX),
        (B4_INDEX, A3_INDEX),
        (E1_INDEX, A2_INDEX),
        (A3_INDEX, A2_INDEX),
        (B1_INDEX, C1_INDEX),
        (D1_INDEX, B3_INDEX),
        (C1_INDEX, B1_INDEX),
        (D1_INDEX, A3_INDEX),
    ];
    let mut is_player1 = true;
    //println!("{}", board);
    for move_vec in move_vec_list {
        board = make_moved_board(&board, move_vec, is_player1);
        //println!("{}, {:?}", if is_player1 { "p1" } else { "p2" }, move_vec);
        //println!("{}", board);
        is_player1 = !is_player1;
    }

    let moved_board = bit_board::bit_board::BitBoard {
        pb1: 0b_000000_000011_101_011_001_000,
        lb: 0b_000000_000000_000_000_001_010,
        kb: 0b_000000_000000_100_010_000_000,
        zb: 0b_000000_000001_001_000_000_000,
        hb: 0b_000000_000010_000_001_000_000,
        nb: 0b_000000_000000_000_000_000_000,
        pb2: 0b_000000_000000_000_000_000_010,
    };
    assert_eq!(moved_board, board);
}

#[test]
fn test3_make_moved_board() {
    let mut board = bit_board::bit_board::BitBoard {
        pb1: 0b_000000_000000_111_010_000_000,
        lb: 0b_000000_000000_010_000_000_010,
        kb: 0b_000000_000000_100_000_000_001,
        zb: 0b_000000_000000_001_000_000_100,
        hb: 0b_000000_000000_000_010_010_000,
        nb: 0b_000000_000000_000_000_000_000,
        pb2: 0b_000000_000000_000_000_010_111,
    };
    let move_vec_list = [
        (C4_INDEX, C3_INDEX),
        (B2_INDEX, A4_INDEX),
        (C3_INDEX, C2_INDEX),
        (A4_INDEX, C2_INDEX),
        (B3_INDEX, B2_INDEX),
        (C2_INDEX, B3_INDEX),
        (B4_INDEX, C4_INDEX),
        (B3_INDEX, B2_INDEX),
        (C4_INDEX, B2_INDEX),
        (E2_INDEX, B4_INDEX),
        (D1_INDEX, B3_INDEX),
        (B1_INDEX, B3_INDEX),
        (B2_INDEX, B1_INDEX),
        (E2_INDEX, C4_INDEX),
    ];
    let mut is_player1 = true;
    for move_vec in move_vec_list {
        board = make_moved_board(&board, move_vec, is_player1);
        println!("{:?}", board);
        is_player1 = !is_player1;
    }
    let moved_board = bit_board::bit_board::BitBoard {
        pb1: 0b_000000_000000_000_000_000_010,
        lb: 0b_000000_000000_000_010_000_010,
        kb: 0b_000000_000000_010_000_000_001,
        zb: 0b_000001_000000_000_000_000_100,
        hb: 0b_000010_000000_100_000_000_000,
        nb: 0b_000000_000000_000_000_000_000,
        pb2: 0b_000011_000000_110_010_000_101,
    };
    assert_eq!(moved_board, board);
}

// #[test]
// fn test1_eval_finction() {
//     let board = bit_board::bit_board::BitBoard {
//         hb1: 4112,
//         lb1: 1024,
//         kb1: 2048,
//         zb1: 512,
//         nb1: 0,
//         hb2: 0,
//         lb2: 32,
//         kb2: 1,
//         zb2: 4,
//         nb2: 0,
//     };
//     let point: i32 = eval_function(&board, &board, true);
//     println!("{}", point);
// }
