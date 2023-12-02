pub struct Part<T>(Box<dyn Fn(&T) -> String + 'static>);

impl<T> Part<T> {
    pub fn new(part: impl Fn(&T) -> String + 'static) -> Self {
        Part(Box::new(part))
    }

    pub fn run(&self, value: &T) -> String {
        (self.0)(value)
    }
}
