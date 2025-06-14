namespace _17;

public class Solution
{
  public static IList<string> LetterCombinations(string digits)
  {
    var digitsLetters = digits.Select(d => digitToLettersMapping[d]);

    var first = digitsLetters
      .FirstOrDefault()?
      .Select(l => $"{l}")
      .ToList() ?? [];

    return digitsLetters.Skip(1)
      .Aggregate(first, Accumulator);
  }

  private static List<string> Accumulator(List<string> combinations, char[] letters)
  {
    return combinations
      .SelectMany(previous => letters.Select(letter => previous + letter))
      .ToList();
  }

  private static readonly Dictionary<char, char[]> digitToLettersMapping = new()
  {
    ['2'] = ['a', 'b', 'c'],
    ['3'] = ['d', 'e', 'f'],
    ['4'] = ['g', 'h', 'i'],
    ['5'] = ['j', 'k', 'l'],
    ['6'] = ['m', 'n', 'o'],
    ['7'] = ['p', 'q', 'r', 's'],
    ['8'] = ['t', 'u', 'v'],
    ['9'] = ['w', 'x', 'y', 'z'],
  };
}
