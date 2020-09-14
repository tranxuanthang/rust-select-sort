fn main() {
    let mut arr = [12, 4, 2, 11, 6, 7, 5, 9, 8];

    sort(&mut arr);
    println!("{:?}", arr);
}

fn sort(arr: &mut[i32]) {
    for i in 0..arr.len() - 1 {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j)
            }
        }
    }
}
