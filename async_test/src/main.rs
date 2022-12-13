use std::thread;
use std::thread::sleep;
use std::time::Duration;


fn main() {
    let v = 4;
    
    println!("Before reading file 1");

    //"move" can copy other function params into your function to use 
    let handle1 = thread::spawn(move || {
        let file1_content = read_file1();
        println!("{:?}", file1_content);
        println!("{:?}", v);
    });
    
    let handle2 = thread::spawn(|| {
        let file2_content = read_file2();
        println!("{:?}", file2_content);
    });
    
    //if not use spawn , the function will excute one by one
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn read_file1() -> String {
    sleep(Duration::new(4,0));
    String::from("Hello , I'm come from file 1")
}

fn read_file2() -> String {
    sleep(Duration::new(2,0));
    String::from("Hello , I'm come from file 2")
}