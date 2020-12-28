pub fn find<T: PartialOrd, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref().iter().enumerate().collect::<Vec<_>>();
    let mut comp = &array[..];
    loop {
        if comp.len() == 0 {
            return None;
        }
        if comp.len() == 1 {
            if comp[0].1 == &key {
                return Some(comp[0].0);
            } else {
                return None;
            }
        }
        let midle_index = comp.len() / 2;
        match comp[midle_index].1 {
            val if val > &key => comp = &comp[0..midle_index],
            val if val < &key => comp = &comp[midle_index..=(comp.len() - 1)],
            _ => return Some(comp[midle_index].0 as usize),
        }
    }
}
