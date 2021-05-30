use crate::common::*;

/// The operation that flattens nested container types.
pub trait Flatten {
    type Output;

    fn flatten(self) -> Self::Output;
}

impl<T> Flatten for Vec<Vec<T>> {
    type Output = Vec<T>;

    fn flatten(self) -> Self::Output {
        self.into_iter().flatten().collect()
    }
}

impl<T> Flatten for Vec<Option<T>> {
    type Output = Vec<T>;

    fn flatten(self) -> Self::Output {
        self.into_iter().flatten().collect()
    }
}

impl<K, L, V> Flatten for HashMap<K, HashMap<L, V>>
where
    K: Eq + Hash + Clone,
    L: Eq + Hash,
{
    type Output = HashMap<(K, L), V>;

    fn flatten(self) -> Self::Output {
        self.into_iter()
            .flat_map(|(k1, map)| {
                map.into_iter()
                    .map(move |(k2, val)| ((k1.clone(), k2), val))
            })
            .collect()
    }
}

impl<K, V> Flatten for HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
{
    type Output = HashMap<(K, usize), V>;

    fn flatten(self) -> Self::Output {
        self.into_iter()
            .flat_map(|(key, vec)| {
                vec.into_iter()
                    .enumerate()
                    .map(move |(index, value)| ((key.clone(), index), value))
            })
            .collect()
    }
}

impl<K, V> Flatten for Vec<HashMap<K, V>>
where
    K: Eq + Hash + Clone,
{
    type Output = HashMap<(usize, K), V>;

    fn flatten(self) -> Self::Output {
        self.into_iter()
            .enumerate()
            .flat_map(|(index, map)| {
                map.into_iter()
                    .map(move |(key, value)| ((index, key), value))
            })
            .collect()
    }
}
