#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<A>(list_1: &[A], list_2: &[A]) -> Comparison
    where A: PartialEq
{
    match (list_1.len(), list_2.len()) {
        (len_1, len_2) if len_1 < len_2 && is_sublist(list_1, list_2) => Comparison::Sublist,
        (len_1, len_2) if len_2 < len_1 && is_sublist(list_2, list_1) => Comparison::Superlist,
        (len_1, len_2) if len_1 == len_2 && is_sublist(list_1, list_2) => Comparison::Equal,
        _ => Comparison::Unequal,
    }
}

fn is_sublist<A>(short_list: &[A], long_list: &[A]) -> bool
    where A: PartialEq
{
    short_list.is_empty() ||
    long_list
        .windows(short_list.len())
        .any(|candidate| candidate == short_list)
}
