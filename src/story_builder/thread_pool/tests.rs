use story_builder::thread_pool::*;

#[test]
/// Creates a thread pool with one thread to evaluate "n + n".
fn test_simple_job() {
    struct TestJob {}
    impl Job<u8, u8> for TestJob {
        fn step(&self, input: u8) -> u8 {
            (input + input)
        }
    }

    let job = Arc::new(TestJob {});
    let mut tp = ThreadPool::new(1, job);
    tp.send(1);
    assert_eq!(2, tp.receive());
}

#[test]
/// Creates a thread pool with one thread to evaluate "n + n" twice.
fn test_simple_job_twice() {
    struct TestJob {}
    impl Job<u8, u8> for TestJob {
        fn step(&self, input: u8) -> u8 {
            (input + input)
        }
    }

    let job = Arc::new(TestJob {});
    let mut tp = ThreadPool::new(1, job);
    tp.send(1);
    let recv = tp.receive();
    assert_eq!(2, recv);
    tp.send(10);
    let recv = tp.receive();
    assert_eq!(20, recv);
}


#[test]
/// Creates a thread pool with two threads. The first one will sleep and the other one
/// will return immediately. Responses must be in reverse in the return function.
fn test_multithread_order() {
    struct TestJob {}
    impl Job<bool, bool> for TestJob {
        fn step(&self, input: bool) -> bool {
            if input {
                thread::sleep_ms(10);
            }
            input
        }
    }

    let job = Arc::new(TestJob {});
    let mut tp = ThreadPool::new(2, job);
    tp.send(true); // sleep the first
    tp.send(false); // instant the second
    assert_eq!(false, tp.receive()); // non-sleeping return first
    assert_eq!(true, tp.receive()); // sleeping return second
}

#[test]
/// Creates a thread pool, send 5 jobs and iterate. Confirm that 5 outputs were received.
fn test_simple_iterate_recv() {
    struct TestJob {}
    impl Job<u8, u8> for TestJob {
        fn step(&self, input: u8) -> u8 {
            input
        }
    }

    let job = Arc::new(TestJob {});
    let mut tp = ThreadPool::new(1, job);
    let iterations = 5;
    for i in 0..iterations {
        tp.send(i);
    }
    let mut results: Vec<u8> = tp.iter().collect();
    results.sort();
    assert_eq!(results, vec![0,1,2,3,4]);
}
