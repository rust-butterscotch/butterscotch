/* ************************************************************************ **
** * ©2020 Michael Baker (butterscotch@notvery.moe) | Apache License v2.0 * **
** ************************************************************************ */

use futures::{
    future::{LocalBoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{cell::{RefCell, UnsafeCell}, future::Future, sync::atomic::{AtomicBool, Ordering}, sync::{Arc}, task::{Context, Poll}, thread::ThreadId};

use crate::container::DoubleBuffer;

/// Task executor that runs futures created on the same thread as it.
/// This means that the futures do not need to be send/sync...
/// But, for this reason, the executor is can't be send or sync.
/// Debug checks are in-place to detect foul-play.
pub struct IncrementalLocalExecutor<T>  {
    #[cfg(debug_assertions)] thread_id: ThreadId,
    retries: u32,
    running: AtomicBool,
    // TODO
    //  Change to slotmap, add "awake" list that references entries in the slotmap.
    //   + Reduces the amount of Arc copies/moves/drops
    //   + Allows us retain ordering whilst skipping over sleeping tasks entirely
    //   ~ Cache-coherency is probbably worse under this, but we weren't getting much of that anyway
    //   - Slotmap might not have a tidy performance overhead on these? 
    //   - Higher memory usage
    queue: RefCell<DoubleBuffer<Arc<LocalTask<T>>>>,
}

impl<T> !Send for IncrementalLocalExecutor<T> {}
impl<T> !Sync for IncrementalLocalExecutor<T> {}

impl<T> IncrementalLocalExecutor<T> {
    pub fn new(retries: u32) -> IncrementalLocalExecutor<T> {
        IncrementalLocalExecutor { 
            #[cfg(debug_assertions)] thread_id: std::thread::current().id(),
            running: false.into(), 
            queue: RefCell::default(),
            retries,
        }
    }

    pub fn spawn(&self, future: impl Future<Output = T> + 'static) {
        self.assert_singlethread();

        let future = future.boxed_local();
        let task = Arc::new(LocalTask {
            future: UnsafeCell::new(future),
            awake: true.into(),
        });

        self.queue.borrow_mut().push(task);
    }

    pub fn run(&self) {
        self.run_cb(&mut |_|{})
    }
    
    pub fn run_cb<F: FnMut(T)>(&self, callback: &mut F) {
        self.assert_singlethread();

        if self.running.swap(true, Ordering::SeqCst) { panic!("Executor already running"); }

        // Buffer current tasks
        let tasks = self.queue.borrow_mut().expect_take();
        for task in tasks.iter() {
            // Put task to sleep so that it can be woken after polling
            // If it was already asleep, skip polling
            if !task.awake.swap(false, Ordering::Acquire) { // Double-check this
                self.queue.borrow_mut().push(task.clone());
                continue;
            }

            self.run_task(task, callback);
        }
        self.queue.borrow_mut().replace(tasks);

        self.running.store(false, Ordering::SeqCst);
    }
}

impl<T> IncrementalLocalExecutor<T> {

    fn run_task<F: FnMut(T)>(&self, task: &Arc<LocalTask<T>>, callback: &mut F) {
        self.assert_singlethread();

        let future = unsafe { &mut *task.future.get() };
        let waker = waker_ref(&task);
        let context = &mut Context::from_waker(&*waker);

        let mut attempts = 0;
        loop {
            attempts += 1;
            match future.as_mut().poll(context) {
                Poll::Ready(v) => { callback(v); break; }
                Poll::Pending => if (attempts > self.retries) || !task.awake.swap(false, Ordering::Acquire) {
                    // If we woke straight back up after polling, then try again, unless we exceed our retries 
                    // Then requeue the task
                    self.queue.borrow_mut().push(task.clone());
                    break;
                },
            }
        }
    }

    #[cfg(debug_assertions)] 
    #[inline(always)] fn assert_singlethread(&self) {
        assert_eq!(self.thread_id, std::thread::current().id(), "Executor can only spawn and process tasks the thread it was created in (Wake is fine for MT/Intterupt)");
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)] fn assert_singlethread(&self) {}


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