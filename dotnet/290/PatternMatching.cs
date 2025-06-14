namespace _290;

public class PatternMatching
{
  public PatternMatching(string pattern, string words)
  {
    Pattern = pattern;
    Words = words.Split(" ");
  }

  public bool Matches()
  {
    if (Pattern.Length != Words.Length)
    {
      return false;
    }

    var tupleList = InitTuples();
    var dict = new Dictionary<char, string>();

    foreach (var item in tupleList)
    {
      if (dict.TryGetValue(item.Item1, out var _))
      {
        if (!(dict[item.Item1] == item.Item2))
          return false;
      }
      else
        dict.Add(item.Item1, item.Item2);
    }
    return dict.GroupBy(e => e.Key).Count() == dict.GroupBy(e => e.Value).Count();
  }

  private string Pattern { get; }
  private string[] Words { get; }

  private List<(char, string)> InitTuples()
  {
    return Pattern.Zip(Words, (patternLetter, word) => (patternLetter, word)).ToList();
  }
}
