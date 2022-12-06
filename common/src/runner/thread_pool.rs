use std::borrow::{Borrow, BorrowMut};
use std::fmt::format;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::Thread;
use tokio::task::JoinHandle;

/**
 * 自定义一个线程池
 * 简单一点，只有线程池大小合具体的工作线程
 */
struct ThreadPool {
    size: i32,
    name: String,
    pool: Vec<Worker>,
    sender: Option<mpsc::Sender<Box<dyn FnOnce()+Send+'static>>>,
}

impl ThreadPool {
    pub fn new(name: String, size: i32) -> Self {
        ThreadPool {
            size,
            name,
            pool: vec![],
            sender: None,
        }
    }
    /**
     * 启动线程池
     */
    pub fn star(&mut self) {
        let (sender, receiver) = channel();
        self.sender = Some(sender);

        let arc = Arc::new(Mutex::new(receiver));
        for i in 0..self.size {
            let clone_arc = Arc::clone(&arc);
            //给每一个 工作线程娶一个名字
            let work_name = format!("thead-{}-{}", &self.name, i);
            self.pool.push(Worker::new(work_name, clone_arc));
        }
    }
    /**
     * 提供一个api往线程池丢任务
     */
    pub fn excute(&self, task: Box<dyn FnOnce() + Send>) {
        //用sender去发送任务
        self.sender.expect(&format!("there is no sender for thread pool {}", self.name))
            .send(task).expect(&format!("send message error for thread pool {}", self.name));
    }
}


/**
 * 工作线程
 */
struct Worker {
    /**
     *线程名称
     */
    work_id: String,
    handler: JoinHandle<()>
}

type ArcReceiver = Arc<Mutex<Receiver<Box<dyn FnOnce()+Send+'static>>>>;

impl Worker {
    pub fn new(work_id: String, receiver: ArcReceiver) -> Self {
        //定义一个task任务,用于实时从receiver获取任务,获取不到则阻塞
        let handle = thread::spawn(
            move || {
                loop {
                    let task = receiver.lock().expect("worker get lock error")
                        .recv().expect("receive task message error");
                    //运行任务
                    task()
                }
            }
        );

        Worker {
            work_id,
            handler
        }
    }


    /**
     * 启动worK任务线程,等待任务到来并执行
     */
    pub fn star_worker(&self) {}

    /**
     *传递一个任务来让工作线程执行
     */
    pub fn execute(task: fn()) {}
    /**
     * 停止工作线程
     */
    pub fn stop_worker() {}
}


