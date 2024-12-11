use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

fn fill_matrix(x: Vec<String>, y: Vec<String>) -> Vec<Vec<i32>>{
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    let n = x.len()+1;
    let m = y.len()+1;

    for _i in 0..n{
        matrix.push(vec![0; m]);
    }

    for i in 1..n {
        for j in 1..m{
            if x[i-1] == y[j-1]{
                matrix[i][j] = matrix[i-1][j-1] + 1;
            }else {
                matrix[i][j] = max(matrix[i-1][j], matrix[i][j-1]);
            }
        }
    }

    matrix
}


fn get_lcs(x: Vec<String>, y: Vec<String>)-> Vec<String>{
    let matrix: Vec<Vec<i32>> = fill_matrix(x.clone(), y.clone());
    let mut res: Vec<String> = Vec::new();

    let mut x_i = x.len();
    let mut y_i = y.len();

    while x_i > 0 && y_i > 0 {
        if x[x_i-1] == y[y_i-1]{
            res.push(x[x_i-1].clone());
            x_i -= 1;
            y_i -= 1;
        }else if matrix[x_i-1][y_i] > matrix[x_i][y_i-1] {
            x_i -= 1;
        }else {
            y_i -= 1;
        }
    }

    res = res.iter().rev().map(|s| s.clone()).collect::<Vec<String>>();
    res
}


fn get_vectors(x_file: File, y_file: File) -> (Vec<String>, Vec<String>) {
    let mut x: Vec<String> = Vec::new();
    let mut y: Vec<String> = Vec::new();

    let x_reader = BufReader::new(x_file);
    let lines = x_reader.lines();
    for line in lines{
        x.push(line.unwrap().trim().to_string());
    }

    let y_reader = BufReader::new(y_file);
    let lines = y_reader.lines();
    for line in lines{
        y.push(line.unwrap().trim().to_string());
    }

    (x, y)
}


fn print_diff(deleted: Vec<(i32, String)>, added: Vec<(i32, String)>){
    if deleted.is_empty() && added.is_empty(){
        return;
    } else if deleted.is_empty() {
        if added.len() == 1{
            println!("{}a{}", added[0].0-1, added[0].0);
        } else {
            println!("{}a{},{}", added[0].0-1, added[0].0, added[added.len() - 1].0);
        }

        for i in 0..added.len() {
            println!("> {}", added.get(i).unwrap().1);
        }
    } else if added.is_empty() {
        if deleted.len() == 1{
            println!("d{}", deleted[0].0);
        } else {
            println!("d{},{}", deleted[0].0, deleted[deleted.len() - 1].0);
        }

        for i in 0..deleted.len() {
            println!("< {}", deleted.get(i).unwrap().1);
        }
    } else {
        if added.len() == 1 && deleted.len() == 1 {
            println!("{}c{}", deleted[0].0, added[0].0);
        } else if added.len() == 1 {
            println!("{},{}c{}", deleted[0].0, deleted[deleted.len()-1].0, added[0].0);
        } else if deleted.len() == 1 {
            println!("{}c{},{}", deleted[0].0, added[0].0, added[added.len()-1].0);
        } else {
            println!("{},{}c{},{}", deleted[0].0, deleted[deleted.len()-1].0,
                     added[0].0, added[added.len()-1].0);
        }

        for i in 0..deleted.len() {
            println!("< {}", deleted.get(i).unwrap().1);
        }
        println!("---");
        for i in 0..added.len() {
            println!("> {}", added.get(i).unwrap().1);
        }
    }
}


fn compare_files(x_file: File, y_file: File){
    let vectors = get_vectors(x_file, y_file);
    let x = vectors.0.clone();
    let y = vectors.1.clone();

    let lcs = get_lcs(vectors.0, vectors.1);
    let mut deleted: Vec<(i32, String)> = Vec::new();
    let mut added = Vec::new();

    let mut lcs_i = 0;
    let mut x_i = 0;
    let mut y_i = 0;

    while x_i < x.len() && y_i < y.len() {
        if  lcs_i < lcs.len() && x[x_i] == lcs[lcs_i] && y[y_i] == lcs[lcs_i]{
            print_diff(deleted.clone(), added.clone());
            deleted.clear();
            added.clear();

            x_i += 1;
            y_i += 1;
            lcs_i += 1;
        } else if lcs_i >= lcs.len() || x[x_i] == lcs[lcs_i] {
            added.push(((y_i + 1) as i32, y[y_i].clone()));
            y_i += 1;
        } else if lcs_i >= lcs.len() || y[y_i] == lcs[lcs_i] {
            deleted.push(((x_i + 1) as i32, x[x_i].clone()));
            x_i += 1;
        } else {
            added.push(((y_i + 1) as i32, y[y_i].clone()));
            deleted.push(((x_i + 1) as i32, x[x_i].clone()));
            y_i += 1;
            x_i += 1;
        }
    }

    while x_i < x.len(){
        deleted.push(((x_i + 1) as i32, x[x_i].clone()));
        x_i += 1;
    }

    while y_i < y.len(){
        added.push(((y_i + 1) as i32, y[y_i].clone()));
        y_i += 1;
    }

    print_diff(deleted.clone(), added.clone());
    deleted.clear();
    added.clear();
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let mut command = Vec::new();
    for x in input.split(' ') {
        command.push(x.to_string());
    }

    if command[0] != "diff"{
        println!("Команда не распознана, введите diff file_1 file_2");
    }else {
        compare_files(File::open(command[1].clone()).unwrap(), File::open(command[2].clone()).unwrap());
    }
}
