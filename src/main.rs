extern crate pancurses;
extern crate term_size;

use pancurses::{endwin, initscr, Input};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, stdin};
use std::path::PathBuf;
use std::{env, io};

fn read_file(path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

//argument 분석기 필요
//2개 thread 필요,
//// 사용자 입력을 통해 buffer에 데이터 넣는 역할
//// buffer를 출력하는 역할

fn main() {
    let mut height_max: usize = 0;
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        height_max = h;
    } else {
        println!("Unable to get term size :(")
    }

    let working_directory = std::env::current_dir().expect("can't find current working diretory");

    for (index, argument) in env::args().into_iter().enumerate() {
        if index == 0 {
            continue;
        }

        let mut path = PathBuf::from(working_directory.clone());
        path.push(argument);

        let file = File::open(path).expect("fail to open");
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().into_iter().enumerate() {
            if index > 0 && index % height_max == 0 {
                break;
            }

            println!("{}", line.unwrap());
        }

        // match read_file(path.to_str().unwrap().to_string()) {
        //     Ok(contents) => {
        //         println!("success file reading");
        //         println!("{contents}");
        //     }
        //     Err(_) => {
        //         println!("fail to read file");
        //     }
        // }
    }
}
