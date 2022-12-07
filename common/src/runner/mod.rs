

mod msg;
mod thread_pool;
mod thread_pool_test;

// struct Service<'a> {
//     pool: Vec<Worker<'a>>,
//     name: &'a str,
// }


// impl<'a> Service<'a> {
//     pub fn new(&self, size: u32, name: &'a str) -> Self {
//         let mut vec = Vec::new();
//         let (sender, receiver) = mpsc::channel::<TransformMsg>();
//         let mutex_receiver = Mutex::new(receiver);
//         let arc = Arc::new(mutex_receiver);
//         for i in 0..size {
//             let arc_receiver = Arc::clone(&arc);
//             vec.push(Worker::new(name + "_" + i, arc_receiver))
//         }
//         self {
//             vec,
//             name,
//         }
//     }
// }
//
// enum TransformMsg {
//     SOME(TransType),
//     NONE,
// }
