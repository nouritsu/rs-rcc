pub struct Emitter(Vec<String>);

impl Emitter {
    pub fn new() -> Self {
        Emitter(Vec::new())
    }

    pub fn emit(&mut self, s: &str) {
        self.0.push(s.to_owned());
    }

    pub fn emit_tabbed(&mut self, n: usize, s: &str) {
        self.0.push(format!("{}{}", "\t".repeat(n), s));
    }

    pub fn emit_label(&mut self, s: &str) {
        self.emit(&format!("{}:", s))
    }

    pub fn emit_instr(&mut self, s: &str) {
        self.emit_tabbed(1, s)
    }

    pub fn emit_directive(&mut self, s: &str) {
        self.emit_tabbed(1, s)
    }

    pub fn collect(self) -> String {
        self.0.join("\n") + "\n"
    }
}
