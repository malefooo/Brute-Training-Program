use std::borrow::Borrow;
use std::fmt::format;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use crate::runner::thread_pool::TransformMsg::TERMINAL;

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
        for _ in &self.pool {
            match &self.sender {
                None => break,
                Some(s) => {
                    s.send(TERMINAL).expect(&format!("send terminal message error for thread pool {}", self.name));
                }
            }
        }
        self.pool.clear();
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
    pub fn new(work_id: &str, locked_receiver: ArcReceiver) -> Self {
        //定义一个task任务,用于实时从receiver获取任务,获取不到则阻塞
        let handle = thread::spawn(
            move || {
                loop {
                    let hold_msg: TransformMsg<Msg>;
                    {
                        //持有了receiver要及时释放，否则会锁住其它线程拿不到这个receiver
                        hold_msg = locked_receiver
                            .lock().expect("worker get lock error")
                            .recv().expect("receive task message error");
                    }
                    //运行任务
                    match hold_msg {
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


pub mod util {
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{channel, Receiver, Sender};

    use crate::runner::thread_pool::{Msg, TransformMsg};

    pub fn generate_channel<T>() -> (Sender<T>, Arc<Mutex<Receiver<T>>>) {
        let (sen, rec) = channel::<T>();
        let arc = Arc::new(Mutex::new(rec));
        (sen, arc)
    }
}

#[cfg(test)]
mod test_thread_pool {
    use std::borrow::Borrow;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::{channel, Sender};
    use std::thread;
    use std::time::Duration;

    use util::generate_channel;

    use crate::runner::thread_pool::{Msg, ThreadPool, TransformMsg, util, Worker};
    use crate::runner::thread_pool::TransformMsg::MSG;

    /**
     * 初始化并启动单个工作线程
     */
    #[test]
    fn test_init_a_worker_thread() {
        let (sender, receiver) = generate_channel::<TransformMsg<Msg>>();
        let worker = Worker::new("test-worker", receiver);
        //发送任务
        let (check_sender, check_receiver) = generate_channel::<TransformMsg<String>>();
        sender.send(MSG(Box::new(move || {
            //回发消息，证明执行了
            check_sender.send(MSG(String::from("hello"))).unwrap();
        }))).unwrap();

        thread::sleep(Duration::from_secs(2));
        if let MSG(msg) =
            check_receiver.lock().unwrap()
                .recv().unwrap() {
            //证明收到了消息，说明工作线程执行了
            assert_eq!("hello", msg);
        };
    }

    /**
     *简单测试下是否能启动线程池
     */
    #[test]
    fn test_star_thread_pool() {
        let mut test_pool = ThreadPool::new("test", 1);
        assert_eq!("test", test_pool.name);
        assert_eq!(1, test_pool.size);
        test_pool.star();
        assert_eq!(1, test_pool.pool.len());
        test_pool.shutdown();
        assert_eq!(0, test_pool.pool.len());
    }

    /**
     * 测试多任务是否能异步并行执
     * 创建100个子线程
     * 给每个子线程一个sender，让他们给main线程发送一个消息,并sleep2秒，
     * main线程会收到100次消息，并执行不超过4秒（考虑误差）
     */
    #[test]
    fn test_multi_thread_run_in_same_time() {
        let thread_count = 100;
        let mut ten_pool = ThreadPool::new("test", thread_count);
        ten_pool.star();
        let (sender, receiver) = channel();
        let arc = Arc::new(Mutex::new(sender));
        for _ in 0..thread_count {
            let arc = Arc::clone(&arc);
            ten_pool.execute(Box::new(move || {
                let sender;
                {
                    sender = arc.lock().unwrap()
                        .send(String::from("cool")).unwrap();
                }
                thread::sleep(Duration::from_secs(2));
            }));
        }
        //当前线程等4秒
        thread::sleep(Duration::from_secs(4));
        let mut count = 0;

        for _ in 0..thread_count {
            let msg = receiver.recv().unwrap();
            count += 1;
            assert_eq!("cool", msg);
        }
        ten_pool.shutdown();
        assert_eq!(thread_count, count);
    }
}


