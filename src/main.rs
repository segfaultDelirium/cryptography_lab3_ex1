use std::fmt::format;

fn parity(vec: Vec<i32>) -> i32{
    return vec.into_iter().reduce(|acc, x| acc + x).unwrap();
}

fn linear_a(z_vec: Vec<i32>) -> i32{
    return (z_vec[0] + z_vec[1] + z_vec[2] + z_vec[3]) % 2;
}

fn linear_b(z_vec: Vec<i32>) -> i32{
    return (z_vec[0] + z_vec[3]) % 2;
}



fn numeric_char_to_number(c: char) -> i32{
    let res = (c as u8 - '0' as u8) as i32;
    return res;
}

fn number_to_numeric_char(x: i32) -> char{
    return (x + '0' as u8 as i32) as u8 as char
}

// n == 4
fn generate_all_vectors(n: i32)-> Vec<String>{
    let top = (2 as i32).pow(n as u32);
    let res: Vec<String> = (0 .. top).map(|x| {
        return format!("{:0>4}", format!("{x:b}"));
    }).collect();
    // println!("{:?}", res);
    return res;
}

fn generate_stream_a(initial: &String, how_long: i32) -> String {
    let m = initial.len() as usize;
    let mut res = initial.clone();
    // println!("res = {res}");
    (0 .. how_long - 4).for_each(|i| {
        // println!("i = {i}");
        let from: Vec<i32> = res.chars().into_iter().skip(i as usize).take(m)
            .map(|c| {
                numeric_char_to_number(c)
            }).collect();
        let new_char = number_to_numeric_char(linear_a(from));
        // println!("new_char: {new_char}");
        res.push(new_char);
    });
    // println!("res = {res}");
    return res;
}

fn generate_stream_b(initial: &String, how_long: i32) -> String {
    let m = initial.len() as usize;
    let mut res = initial.clone();
    // println!("res = {res}");
    (0 .. how_long - 4).for_each(|i| {
        // println!("i = {i}");
        let from: Vec<i32> = res.chars().into_iter().skip(i as usize).take(m)
            .map(|c| {
                numeric_char_to_number(c)
            }).collect();
        let new_char = number_to_numeric_char(linear_b(from));
        // println!("new_char: {new_char}");
        res.push(new_char);
    });
    // println!("res = {res}");
    return res;
}

fn find_longest_substring(long_text: String, m: i32) -> i32{

    let substring = long_text.chars().take(m as usize)
        .map(|c| c.to_string())
        .reduce(|acc, c| [acc, c].join("")).expect("this should never happen");

    let mut substring_counter = 0;
    let mut is_ok = true;
    for i in m .. (long_text.len() as i32){
        let c = long_text.chars().nth(i as usize).unwrap();
        if c != substring.chars().nth(substring_counter).unwrap() {
            is_ok = false;
            break;
        }
        substring_counter += 1;
        if(substring_counter == substring.len()) {substring_counter = 0}
    }
    if is_ok {return m};
    return find_longest_substring(long_text, m+1);
}

fn z1_a(){
    let how_long = 50;
    let vectors = generate_all_vectors(4);
    println!("vectors: {:?}", vectors);
    let streams: Vec<String> = vectors.iter().map(|initial| {
        let stream = generate_stream_a(initial, how_long);
        // println!("stream = {}", stream);
        return stream;
    }).collect();
    println!("streams: {:?}", streams);

    let periods: Vec<i32> = streams.into_iter().map(|stream| {
        return find_longest_substring(stream, 1);
    }).collect();


    vectors.iter().zip(periods).for_each(|(input, output)| {
        println!("P({input}) = {output}")
    });

    return;
}

fn z1_b(){
    let how_long = 50;
    let vectors = generate_all_vectors(4);
    println!("vectors: {:?}", vectors);
    let streams: Vec<String> = vectors.iter().map(|initial| {
        let stream = generate_stream_b(initial, how_long);
        // println!("stream = {}", stream);
        return stream;
    }).collect();
    println!("streams: {:?}", streams);

    let periods: Vec<i32> = streams.into_iter().map(|stream| {
        return find_longest_substring(stream, 1);
    }).collect();

    vectors.iter().zip(periods).for_each(|(input, output)| {
        println!("P({input}) = {output}")
    });
    return;
}

fn main() {
    // println!("Hello, world!");
    println!("a):");
    z1_a();
    println!("b):");
    z1_b();
}




