namespace _242;

public class Tests
{
    [Test]
    [TestCase("antagonist", "stagnation", true)]
    [TestCase("joao", "vitor", false)]
    [TestCase("aa", "a", false)]
    [TestCase("rat", "car", false)]
    [TestCase("a", "ab", false)]
    public void Leetcode(string a, string b, bool expected)
    {
        Assert.That(Solution.IsPermutation(a,b), Is.EqualTo(expected));
    }
}
