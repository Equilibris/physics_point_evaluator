pub struct Symbol {
    name: String,
    value: Option<f64>,
}

impl Symbol {
    pub fn new(name: String) -> Symbol {
        Symbol { name, value: None }
    }
    pub fn associate_value(&mut self, value: f64) {
        self.value = Some(value);
    }
}
