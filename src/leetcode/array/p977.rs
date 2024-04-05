/// # [977.有序数组的平方](https://leetcode.cn/problems/squares-of-a-sorted-array/description/)
/// 
/// 给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
/// 
/// ## 示例 1
/// 
/// ```text
/// 输入：nums = [-4,-1,0,3,10]
/// 输出：[0,1,9,16,100]
/// 解释：平方后，数组变为 [16,1,0,9,100]
/// 排序后，数组变为 [0,1,9,16,100]
/// ```
/// 
/// ## 示例 2
/// 
/// ```text
/// 输入：nums = [-7,-3,2,3,11]
/// 输出：[4,9,9,49,121]
/// ```
pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return nums;
        }

        let mut result = Vec::with_capacity(nums.len());

        let mut letf_i: usize = 0;
        let mut right_i: usize = nums.len() - 1;

        while letf_i <= right_i {
            let val1 = nums[letf_i].pow(2);
            let val2 = nums[right_i].pow(2);
            if val1 >= val2 {
                result.push(val1);
                letf_i += 1;
            } else {
                result.push(val2);
                right_i -= 1;
            }
        }

        result.reverse();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
        assert_eq!(vec![1], Solution::sorted_squares(vec![1]));
    }
}
