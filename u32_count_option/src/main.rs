/**
 * 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
 */
pub fn parse_u32_array(args: &[u32]) -> Option<u32> {
    if args.is_empty() {
        return None;
    }
    let result = args.iter().fold(0, |sum, acm| sum + acm);
    Some(result)
}

fn main() {
    let list: &[u32] = &[2000000, 1000000, 3000000];

    let sum = parse_u32_array(list);
    //方法 match
    match sum {
        Some(num) => println!("sum is: {}", num),
        None => println!("sum is None"),
    }
}
