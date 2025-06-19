namespace _242;

public class Solution
{
    public static bool IsPermutation(string a, string b)
    {
        var aFreq = LettersFrequency(a);
        var bFreq = LettersFrequency(b);

        return aFreq.Count == bFreq.Count && !aFreq.Except(bFreq).Any();
    }

    private static Dictionary<char, int> LettersFrequency(string word)
    {
        return word.Aggregate(new Dictionary<char, int>(), Accumulator);

        static Dictionary<char, int> Accumulator(Dictionary<char, int> acc, char letter)
        {
            acc[letter] = acc.GetValueOrDefault(letter) + 1;
            return acc;
        }
    }
}
