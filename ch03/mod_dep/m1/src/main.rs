mod abc;
use m2::abi::publish;
fn main() {
    println!("Hello, world!");
    abc::abc_message();

    publish::pubinfo();
}
