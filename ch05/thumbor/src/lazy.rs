use lazy_static::lazy_static;
use std::collections::HashMap;

// 定义一个延迟初始化的 HashMap 静态变量
lazy_static! {

    static ref GLOBAL_MAP: HashMap<u32, &'static str> = {
        println!("lazy static block init GLOBAL_MAP");
        // 这里可以写复杂的初始化逻辑（运行时执行）
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map  // 返回初始化后的变量
    };

    // 也可以定义其他类型，如字符串、自定义结构体等
    static ref APP_NAME: String = {
        println!("lazy static block init APP_NAME");
        format!("MyApp-{}", 1.0)
    };
}

fn main() {
    println!("call main first");
    // 第一次访问时触发初始化（仅一次）
    println!("{:#?}", *GLOBAL_MAP);  // 输出: {1: "one", 2: "two"}
    println!("{}", *APP_NAME);      // 输出: MyApp-1.0

    // 后续访问直接使用已初始化的值
    let value = GLOBAL_MAP.get(&1).unwrap();
    println!("{}", value);  // 输出: one
}