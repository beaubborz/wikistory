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
    let tp = ThreadPool::new(1, job);
    tp.send(1);
    assert_eq!(2, tp.receive());
}
