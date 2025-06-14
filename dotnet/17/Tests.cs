namespace _17;

public class Tests
{
  [Test]
  public void Test_23()
  {
    List<string> expected = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];

    var result = Solution.LetterCombinations("23");

    Assert.That(result, Is.EqualTo(expected));
  }

  [Test]
  public void Test_empty()
  {
    List<string> expected = [];

    var result = Solution.LetterCombinations("");

    Assert.That(result, Is.EqualTo(expected));
  }
}
