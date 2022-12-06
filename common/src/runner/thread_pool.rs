use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

/**
 * 自定义一个线程池
 * 简单一点，只有线程池大小合具体的工作线程
 */
struct ThreadPool {
    size: i32,
    name: String,
    pool: Vec<Worker>,
    sender: Option<mpsc::Sender<dyn FnOnce()>>,
}

impl ThreadPool {
    pub fn new(name: String, size: i32) -> Self {
        ThreadPool {
            size: size,
            name: name,
            pool: vec![],
            sender: None,
        }
    }
    /*
    * 启动线程池
     */
    pub fn star(&mut self, runner: Box<dyn FnOnce()>) {
        let (sender, receiver) = channel::<dyn FnOnce()+Send>();
        self.sender = Option::Some(sender);

        let arc = Arc::new(Mutex::new(receiver));
        for i in 0..self.size {
            let clone_arc = Arc::clone(&arc);
            //给每一个 工作线程娶一个名字
            let work_name = &self.name + "-" + "i";
            self.pool.push(Worker::new(work_name, clone_arc));
        }
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
    receiver: ArcReceiver,
}

type ArcReceiver = Arc<Mutex<Receiver<dyn FnOnce()>>>;

impl Worker {
    pub fn new(work_id: String, receiver: ArcReceiver) -> Self {
        //定义一个task任务,用于实时从receiver获取任务
        let a = ||{
            loop {
                  receiver.lock().expect("worker get lock error")
                      .re
            }
        };


        Worker{
            work_id,
            receiver,
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

