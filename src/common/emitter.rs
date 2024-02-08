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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let emitter = Emitter::new();
        assert_eq!(emitter.0, Vec::<String>::new());
    }

    #[test]
    fn emit() {
        let mut emitter = Emitter::new();
        emitter.emit("test");
        assert_eq!(emitter.0, vec!["test"]);
    }

    #[test]
    fn emit_tabbed() {
        let mut emitter = Emitter::new();
        emitter.emit_tabbed(2, "test");
        assert_eq!(emitter.0, vec!["\t\ttest"]);
    }

    #[test]
    fn emit_label() {
        let mut emitter = Emitter::new();
        emitter.emit_label("test");
        assert_eq!(emitter.0, vec!["test:"]);
    }

    #[test]
    fn emit_instr() {
        let mut emitter = Emitter::new();
        emitter.emit_instr("test");
        assert_eq!(emitter.0, vec!["\ttest"]);
    }

    #[test]
    fn emit_directive() {
        let mut emitter = Emitter::new();
        emitter.emit_directive("test");
        assert_eq!(emitter.0, vec!["\ttest"]);
    }

    #[test]
    fn collect() {
        let mut emitter = Emitter::new();
        emitter.emit("test1");
        emitter.emit("test2");
        assert_eq!(emitter.collect(), "test1\ntest2\n");
    }
}
