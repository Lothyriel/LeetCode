namespace TwoSum
{
    public class Tests
    {
        [Test]
        [TestCase(new int[] { 2, 7, 11, 15 }, 9, ExpectedResult = new int[] { 0, 1 })]
        [TestCase(new int[] { 3, 2, 4 }, 6, ExpectedResult = new int[] { 1, 2 })]
        [TestCase(new int[] { 3, 3 }, 6, ExpectedResult = new int[] { 0, 1 })]
        [TestCase(new int[] { 2, 5, 5, 11 }, 10, ExpectedResult = new int[] { 1, 2 })]
        public int[] TwoSum(int[] nums, int target)
        {
            for (int i = 0; i < nums.Length - 1; i++) 
            {
                for (int j = i + 1; j < nums.Length; j++)
                {
                    if (nums[i] + nums[j] == target)
                    {
                        return new int[] { i, j };
                    }
                }
            }
            return Array.Empty<int>();
        }
    }
}