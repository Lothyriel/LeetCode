using FluentAssertions;
using NUnit.Framework;

namespace Pattern
{
    public class Tests
    {
        [Test]
        [TestCase("abba", "dog cat cat dog", true)]
        [TestCase("abba", "dog cat cat fish", false)]
        [TestCase("aaaa", "dog cat cat dog", false)]
        [TestCase("abba", "dog dog dog dog", false)]
        [TestCase("aaa", "aa aa aa aa", false)]
        public void Test(string pattern, string input, bool expectedResult)
        {
            var patternMatcher = new PatternMatching(pattern, input);
            var isMatch = patternMatcher.Matches();
            isMatch.Should().Be(expectedResult);
        }
    }
}