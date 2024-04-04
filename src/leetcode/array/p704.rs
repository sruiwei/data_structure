//! [二分查找](https://leetcode.cn/problems/binary-search/description/)
/// # 704. [二分查找](https://leetcode.cn/problems/binary-search/description/)
///
/// 给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。
///
/// ## 示例 1
///
/// ```text
/// 输入: nums = [-1,0,3,5,9,12], target = 9
/// 输出: 4
/// 解释: 9 出现在 nums 中并且下标为 4
/// ```
///
/// ## 示例 2
///
/// ```text
/// 输入: nums = [-1,0,3,5,9,12], target = 2
/// 输出: -1
/// 解释: 2 不存在 nums 中因此返回 -1
/// ```
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        if len == 0 {
            return -1;
        }

        let mut start: usize = 0;
        let mut end: usize = len - 1;
        // let mut mid: usize = 0;

        let mut index: i32 = -1;

        while start <= end {
            let mid = start + (end - start) / 2;
            match nums.get(mid) {
                Some(x) => {
                    if *x == target {
                        index = mid as i32;
                        break;
                    }
                    if *x > target {
                        if mid <= 0 {
                            break;
                        }
                        end = mid - 1;
                    } else {
                        start = mid + 1;
                    }
                }
                None => {
                    break;
                }
            }
        }

        return index;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search() {
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(-1, Solution::search(vec![5], -5));
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 13));
    }
}
