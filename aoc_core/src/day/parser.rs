pub struct Parser<T>(Box<dyn Fn(String) -> T + 'static>);

impl<T> Parser<T> {
    pub fn new(parser: impl Fn(String) -> T + 'static) -> Self {
        Parser(Box::new(parser))
    }

    pub fn run(&self, value: String) -> T {
        (self.0)(value)
    }
}
