fn sum_list(lst: &[u32]) -> Option<u32> {
    if lst == [] {
        return None;
    } else {
        let mut sum: u32 = 0;
        for item in lst {
            match sum.checked_add(*item) {
                Some(v) => sum = v,
                None => return None,
            }
        }
        return Some(sum);
    }
}

fn main() {
    let lst:&[u32] = &[1,2,3,4294967295];
    let res = sum_list(&lst);
    println!("sum result is {:?}", res);

    let lst1:&[u32] = &[1,2,3,4];
    let res1 = sum_list(&lst1);
    println!("sum result is {:?}", res1);
}
