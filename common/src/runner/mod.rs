mod msg;

struct  Worker<'a> {
    work_id: &'a str,
    content: TransType
}

impl Worker {
    pub fn new(work_id: &str, content: TransType) -> Self {
        Self { work_id, content }
    }
}

type TransType = Box<dyn FnOnce() + Send>;
enum TransformMsg{
    SOME(TransType),
    NONE
}