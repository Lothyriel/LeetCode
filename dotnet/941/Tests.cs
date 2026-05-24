namespace _941;

public class Tests
{
  [Test]
  [TestCase(new[] { 2, 1 }, ExpectedResult = false)]
  [TestCase(new[] { 3, 5, 5 }, ExpectedResult = false)]
  [TestCase(new[] { 0, 3, 2, 1 }, ExpectedResult = true)]
  [TestCase(new[] { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 }, ExpectedResult = false)]
  [TestCase(new[] { 9, 8, 7, 6, 5, 4, 3, 2, 1, 0 }, ExpectedResult = false)]
  [TestCase(new[] { 1, 7, 9, 5, 4, 1, 2 }, ExpectedResult = false)]
  [TestCase(new[] { 3, 7, 20, 14, 15, 14, 10, 8, 2, 1 }, ExpectedResult = false)]
  public bool ValidMountainArray(int [] arr)
  {
    return Solution.ValidMountainArray(arr);
  }
}