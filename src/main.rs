#[warn(unused_variables)]
extern crate fxhash;
use fxhash::FxHashMap;
mod state;

use crate::state::state::PlayerInfo;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

fn main() {
    // let a:Option<&str> = Some("a");
    // let start = Instant::now();
    // // 1.234秒休んでみる
    // for _ in 1000000{
    //     match a {
    //         None => (),
    //         Some(x) =>
    //     }
    // }

    // let end = start.elapsed();
    // println!(
    //     "{}.{:03}秒経過しました。",
    //     end.as_secs(),
    //     end.subsec_nanos() / 1_000_000
    // );

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

                let mut buffer: Vec<u8> = Vec::new();
                reader
                    .read_until(b'\n', &mut buffer)
                    .expect("Receive failure.");
                // "You are Player1" or "You are Player2"

                // プレイヤー情報の設定
                let mut player_info = PlayerInfo {
                    turn: String::from("1"),
                    try_line: String::from("1"),
                    hand_alf: String::from("D"),
                    opponent_turn: String::from("2"),
                    opponent_try_line: String::from("4"),
                    opponent_hand_alf: String::from("E"),
                };

                if buffer[14] == b'1' {
                    println!("You are Player 1.");
                } else if buffer[14] == b'2' {
                    // 後攻の場合,プレイヤー情報を書き換える
                    println!("You are Player 2.");
                    player_info.turn = String::from("2");
                    player_info.try_line = String::from("4");
                    player_info.hand_alf = String::from("E");
                    player_info.opponent_turn = String::from("1");
                    player_info.opponent_try_line = String::from("1");
                    player_info.opponent_hand_alf = String::from("D");
                } else {
                    println!("Could not get player data.");
                }

                loop {
                    let mut buffer: Vec<u8> = Vec::new();
                    // 自分のターンを確認
                    write_socket(&mut writer, "turn");
                    reader
                        .read_until(b'\n', &mut buffer)
                        .expect("Receive failure.");

                    // 自分のターンの場合
                    let buffer_str = std::str::from_utf8(&buffer).unwrap();

                    if &buffer_str[6..7] == &player_info.turn {
                        // 盤面情報を取得
                        let mut buffer: Vec<u8> = Vec::new();
                        write_socket(&mut writer, "board");
                        reader
                            .read_until(b'\n', &mut buffer)
                            .expect("Receive failure.");

                        let board_str = std::str::from_utf8(&buffer).unwrap();

                        //println!("{}", board);

                        let mut board: FxHashMap<&str, &str> = make_map(board_str);

                        //let t = ("C4", "C3");
                        let t = ("B3", "B2");
                        player_info.make_next_board(&board, t);
                        //相手のターンが終わるまで待つ
                        loop {
                            let mut buffer: Vec<u8> = Vec::new();
                            write_socket(&mut writer, "turn");
                            reader
                                .read_until(b'\n', &mut buffer)
                                .expect("Receive failure.");
                            let buffer_str = std::str::from_utf8(&buffer).unwrap();
                            // 自分のターンが来たらループを抜ける
                            if &buffer_str[6..7] == &player_info.turn {
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Invalid Host:Port Number.");
    }
}

fn write_socket(writer: &mut BufWriter<&TcpStream>, msg: &str) {
    let buf = format!("{}\n", msg);
    writer.write(buf.as_bytes()).expect("Send failure.");
    let _ = writer.flush();
}

fn make_map(str_board: &str) -> FxHashMap<&str, &str> {
    let mut board_map: FxHashMap<&str, &str> = HashMap::default();
    let v: Vec<&str> = str_board.split(", ").collect();

    for s in v {
        let temp: Vec<&str> = s.split(" ").collect();
        if temp[0] != "\n" && temp[1] != "--" {
            board_map.insert(temp[0], temp[1]);
        }
    }
    return board_map;
}

//fn search_next_move(board_map: FxHashMap<&str, &str>, player_info: PlayerInfo) {}
