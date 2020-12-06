use crate::solutions::input::get_data_string;
use itertools::Itertools;

pub fn run() -> String {
    // grabs the entire input as a single String
    get_data_string("input/input_06")
        // split each group of people by the empty line in the input. This will return an iterator which allows me to operate on each group
        .split("\n\n")
        // taking the group iterator from above, use map to convert each group into how many unique "yes" answers are in each group
        .map(
            // map will pass each group as a string |s| into an operation.
            // s.chars() returns in an iter of all the characters in the string. This is all of the "yes" answers given by the group
            |s| s.chars()
                // we have some new lines still in the chars that we don't care about. this filters them out.
                .filter(|c| !c.is_whitespace())
                // itertools provides this .unique function which will provide an iterator for all of the unique elements in an iteration
                // so this .unique().count() will count how many unique "yes" answers there are
                // this is returned as the value that the map will convert the group to
                .unique().count()
        )
        // now that we have a bunch of unique answer counts for each group. we sum them all together for the answer
        .sum::<usize>()
        // convert to string because my framework expects a string as a result
        .to_string()
}