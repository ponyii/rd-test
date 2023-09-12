use std::convert::TryFrom;
use std::io::Write;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[repr(i8)]
#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self, clockwise: bool) {
        let diff = if clockwise { 1 } else { -1 };
        *self = Direction::try_from((*self as i8 + diff).rem_euclid(4)).unwrap()
    }
}

impl TryFrom<i8> for Direction {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, ()> {
        match value {
            x if x == Direction::Up as i8 => Ok(Direction::Up),
            x if x == Direction::Left as i8 => Ok(Direction::Left),
            x if x == Direction::Down as i8 => Ok(Direction::Down),
            x if x == Direction::Right as i8 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

const SIZE: usize = 1024;

type Coords = (usize, usize);

struct Ant {
    path: Vec<bool>, // `path[i]` indicates the i-th path pixel's color.
    position: Coords,
    direction: Direction,
    black_pixel_num: usize,
}

impl Ant {
    const START_POS: Coords = (SIZE / 2, SIZE / 2);
    const START_DIR: Direction = Direction::Up;

    pub fn new() -> Self {
        Ant {
            path: vec![],
            position: Self::START_POS,
            direction: Self::START_DIR,
            black_pixel_num: 0,
        }
    }

    // Get pixel colour.
    // There's no need to re-trace the path for each pixel in fact,
    // colours of a field segment could have been cached.
    fn is_white(&self, pixel: &Coords) -> bool {
        let mut fake_position = Self::START_POS;
        let mut fake_direction = Self::START_DIR;
        let mut result = *pixel == fake_position;
        for color in &self.path {
            fake_direction.turn(*color);
            Self::one_step(&mut fake_position, &fake_direction);
            if *pixel == fake_position {
                result = !result;
            }
        }
        result
    }

    pub fn run(&mut self) {
        loop {
            let is_white = self.is_white(&self.position);
            match is_white {
                true => self.black_pixel_num += 1,
                false => self.black_pixel_num -= 1,
            };
            self.path.push(is_white);
            self.direction.turn(is_white);
            if !Self::one_step(&mut self.position, &self.direction) {
                break;
            }
        }
    }

    // Make a step and return `true` if the ant is still on the field.
    fn one_step(coords: &mut Coords, dir: &Direction) -> bool {
        match dir {
            Direction::Up => coords.0 = coords.0.wrapping_sub(1),
            Direction::Left => coords.1 = coords.1.wrapping_sub(1),
            Direction::Down => coords.0 = coords.0.wrapping_add(1),
            Direction::Right => coords.1 = coords.1.wrapping_add(1),
        }
        return (0..SIZE).contains(&coords.0) && (0..SIZE).contains(&coords.1);
    }

    // Mark all the visited pixels of the given row.
    fn visited_pixels(&self, row_n: usize) -> [bool; SIZE] {
        let mut fake_position = Self::START_POS;
        let mut fake_direction = Self::START_DIR;
        let mut result = [false; SIZE];
        for color in &self.path {
            if fake_position.0 == row_n {
                result[fake_position.1] = true;
            }
            fake_direction.turn(*color);
            Self::one_step(&mut fake_position, &fake_direction);
        }
        result
    }

    pub fn path_to_png(&self, path: &Path) {
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, SIZE as u32, SIZE as u32);
        encoder.set_depth(png::BitDepth::One);
        let mut writer = encoder.write_header().unwrap();
        let mut s_writer = writer.stream_writer_with_size(SIZE / 8).unwrap();

        for row in 0..SIZE {
            let mut data = [u8::MAX; SIZE / 8];
            let visited = self.visited_pixels(row);
            for i in 0..visited.len() {
                if visited[i] {
                    data[i / 8] &= u8::MAX - 2u8.pow(7 - (i as u32 % 8));
                }
            }
            s_writer.write(&data).unwrap();
        }
        s_writer.finish().unwrap();
    }
}

fn main() {
    let mut ant = Ant::new();
    ant.run();
    println!("ant.path.len() = {:?}", ant.path.len());
    println!("ant.black_pixel_num = {:?}", ant.black_pixel_num);

    let path = Path::new(r"./result.png");
    ant.path_to_png(path);
}
