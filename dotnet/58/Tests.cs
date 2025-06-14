namespace _58;

public class Tests
{
  [Test]
  [TestCase("Hello World", 5)]
  [TestCase("   fly me   to   the moon  ", 4)]
  [TestCase("luffy is still joyboy", 6)]
  public void ExampleInputs(string input, int expected)
  {
    Assert.That(Solution.LengthOfLastWord(input), Is.EqualTo(expected));
  }
}
