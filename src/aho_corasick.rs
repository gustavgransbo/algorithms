use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

struct State {
    value: char,
    fail_state: Option<Rc<RefCell<State>>>,
    next_states: HashMap<char, Rc<RefCell<State>>>,
    output: HashSet<String>,
}

/// A pattern finder implemented using using the Aho-Corasick algorithm
///
/// A `PatternFinder` instance is created from a set of patterns, and can then
/// be used to efficiently find occurences of those patterns in a text.
///
/// # Examples
///
/// ```
/// use algorithms::aho_corasick::PatternFinder;
///
/// let patterns = vec![String::from("foo"), String::from("oof"), String::from("o")];
/// let pattern_finder = PatternFinder::new(patterns);
/// let pattern_locations = pattern_finder.find_patterns(&String::from("foof"));
///
/// assert_eq!(pattern_locations[&String::from("foo")], vec![0]);
/// assert_eq!(pattern_locations[&String::from("oof")], vec![1]);
/// assert_eq!(pattern_locations[&String::from("o")], vec![1, 2]);
/// ```

pub struct PatternFinder {
    root_state: Rc<RefCell<State>>,
}

impl PatternFinder {
    /// Creates a new `PatternFinder`
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::aho_corasick::PatternFinder;
    /// let pattern_finder = PatternFinder::new(
    ///     vec![String::from("a pattern"), String::from("another pattern")]
    /// );
    /// ```
    pub fn new(patterns: Vec<String>) -> PatternFinder {
        let mut pattern_finder = PatternFinder {
            root_state: Rc::new(RefCell::new(State {
                value: '\0',
                fail_state: None,
                next_states: HashMap::new(),
                output: HashSet::new(),
            })),
        };
        for pattern in patterns {
            pattern_finder.add_pattern(String::clone(&pattern));
        }
        pattern_finder.set_fail_states();

        pattern_finder
    }

    fn add_pattern(&mut self, pattern: String) {
        let mut state = Some(Rc::clone(&self.root_state));
        for c in pattern.chars() {
            state = {
                let state_some = state.take().unwrap();
                let mut state_borrowed = state_some.borrow_mut();
                if let Some(next_state) = state_borrowed.next_state(&c) {
                    Some(next_state)
                } else {
                    let next_state = Rc::new(RefCell::new(State {
                        value: c,
                        fail_state: None,
                        next_states: HashMap::new(),
                        output: HashSet::new(),
                    }));
                    state_borrowed
                        .next_states
                        .insert(char::clone(&c), Rc::clone(&next_state));
                    Some(next_state)
                }
            };
        }
        state.unwrap().borrow_mut().output.insert(pattern);
    }

    fn set_fail_states(&mut self) {
        let root_state = Rc::clone(&self.root_state);
        let mut queue = VecDeque::new();

        for state in root_state.borrow().next_states.values() {
            state.borrow_mut().set_fail_state(Rc::clone(&root_state));
            queue.push_back(Rc::clone(state));
        }

        while !queue.is_empty() {
            let mut state_option = queue.pop_front();
            let state = state_option.take().unwrap();
            for child in state.borrow().next_states.values() {
                queue.push_back(Rc::clone(child));
                let child_value = &char::clone(&child.borrow().value);
                if let Some(fail_state) = state
                    .borrow()
                    .fail_state
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .next_state(child_value)
                {
                    child.borrow_mut().set_fail_state(Rc::clone(&fail_state));
                } else {
                    child
                        .borrow_mut()
                        .set_fail_state(Rc::clone(&self.root_state));
                }
            }
        }
    }
    /// Searches a text for any occurences of the patterns in the `PatternFinder`
    ///
    /// Returns a HashMap from each occuring pattern to a vector containing the indices at which the pattern occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorithms::aho_corasick::PatternFinder;
    ///
    /// let patterns = vec![String::from("foo"), String::from("oof"), String::from("o")];
    /// let pattern_finder = PatternFinder::new(patterns);
    /// let pattern_locations = pattern_finder.find_patterns(&String::from("foof"));
    ///
    /// assert_eq!(pattern_locations[&String::from("foo")], vec![0]);
    /// assert_eq!(pattern_locations[&String::from("oof")], vec![1]);
    /// assert_eq!(pattern_locations[&String::from("o")], vec![1, 2]);
    /// ```
    pub fn find_patterns(&self, text: &String) -> HashMap<String, Vec<usize>> {
        let mut result: HashMap<String, Vec<usize>> = HashMap::new();
        let mut state = Some(Rc::clone(&self.root_state));
        for (i, c) in text.char_indices() {
            let state_some = state.take().unwrap();
            let state_borrowed = state_some.borrow();
            if let Some(new_state) = state_borrowed.next_state(&c) {
                for pattern in new_state.borrow().output.iter() {
                    result
                        .entry(String::clone(pattern))
                        .or_default()
                        .push(1 + i - pattern.len());
                }
                state = Some(new_state);
            } else {
                state = Some(Rc::clone(&self.root_state));
            }
        }
        result
    }
}

impl State {
    fn next_state(&self, value: &char) -> Option<Rc<RefCell<State>>> {
        if let Some(state) = self.next_states.get(value) {
            return Some(Rc::clone(state));
        } else if self.fail_state.is_some() {
            self.fail_state.as_ref().unwrap().borrow().next_state(value)
        } else {
            None
        }
    }

    fn set_fail_state(&mut self, fail_state: Rc<RefCell<State>>) {
        self.fail_state = Some(Rc::clone(&fail_state));
        self.output
            .extend(fail_state.borrow().output.iter().cloned());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::iter::FromIterator;

    fn check_correct_output(
        patterns: Vec<String>,
        text: String,
        pattern_locations: &[(String, Vec<usize>)],
    ) {
        let expected_output = HashMap::from_iter(pattern_locations.iter().cloned());
        let pattern_finder = PatternFinder::new(patterns);
        let output = pattern_finder.find_patterns(&text);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_empty() {
        check_correct_output(vec![], String::from(""), &[]);
    }

    #[test]
    fn test_single() {
        check_correct_output(
            vec![String::from("a")],
            String::from("aa"),
            &[(String::from("a"), vec![0, 1])],
        );
    }

    #[test]
    fn test_simple_overlap() {
        check_correct_output(
            vec![String::from("aa"), String::from("ab")],
            String::from("aab"),
            &[(String::from("aa"), vec![0]), (String::from("ab"), vec![1])],
        );
    }

    #[test]
    fn test_sub_pattern() {
        check_correct_output(
            vec![String::from("a"), String::from("aa")],
            String::from("aaa"),
            &[
                (String::from("a"), vec![0, 1, 2]),
                (String::from("aa"), vec![0, 1]),
            ],
        );
    }

    #[test]
    fn test_duplicate_pattern() {
        check_correct_output(
            vec![String::from("a"), String::from("a")],
            String::from("aaa"),
            &[(String::from("a"), vec![0, 1, 2])],
        );
    }

    #[test]
    fn test_bananananaspaj() {
        check_correct_output(
            vec![
                String::from("anas"),
                String::from("ana"),
                String::from("an"),
                String::from("a"),
            ],
            String::from("bananananaspaj"),
            &[
                (String::from("anas"), vec![7]),
                (String::from("ana"), vec![1, 3, 5, 7]),
                (String::from("an"), vec![1, 3, 5, 7]),
                (String::from("a"), vec![1, 3, 5, 7, 9, 12]),
            ],
        );
    }
}
