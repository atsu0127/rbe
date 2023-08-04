use crate::Runner;

pub struct Method {}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // これは関連型(型に紐づいたものなので、インスタンス化しないで呼び出せる)
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // これはメソッド、&selfはself: &Selfのこと
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // &mut selfはself: &mut Selfのこと
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;

        self.p2.x += x;
        self.p2.y += y;
    }
}

// ヒープ上のi32を持っている
#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // selfはself: Selfのこと
    // 借用ではなくオブジェクトを消費する
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({first}, {second})");
    }
}

impl Runner for Method {
    fn run(&self) {
        let rectangle = Rectangle {
            // 関連関数は以下で呼び出す
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };

        // メソッド呼び出し
        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area());

        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };
        println!("Before translate {square:?}");

        // rectangleはmutじゃないのでtranslateできない
        // cannot borrow `rectangle` as mutable, as it is not declared as mutable
        // rectangle.translate(1.0, 1.0);

        // squareはmutなのでtranslateできる
        square.translate(1.0, 1.0);
        println!("After translate {square:?}");

        let pair = Pair(Box::new(1), Box::new(2));
        println!("Before destroy {pair:?}");
        pair.destroy();
        // 以下はpairがmoveされているので実行できない
        // borrow of moved value: `pair`
        // println!("After destroy {pair:?}");
    }
}
