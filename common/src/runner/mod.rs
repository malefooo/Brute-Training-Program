use std::ops::Receiver;
use std::thread::Thread;
use std::sync::{Arc, mpsc};
mod msg;

struct Service<'a> {
    pool: Vec<Worker<'a>>,
    name: &'a str,
}


/**
* 提供一个异步运行时
*/
impl<'a> Service<'a> {
    pub fn new(&self,size: u32, name: &'a str) -> Self {
        let mut vec = Vec::new();
        let (sender,receiver) = mpsc::channel();
        for i in 0..size {
            vec.push(Worker::new(name+"_"+i,Arc::new(Receiver) ))
        }
        self{
            vec,
            name
        }

    }
}


type TransType = Box<dyn FnOnce() + Send>;
/**
* 工作线程
*/
struct Worker<'a> {
    work_id: &'a str,
    content: Arc<dyn Receiver>,
}

impl Worker {
    pub fn new(work_id: &str, content: Arc<dyn Receiver>) -> Self {
        Self { work_id, content }
    }

    /**
    *传递一个任务来让工作线程执行
    */
    pub fn execute(task:fn()){

    }
}


enum TransformMsg {
    SOME(TransType),
    NONE,
}