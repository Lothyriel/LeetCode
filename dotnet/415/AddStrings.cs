using System.Text;

public class Solution
{
  public static string AddStrings(string num1, string num2)
  {
    (num1, num2) = Pad(num1, num2);

    var carry = 0;

    var result = new StringBuilder();

    foreach (var (first, second) in num1.Reverse().Zip(num2.Reverse()))
    {
      var digit = Convert.ToInt16(first - 48) + Convert.ToInt16(second - 48) + carry;

      carry = digit / 10;

      result.Insert(0, $"{digit % 10}");
    }

    if (carry != 0)
    {
      result.Insert(0, $"{carry}");
    }

    return result.ToString();
  }

  public static (string, string) Pad(string num1, string num2)
  {
    var longest = num1.Length > num2.Length ? num1 : num2;

    return (num1.PadLeft(longest.Length, '0'), num2.PadLeft(longest.Length, '0'));
  }
}
