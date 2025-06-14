namespace _22;

public class Tests
{
  [Test]
  public void Test_1()
  {
    List<string> expected = ["()"];

    var result = Solution.GenerateParenthesis(1);

    Assert.That(result, Is.EqualTo(expected));
  }

  [Test]
  public void Test_3()
  {
    List<string> expected = ["((()))","(()())","(())()","()(())","()()()"];

    var result = Solution.GenerateParenthesis(3);

    Assert.That(result, Is.EqualTo(expected));
  }
}
