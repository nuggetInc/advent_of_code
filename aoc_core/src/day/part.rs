pub struct DayPart<T>(Box<dyn Fn(&T) -> String + 'static>);

impl<T> DayPart<T> {
    pub fn new(part: impl Fn(&T) -> String + 'static) -> Self {
        DayPart(Box::new(part))
    }

    pub fn run(&self, value: &T) -> String {
        (self.0)(value)
    }
}
