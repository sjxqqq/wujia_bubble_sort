use std::cmp::PartialOrd;

// 固定类型版
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 泛型版（任意类型版）
fn generics_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // 固定类型数组演示
    let mut arr1 = [88, 44, 5, 22, 1];
    println!("固定类型：原始数组: {:?}", arr1);
    bubble_sort(&mut arr1);
    println!("固定类型：重排数组: {:?}", arr1);

    // 泛型版演示
    let mut arr2 = ["zhangsan", "huaqiang", "jiege", "kunkun"];
    println!("泛型版：原始数组: {:?}", arr2);
    generics_bubble_sort(&mut arr2);
    println!("泛型版：重排数组: {:?}", arr2);
}

