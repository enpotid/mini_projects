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

    let rotation_matrix: [[f32; 3]; 3] = [
        [0.9999238, 0.00007615, 0.0123407],
        [0.00007615, 0.9999238, -0.0123407],
        [-0.0123407, 0.0123407, 0.9998477],
    ];

    loop {
        println!("\x1B[2J\x1B[H");

        let mut screen: Vec<Vec<u8>> = Vec::new();

        let mut min_y = f32::MAX;
        let mut min_list = Vec::new();
        for i in 0..8 {
            vertices[i] = matvec(rotation_matrix, vertices[i]);
            if min_y > vertices[i][1] {
                min_y = vertices[i][1];
                min_list = vec![i];
            } else if min_y == vertices[i][1] {
                min_list.push(i);
            }
        }

        match min_list.len() {
            1 => {
                for p in [1, 2, 4] {
                    let v = vertices[min_list[0] ^ p];
                    let mut face_vertices = vec![[v[0], v[2]]];

                    for i in [1, 2, 4] {
                        let v = vertices[min_list[0] ^ (p | (i ^ 7))];
                        face_vertices.push([v[0], v[2]]);
                    }

                    draw(&mut screen, face_vertices);
                }
            }
            2 => {
                let x = (min_list[0] ^ min_list[1]).ilog2() as usize;
                let o = 1 << ((x + 1) % 3);
                let t = 1 << ((x + 2) % 3);

                let mut d = Vec::new();
                for y in &min_list {
                    let v = vertices[y ^ (o | t)];
                    d.push([v[0], v[2]]);
                }

                for i in [o, t] {
                    let mut face_vertices = Vec::new();
                    for y in &min_list {
                        let v = vertices[y ^ i];
                        face_vertices.push([v[0], v[2]]);
                    }

                    face_vertices.extend(&d);
                    draw(&mut screen, face_vertices);
                }
            }
            4 => {
                let mut face_vertices = Vec::new();
                for i in min_list {
                    let v = vertices[i];
                    face_vertices.push([v[0], v[2]]);
                }

                draw(&mut screen, face_vertices);
            }
            _ => {}
        }

        thread::sleep(Duration::from_micros(1));
    }
}

fn draw(screen: &mut Vec<Vec<u8>>, vertices: Vec<[f32; 2]>) {}

fn matvec(matrix: [[f32; 3]; 3], vector: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}
