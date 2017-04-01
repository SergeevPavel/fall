use std::ascii::AsciiExt;

use ast::*;

macro_rules! line {
    ($name:ident, $($e:expr),*) => { $name.line(&format!($($e),*)) };
}

impl Grammar {
    pub fn generate(&self) -> String {
        let mut buff = Buff::new();
        let has_syn_rules = !self.syn_rules.is_empty();

        buff.line("use std::sync::{Once, ONCE_INIT};");
        buff.line("use fall_tree::{NodeType, NodeTypeInfo};");
        buff.line("use fall_parse::Rule;");
        if has_syn_rules {
            buff.line("use fall_parse::syn;");
        }
        buff.line("pub use fall_tree::{ERROR, WHITESPACE};");
        buff.blank_line();

        for (i, t) in self.node_types.iter().enumerate() {
            line!(buff, "pub const {:10}: NodeType = NodeType({});", scream(t), 100 + i);
        }
        buff.blank_line();

        buff.line("pub fn register_node_types() {");
        {
            buff.indent();
            buff.line("static REGISTER: Once = ONCE_INIT;");
            buff.line("REGISTER.call_once(||{");
            buff.indent();
            for t in self.node_types.iter() {
                line!(buff, "{}.register(NodeTypeInfo {{ name: {:?} }});", scream(t), scream(t));
            }
            buff.dedent();
            buff.line("});");
            buff.dedent();
        }
        buff.line("}");
        buff.blank_line();

        buff.line("pub const TOKENIZER: &'static [Rule] = &[");
        {
            buff.indent();
            for rule in self.lex_rules.iter() {
                let f = match rule.f {
                    None => "None".to_owned(),
                    Some(ref f) => format!("Some({})", f)
                };
                line!(buff, "Rule {{ ty: {}, re: {}, f: {} }},", scream(&rule.ty), rule.re, f);
            }
            buff.dedent();
        }
        buff.line("];");


        if has_syn_rules {
            buff.blank_line();
            self.generate_syn_rules(&mut buff);
        }

        buff.done()
    }

    fn generate_syn_rules(&self, buff: &mut Buff) {
        buff.line("pub const PARSER: &'static [syn::Rule] = &[");
        buff.indent();
        for rule in self.syn_rules.iter() {
            let ty = if self.node_types.contains(&rule.name) {
                format!("Some({})", scream(&rule.name))
            } else {
                "None".to_owned()
            };
            let alts = rule.alts.iter().map(|a| self.generate_alt(a)).collect::<Vec<_>>();
            line!(buff, r#"syn::Rule {{ name: "{}", ty: {}, alts: &[{}] }},"#,
                                   rule.name, ty, alts.join(", "));
        }
        buff.dedent();
        buff.line("];");
    }

    fn generate_alt(&self, alt: &Alt) -> String {
        let parts = alt.parts.iter().map(|p| self.generate_part(p)).collect::<Vec<_>>();
        format!("syn::Alt {{ parts: &[{}], commit: {:?} }}", parts.join(", "), alt.commit)
    }

    fn generate_part(&self, part: &Part) -> String {
        match *part {
            Part::Rule(ref name) => {
                if self.lex_rules.iter().any(|l| &l.ty == name) {
                    format!("syn::Part::Token({})", scream(name))
                } else {
                    format!("syn::Part::Rule({:?})", name)
                }
            }
            Part::Rep(ref a) => format!("syn::Part::Rep({})", self.generate_alt(a)),
        }
    }
}


fn scream(word: &str) -> String {
    word.chars().map(|c| c.to_ascii_uppercase()).collect()
}


struct Buff {
    inner: String,
    indent: usize
}

impl Buff {
    fn new() -> Buff {
        Buff { inner: String::new(), indent: 0 }
    }

    fn line(&mut self, line: &str) {
        for _ in 0..self.indent {
            self.inner.push_str("    ");
        }
        self.inner.push_str(line);
        self.inner.push('\n');
    }

    fn blank_line(&mut self) {
        self.inner.push('\n');
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        self.indent -= 1;
    }

    fn done(self) -> String {
        self.inner
    }
}
