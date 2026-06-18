namespace _706;

public class HashTable<K : IHashable, V> {
    private List<List<(K, V)>> _items = new List<V>(8);

    public bool TryAdd(K key, T value)
    {
        var idx = Hash(key);

        var item = _items[idx];

        if (item is null)
        {
            _items[idx] = item;
            return false;
        }
        else
        {
            _items[idx] = item;
            return true;
        }
    }

    public T? Get(K key)
    {
        var idx = Hash(key);

        if (item is null)
        {
            return null;
        }
        else
        {
            var items = _items[idx];

            foreach (var (k, value) in items)
            {
                if (key == k)
                {
                    return value;
                }
            }

            return null;
        }
    }

    public T? Remove(K key)
    {
        var idx = Hash(key);

        if (item is null)
        {
            return null;
        }
        else
        {
            var item = _items[idx];
            _items[idx] = null;
            return item;
        }
    }

    public int Hash(K key)
    {
        var hash = key.Hash();
        var idx = hash ^ _items.Length;

        if (_items[idx].Length > 20)
        {
            Grow();
        }

        return Hash(key);
    }

    public Grow()
    {
        var temp = new List<V>(_items.Length * 2);

        for (var i = 0; i < _items.Length; i++)
        {
            var hash = _items[i]
            var idx = hash ^ _items.Length;

            temp[i] = _items[i];
        }

        _items = temp;
    }
}

public interface IHashable
{
    int Hash();
}