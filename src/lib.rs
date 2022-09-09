use std::cmp::Ordering;

pub fn chop_2(n: usize, arr: &[usize]) -> Option<usize> {
    if arr.len()==0{return None}
    let middle_index = (arr.len() - 1) / 2; //takes the floor anyway since len is an int
    let result = n.cmp(&arr[middle_index]);
    match result{
        Ordering::Greater => {
          let result = chop_2(n, &arr[middle_index + 1..]);
          match result {
              None => None,
              Some(x) => Some(x + middle_index + 1)
          }
        }
        Ordering::Less => chop_2(n, &arr[..middle_index]),
        Ordering::Equal => Some(middle_index)
    }
}

pub fn chop_1(n: usize, arr: &[usize]) -> i32 {
    if arr.len() == 0 {
        return -1;
    }
    let middle_index = (arr.len() - 1) / 2; //takes the floor anyway since len is an int
    println!("Index: {} Arr:{:#?}", middle_index, arr);
    if n > arr[middle_index] {
        let result = chop_1(n, &arr[middle_index + 1..]);
        if result == -1 {
            return -1;
        } else {
            return result + middle_index as i32 + 1;
        }
    } else if n < arr[middle_index] {
        let result = chop_1(n, &arr[..middle_index]);
        if result == -1 {
            return -1;
        } else {
            return result;
        }
    } else {
        return middle_index as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_2_twerks_edge() {
        assert_eq!(None, chop_2(3, &[]));
        assert_eq!(None, chop_2(3, &[1]));
        assert_eq!(Some(0), chop_2(1, &[1]));
    }
    #[test]
    fn it_2_twerks_3() {
        assert_eq!(Some(0), chop_2(1, &[1, 3, 5]));
        assert_eq!(Some(1), chop_2(3, &[1, 3, 5]));
        assert_eq!(Some(2), chop_2(5, &[1, 3, 5]));
        assert_eq!(None, chop_2(0, &[1, 3, 5]));
        assert_eq!(None, chop_2(2, &[1, 3, 5]));
        assert_eq!(None, chop_2(4, &[1, 3, 5]));
        assert_eq!(None, chop_2(6, &[1, 3, 5]));
    }
    #[test]
    fn it_2_twerks_4() {
        assert_eq!(Some(0), chop_2(1, &[1, 3, 5, 7]));
        assert_eq!(Some(1), chop_2(3, &[1, 3, 5, 7]));
        assert_eq!(Some(2), chop_2(5, &[1, 3, 5, 7]));
        assert_eq!(Some(3), chop_2(7, &[1, 3, 5, 7]));
        assert_eq!(None, chop_2(0, &[1, 3, 5, 7]));
        assert_eq!(None, chop_2(2, &[1, 3, 5, 7]));
        assert_eq!(None, chop_2(4, &[1, 3, 5, 7]));
        assert_eq!(None, chop_2(6, &[1, 3, 5, 7]));
        assert_eq!(None, chop_2(8, &[1, 3, 5, 7]));
    }
    #[test]
    fn it_twerks_edge() {
        assert_eq!(-1, chop_1(3, &[]));
        assert_eq!(-1, chop_1(3, &[1]));
        assert_eq!(0, chop_1(1, &[1]));
    }
    #[test]
    fn it_twerks_3() {
        assert_eq!(0, chop_1(1, &[1, 3, 5]));
        assert_eq!(1, chop_1(3, &[1, 3, 5]));
        assert_eq!(2, chop_1(5, &[1, 3, 5]));
        assert_eq!(-1, chop_1(0, &[1, 3, 5]));
        assert_eq!(-1, chop_1(2, &[1, 3, 5]));
        assert_eq!(-1, chop_1(4, &[1, 3, 5]));
        assert_eq!(-1, chop_1(6, &[1, 3, 5]));
    }
    #[test]
    fn it_twerks_4() {
        assert_eq!(0, chop_1(1, &[1, 3, 5, 7]));
        assert_eq!(1, chop_1(3, &[1, 3, 5, 7]));
        assert_eq!(2, chop_1(5, &[1, 3, 5, 7]));
        assert_eq!(3, chop_1(7, &[1, 3, 5, 7]));
        assert_eq!(-1, chop_1(0, &[1, 3, 5, 7]));
        assert_eq!(-1, chop_1(2, &[1, 3, 5, 7]));
        assert_eq!(-1, chop_1(4, &[1, 3, 5, 7]));
        assert_eq!(-1, chop_1(6, &[1, 3, 5, 7]));
        assert_eq!(-1, chop_1(8, &[1, 3, 5, 7]));
    }
}
