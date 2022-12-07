use std::borrow::Borrow;
use std::fmt::format;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::JoinHandle;

/**
 * 自定义一个线程池
 * 简单一点，只有线程池大小合具体的工作线程
 */
struct ThreadPool {
    size: i32,
    name: String,
    pool: Vec<Worker>,
    sender: Option<mpsc::Sender<TransformMsg<Msg>>>,
}

impl ThreadPool {
    pub fn new(name: &str, size: i32) -> Self {
        ThreadPool {
            size,
            name: String::from(name),
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
            self.pool.push(Worker::new(&work_name, clone_arc));
        }
    }
    /**
     * 提供一个api关闭池子
     * 简单一点，清空队列即可
     */
    pub fn shutdown(&mut self) {
        self.pool.clear()
    }
    /**
     * 提供一个api往线程池丢任务
     */
    pub fn execute(&self, task: Box<dyn FnOnce() + Send>) {
        //用sender去发送任务
        if let Some(sender) = &self.sender {
            sender.send(TransformMsg::MSG(task)).expect(&format!("send message error for thread pool {}", self.name));
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
    handler: JoinHandle<()>,
}

type ArcReceiver = Arc<Mutex<Receiver<TransformMsg<Msg>>>>;
type Msg = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    pub fn new(work_id: &str, receiver: ArcReceiver) -> Self {
        //定义一个task任务,用于实时从receiver获取任务,获取不到则阻塞
        let handle = thread::spawn(
            move || {
                loop {
                    let msg = receiver.lock().expect("worker get lock error")
                        .recv().expect("receive task message error");
                    //运行任务
                    match msg {
                        TransformMsg::MSG(task) => task(),
                        TransformMsg::TERMINAL => break
                    }
                }
                print!("work stop");
            }
        );

        Worker {
            work_id: String::from(work_id),
            handler: handle,
        }
    }
}

pub enum TransformMsg<T> {
    MSG(T),
    TERMINAL,
}

