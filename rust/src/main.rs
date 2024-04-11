fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

#[derive(Debug)]
struct Point{
    x: i32, 
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let (dir, amount) = line.split_once(" ").expect("must contain a whitespace");
    let amount = str::parse::<i32>(amount).expect("Second arg must be an integer"); 

    match dir {
        "forward" => Point {x: amount, y: 0},
        "up" => Point {x: 0, y: -amount},
        "down" => Point {x: 0, y: amount},
        _ => panic!("Invalid direction")
    }
}

fn main() {
    let result = get_input()
        .lines()
        .map(parse_line)
        .fold(Point {x:0,y:0}, |mut acc, point|{
            acc.x += point.x;
            acc.y += point.y;
            return acc;
        });
    
    println!("{:?}", result);
}
