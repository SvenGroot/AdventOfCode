use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::{Operation, Part, Rule, RuleCondition, Workflow};

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) = alt((char('<'), char('>')))(input)?;
    let operation = match operation {
        '<' => Operation::LessThan,
        '>' => Operation::GreaterThan,
        _ => unreachable!(),
    };

    Ok((input, operation))
}

fn condition(input: &str) -> IResult<&str, RuleCondition> {
    let (input, (field, op, value)) =
        tuple((anychar, operation, nom::character::complete::u64))(input)?;

    Ok((
        input,
        RuleCondition {
            field,
            operation: op,
            value,
        },
    ))
}

fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, (condition, target)) =
        tuple((opt(terminated(condition, tag(":"))), alpha1))(input)?;
    Ok((
        input,
        Rule {
            condition,
            target: target.into(),
        },
    ))
}

pub fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, (name, rules)) = tuple((
        alpha1,
        delimited(tag("{"), separated_list1(tag(","), rule), tag("}")),
    ))(input)?;

    Ok((
        input,
        Workflow {
            name: name.into(),
            rules,
        },
    ))
}

fn rating(input: &str) -> IResult<&str, (char, u64)> {
    let (input, (field, _, value)) =
        tuple((anychar, char('='), nom::character::complete::u64))(input)?;

    Ok((input, (field, value)))
}

pub fn part(input: &str) -> IResult<&str, Part> {
    let (input, ratings) = delimited(tag("{"), separated_list1(tag(","), rating), tag("}"))(input)?;

    Ok((
        input,
        Part {
            ratings: ratings.into_iter().collect(),
        },
    ))
}
