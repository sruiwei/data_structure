/// # [27. 移除元素](https://leetcode.cn/problems/remove-element/description/)
///
/// 给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
///
/// 不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并 原地 修改输入数组。
///
/// 元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
///
/// ## 示例 1
///
/// ```text
/// 输入：nums = [3,2,2,3], val = 3
/// 输出：2, nums = [2,2]
/// 解释：函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。你不需要考虑数组中超出新长度后面的元素。例如，函数返回的新长度为 2 ，而 nums = [2,/// 2,3,3] 或 nums = [2,2,0,0]，也会被视作正确答案。
/// ```
///
/// ## 示例 2
///
/// ```text
/// 输入：nums = [0,1,2,2,3,0,4,2], val = 2
/// 输出：5, nums = [0,1,3,0,4]
/// 解释：函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。注意这五个元素可为任意顺序。你不需要考虑数组中超出新长度后面的元素。
/// ```
pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow_i: usize = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[slow_i] = nums[i];
                slow_i += 1;
            }
        }
        return slow_i as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_element() {
        let mut a = vec![3, 2, 2, 3];
        assert_eq!(2, Solution::remove_element(&mut a, 3));
        assert_eq!(
            5,
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2)
        );
    }
}
