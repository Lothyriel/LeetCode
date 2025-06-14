namespace _22;

public class Solution
{
    public static IList<string> GenerateParenthesis(int n)
    {
        return new Generator { N = n }.GenerateParenthesis();
    }
}

public class Generator
{
    public required int N { get; set; }
    private readonly List<string> result = [];

    public IList<string> GenerateParenthesis()
    {
        Build("", 0, 0);

        return result;
    }

    private void Build(string cur, int opened, int closed)
    {
        if (cur.Length == 2 * N)
        {
            result.Add(cur);
            return;
        }

        if (opened < N)
        {
            Build(cur + '(', opened + 1, closed);
        }

        if (closed < opened)
        {
            Build(cur + ')', opened, closed + 1);
        }
    }
}
