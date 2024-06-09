pub struct ContextManager<T> {
    item: T,
}

impl<T> ContextManager<T> {
    pub fn new(item: T) -> Self {
        // setup code here
        println!("Setting up...");

        Self { item }
    }
}

impl<T> Drop for ContextManager<T> {
    fn drop(&mut self) {
        // teardown code here
        println!("Tearing down...");
    }
}
