namespace _1508;

public static class Solution {
    public static int RangeSum(int[] nums, int n, int left, int right) {
        List<int> rangeSum = [];

        for (var i = 0; i < n; i++)
        {
            for (var j = i + 1; j <= n; j++)
            {
                var range = nums[i..j];
                var sum = range.Sum();
                rangeSum.Add(sum);
            }
        }

        rangeSum.Sort();
        
        return rangeSum.Slice(left - 1, right - left + 1).Aggregate((acc, x) => (acc + x) % 1_000_000_007);
    }
}