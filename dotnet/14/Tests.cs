namespace _14
{
  public class Tests
  {
    [Test]
    public void Test1()
    {
      Assert.That(FindPrefix(["flower", "flow", "flight"]), Is.EqualTo("fl"));
    }

    private static string FindPrefix(string[] palavras) => palavras.First().Aggregate("", (biggest, next) => palavras.All(palavra => palavra.StartsWith(biggest)) ? biggest += next : biggest)[..^1];
  }
}
