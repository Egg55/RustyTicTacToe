fn main() {
    let mut vec = vec![vec![0; 3]; 3];
    let mut gmo = false;
    let mut x = 0;
    let mut p; 
    display(&mut vec);
    while gmo == false {
        x+=1;
        if x % 2 == 0 {
            p = 2;
        }else {
            p = 1;}
        iput(&mut vec, p);
        display(&mut vec);
        gmo = checkb(&mut vec, p);
    }
}

fn ps(p: i32) -> &'static str {
    if p == 1 {
        return "X";
    }else if p == 2 {
        return "O"
    }else {
        return " ";
    }
}

fn checkb(vec: &mut Vec<Vec<i32>>, p: i32) -> bool {
    let mut bf  = true;
    for i in 0..3 {
        if vec[i].contains(&0) {
            bf = false;
        }
    }

    if (vec[0][0] == p && vec[0][1] == p && vec[0][2] == p) || (vec[1][0] == p && vec[1][1] == p && vec[1][2] == p) || (vec[2][0] == p && vec[2][1] == p && vec[2][2] == p) || (vec[0][0] == p && vec[1][0] == p && vec[2][0] == p) || (vec[0][1] == p && vec[1][1] == p && vec[2][1] == p) || (vec[0][2] == p && vec[1][2] == p && vec[2][2] == p) || (vec[0][0] == p && vec[1][1] == p && vec[2][2] == p) || (vec[0][2] == p && vec[1][1] == p && vec[2][0] == p) {
        println!("{} Wins!",ps(p));
        true
    }else if bf {
        println!("Tie Game!");
        true
    } else {
        false
    }
}


fn iput(vec: &mut Vec<Vec<i32>>, p: i32) {
    let pp = p;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut arr: Vec<&str> = input.split_terminator("").skip(1).collect();
    arr.truncate(2);

    let mut cord = vec![];
    cord.push(usize::from(arr[0].parse::<u8>().unwrap()));

    if arr.contains(&"a") {cord.push(1)
    }else if arr.contains(&"b") {cord.push(2)
    }else if arr.contains(&"c") {cord.push(3)
    }else {
        println!("Invalid Position");
    }

    if vec[cord[1]-1][cord[0]-1] != 0 {
        println!("This Position Cannot Be Overwriten!");
        iput(vec,pp)
    }else {
        vec[cord[1]-1][cord[0]-1] = p;
    }
}

fn display(vec: &mut Vec<Vec<i32>>) {
    println!(
    "+---+---+---+---+\n| # | 1 | 2 | 3 |\n+---+---+---+---+\n| A | {} | {} | {} |\n+---+---+---+---+\n| B | {} | {} | {} |\n+---+---+---+---+\n| C | {} | {} | {} |\n+---+---+---+---+\n",
     ps(vec[0][0]), ps(vec[0][1]), ps(vec[0][2]),
     ps(vec[1][0]), ps(vec[1][1]), ps(vec[1][2]),
     ps(vec[2][0]), ps(vec[2][1]), ps(vec[2][2])
    );
}
