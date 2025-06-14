namespace _58;

public class Solution
{
  public static int LengthOfLastWord(string s)
  {
    var lastWord = s
        .Trim()
        .Split(' ')
        .LastOrDefault();

    return lastWord?.Length ?? 0;
  }
}
