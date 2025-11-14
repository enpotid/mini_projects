use std::{thread, time::Duration};

fn main() {
    let height = 10;
    let width = 26;
    let offset = 50;
    let tiles = " @#$%^&".chars().collect::<Vec<char>>();

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
        [0.00011923, 0.9998746, -0.01583074],
        [-0.01234036, 0.01583101, 0.99979853],
    ];

    loop {
        println!("\x1B[2J\x1B[H");

        let mut screen: Vec<Vec<usize>> = vec![vec![0; width + offset * 2]; height + offset * 2];

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

        for p in [1, 2, 4] {
            let mut x = min_list[0] ^ p;
            let mut bx = min_list[0] ^ p ^ 7;
            let v = vertices[min_list[0] ^ p];
            let mut face_vertices = vec![[v[0], v[2]]];

            for i in [1, 2, 4] {
                x &= min_list[0] ^ (p | (i ^ 7));
                bx &= min_list[0] ^ (p | (i ^ 7)) ^ 7;
                let v = vertices[min_list[0] ^ (p | (i ^ 7))];
                face_vertices.push([v[0], v[2]]);
            }

            x += bx << 3;
            x <<= 1;

            draw(
                &mut screen,
                face_vertices
                    .iter()
                    .map(|s| {
                        [
                            s[0] * width as f32 + offset as f32,
                            s[1] * height as f32 + offset as f32,
                        ]
                    })
                    .collect(),
                x.ilog2() as usize,
            );
        }

        for line in screen {
            println!(
                "{}",
                line.iter()
                    .map(|s| format!("{}", tiles[*s].to_string()))
                    .collect::<Vec<String>>()
                    .join("")
            )
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn draw(screen: &mut Vec<Vec<usize>>, vertices: Vec<[f32; 2]>, t: usize) {
    let sorted_vertices = sort_quad_vertices(&vertices);

    for i in 0..screen.len() {
        for j in 0..screen[0].len() {
            let px = j as f32;
            let py = i as f32;
            if point_in_sorted_quad(px, py, &sorted_vertices) {
                screen[i][j] = t;
            }
        }
    }
}

fn sort_quad_vertices(quad: &Vec<[f32; 2]>) -> Vec<[f32; 2]> {
    if quad.len() != 4 {
        return quad.clone();
    }

    let center = [
        quad.iter().map(|p| p[0]).sum::<f32>() / 4.0,
        quad.iter().map(|p| p[1]).sum::<f32>() / 4.0,
    ];

    let mut sorted = quad.clone();
    sorted.sort_by(|a, b| {
        let angle_a = (a[1] - center[1]).atan2(a[0] - center[0]);
        let angle_b = (b[1] - center[1]).atan2(b[0] - center[0]);
        angle_a.partial_cmp(&angle_b).unwrap()
    });

    sorted
}

fn point_in_sorted_quad(px: f32, py: f32, quad: &Vec<[f32; 2]>) -> bool {
    fn cross(a: [f32; 2], b: [f32; 2]) -> f32 {
        a[0] * b[1] - a[1] * b[0]
    }

    if quad.len() != 4 {
        return false;
    }

    let mut sign: Option<bool> = None;

    for i in 0..4 {
        let a = quad[i];
        let b = quad[(i + 1) % 4];
        let edge = [b[0] - a[0], b[1] - a[1]];
        let to_point = [px - a[0], py - a[1]];
        let current = cross(edge, to_point);

        if current.abs() > 0.0 {
            let is_positive = current > 0.0;
            match sign {
                None => sign = Some(is_positive),
                Some(s) => {
                    if s != is_positive {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn matvec(matrix: [[f32; 3]; 3], vector: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}
