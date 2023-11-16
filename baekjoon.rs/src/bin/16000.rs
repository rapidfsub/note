use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<_> = input.split_whitespace().collect();
    let (n, m): (usize, usize) = (tokens[0].parse().unwrap(), tokens[1].parse().unwrap());
    let mut matrix = vec![];
    for line in tokens[2..].iter() {
        matrix.push(vec![]);
        for letter in line.chars() {
            matrix.last_mut().unwrap().push(letter);
        }
    }
    dbg!(&matrix);

    let mut id_matrix = vec![vec![0usize; m]; n];
    let mut id = 1usize;
    for r in 0..n {
        for c in 0..m {
            if id_matrix[r][c] == 0 {
                set_id(&matrix, &mut id_matrix, r, c, id);
                id += 1;
            }
        }
    }
    println!("{:?}", &id_matrix);

    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); id];
    for r in 0..(n - 1) {
        for c in 0..m {
            if id_matrix[r][c] != id_matrix[r + 1][c] {
                graph[id_matrix[r][c]].insert(id_matrix[r + 1][c]);
                graph[id_matrix[r + 1][c]].insert(id_matrix[r][c]);
            }
        }
    }
    for r in 0..n {
        for c in 0..(m - 1) {
            if id_matrix[r][c] != id_matrix[r][c + 1] {
                graph[id_matrix[r][c]].insert(id_matrix[r][c + 1]);
                graph[id_matrix[r][c + 1]].insert(id_matrix[r][c]);
            }
        }
    }
    let graph: Vec<Vec<_>> = graph.iter().map(|x| x.iter().collect()).collect();
    println!("{:?}", &graph);
}

fn set_id(matrix: &[Vec<char>], id_matrix: &mut [Vec<usize>], r: usize, c: usize, id: usize) {
    let mut queue = VecDeque::from([(r, c)]);
    id_matrix[r][c] = id;
    while let Some((r, c)) = queue.pop_front() {
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for &(dr, dc) in DIRECTIONS.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if 0 <= nr && 0 <= nc {
                let nr = nr as usize;
                let nc = nc as usize;
                if nr < matrix.len()
                    && nc < matrix[nr].len()
                    && id_matrix[nr][nc] == 0
                    && matrix[r][c] == matrix[nr][nc]
                {
                    id_matrix[nr][nc] = id;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
}
