using FluentAssertions;

namespace TestProject1
{
    public class Tests
    {
        [Test]
        public void Test1()
        {
            "fl".Should().Be(AcharPrefix(new string[] { "flower", "flow", "flight" }));
        }

        private static string AcharPrefix(string[] palavras) => palavras.First().Aggregate("", (biggest, next) => palavras.All(palavra => palavra.StartsWith(biggest)) ? biggest += next : biggest)[..^1];
    }
}