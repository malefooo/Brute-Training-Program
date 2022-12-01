mod msg;

struct  Worker<'a> {
    work_id: &'a str,
    content: trans_type
}

impl Worker {
    pub fn new(work_id: &str, content: trans_type) -> Self {
        Self { work_id, content }
    }
}

type trans_type = Box<dyn FnOnce() + Send>;
enum TransformMsg{
    SOME(trans_type),
    NONE
}