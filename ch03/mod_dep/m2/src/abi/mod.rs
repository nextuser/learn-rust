pub mod publish;

pub fn abi_message() {
    println!("This is an ABI message");
    println!("publish info is {}", publish::pubinfo())
}
