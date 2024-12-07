use std::collections::{HashMap, HashSet};

// Day 5
//
// This solves a restricted version of the problem, where we assume that the rules are complete,
// meaning we don't have to consider non-consecutive elements of a page to determine if it is
// consistent with the rules.
//
// This is not true in general.  Consider the rules 1|3 and 3|4.  By these rules, 4,1,3
// is not valid because 3 appears before 4.  But this cannot be discovered by only considering
// the parirs (4, 1) and (1, 3) unless we derive all the consequent rules (in this case, 1|4).
fn main() -> std::io::Result<()> {
    let inputs = rust_advent::read_rules_and_updates("05")?;
    println!(
        "Sum of middle values valid pages: {}",
        sum_of_middle_values_of_valid_pages(&inputs.pages, &inputs.before)
    );
    println!(
        "Sum of middle values after correction: {}",
        sum_of_corrected_middle_values(&inputs.pages, &inputs.before)
    );
 
    Ok(())
}

fn is_valid_page(page: &[u16], before: &HashMap<u16, HashSet<u16>>) -> bool {
    page.iter().is_sorted_by(|a, b| before[b].contains(a))
}

fn sum_of_middle_values_of_valid_pages(
    pages: &[Vec<u16>],
    before: &HashMap<u16, HashSet<u16>>,
) -> u32 {
    pages
        .iter()
        .filter(|p| is_valid_page(p, before))
        .map(|p| p[p.len() / 2] as u32)
        .sum()
}

fn sum_of_corrected_middle_values(pages: &[Vec<u16>], before: &HashMap<u16, HashSet<u16>>) -> u32 {
    pages
        .iter()
        .filter(|p| !is_valid_page(p, before))
        .cloned()
        .map(|mut p| {
            p.sort_by(|a, b| before[b].contains(a).cmp(&true));
            p[p.len() / 2] as u32
        })
        .sum()
}
