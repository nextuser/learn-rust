# 演示workspace和模块之间的依赖关系
- Cargo.toml  workspace 配置和cargo的关系
```toml
[workspace]
members = ["m1", "m2"]

```
- 模块内，文件名做模块名
  - m2/abi/mod.rs 里面的模块是abi模块
- 里面的函数对应的模块名时目录名
  - /m2/abi/publish.rs  模块名是publish
# crate m1 call m2 function
- m2/lib.rs 中使用pub mod abi 定义模块可以公开访问
 
```rust
//需要公开的模块，需要再lib.rs 中声明
pub mod abi;
  ```
- m1 Cargo.toml 定义依赖关系
  ```toml
  [dependencies]
  m2 = { path = "../m2" }
  ```
- m1/src/main.rs 中调用m2的publish函数
```rust
use m2::abi::publish;
fn main() {
    
    publish::pubinfo();
}
  ```