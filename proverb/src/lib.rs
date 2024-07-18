pub fn build_proverb(list: &[&str]) -> String {
    match list.split_first() {
        None => String::new(),
        Some((&head, tail)) => tail
            .iter()
            .scan(head, |previous, &current| {
                let line = Some(format!(
                    "For want of a {} the {} was lost.\n",
                    previous, current
                ));
                *previous = current;
                line
            })
            .chain([format!("And all for the want of a {}.", head)])
            .collect(),
    }
}
