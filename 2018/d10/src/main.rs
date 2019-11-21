use std::fs::read_to_string;
use std::io::Result;
use std::path::Path;

extern crate piston_window;

use piston_window::*;
use regex::Regex;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Velocity {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Point {
    position: Position,
    velocity: Velocity,
}

fn parse(s: String) -> Vec<Point> {
    let mut acc = Vec::<Point>::new();
    let re =
        Regex::new(r"^position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>").unwrap();

    fn extract(captures: &regex::Captures, index: usize) -> f64 {
        captures
            .get(index)
            .map_or_else(|| 0.0, |e| e.as_str().parse().unwrap())
    }

    for line in s.lines() {
        if re.is_match(line) {
            let captures = re.captures(line).unwrap();

            acc.push(Point {
                position: Position {
                    x: extract(&captures, 1),
                    y: extract(&captures, 2),
                },
                velocity: Velocity {
                    x: extract(&captures, 3),
                    y: extract(&captures, 4),
                },
            });
        }
    }

    acc
}

fn main() -> Result<()> {
    let mut set = parse(read_to_string(Path::new("data/input"))?);

    println!("{:?}", set);

    let winsize = (1000.0, 1000.0);
    let mut stop = false;
    let mut offset = (0.0, 0.0);
    let mut timer = 0;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", winsize)
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g| {
            clear(color::WHITE, g);

            let mut draw_points = |set: &Vec<Point>| {
                for e in set.iter() {
                    rectangle(
                        color::hex("AA0055"),
                        [
                            winsize.0 / 2.0 + e.position.x * 5.0 + offset.0,
                            winsize.1 / 2.0 + e.position.y * 5.0 + offset.1,
                            5.0,
                            5.0,
                        ],
                        c.transform,
                        g,
                    )
                }
            };

            draw_points(&set);
        });

        let mut transform_points = |set: &mut Vec<Point>| {
            for e in set.iter_mut() {
                e.position.x += e.velocity.x * 1.0;
                e.position.y += e.velocity.y * 1.0;
            }

            timer += 1;
            println!("Timer: {}", timer);
        };

        if !stop {
            transform_points(&mut set);
        }

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(Key::Space) => stop = !stop,
                Button::Keyboard(Key::N) => transform_points(&mut set),
                Button::Keyboard(Key::Left) => offset.0 -= 10.0,
                Button::Keyboard(Key::Right) => offset.0 += 10.0,
                Button::Keyboard(Key::Up) => offset.1 -= 10.0,
                Button::Keyboard(Key::Down) => offset.1 += 10.0,
                _ => (),
            }
        };
    }

    Ok(())
}
