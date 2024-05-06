use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Copy, Clone)]
struct Circle {
    center: Point3D,
    radius: f64,
}

#[derive(Debug, Copy, Clone)]
enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_point(point_str: &str) -> Point3D {
    let coords: Vec<f64> = point_str.split(',').map(|s| s.trim().parse().unwrap()).collect();
    if coords.len() != 3 {
        panic!("Invalid point format");
    }
    Point3D {
        x: coords[0],
        y: coords[1],
        z: coords[2],
    }
}

fn parse_linear_motion(line: &str) -> (Point3D, Point3D) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        panic!("Invalid linear motion format");
    }
    let start = parse_point(parts[1]);
    let end = parse_point(parts[2]);
    (start, end)
}

fn calculate_linear_positions(start: Point3D, end: Point3D) {
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;
    let delta_z = end.z - start.z;
    let num_steps = (delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2)).sqrt() as i32;

    let step_x = delta_x / num_steps as f64;
    let step_y = delta_y / num_steps as f64;
    let step_z = delta_z / num_steps as f64;

    let mut current_pos = start;
    for _step in 0..=num_steps {
        println!("{:.2}, {:.2}, {:.2}", current_pos.x, current_pos.y, current_pos.z);
        current_pos.x += step_x;
        current_pos.y += step_y;
        current_pos.z += step_z;
    }
}

fn parse_rotational_motion(line: &str) -> (Circle, f64, RotationDirection) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 5 {
        panic!("Invalid rotational motion format");
    }
    let center = parse_point(parts[1]);
    let radius = parts[2].parse().unwrap();
    let direction = match parts[3] {
        "clockwise" => RotationDirection::Clockwise,
        "counterclockwise" => RotationDirection::Counterclockwise,
        _ => panic!("Invalid direction"),
    };
    let stop_angle = parts[4].parse().unwrap();
    (Circle { center, radius }, stop_angle, direction)
}

fn calculate_rotational_positions(circle: Circle, stop_angle: f64, direction: RotationDirection) {
    let start_angle = 0.0;
    let num_steps = ((stop_angle - start_angle) / 5.0).ceil() as i32;

    let step_angle = (stop_angle - start_angle) / num_steps as f64;

    let mut current_angle = start_angle;
    for _step in 0..=num_steps {
        let x = circle.center.x + circle.radius * current_angle.to_radians().cos();
        let y = circle.center.y + circle.radius * current_angle.to_radians().sin();
        println!("{:.2}, {:.2}, {:.2}", x, y, circle.center.z);
        current_angle += match direction {
            RotationDirection::Clockwise => -step_angle,
            RotationDirection::Counterclockwise => step_angle,
        };
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let filename = "input.txt";
    let lines = read_lines(filename)?;

    for line in lines {
        let command = line?;
        if command.starts_with("linear") {
            let (start, end) = parse_linear_motion(&command);
            calculate_linear_positions(start, end);
        } else if command.starts_with("rotational") {
            let (circle, stop_angle, direction) = parse_rotational_motion(&command);
            calculate_rotational_positions(circle, stop_angle, direction);
        }
    }

    Ok(())
}
