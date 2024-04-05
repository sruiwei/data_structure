/// # [209. 长度最小的子数组](https://leetcode.cn/problems/minimum-size-subarray-sum/description/)
/// 
/// 给定一个含有 n 个正整数的数组和一个正整数 target 。
/// 
/// 找出该数组中满足其总和大于等于 target 的长度最小的 连续
/// 子数组
///  [numsl, numsl+1, ..., numsr-1, numsr] ，并返回其长度。如果不存在符合条件的子数组，返回 0 。
/// 
/// ## 示例 1
/// 
/// ```text
/// 输入：target = 7, nums = [2,3,1,2,4,3]
/// 输出：2
/// 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
/// ```
/// 
/// ## 示例 2
/// 
/// ```text
/// 输入：target = 4, nums = [1,4,4]
/// 输出：1
/// ```
pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut result = 0;

        let mut sum: i32 = 0;
        let mut j: usize = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            while sum >= target {
                let len = i - j + 1;
                if len < result || result == 0 {
                    result = len;
                }
                sum -= nums[j];
                j += 1;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
        assert_eq!(1, Solution::min_sub_array_len(4, vec![1, 4, 4]));
        assert_eq!(
            0,
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
        );
    }
}
