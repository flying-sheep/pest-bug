extern crate pest;
extern crate pest_derive;

pub mod no_title {
    #[derive(pest_derive::Parser)]
    #[grammar = "no-title.pest"]
    pub struct Parser_;
}

pub mod with_title {
    #[derive(pest_derive::Parser)]
    #[grammar = "with-title.pest"]
    pub struct Parser_;
}


#[cfg(test)]
mod test {
    use pest::{parses_to, consumes_to};
    
    macro_rules! parse_nested_lists { ($parse_mod:path) => {{
        use $parse_mod::*;
        
        parses_to! {
            parser: Parser_,
            input: "\
paragraph

-  item 1
-  item 2
   more text
   more text 2
   more text 3
   - nested item 1
   - nested item 2
   - nested item 3
",
            rule: Rule::document,
            tokens: [
                paragraph(0, 10, [ line(0, 10) ]),
                bullet_list(11, 131, [
                    bullet_item(11, 21, [ line(14, 21) ]),
                    bullet_item(21, 131, [
                        line(24, 31),
                        paragraph(34, 74, [
                            line(34, 44),
                            line(47, 59),
                            line(62, 74),
                        ]),
                        bullet_list(77, 131, [
                            bullet_item(77, 93, [ line(79, 93) ]),
                            bullet_item(96, 112, [ line(98, 112) ]),
                            bullet_item(115, 131, [ line(117, 131) ]),
                        ]),
                    ]),
                ]),
            ]
        }
    }} }
    
    #[test]
    fn no_title() {
        parse_nested_lists!(no_title);
    }
    
    #[test]
    fn with_title() {
        parse_nested_lists!(with_title);
    }
}