fn get_numbers() -> String{
        use std::io::{stdin};
        let mut input = String::new();
        stdin().read_line(&mut input).expect("this is not a string");
        input
}

fn make_vector() -> Vec<i32>{
    let f: &str = &get_numbers();
    let v: Vec<&str> = f.split(' ').collect();
    let mut m: Vec<i32> = Vec::new();
    for element in v{
        m.push(element.trim().parse::<i32>().unwrap())
    }
    m
}

fn diagonal_diference(vectors: Vec<Vec<i32>>, n_element: i32) -> i32 {
    let mut diag1: i32 = 0;
    let mut diag2: i32 = diag1;
    let mut aux = diag1;
    let mut x: i32 = n_element - 1;
    for vector in vectors{
        diag1 += vector[x as usize];
        diag2 += vector[aux as usize];
        aux += 1;
        x -= 1;
    }
    (diag1 - diag2).abs()
}


fn get_result(x: i32) -> i32{
    let mut m_m: Vec<Vec<i32>> = Vec::new();
    for _ in 0..x{
        let v_m: fn() -> Vec<i32> = make_vector;
        m_m.push(v_m())
    }
    diagonal_diference(m_m, x)
}

fn main() {
    let x: i32 = get_numbers().trim().parse::<i32>().unwrap();
    println!("{}", get_result(x));
}
