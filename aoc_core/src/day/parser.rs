pub struct DayParser<T>(Box<dyn Fn(String) -> T + 'static>);

impl<T> DayParser<T> {
    pub fn new(parser: impl Fn(String) -> T + 'static) -> Self {
        DayParser(Box::new(parser))
    }

    pub fn run(&self, value: String) -> T {
        (self.0)(value)
    }
}
