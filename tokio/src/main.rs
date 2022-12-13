use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Before reading file 1");
    let h1 = tokio::spawn(async {
        let file1_content = read_file1().await;
        println!("{:?}", file1_content);
    });
    let h2 = tokio::spawn(async {
        let file2_content = read_file2().await;
        println!("{:?}", file2_content);
    });
    let _ = tokio::join!(h1, h2);
    
}

async fn read_file1() -> String {
    sleep(Duration::new(4,0));
    println!("{:?}", "Processing file 1");
    String::from("Hello , I'm come from file 1")
}

async fn read_file2() -> String {
    sleep(Duration::new(2,0));
    println!("{:?}", "Processing file 2");
    String::from("Hello , I'm come from file 2")
}
