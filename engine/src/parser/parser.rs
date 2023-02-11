pub mod parser {
    extern crate pest;

    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "parser/calc.pest"]
    pub struct CalcParser;
}

#[cfg(test)]
mod tests {}
