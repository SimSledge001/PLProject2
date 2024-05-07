
// Import necessary modules from the standard library

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Struct representing a point in 3D space
#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}


// Struct representing a circle in 3D space
#[derive(Debug, Copy, Clone)]
struct Circle {
    center: Point3D,
    radius: f64,
}

// Enum representing the direction of rotation
#[derive(Debug, Copy, Clone)]
enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

// Opens a file and returns an iterator over its lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // Attempt to open the file
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



// Import necessary modules from the standard library
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Struct representing a point in 3D space
#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// Struct representing a circle in 3D space
#[derive(Debug, Copy, Clone)]
struct Circle {
    center: Point3D,
    radius: f64,
}

// Enum representing the direction of rotation
#[derive(Debug, Copy, Clone)]
enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

// Opens a file and returns an iterator over its lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    // Attempt to open the file
    let file = File::open(filename)?;
    // Wrap the file reader in a buffered reader and return an iterator over its lines
    Ok(io::BufReader::new(file).lines())
}

// Parses a string into a Point3D struct
fn parse_point(point_str: &str) -> Point3D {
    // Split the string by commas, trim whitespace, and parse each coordinate as f64
    let coords: Vec<f64> = point_str.split(',').map(|s| s.trim().parse().unwrap()).collect();
    // Return a Point3D struct with the parsed coordinates
    Point3D { x: coords[0], y: coords[1], z: coords[2] }
}

// Parses a linear motion command into start and end points
fn parse_linear_motion(line: &str) -> (Point3D, Point3D) {
    // Split the command by whitespace
    let parts: Vec<&str> = line.split_whitespace().collect();
    // Parse the start and end points from the command
    let start = parse_point(parts[1]);
    let end = parse_point(parts[2]);
    // Return a tuple containing the start and end points
    (start, end)
}

// Calculates and prints intermediate positions for a linear motion
fn calculate_linear_positions(start: Point3D, end: Point3D) {
    // Calculate the differences between coordinates
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;
    let delta_z = end.z - start.z;
    // Calculate the number of steps based on Euclidean distance
    let num_steps = (delta_x.powi(2) + delta_y.powi(2) + delta_z.powi(2)).sqrt() as i32;

    // Calculate step sizes for each coordinate
    let step_x = delta_x / num_steps as f64;
    let step_y = delta_y / num_steps as f64;
    let step_z = delta_z / num_steps as f64;

    // Initialize the current position to the start point
    let mut current_pos = start;
    // Iterate over each step and print the intermediate positions
    for _step in 0..=num_steps {
        println!("{:.2}, {:.2}, {:.2}", current_pos.x, current_pos.y, current_pos.z);
        // Update the current position for the next step
        current_pos.x += step_x;
        current_pos.y += step_y;
        current_pos.z += step_z;
    }
}

// Parses a rotational motion command into a Circle, stop angle, and rotation direction
fn parse_rotational_motion(line: &str) -> (Circle, f64, RotationDirection) {
    // Split the command by whitespace
    let parts: Vec<&str> = line.split_whitespace().collect();
    // Parse the center point, radius, direction, and stop angle from the command
    let center = parse_point(parts[1]);
    let radius = parts[2].parse().unwrap();
    let direction = match parts[3] {
        "clockwise" => RotationDirection::Clockwise,
        "counterclockwise" => RotationDirection::Counterclockwise,
        _ => panic!("Invalid direction"),
    };
    let stop_angle = parts[4].parse().unwrap();
    // Return a tuple containing the Circle, stop angle, and rotation direction
    (Circle { center, radius }, stop_angle, direction)
}

// Calculates and prints intermediate positions for a rotational motion
fn calculate_rotational_positions(circle: Circle, stop_angle: f64, direction: RotationDirection) {
    // Initialize the start angle to 0
    let start_angle = 0.0;
    // Calculate the number of steps based on the difference between start and stop angles
    let num_steps = ((stop_angle - start_angle) / 5.0).ceil() as i32;
    // Calculate the step size for the angle
    let step_angle = (stop_angle - start_angle) / num_steps as f64;

    // Initialize the current angle to the start angle
    let mut current_angle = start_angle;
    // Iterate over each step and print the intermediate positions
    for _step in 0..=num_steps {
        // Calculate the x and y coordinates using trigonometric functions
        let x = circle.center.x + circle.radius * current_angle.to_radians().cos();
        let y = circle.center.y + circle.radius * current_angle.to_radians().sin();
        println!("{:.2}, {:.2}, {:.2}", x, y, circle.center.z);
        // Update the current angle based on the rotation direction and step size
        current_angle += match direction {
            RotationDirection::Clockwise => -step_angle,
            RotationDirection::Counterclockwise => step_angle,
        };
    }
}

// Entry point of the program
fn main() {
    // If an error occurs during execution, print an error message and exit with status code 1
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

// Runs the program
fn run() -> io::Result<()> {
    // Specify the input filename
    let filename = "input.txt";
    // Read the lines from the input file
    let lines = read_lines(filename)?;

    // Iterate over each line in the input file
    for line in lines {
        // Unwrap the line or handle the error
        let command = line?;
        // Check if the command represents a linear motion
        if command.starts_with("linear") {
            // Parse the linear motion command into start and end points
            let (start, end) = parse_linear_motion(&command);
            // Calculate and print the intermediate positions for the linear motion
            calculate_linear_positions(start, end);
        }
        // Check if the command represents a rotational motion
        else if command.starts_with("rotational") {
            // Parse the rotational motion command into a Circle, stop angle, and rotation direction
            let (circle, stop_angle, direction) = parse_rotational_motion(&command);
            // Calculate and print the intermediate positions for the rotational motion
            calculate_rotational_positions(circle, stop_angle, direction);
        }
    }

    // Return Ok if execution is successful
    Ok(())
}