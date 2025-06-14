namespace _415;

public class Tests
{
  [Test]
  [TestCase("11", "123", "134")]
  [TestCase("456", "77", "533")]
  [TestCase("0", "0", "0")]
  public void Test1(string num1, string num2, string expected)
  {
    Assert.That(Solution.AddStrings(num1, num2), Is.EqualTo(expected));
  }
}
