namespace _706;

public class HashTable<TKey, TValue> where TKey : IHashable<TKey>
{
    private List<List<(TKey Key, TValue Value, long Hash)>> _items = Enumerable.Range(0, 8)
        .Select(_ => new List<(TKey, TValue, long)>())
        .ToList();

    private int _count;
    
    private const float LoadFactor = 0.75f;

    public bool TryAdd(TKey key, TValue value)
    {
        if ((float)_count / _items.Count > LoadFactor)
        {
            Grow();
        }
        
        var (bucketIdx, hash) = Hash(key);
        var bucket = _items[bucketIdx];

        if (bucket.Any(e => e.Key.Compare(key)))
        {
            return false;
        }

        bucket.Add((key, value, hash));
        _count++;
        return true;
    }

    public bool TryGet(TKey key, out TValue? value)
    {
        var (bucketIdx, _) = Hash(key);
        var bucket = _items[bucketIdx];
        value = default;
        
        var idx = bucket.FindIndex(e => e.Key.Compare(key));

        if (idx is -1)
        {
            return false;
        }

        value = bucket[idx].Value;
        return true;
    }

    public TValue? Remove(TKey key)
    {
        var (bucketIdx, _) = Hash(key);
        var bucket = _items[bucketIdx];
        
        var idx = bucket.FindIndex(e => e.Key.Compare(key));

        if (idx is -1)
        {
            return default;
        }

        var entry = bucket[idx];

        // swap remove from bucket to avoid shifting elements
        var lastIdx = bucket.Count - 1;
        bucket[idx] = bucket[lastIdx];
        bucket.RemoveAt(lastIdx);

        _count--;
        
        return entry.Value;
    }

    private (int, long) Hash(TKey key)
    {
        var hash = key.Hash();
        var idx = hash % _items.Count;

        return ((int)idx, hash);
    }

    private void Grow()
    {
        var temp = Enumerable.Range(0, _items.Count * 2)
            .Select(_ => new List<(TKey, TValue, long)>())
            .ToList();

        foreach (var bucket in _items)
        {
            foreach (var entry in bucket)
            {
                var idx = entry.Hash % temp.Count;

                temp[(int)idx].Add(entry);
            }
        }

        _items = temp;
    }
}

public interface IHashable<in T>
{
    long Hash();
    bool Compare(T other);
}