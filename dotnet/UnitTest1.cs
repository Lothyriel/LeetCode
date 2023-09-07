namespace TestProject1
{
    public class Tests
    {
        [SetUp]
        public void Setup()
        {
        }

        [Test]
        public void Test1()
        {
            Assert.AreEqual("fl", AcharPrefix(new string[] { "flower", "flow", "flight" }));
        }

        private static string AcharPrefix(string[] palavras) => palavras.First().Aggregate("", (biggest, next) => palavras.All(palavra => palavra.StartsWith(biggest)) ? biggest += next : biggest)[..^1];
    }
}