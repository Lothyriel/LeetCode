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
        var freq = new Dictionary<char, int>();

        foreach (var letter in word)
        {
            freq[letter] = freq.GetValueOrDefault(letter) + 1;
        }

        return freq;
    }
}
