pub struct EnumType {}

use crate::Runner;

impl Runner for EnumType {
    fn run(&self) {
        // enum作ってみる
        enum WebEvent {
            PageLoad,
            PageUnload,
            KeyPress(char),
            Paste(String),
            Click { x: i64, y: i64 },
        }

        // どの状態かわかるようにする
        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                WebEvent::KeyPress(c) => println!("pressed '{}.'", c),
                WebEvent::Paste(s) => println!("pasted \"{}\"", s),
                WebEvent::Click { x, y } => println!("clicked at ({}, {})", x, y),
            }
        }

        // 使ってみる
        inspect(WebEvent::KeyPress('x'));
        inspect(WebEvent::Paste("my text".to_owned()));
        inspect(WebEvent::Click { x: 20, y: 80 });
        inspect(WebEvent::PageLoad);
        inspect(WebEvent::PageUnload);

        // type aliasも使える
        type Event = WebEvent;
        inspect(Event::PageLoad);

        // 以下みたいなSelfが一番あるある
        impl WebEvent {
            fn inspect(&self) {
                match self {
                    Self::PageLoad => println!("page loaded"),
                    Self::PageUnload => println!("page unloaded"),
                    Self::KeyPress(c) => println!("pressed '{}.'", c),
                    Self::Paste(s) => println!("pasted \"{}\"", s),
                    Self::Click { x, y } => println!("clicked at ({}, {})", x, y),
                }
            }
        }
        WebEvent::PageLoad.inspect();

        // use keywordを使えば以下の感じで使える
        // use crate::WebEvent::*;
        // let event = PageLoad;

        // C言語っぽいenumも可能
        // 暗黙的に各要素に値がつく(0 origin)
        enum Number {
            Zero,
            One,
            Two,
        }
        // 明示的につける場合は以下
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);
        println!("red is #{:06x}", Color::Red as i32);
        println!("red is #{:06x}", Color::Blue as i32);

        // LinkedListを実装してみる
        enum List {
            Cons(u32, Box<List>),
            Nil,
        }

        impl List {
            fn new() -> List {
                List::Nil
            }

            fn prepend(self, elem: u32) -> List {
                List::Cons(elem, Box::new(self))
            }

            fn len(&self) -> u32 {
                match self {
                    List::Cons(_, tail) => 1 + tail.len(),
                    List::Nil => 0,
                }
            }

            fn stringify(&self) -> String {
                match self {
                    List::Cons(head, tail) => {
                        format!("{}, {}", head, tail.stringify())
                    }
                    List::Nil => "Nil".to_string(),
                }
            }
        }

        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        println!("item.len => {}", list.len());
        println!("{}", list.stringify());
    }
}
