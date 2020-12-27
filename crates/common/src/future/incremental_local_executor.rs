/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use futures::{
    future::{LocalBoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{cell::{RefCell, UnsafeCell}, future::Future, sync::atomic::{AtomicBool, Ordering}, sync::{Arc}, task::{Context, Poll}, thread::ThreadId};

use crate::container::DoubleBuffer;

/// Task executor that receives tasks off of a channel and runs them.
pub struct IncrementalLocalExecutor<T>  {
    running: AtomicBool,
    ready_queue: RefCell<DoubleBuffer<Arc<Task<T>>>>,
    thread_id: ThreadId,
    aggressiveness: u32,
}

impl<T> IncrementalLocalExecutor<T> {
    pub fn new(aggressiveness: u32) -> IncrementalLocalExecutor<T> {
        IncrementalLocalExecutor { 
            running: false.into(), 
            ready_queue: RefCell::new(DoubleBuffer::default()),
            thread_id: std::thread::current().id(),
            aggressiveness
        }
    }

    fn run_task<F: FnMut(T)>(&self, task: &Arc<Task<T>>, callback: &mut F) {
        loop {
            // Poll and requeue future if still pending
            let future = unsafe { &mut *task.future.get() };
            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&*waker);
            match future.as_mut().poll(context) {
                Poll::Pending => {
                    if self.aggressiveness > 0 {
                        // If we woke straight back up after polling, then...
                        if task.awake.swap(false, Ordering::Acquire) { // Double-check this
                            continue;
                        }
                    }
                    
                    self.ready_queue.borrow_mut().push(task.clone());
                    return;
                },
                Poll::Ready(v) => {
                    callback(v);
                    return;
                }
            }
        }
    }
    
    pub fn run_cb<F: FnMut(T)>(&self, callback: &mut F) {
        self.assert_singlethread();

        if self.running.swap(true, Ordering::SeqCst) { panic!("Executor already running"); }

        // Buffer current tasks
        let tasks = self.ready_queue.borrow_mut().expect_take();
        for task in tasks.iter() {
            // Put task to sleep so that it can be woken after polling
            // If it was already asleep, skip polling
            if !task.awake.swap(false, Ordering::Acquire) { // Double-check this
                self.ready_queue.borrow_mut().push(task.clone());
                continue;
            }

            self.run_task(task, callback);
        }
        self.ready_queue.borrow_mut().replace(tasks);

        self.running.store(false, Ordering::SeqCst);
    }

    pub fn run(&self) {
        self.run_cb(&mut |_|{})
    }

    pub fn spawn(&self, future: impl Future<Output = T> + 'static, run_immediately: bool) {
        self.assert_singlethread();

        let future = future.boxed_local();
        let task = Arc::new(Task {
            future: UnsafeCell::new(future),
            awake: true.into(),
        });

        if run_immediately {
            self.run_task(&task, &mut |_|{});
        } else {
            self.ready_queue.borrow_mut().push(task);
        }
    }

    fn assert_singlethread(&self) {
        assert_eq!(self.thread_id, std::thread::current().id(), "Executor can only spawn and process tasks the thread it was created in (Wake is fine for MT/Intterupt)");
    }
}

unsafe impl<T> Send for Task<T> {}
unsafe impl<T> Sync for Task<T> {}
struct Task<T> {
    future: UnsafeCell<LocalBoxFuture<'static, T>>, // Here's the plan. This isn't threadsafe but we don't use this on more than 1 thread... so we just lie to the compiler. Don't tell it
    awake: AtomicBool,
}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.awake.store(true, Ordering::Release);
    }
}