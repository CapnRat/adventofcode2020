use crate::solutions::input::get_data_string;
use itertools::Itertools;

pub fn run() -> String {
    // grabs the entire input as a single String (my input handling strips all the \r before this on windows which is important for the rest of this to work)
    get_data_string("input/input_06")
        // split each group of people by the empty line in the input. This will return an iterator which allows me to operate on each group
        .split("\n\n")
        // taking the group iterator from above, use map to convert each group into how many unique "yes" answers are shared for all members of a group
        .map(|group| group.chars()
            // filter out all the new lines, we don't care about them
            .filter(|&c| c != '\n')
            // using itertools create an iterator that only has one element for each unique answer in the group
            .unique()
            // convert each of those unique answers into how many times that answer was answered in that group
            .map(|answer| group.chars().filter(|&c| c == answer).count())
            // we know how many people are in a group by the number of lines in the group string.
            // compare the number of people to the count of each answer. filter out any that aren't equal, because that means at least one person did not answer yes to that question
            .filter(|&count| count == group.lines().count())
            // now we have an iterator of counts that all equal the number of people in the group
            // count how many of them there are and that's how many "yes" answers were answered by the entire group
            .count())
        // add the counts from each group together for the answer
        .sum::<usize>()
        .to_string()
}