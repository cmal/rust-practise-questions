#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum Quadrant {
    Origin,
    XAxis,
    YAxis,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

fn quadrant(p: Point) -> Quadrant {
    match p {
        Point { x: 0, y: 0 } => Quadrant::Origin,
        Point { y: 0, .. } => Quadrant::XAxis,
        Point { x: 0, .. } => Quadrant::YAxis,
        Point { x, y } if x > 0 && y > 0 => Quadrant::TopRight,
        Point { x, y } if x > 0 && y < 0 => Quadrant::BottomRight,
        Point { x, y } if x < 0 && y < 0 => Quadrant::BottomLeft,
        _ => Quadrant::TopLeft
    }
}

fn main() {
    let p1 = Point{ x: 0, y: 0};
    let p2 = Point{ x: 0, y: 1};
    let p3 = Point{ x: 1, y: 0};
    let p4 = Point{ x: 1, y: -1};
    let p5 = Point{ x: -1, y: 1};
    let p6 = Point{ x: -1, y: -1};
    let p7 = Point{ x: 1, y: 1};

    let v = vec!(p1, p2, p3, p4, p5, p6, p7);
    for p in v.into_iter() {
        println!("{:?} site in {:?}", &p.clone(), quadrant(p));
    }
}
