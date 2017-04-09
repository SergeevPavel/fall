use fall_tree::{NodeType, NodeTypeInfo, Language, LanguageImpl};
pub use fall_tree::{ERROR, WHITESPACE};

pub const ATOM: NodeType = NodeType(100);
pub const LPAREN: NodeType = NodeType(101);
pub const RPAREN: NodeType = NodeType(102);
pub const FILE: NodeType = NodeType(103);
pub const LIST: NodeType = NodeType(104);

lazy_static! {
    pub static ref LANG: Language = {
        use fall_parse::{LexRule, SynRule, Alt, Part, Parser};

        ATOM.register(NodeTypeInfo { name: "ATOM" });
        LPAREN.register(NodeTypeInfo { name: "LPAREN" });
        RPAREN.register(NodeTypeInfo { name: "RPAREN" });
        FILE.register(NodeTypeInfo { name: "FILE" });
        LIST.register(NodeTypeInfo { name: "LIST" });

        const PARSER: &'static [SynRule] = &[
            SynRule {
                ty: Some(FILE),
                alts: &[Alt { parts: &[Part::Rep(Alt { parts: &[Part::Rule(1)], commit: None })], commit: None }],
            },
            SynRule {
                ty: None,
                alts: &[Alt { parts: &[Part::Token(ATOM)], commit: None }, Alt { parts: &[Part::Rule(2)], commit: None }],
            },
            SynRule {
                ty: Some(LIST),
                alts: &[Alt { parts: &[Part::Token(LPAREN), Part::Rep(Alt { parts: &[Part::Rule(1)], commit: None }), Part::Token(RPAREN)], commit: None }],
            },
        ];

        struct Impl { tokenizer: Vec<LexRule> };
        impl LanguageImpl for Impl {
            fn parse(&self, lang: Language, text: String) -> ::fall_tree::File {
                ::fall_parse::parse(lang, text, FILE, &self.tokenizer, &|b| Parser::new(PARSER).parse(b))
            }
        }

        Language::new(Impl {
            tokenizer: vec![
                LexRule::new(LPAREN, "\\(", None),
                LexRule::new(RPAREN, "\\)", None),
                LexRule::new(WHITESPACE, "\\s+", None),
                LexRule::new(ATOM, "\\w+", None),
            ]
        })
    };
}


