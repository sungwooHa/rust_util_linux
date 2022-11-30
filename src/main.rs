extern crate term_size;

use crossterm::event::KeyCode;
use pancurses::{endwin, initscr, Input};
use std::error::Error;
use std::fs::{self, File, read};
use std::io::{BufRead, BufReader, stdin, Read, stdout};
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

    let mut working_directory = std::env::current_dir().expect("can't find current working diretory");

    for (index, argument) in env::args().into_iter().enumerate() {
        if index == 0 {
            continue;
        } else if index == 1{
            working_directory.push(argument);
        }
        else if argument == "more" {
            more(&working_directory);
        }
    }
}

fn get_cmd_current_height() -> Option<usize> {
    if let Some((w, h)) = term_size::dimensions() {
        Some(h)
    }
    else {
        None
    }

}

fn more(file_path : &PathBuf) {
    let Some(height_size) = get_cmd_current_height() else {
        println!("Unable to get term size :(");
        return;
    };

    println!("path : {:?}", file_path);
    let file = File::open(file_path).expect("fail to open");
    let reader = BufReader::new(file);

    let vec_line : Vec<_> = reader.lines().into_iter().collect();

    let mut index = 0;
    loop {
        for line in &vec_line[index*height_size..(index+1)*height_size] {
            if let Ok(str_line) = line {
                println!("{}", str_line );
            }
        }
        
        let event = crossterm::event::read().expect("fail read event");

        println!("{:?}", event);
        match event {
            crossterm::event::Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Left => todo!(),
                    KeyCode::Right => todo!(),
                    KeyCode::Up => {
                        index = {
                            if index > 0 {
                                index - 1
                            }
                            else {
                                index
                            }
                        }
                    }
                    KeyCode::Esc => {
                        return;
                    },
                    KeyCode::Down => {
                        index += 1;
                        continue;
                    },
                    _=> {
                        println!("not support");
                    }
                }

            },
            _ => {
                println!("?")
            }
        }

    }

}