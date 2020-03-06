struct Context<'a> {
    raw_text: &'a str,
}

// use lifetimes to underscore that the returned reference lifetime is not
// the same as the lifetime of Context
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

// This is no longer necessary and it is not even valid syntax:
// impl<'c, 's> Parser<'c, 's: 'c> {
impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.raw_text[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

fn main() {
    let text = String::from("basic string");
    let ctx = Context { raw_text: &text };
    let res = parse_context(ctx);
    println!("res: {:?}", res);
}
