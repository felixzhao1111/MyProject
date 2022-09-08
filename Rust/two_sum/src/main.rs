// // 暴力
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let len = nums.len();
//         for i in 0..len {
//             let mut left = i;
//             let mut right = len - 1;
//             while left < right {
//                 if nums[left] + nums[right] == target {
//                     return vec![left as i32, right as i32];
//                 }
//                 right = right - 1;
//             }
//         }
//         Vec::new()
//     }
// }

// // hashmap
// use std::collections::HashMap;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut map = HashMap::new();

//         for i in 0..nums.len() {
//             if let Some(val) = map.get(&(target - nums[i])) {
//                 if i != *val {
//                     return vec![i as i32, *val as i32];
//                 }
//             }
//             map.insert(nums[i], i);
//         }
//         Vec::new()
//     }
// }

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    for i in 0..len {
        let mut left = i;
        let mut right = len - 1;
        while left < right {
            if nums[left] + nums[right] == target {
                return vec![left as i32, right as i32];
            }
            right = right - 1;
        }
    }
    Vec::new()
}

fn main() {
    let a = Vec::new();
    two_sum()
}
