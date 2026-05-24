namespace _941;

public static class Solution {
    public static bool ValidMountainArray(int[] mountain) {
        var n = mountain.Length;

        if (n < 3) {
            return false;
        }

        var i = 0;

        while (i + 1 < n && mountain[i] < mountain[i + 1]) {
            i += 1;
        }

        if (i == 0 || i == n - 1) {
            return false;
        }

        while (i + 1 < n && mountain[i] > mountain[i + 1]) {
            i += 1;
        }

        return i == n - 1;
    }
}