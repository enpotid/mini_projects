use std::{thread, time::Duration};

fn main() {
    let size = 10;
    let tiles = "@#$%^&".chars().collect::<Vec<char>>();

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                vertices.push([x, y, z]);
            }
        }
    }

    let sin45 = 0.5_f32.sqrt();
    let rotation_matrix_a: [[f32; 3]; 3] =
        [[0.5, 0.5, sin45], [0.5, 0.5, -sin45], [-sin45, sin45, 0.0]];

    loop {
        println!("\x1B[2J\x1B[H");

        let mut min_y = f32::MAX;
        let mut min_list = Vec::new();
        for i in 0..8 {
            vertices[i] = matvec(rotation_matrix_a, vertices[i]);
            if min_y > vertices[i][1] {
                min_y = vertices[i][1];
                min_list = vec![i];
            } else if min_y == vertices[i][1] {
                min_list.push(i);
            }
        }

        match min_list.len() {
            1 => {}
            2 => {}
            4 => {}
            _ => {}
        }

        draw(&vertices);

        thread::sleep(Duration::from_micros(1));
    }
}

fn draw(vertices: &Vec<[f32; 3]>) {}

fn matvec(matrix: [[f32; 3]; 3], vector: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}
