#![windows_subsystem = "windows"]

extern crate macroquad;
extern crate rand;
use macroquad::prelude::*;
use std::env;
use rand::prelude::*;
use bit_vec::*;
use base32::*;

#[macroquad::main("Automata")]
async fn main() {
    match std::fs::create_dir("./results") {
        Ok(_) => {}
        Err(_) => {}
    }
    clear_background(WHITE);
    let mut img = Image::gen_image_color(640, 480, WHITE);
    let args: Vec<String> = env::args().collect();
    let rule: u8;
    if args.len() == 1 {
        rule = 110;
    }
    else {
        rule = args[1].parse().unwrap();
    }
    let mut initial: Vec<bool> = Vec::new();
    let mut rows: Vec<Vec<bool>> = Vec::new();
    for _ in 0..640 {
        initial.push(random())
    }
    let name = base32::encode(Alphabet::Crockford, &(initial.clone().into_iter().collect::<BitVec>().to_bytes())[..]);
    rows.push(initial);
    println!("{}", name);
    for i in 0..479 {
        rows.push(gen_row(&rows[i], get_rule(rule)));
    }
    clear_background(WHITE);
    for (y, n) in rows.iter().enumerate() {
        for (x, i) in n.iter().enumerate() {
            match i {
                true => {img.set_pixel(x as _, y as _, BLACK)}
                false => {}
            }
        }
    }
    export(&img, format!("./results/{}-{1}.png", name, rule).as_str());
    let texture = Texture2D::from_image(&img);
    loop {
        if is_key_down(KeyCode::Enter) {
            break;
        }
        clear_background(WHITE);
        draw_texture(texture, 0., 0., WHITE);
        next_frame().await;
    }
}   

fn get_rule(num: u8) -> Vec<bool> {
    format!("{:08b}", num).chars().map(|i| i.to_digit(2).unwrap()).map(|i| match i {1 => true, 0 => false, _ => panic!()}).collect()
}

fn gen_row(row: &Vec<bool>, rule: Vec<bool>) -> Vec<bool>{
    let mut tmp: Vec<bool> = Vec::new();
    for (i, n) in row.iter().enumerate() {
        if i == 0 || i == 1 || i == row.iter().count()-1 {
            tmp.push(true)
        }
        else {
            match (row[i-1], n, row[i+1]) {
                (true, true, true) => {
                    tmp.push(rule[0])
                }
                (true, true, false) => {
                    tmp.push(rule[1])
                }
                (true, false, true) => {
                    tmp.push(rule[2])
                }
                (true, false, false) => {
                    tmp.push(rule[3])
                }
                (false, true, true) => {
                    tmp.push(rule[4])
                }
                (false, true, false) => {
                    tmp.push(rule[5])
                }
                (false, false, true) => {
                    tmp.push(rule[6])
                }
                (false, false, false) => {
                    tmp.push(rule[7])
                }
            }
        }
    }
    tmp
}

fn export(img: &Image, path: &str) {
    image::save_buffer(
        path,
        &img.bytes[..],
        img.width as _,
        img.height as _,
        image::ColorType::Rgba8,
    )
    .unwrap();
}