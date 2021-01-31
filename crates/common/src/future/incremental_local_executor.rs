/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use futures::{
    future::{LocalBoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{cell::UnsafeCell, future::Future, sync::atomic::{AtomicBool, Ordering}, sync::{Arc, Mutex}, task::{Context, Poll}, thread::ThreadId};

use crate::container::DoubleBuffer;

/// Task executor that runs futures created on the same thread as it.
/// This means that the futures do not need to be send/sync...
/// But, for this reason, the executor can only run on a single thread.
/// We can recieve "Send" futures from other threads, as well as wakes.
pub struct IncrementalLocalExecutor<T>  {
    thread_id: ThreadId,
    running: AtomicBool,
    // TODO This isn't super flexible... 
    // Can't support getting/storing references to a future
    // Lots of locking/unlocking currrently
    queue: Mutex<DoubleBuffer<Arc<LocalTask<T>>>>,
}

impl<T> Default for IncrementalLocalExecutor<T> {
    fn default() -> Self { Self::new() }
}

impl<T> IncrementalLocalExecutor<T> {
    pub fn new() -> IncrementalLocalExecutor<T> {
        IncrementalLocalExecutor { 
            thread_id: std::thread::current().id(),
            running: false.into(), 
            queue: Mutex::default(),
        }
    }

    pub fn defer(&self, future: impl Future<Output = T> + Send + 'static) {
        self.spawn(future);
    }

    pub fn exec(&self, future: impl Future<Output = T> + 'static) {
        self.assert_singlethread();
        self.spawn(future);
    }

    pub fn proccess<F: FnMut(T)>(&self, callback: &mut F) {
        self.assert_singlethread();
        if self.running.swap(true, Ordering::SeqCst) { panic!("Executor already running"); }

        // Buffer current tasks
        let tasks = self.queue.lock().unwrap().expect_take();
        for task in tasks.iter() {
            // Put task to sleep so that it can be woken after polling
            // If it was already asleep, skip polling
            if !task.awake.swap(false, Ordering::Acquire) { // Double-check this
                self.queue.lock().unwrap().push(task.clone());
                continue;
            }

            let future = unsafe { &mut *task.future.get() };
            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&*waker);
            match future.as_mut().poll(context) {
                Poll::Pending  => self.queue.lock().unwrap().push(task.clone()),
                Poll::Ready(v) => callback(v),
            }
        }
        self.queue.lock().unwrap().replace(tasks);
        self.running.store(false, Ordering::SeqCst);
    }
}

impl<T> IncrementalLocalExecutor<T> {
    fn spawn(&self, future: impl Future<Output = T> + 'static) {
        let future = future.boxed_local();
        let task = Arc::new(LocalTask {
            future: UnsafeCell::new(future),
            awake: true.into(),
        });
        self.queue.lock().unwrap().push(task);
    }

    #[inline(always)] fn assert_singlethread(&self) {
        assert_eq!(
            self.thread_id, std::thread::current().id(), 
            "IncrementalLocalExecutor can only process tasks on the same thread it was created on."
        );
    }
}

unsafe impl<T> Send for LocalTask<T> {}
unsafe impl<T> Sync for LocalTask<T> {}
struct LocalTask<T> {
    // Here's the plan. Only this part isn't threadsafe but we don't use this on more than 1 thread... so we just lie to the compiler and do runtime checks. Don't tell on us please
    future: UnsafeCell<LocalBoxFuture<'static, T>>, 
    awake: AtomicBool,
}

impl<T> ArcWake for LocalTask<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.awake.store(true, Ordering::Release);
    }
}