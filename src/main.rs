use std::time::Duration;

#[tokio::main(flavor="current_thread")]
async fn main() {
    test_something().await;
    let num = test_back().await;
    println!("{}",num );
}

async fn test_something(){
    std::thread::sleep(Duration::from_millis(5000));
    println!("print a log");
}
async fn test_back() -> i8{
    5
}