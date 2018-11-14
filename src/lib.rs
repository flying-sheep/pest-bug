extern crate pest;
extern crate pest_derive;

pub mod title_one_rule {
    #[derive(pest_derive::Parser)]
    #[grammar = "title-one-rule.pest"]
    pub struct Parser_;
}

pub mod title_two_rules {
    #[derive(pest_derive::Parser)]
    #[grammar = "title-two-rules.pest"]
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
    fn title_one_rule() {
        parse_nested_lists!(title_one_rule);
    }
    
    #[test]
    fn title_two_rules() {
        parse_nested_lists!(title_two_rules);
    }
}