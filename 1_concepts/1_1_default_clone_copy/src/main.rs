mod dots {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    
    #[derive(Debug, Clone)]
    pub struct Polyline {
        pub points: Vec<Point>
    }
    
    impl Polyline {
        pub fn new(points: Vec<Point>) -> Self {
            if points.len() > 0 {
                Polyline {points} 
            } else {
                panic!("Add at least one Point")
            }
        }
    }
}

fn main() {
    let a = dots::Point {x: 3, y: 2};
    let b = dots::Polyline::new(vec![a]);

    println!("{} {} {:?}", a.x, a.y, b.points);
}
