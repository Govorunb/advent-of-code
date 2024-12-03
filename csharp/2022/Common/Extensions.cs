using System.Collections;
using System.Diagnostics;
using System.Drawing;
using System.Text.RegularExpressions;

namespace AoC2022.Common; 
public static class CollectionsExtensions {
    public static void Add<T>(this ICollection<T> collection, params T[] items) {
        foreach (T item in items)
            collection.Add(item);
    }

    public static TValue GetOrAdd<TKey, TValue>(this IDictionary<TKey, TValue> dict, TKey key, TValue defaultValue) where TKey : notnull {
        if (key is null) throw new ArgumentNullException(nameof(key));
#pragma warning disable CS8600 // yes compiler, it's [MaybeNullWhen(false)]... but it's only used when it returns true
        if (dict.TryGetValue(key, out TValue value))
            return value;
#pragma warning restore CS8600
        dict.Add(key, defaultValue);
        return defaultValue;
    }

    public static TValue GetOrAdd<TKey, TValue>(this IDictionary<TKey, TValue> dict, TKey key, Func<TValue> valueFactory) where TKey : notnull
        => GetOrAdd(dict, key, (_) => valueFactory());

    public static TValue GetOrAdd<TKey, TValue>(this IDictionary<TKey, TValue> dict, TKey key, Func<TKey, TValue> valueFactory) where TKey : notnull {
#pragma warning disable CS8600 // yes compiler, it's [MaybeNullWhen(false)]... but it's only used when it returns true
        if (dict.TryGetValue(key, out TValue value))
            return value;
#pragma warning restore CS8600
        var producedValue = valueFactory(key);
        dict.TryAdd(key, producedValue);
        return dict[key];
    }

    public static TValue GetOrAdd<TKey, TArg, TValue>(this IDictionary<TKey, TValue> dict, TKey key, Func<TKey, TArg, TValue> valueFactory, TArg factoryArguments) where TKey : notnull {
#pragma warning disable CS8600 // yes compiler, it's [MaybeNullWhen(false)]... but it's only used when it returns true
        if (dict.TryGetValue(key, out TValue value))
            return value;
#pragma warning restore CS8600
        var producedValue = valueFactory(key, factoryArguments);
        dict.TryAdd(key, producedValue);
        return dict[key];
    }

    public static void AddRange<TKey, TValue>(this IDictionary<TKey, TValue> dict, IEnumerable<KeyValuePair<TKey, TValue>> pairs) {
        foreach (var pair in pairs)
            dict.Add(pair);
    }

    public static void AddRange<TKey, TValue>(this IDictionary<TKey, TValue> dict, IEnumerable<(TKey, TValue)> pairs) {
        foreach (var pair in pairs)
            dict.Add(pair.Item1, pair.Item2);
    }

    public static void AddRange<TKey, TValue>(this IDictionary<TKey, TValue> dict, IEnumerable<TKey> keys, IEnumerable<TValue> values)
        => AddRange(dict, keys.Zip(values));

    public static void CopyTo<TKey, TValue>(this IReadOnlyDictionary<TKey, TValue> source, IDictionary<TKey, TValue> destination) {
        destination.AddRange(source);
    }

    public static void AddRange<TItem>(this ICollection<TItem> collection, IEnumerable<TItem> items) {
        foreach (var item in items)
            collection.Add(item);
    }

    public static void AddRange<TItem>(this ICollection<TItem> collection, params TItem[] items)
        => AddRange(collection, items.AsEnumerable());
}

[DebuggerStepThrough]
public static class LinqExtensions {
    public static IEnumerable<TResult> ZipLongest<TFirst, TSecond, TResult>(
        this IEnumerable<TFirst> first,
        IEnumerable<TSecond> second,
        Func<TFirst?, TSecond?, TResult> func,
        TFirst? padder1 = default(TFirst),
        TSecond? padder2 = default(TSecond)) {
        var firstExp = first.Cast<TFirst?>().Concat(
            Enumerable.Repeat(
                padder1,
                Math.Max(second.Count() - first.Count(), 0)
            )
        );
        var secExp = second.Cast<TSecond?>().Concat(
            Enumerable.Repeat(
                padder2,
                Math.Max(first.Count() - second.Count(), 0)
            )
        );
        return firstExp.Zip(secExp, (a, b) => func(a, b));
    }

    public static IEnumerable<(TFirst? First, TSecond? Second)> ZipLongest<TFirst, TSecond>(
        this IEnumerable<TFirst> first,
        IEnumerable<TSecond> second
    ) => first.ZipLongest<TFirst, TSecond, (TFirst? First, TSecond? Second)>(second, (a, b) => (a, b));

    // https://stackoverflow.com/a/17977375
    public static IEnumerable<TResult> ZipMany<TSource, TResult>(this IEnumerable<IEnumerable<TSource>> source, Func<IEnumerable<TSource>, TResult> selector) {
        // ToList is necessary to avoid deferred execution
        var enumerators = source.Select(seq => seq.GetEnumerator()).ToList();
        try {
            while (true) {
                foreach (var e in enumerators) {
                    bool b = e.MoveNext();
                    if (!b)
                        yield break;
                }
                // Again, ToList (or ToArray) is necessary to avoid deferred execution
                yield return selector(enumerators.Select(e => e.Current).ToList());
            }
        }
        finally {
            foreach (var e in enumerators)
                e.Dispose();
        }
    }

    public static IEnumerable<IEnumerable<TSource>> ZipMany<TSource>(this IEnumerable<IEnumerable<TSource>> source)
        => ZipMany(source, (items) => items.ToList());

    public static bool AllEqual<T>(this IEnumerable<T> enumerable) {
        using (var enumerator = enumerable.GetEnumerator()) {
            T? first = default;
            if (enumerator.MoveNext())
                first = enumerator.Current;
            while (enumerator.MoveNext())
                if (!(first?.Equals(enumerator.Current) ?? false))
                    return false;
        }
        return true;
    }

    public static IEnumerable<T> GetRange<T>(this IEnumerable coll, Range range) {
        var castColl = coll.Cast<T>();
        var (start, count) = range.GetOffsetAndLength(castColl.Count());
        return castColl.Skip(start).Take(count);
    }

    public static async IAsyncEnumerable<T> ToAsyncEnumerable<T>(this IEnumerable<T> enumerable) {
        foreach (var item in enumerable) {
            yield return await Task.FromResult(item);
        }
    }

    public static IEnumerable<(T Element, int Index)> WithIndex<T>(this IEnumerable<T> enumerable)
        => enumerable.Select((elem, index) => ((T Element, int Index))(elem, index));

    /// <returns>An <see cref="IEnumerable{out T}"/> whose elements are the elements of every <see cref="IEnumerable{T}"/> in <paramref name="enumerable"/>.</returns>
    /// <inheritdoc cref="Enumerable.SelectMany"/>
    public static IEnumerable<T> Flatten<T>(this IEnumerable<IEnumerable<T>> enumerable)
        => enumerable.SelectMany(x => x);

    public static IEnumerable<T> Except<T>(this IEnumerable<T> enumerable, T element)
        => enumerable.Except(new[] { element });

    public static bool All(this IEnumerable<bool> enumerable)
        => enumerable.All(x => x);
}

public static class DrawingExtensions {
    public static Point OffsetFrom(this Point start, Point pt)
        => new(start.X - pt.X, start.Y - pt.Y);
}

public static class StringExtensions {
    public static readonly char[] DirectorySeparators = new char[] { Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar };
    public static readonly string DoubleNewLine = Environment.NewLine + Environment.NewLine;

    public static string[] SplitPath(this string path, int maxTimes = 0) {
        if (maxTimes > 0)
            return path.Split(DirectorySeparators, maxTimes, StringSplitOptions.RemoveEmptyEntries);
        else
            return path.Split(DirectorySeparators, StringSplitOptions.RemoveEmptyEntries);
    }

    public static string GetRelativePathTo(this string path, string folder) {
        // https://stackoverflow.com/a/703292
        return new Uri(folder).MakeRelativeUri(new Uri(path))
            .ToString()
            .Replace('/', Path.DirectorySeparatorChar);
    }
    public static int CountOf(this string @this, char value) {
        return @this.Sum(x => x == value ? 1 : 0);
    }
    public static bool Matches(this string @this, Regex pattern)
        => pattern.IsMatch(@this);
    public static bool Matches(this string @this, /* lang=regex*/ string regexPattern)
        => new Regex(regexPattern).IsMatch(@this);

    public static string[] Lines(this string @this)
        => @this.Split(Environment.NewLine);
}