use image::{Rgb, RgbImage};
use rusttype::{Font, Scale};
use std::fs::File;
use std::io::{BufReader, BufRead};
use imageproc::drawing::{draw_line_segment_mut, draw_text_mut};
use rand::thread_rng;
use textwrap::fill;
use std::borrow::Borrow;
use rand::seq::SliceRandom;
use log::{info, error};

const WHITE: Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
const BLACK: Rgb<u8> = Rgb([0u8, 0u8, 0u8]);
const RED: Rgb<u8> = Rgb([255u8, 0u8, 0u8]);

pub struct Bingo<'a>
{
    canvas_size: usize,
    cell_count: usize,
    cell_size: usize,
    canvas: Option<RgbImage>,
    sources: Vec<String>,
    bonus_tile: String,
    font: Font<'a>,
}

impl<'a> Bingo<'a> {
    pub fn new(size: usize, count: usize, source_path: &str) -> Self {
        let (s, bonus) = Bingo::read_source(source_path, count);
        let f = Font::from_bytes(ttf_firacode::REGULAR).expect("failed loading font");

        Bingo {
            canvas_size: size,
            cell_count: count,
            cell_size: size / count,
            canvas: None,
            sources: s,
            bonus_tile: bonus,
            font: f,
        }
    }

    pub fn shuffle(&mut self) -> &mut Self {
        self.sources.shuffle(&mut thread_rng());
        self
    }

    pub fn draw(&mut self) -> &Self {
        self.build_canvas();
        self
    }

    pub fn dump_to(&self, output_path: &str) {
        self.canvas.as_ref().unwrap().save(output_path).unwrap();
        info!("Saved to: {}", output_path);
    }

    fn read_source(filename: &str, cell_count: usize) -> (Vec<String>, String) {
        let file = match File::open(filename) {
            Ok(t) => { t }
            Err(_) => {
                error!("Error opening {}", &filename);
                std::process::exit(2);
            }
        };
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let bonus_tile = lines.next().unwrap().unwrap();
        let mut source_vec: Vec<String> = Vec::with_capacity(cell_count * cell_count);

        for line in lines {
            let line = line.unwrap();
            source_vec.push(line);
        }

        (source_vec, bonus_tile)
    }

    fn build_canvas(&mut self) {
        let mut canvas = RgbImage::new((self.canvas_size + 1) as u32, (self.canvas_size + 1) as u32);
        Bingo::paint_white(&mut canvas);
        self.draw_grid(&mut canvas);
        self.draw_sources(&mut canvas);
        self.canvas = Some(canvas);
    }

    fn paint_white(image: &mut RgbImage) {
        for pixel in image.pixels_mut() {
            *pixel = WHITE;
        }
    }

    fn draw_grid(&mut self, canv: &mut RgbImage)
    {
        for i in 0..=self.cell_count {
            draw_line_segment_mut(
                canv,
                (0.0, (i * self.cell_size) as f32),
                (self.canvas_size as f32, (i * self.cell_size) as f32),
                BLACK,
            );

            draw_line_segment_mut(
                canv,
                ((i * self.cell_size) as f32, 0.0),
                ((i * self.cell_size) as f32, self.canvas_size as f32),
                BLACK,
            );
        }
    }

    fn draw_sources(&mut self, canvas: &mut RgbImage) {
        let height: usize = self.cell_size / 8;
        let max_cell_lines: usize = self.cell_size / height;

        for (col, mut cell) in self.sources.iter().enumerate() {
            if col > self.cell_count * self.cell_count {
                break;
            }

            let line = (col - (col % self.cell_count)) / self.cell_count;
            let col = col % self.cell_count;

            let color = if col == self.cell_count / 2 && line == self.cell_count / 2 {
                cell = &self.bonus_tile;
                RED
            } else {
                BLACK
            };

            let max_chars_in_cell = (self.cell_size * 2 / height) - 2;
            let cell = fill(cell, max_chars_in_cell as usize);
            let cell_lines = cell.lines();
            let iter = cell_lines.clone().enumerate();
            let x = iter.count();
            let starting_line = (max_cell_lines / 2) - (x / 2) - 1;

            for (i, cell_line) in cell_lines.enumerate() {
                let cell_line = format!("{:^16}", cell_line);

                draw_text_mut(
                    canvas,
                    color,
                    (col * self.cell_size) as u32,
                    (line * self.cell_size + i * height + starting_line * height) as u32,
                    Scale::uniform(height as f32),
                    self.font.borrow(),
                    cell_line.as_str(),
                );
            }
        }
    }
}
