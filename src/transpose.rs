use crate::common::*;

/// Transposition of nested container types.
pub trait Transpose {
    type Output;

    fn transpose(self) -> Self::Output;
}

impl<T> Transpose for Vec<Vec<T>> {
    type Output = Option<Vec<Vec<T>>>;

    fn transpose(self) -> Self::Output {
        let n_cols = match self.first() {
            Some(row) => row.len(),
            None => return Some(vec![]),
        };

        let mut iters: Vec<_> = self.into_iter().map(|row| row.into_iter()).collect();
        let cols: Option<Vec<_>> = (0..n_cols)
            .map(|_| {
                let col: Option<Vec<_>> = iters.iter_mut().map(|iter| iter.next()).collect();
                col
            })
            .collect();
        let cols = cols?;
        let ok = iters.iter_mut().all(|iter| iter.next().is_none());
        ok.then(|| cols)
    }
}

impl<T, E, F> Transpose for Result<Result<T, E>, F> {
    type Output = Result<Result<T, F>, E>;

    fn transpose(self) -> Self::Output {
        match self {
            Ok(Ok(item)) => Ok(Ok(item)),
            Ok(Err(err)) => Err(err),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<T> Transpose for Option<Option<T>> {
    type Output = Option<Option<T>>;

    fn transpose(self) -> Self::Output {
        match self {
            Some(Some(item)) => Some(Some(item)),
            Some(None) => None,
            None => Some(None),
        }
    }
}

impl<T, const X: usize, const Y: usize> Transpose for [[T; X]; Y] {
    type Output = [[T; Y]; X];

    fn transpose(self) -> Self::Output {
        let vecs: Vec<Vec<T>> = array::IntoIter::new(self)
            .map(|array| array::IntoIter::new(array).collect())
            .collect();

        let vecs: Result<Vec<[T; Y]>, _> = vecs
            .transpose()
            .unwrap()
            .into_iter()
            .map(TryInto::try_into)
            .collect();
        vecs.map_err(|_| ())
            .unwrap()
            .try_into()
            .map_err(|_| ())
            .unwrap()
    }
}

impl<T, E> Transpose for Vec<Result<T, E>> {
    type Output = Result<Vec<T>, E>;

    fn transpose(self) -> Self::Output {
        self.into_iter().collect()
    }
}

impl<T, E> Transpose for Result<Vec<T>, E> {
    type Output = Vec<Result<T, E>>;

    fn transpose(self) -> Self::Output {
        match self {
            Ok(vec) => vec.into_iter().map(Ok).collect(),
            Err(err) => vec![Err(err)],
        }
    }
}

impl<T> Transpose for Vec<Option<T>> {
    type Output = Option<Vec<T>>;

    fn transpose(self) -> Self::Output {
        self.into_iter().collect()
    }
}

impl<T> Transpose for Option<Vec<T>> {
    type Output = Vec<Option<T>>;

    fn transpose(self) -> Self::Output {
        match self {
            Some(vec) => vec.into_iter().map(Some).collect(),
            None => vec![None],
        }
    }
}

impl<K, L, V> Transpose for HashMap<K, HashMap<L, V>>
where
    K: Eq + Hash + Clone,
    L: Eq + Hash,
{
    type Output = HashMap<L, HashMap<K, V>>;

    fn transpose(self) -> Self::Output {
        self.into_iter()
            .flat_map(|(k1, map)| {
                map.into_iter()
                    .map(move |(k2, item)| (k1.clone(), k2, item))
            })
            .map(|(k1, k2, item)| (k2, (k1, item)))
            .into_grouping_map()
            .collect()
    }
}

impl<K, V> Transpose for HashMap<K, Vec<V>>
where
    K: Eq + Hash + Clone,
{
    type Output = Vec<HashMap<K, V>>;

    fn transpose(self) -> Self::Output {
        let mut iters: HashMap<K, _> = self
            .into_iter()
            .map(|(key, vec)| (key, vec.into_iter()))
            .collect();

        let mut vec = vec![];

        loop {
            let mut map = HashMap::new();

            iters.iter_mut().for_each(|(key, iter)| {
                if let Some(item) = iter.next() {
                    map.insert(key.clone(), item);
                }
            });

            if !map.is_empty() {
                vec.push(map);
            } else {
                break;
            }
        }

        vec
    }
}

impl<K, V> Transpose for Vec<HashMap<K, V>>
where
    K: Eq + Hash + Clone,
{
    type Output = Option<HashMap<K, Vec<V>>>;

    fn transpose(self) -> Self::Output {
        if self.is_empty() {
            return Some(HashMap::new());
        }

        let len = self.len();
        let mut output = HashMap::new();

        for (index, map) in self.into_iter().enumerate() {
            for (key, value) in map {
                let vec = match output.entry(key) {
                    hash_map::Entry::Occupied(entry) => entry.into_mut(),
                    hash_map::Entry::Vacant(entry) => entry.insert(vec![]),
                };

                if vec.len() != index {
                    return None;
                }

                vec.push(value);
            }
        }

        let ok = output.values().all(|vec| vec.len() == len);
        ok.then(|| output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_vec_vec() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6]].transpose(),
            Some(vec![vec![1, 4], vec![2, 5], vec![3, 6]])
        );
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5]].transpose(), None);
    }
}
