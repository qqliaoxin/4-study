/**
 * 为枚举交通信号灯实现一个 trait,trait里包含一个返回时间的方法,不同的灯持续的时间不同
 */

// 信号灯
enum Linght {
    Red,
    Yellow,
    Green,
    Blue,
}

//  返回时间
trait Move {
    fn times(&mut self) -> u8;
}

impl Move for Linght {
    fn times(&mut self) -> u8 {
        match self {
            Linght::Red => 1,
            Linght::Yellow => 2,
            Linght::Green => 3,
            Linght::Blue => 4,
        }
    }
}

fn main() {
    let mut red = Linght::Red;
    println!("red：{}s", red.times());
    let mut yellow = Linght::Yellow;
    println!("yellow：{}s", yellow.times());
    let mut green = Linght::Green;
    println!("green：{}s", green.times());
    let mut blue = Linght::Blue;
    println!("blue：{}s", blue.times());
}
