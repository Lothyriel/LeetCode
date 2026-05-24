namespace _1508;

public class Tests
{
  [Test]
  [TestCase(new[] { 1,2,3,4 }, 4, 1, 5, ExpectedResult = 13)]
  [TestCase(new[] { 1,2,3,4 }, 4, 3, 4, ExpectedResult = 6)]
  [TestCase(new[] { 1,2,3,4 }, 4, 1, 10, ExpectedResult = 50)]
  public int TwoSum(int [] nums, int n, int left, int right)
  {
    return Solution.RangeSum(nums, n, left, right);
  }
}