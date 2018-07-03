fn main() {
    let mut arr = [61, 21, 27, 79, 57, 60, 46, 42, 27, 92, 66, 26];

    sort(&mut arr);
    // TODO: print `arr`
}

// TODO: write `sort()` function
fn sort(array: &mut [u64]){
    for i in 0..array.len()-1 {
        let mut smallest_elem_idx = i;
        for j in i+1..array.len() {
            if array[j] < array[smallest_elem_idx] {
                smallest_elem_idx = j; 
            }
        }

        // interchange i and smallest_elem
        let mut tmp = array[smallest_elem_idx];
        array[smallest_elem_idx] = array[i];
        array[i] = tmp;
    }
}

#[test]
fn sort_array() {
    let mut arr =  [
        61, 21, 27, 79, 57, 60, 46, 92, 66, 26, 37, 15, 29, 70, 30, 55, 62, 81,
        84, 35, 34, 52, 98, 50, 39, 42, 41, 24, 28, 64, 95, 47, 43, 23, 14, 71,
        78, 86, 51, 20, 9, 1, 18, 17, 94, 33, 3, 91, 65, 2, 38, 59, 96, 8, 83,
        19, 90, 63, 16, 58, 68, 48
    ];
    sort(&mut arr);
    assert_eq!(&arr as &[u64], &[
        1u64, 2, 3, 8, 9, 14, 15, 16, 17, 18, 19, 20, 21, 23, 24, 26, 27, 28, 29,
        30, 33, 34, 35, 37, 38, 39, 41, 42, 43, 46, 47, 48, 50, 51, 52, 55, 57,
        58, 59, 60, 61, 62, 63, 64, 65, 66, 68, 70, 71, 78, 79, 81, 83, 84, 86,
        90, 91, 92, 94, 95, 96, 98,
    ] as &[u64]);
}
