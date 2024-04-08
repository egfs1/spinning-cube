use std::thread;
use std::time::Duration;

const CUBE_WIDTH: f32 = 16.0;
const WIDTH: f32 = 60.0;
const HEIGHT: f32 = 26.0;
const BACKGROUND_ASCII_CODE: char = ' ';
const INCREMENT_SPEED: f32 = 0.6;
const DISTANCE_FROM_CAMERA: f32 = 100.0;
const K1: f32 = 40.0;

struct Cube3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Cube3D {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Cube3D { x, y, z }
    }

    fn project(&self) -> (i32, i32, f32) {
        let ooz = 1.0 / self.z;
        let xp = (WIDTH / 2.0 + K1 * self.x * ooz * 2.0) as i32;
        let yp = (HEIGHT / 2.0 + K1 * self.y * ooz) as i32;
        (xp, yp, ooz)
    }
}

fn calculate_for_surface(cube_x: f32, cube_y: f32, cube_z: f32, a: f32, b: f32, c: f32, ascii_code: char, buffer: &mut [char; (WIDTH * HEIGHT) as usize], z_buffer: &mut [f32; (WIDTH * HEIGHT) as usize]) {
    let cube = Cube3D::new(
        calculate_x(cube_x, cube_y, cube_z, a, b, c),
        calculate_y(cube_x, cube_y, cube_z, a, b, c),
        calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAMERA,
    );
    let (xp, yp, ooz) = cube.project();
    let idx = (yp * WIDTH as i32 + xp) as usize;

    if idx < WIDTH as usize * HEIGHT as usize && ooz > z_buffer[idx] {
        z_buffer[idx] = ooz;
        buffer[idx] = ascii_code;
    }
}

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.sin() * b.sin() * c.cos() -
    k * a.cos() * b.sin() * c.cos() +
    j * a.cos() * c.sin() +
    k * a.sin() * c.sin() +
    i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.cos() * c.cos() +
    k * a.sin() * c.cos() -
    j * a.sin() * b.sin() * c.sin() +
    k * a.cos() * b.sin() * c.sin() -
    i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32) -> f32 {
    k * a.cos() * b.cos() -
    j * a.sin() * b.cos() +
    i * b.sin()
}

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;

    let mut buffer: [char; (WIDTH * HEIGHT) as usize] = [' '; (WIDTH * HEIGHT) as usize];
    let mut z_buffer: [f32; (WIDTH * HEIGHT) as usize] = [0.0; (WIDTH * HEIGHT) as usize];

    println!("\x1b[2J");
    loop {
        buffer.iter_mut().for_each(|c| *c = BACKGROUND_ASCII_CODE);
        z_buffer.iter_mut().for_each(|z| *z = 0.0);

        let cube_z: f32 = -CUBE_WIDTH;
        let mut cube_x = -CUBE_WIDTH;
        while cube_x < CUBE_WIDTH {
            let mut cube_y = -CUBE_WIDTH;
            while cube_y < CUBE_WIDTH {
                calculate_for_surface(cube_x, cube_y, cube_z, a, b, c, '.', &mut buffer, &mut z_buffer);
                calculate_for_surface(-cube_z, cube_y, cube_x, a, b, c, '#', &mut buffer, &mut z_buffer);
                calculate_for_surface(cube_z, cube_y, -cube_x, a, b, c, '~', &mut buffer, &mut z_buffer);
                calculate_for_surface(-cube_x, cube_y, -cube_z, a, b, c, '$', &mut buffer, &mut z_buffer);
                calculate_for_surface(cube_x, -cube_z, -cube_y, a, b, c, ';', &mut buffer, &mut z_buffer);
                calculate_for_surface(cube_x, cube_z, cube_y, a, b, c, '+', &mut buffer, &mut z_buffer);
                cube_y += INCREMENT_SPEED;
            }
            cube_x += INCREMENT_SPEED;
        }
        println!("\x1b[H");
        for chunk in buffer.chunks(WIDTH as usize) {
            println!("{}", chunk.iter().collect::<String>());
        }

        a += 0.05;
        b += 0.05;
        c += 0.01;
        thread::sleep(Duration::from_micros(24000));
    }
}
