namespace _242;

public class Solution
{
    public static bool IsPermutation(string a, string b)
    {
        var aFreq = LettersFrequency(a);
        var bFreq = LettersFrequency(b);

        foreach (var (letter, freq) in aFreq)
        {
            if (bFreq.TryGetValue(letter, out var bLetterFreq))
            {
                if (bLetterFreq != freq)
                {
                    return false;
                }
            }

            return false;
        }

        return true;
    }

    private static Dictionary<char, int> LettersFrequency(string word)
    {
        var freq = new Dictionary<char, int>();

        foreach (var letter in word)
        {
            var letterFrequency = freq.GetValueOrDefault(letter);
            letterFrequency++;
        }

        return freq;
    }
}
