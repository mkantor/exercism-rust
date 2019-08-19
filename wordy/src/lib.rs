mod parser {
    use std::str::FromStr;

    pub enum ParseError {
        NotANumber,
        InvalidOperator,
        MalformedQuestion,
    }

    #[derive(Clone, Copy)]
    pub enum Operator {
        Plus,
        Minus,
        MultipliedBy,
        DividedBy,
    }

    pub struct WordProblem {
        pub head: isize,
        pub tail: Vec<(Operator, isize)>,
    }

    impl FromStr for WordProblem {
        type Err = ParseError;

        /// Parses strings like "What is 1 plus 1 minus 2?".
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            const PREFIX: &str = "What is ";
            const SUFFIX: &str = "?";

            if input.starts_with(PREFIX) && input.ends_with(SUFFIX) {
                let operations = &input[PREFIX.len()..(input.len() - SUFFIX.len())];

                let (head, mut input) = parse_number(operations).ok_or(ParseError::NotANumber)?;
                let mut tail = Vec::new();
                while input != "" {
                    let (operator, remaining_input) =
                        parse_operator(input).ok_or(ParseError::InvalidOperator)?;

                    let (operand, remaining_input) =
                        parse_number(remaining_input).ok_or(ParseError::NotANumber)?;

                    tail.push((operator, operand));
                    input = remaining_input;
                }
                Ok(WordProblem { head, tail })
            } else {
                Err(ParseError::MalformedQuestion)
            }
        }
    }

    fn parse_operator(input: &str) -> Option<(Operator, &str)> {
        let try_operator = |literal: &'static str, operator: Operator| {
            input
                .get(0..literal.len())
                .filter(|token| *token == literal)
                .map(|token| (operator, &input[token.len()..]))
        };

        try_operator(" plus ", Operator::Plus)
            .or_else(|| try_operator(" minus ", Operator::Minus))
            .or_else(|| try_operator(" multiplied by ", Operator::MultipliedBy))
            .or_else(|| try_operator(" divided by ", Operator::DividedBy))
    }

    fn parse_number(input: &str) -> Option<(isize, &str)> {
        let token = input
            .chars()
            .take_while(|&character| character.is_digit(10) || character == '-')
            .collect::<String>();

        let token_length = token.len();
        token
            .parse::<isize>()
            .ok()
            .map(|number| (number, &input[token_length..]))
    }
}

mod evaluator {
    use super::parser::Operator;
    use super::parser::WordProblem;

    pub fn evaluate(problem: WordProblem) -> isize {
        problem.tail.iter().fold(
            problem.head,
            |evaluated, (operator, operand)| match operator {
                Operator::Plus => evaluated + operand,
                Operator::Minus => evaluated - operand,
                Operator::MultipliedBy => evaluated * operand,
                Operator::DividedBy => evaluated / operand,
            },
        )
    }
}

use evaluator::evaluate;
use parser::WordProblem;

pub fn answer(input: &str) -> Option<isize> {
    input.parse::<WordProblem>().ok().map(evaluate)
}
