use std::io;

fn input() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().split_whitespace().map(|x| x.parse().ok().unwrap()).collect();
}

fn main(){
    let mut lines = Vec::new();
    loop {
        let line = input();
        if line.len() == 0 {
            break;
        }
        lines.push(line);
    }
    let parts = lines[0][0].parse::<i32>().unwrap();
    let mut left = 0;
    let mut right = 1;
    let mut cost = 0;
    for i in 1..lines.len() {
        let side = &lines[i][0].clone();
        let part_index = lines[i][1].parse::<i32>().unwrap();
        match side.as_str() {
                "R" => {
                if right < part_index {
                    if left < right {
                        cost += (part_index - right).abs();
                        println!("{}",1);
                        right = part_index;
                    } else {
                        cost +=  (right + parts - part_index).abs();
                        println!("{}",2);
                        right = part_index;
                    }
                }else if right > part_index {
                    if left < right {
                        cost +=  (parts - right + part_index).abs();
                        println!("{}",3);
                        right = part_index;
                    }else {
                        cost += (part_index - right).abs();
                        println!("{}",4);
                        left = part_index;
                    }
                }else if right == part_index {
                    cost += 0;
                }
                
            },
            "L" => {
                if left < part_index {
                    if left < right {
                        cost +=  (part_index - left).abs();
                        println!("{}",5);
                        left = part_index;
                    } else {
                        cost += (parts - left + part_index).abs();
                        println!("{}",6);
                        left = part_index;
                    }
                }else if left > part_index {
                    if left < right {
                        cost +=  (part_index - left).abs();
                        println!("{}",7);
                        right = part_index;
                    }else {
                        cost += (parts - left + part_index).abs();
                        println!("{}",8);
                        left = part_index;
                    }
                }else if left == part_index {
                    cost += 0;
                }
            },
            _ => {
                cost += 0;
            }
        }
        println!("left: {}, right: {}, cost: {}", left, right, cost);
    }
    println!("{}", cost);
}