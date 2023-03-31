//! Types related to task management

use crate::{config::MAX_SYSCALL_NUM, syscall::process::TaskInfo, timer::get_time_us};

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,

    // LAB1: Add whatever you need about the Task.
    pub syscall_times: [u32; MAX_SYSCALL_NUM], // 任务使用的系统调用及调用次数
    pub start_timestamp_us: usize,             // 任务开始时间戳
}

impl TaskControlBlock {
    pub fn task_info(&self) -> TaskInfo {
        TaskInfo {
            status: self.task_status,
            syscall_times: self.syscall_times,
            time: (get_time_us() - self.start_timestamp_us) / 1000,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
