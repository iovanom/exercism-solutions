#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // This is the implementation using interator and windows but this is slower than
    // the implementation with for loop
    // ```
    //    fn _is_sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
    //        l2.windows(l1.len()).find(|&l| l == l1).is_some()
    //    }
    //```

    fn _is_sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
        let (len1, len2) = (l1.len(), l2.len());
        for i in 0..=(len2 - len1) {
            if l1 == &l2[i..i + len1] {
                return true;
            }
        }
        false
    }

    match (_first_list.len(), _second_list.len()) {
        (l1, l2) if l1 == l2 && _first_list == _second_list => Comparison::Equal,
        (l1, l2) if l1 == 0 || (l1 < l2 && _is_sublist(_first_list, _second_list)) => Comparison::Sublist,
        (l1, l2) if l2 == 0 || (l1 > l2 && _is_sublist(_second_list, _first_list)) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
