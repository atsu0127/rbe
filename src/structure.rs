use crate::Runner;

pub struct Structure {}

impl Runner for Structure {
    fn run(&self) {
        // 色々な構造体の宣言がある
        // unit struct
        struct Unit;

        // Tuple Struct(名称付きTuple)
        struct Pair(i32, i32);

        // 2つのフィールドを持つStruct
        struct Point {
            x: f32,
            y: f32,
        }

        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        // 構造体をメンバーに持つ構造体
        struct Rectangle {
            top_left: Point,
            bottom_right: Point,
        }

        // Person使ってみる
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        println!("{:?}", peter);

        // Point使ってみる
        let point: Point = Point { x: 10.3, y: 0.4 };
        println!("point coordinates: ({}, {})", point.x, point.y);
        // 既存のpoint変数を埋め込める(yがpointと同じになる)
        let bottom_right = Point { x: 5.2, ..point };
        println!(
            "bottom_right coordinates: ({}, {})",
            bottom_right.x, bottom_right.y
        );
        // デストラクチャも可能
        let Point {
            x: left_edge,
            y: top_edge,
        } = point;
        // それを使ってRectangle作ってみる
        let _rectangle = Rectangle {
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right: bottom_right,
        };
        println!(
            "Rectangle coordinates: ({}, {}), ({}, {})",
            _rectangle.top_left.x,
            _rectangle.top_left.y,
            _rectangle.bottom_right.x,
            _rectangle.bottom_right.y
        );

        // Unit使ってみる
        let _unit = Unit;

        // Pair使ってみる
        let pair = Pair(1, 2);
        println!("pair contains {:?} and {:?}", pair.0, pair.1);
        // デストラクチャも可能
        let Pair(first, second) = pair;
        println!("destructed pair first: {:?}, second: {:?}", first, second);
    }

    fn train(&self) {
        // Rectangleの面積を計算するrect_areaの実装
        #[derive(PartialEq, Debug)]
        struct Point {
            x: f32,
            y: f32,
        }
        #[derive(PartialEq, Debug)]
        struct Rectangle {
            top_left: Point,
            bottom_right: Point,
        }

        impl Rectangle {
            fn rect_area(&self) -> f32 {
                (self.top_left.y - self.bottom_right.y) * (self.bottom_right.x - self.top_left.x)
            }
        }

        let rect = Rectangle {
            top_left: Point { x: 3.3, y: 12.5 },
            bottom_right: Point { x: 10.3, y: 0.4 },
        };
        let expected: f32 = (10.3 - 3.3) * (12.5 - 0.4);
        println!("rect_area is {:?}", rect.rect_area());
        assert_eq!(expected, rect.rect_area());

        // Pointとf32を受け取ってRectangleを返すsquareを実装
        // Pointが左上になり、f32は縦横の長さ
        impl Rectangle {
            fn new(p: Point, length: f32) -> Self {
                let Point { x, y } = p;
                Rectangle {
                    top_left: Point { x, y },
                    bottom_right: Point {
                        x: x + length,
                        y: y - length,
                    },
                }
            }
        }
        let rect2 = Rectangle::new(Point { x: 5.1, y: 4.2 }, 1.1);
        let expected_top_left = Point { x: 5.1, y: 4.2 };
        let expected_bottom_right = Point { x: 6.2, y: 3.1 };
        println!(
            "Rectangle coordinates: ({}, {}), ({}, {})",
            rect2.top_left.x, rect2.top_left.y, rect2.bottom_right.x, rect2.bottom_right.y
        );
        assert_eq!(expected_top_left, rect2.top_left);
        assert_eq!(expected_bottom_right, rect2.bottom_right);
    }
}
