/**
 * // 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
 */ //定义一个特征 有一个打印面积函数
trait Shape {
    fn area(&self);
}
// 定义圆型结构体
struct Circle {
    radius: f64, // 半径
}
//定义长方形结构体
struct Rectangle {
    width: u64, // 长度
    long: u64,  // 宽度
}

//定义正方形结构体
struct Square {
    width: u64, // 长度
    long: u64,  // 宽度
}
//能打印面积的都是写Shape的特征
impl Shape for Circle {
    fn area(&self) {
        println!(
            "area is（圆形的面积是）: {}",
            3.14 * self.radius * self.radius
        ); // πR²的浮点值
    }
}
impl Shape for Rectangle {
    fn area(&self) {
        println!("area is（长方形的面积是）: {}", self.long * self.width);
    }
}

impl Shape for Square {
    fn area(&self) {
        println!("area is（正方形的面积是）: {}", self.long * self.width);
    }
}
//打印函数  T限定为 实现了可打印面积特征Shape
fn pringt_shape_area<T: Shape>(shape: T) {
    shape.area();
}
fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 600,
        long: 200,
    };
    let square = Square {
        width: 200,
        long: 200,
    };
    pringt_shape_area(circle);
    pringt_shape_area(rectangle);
    pringt_shape_area(square);
}
