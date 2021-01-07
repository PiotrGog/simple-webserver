use crate::job::Job;

pub enum Message {
    Execute(Job),
    Terminate,
}
