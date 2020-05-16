use std::collections::BTreeMap;

pub fn prefix_map<K, V>(prefix: K, map: BTreeMap<Vec<K>, V>) -> BTreeMap<Vec<K>, V>
where
    K: Ord + Copy,
{
    map.into_iter()
        .map(|(mut key, value)| {
            key.insert(0, prefix);
            (key, value)
        })
        .collect()
}

pub fn simplify_map<K, V>(map: BTreeMap<Vec<K>, V>) -> BTreeMap<Vec<K>, V>
where
    K: Ord + Clone,
{
    if map.keys().map(|v| v.len()).max().unwrap_or(0) <= 1 {
        return map;
    }
    let mut aggregated_map: BTreeMap<Vec<K>, Vec<(Vec<K>, V)>> = BTreeMap::new();
    for entry in map.into_iter() {
        let mut key = entry.0.clone();
        if key.len() > 1 {
            key.remove(0);
        }
        aggregated_map.entry(key).or_default().push(entry)
    }

    aggregated_map
        .into_iter()
        .flat_map(|(_key, entries)| {
            if entries.len() <= 1 {
                entries
                    .into_iter()
                    .map(|(mut key, value)| {
                        if key.len() > 1 {
                            key.remove(0);
                        }
                        (key, value)
                    })
                    .collect()
            } else {
                entries
            }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    macro_rules! map {
        ($($key:expr => $value:expr),* $(,)?) => {
            vec![$( ($key, $value), )*].into_iter().collect()
        };
    }

    #[test]
    fn test_simplify_map_flat_case() {
        let map: BTreeMap<_, u8> = map!(
            vec!['a'] => 1,
            vec!['b'] => 2,
            vec!['c'] => 3,
            vec!['d'] => 4,
        );
        assert_eq!(simplify_map(map.clone()), map)
    }

    #[test]
    fn test_simplify_map_simple_case() {
        let map: BTreeMap<_, u8> = map!(
            vec!['a'] => 1,
            vec!['a', 'b'] => 2,
            vec!['a', 'c'] => 3,
            vec!['a', 'd'] => 4,
        );
        assert_eq!(
            simplify_map(map.clone()),
            map!(
                vec!['a'] => 1,
                vec!['b'] => 2,
                vec!['c'] => 3,
                vec!['d'] => 4,
            )
        )
    }
    #[test]
    fn test_simplify_map_complex_case() {
        let map: BTreeMap<_, u8> = map!(
            vec!['a'] => 1,
            vec!['a', 'b'] => 2,
            vec!['c', 'b'] => 3,
            vec!['d', 'b'] => 4,
        );
        assert_eq!(simplify_map(map.clone()), map)
    }
    #[test]
    #[ignore]
    fn test_simplify_map_complex_simplify_case() {
        let map: BTreeMap<_, u8> = map!(
            vec!['a'] => 1,
            vec!['a', 'x', 'y', 'b'] => 2,
            vec!['c', 'x', 'y', 'b'] => 3,
            vec!['d', 'x', 'y', 'b'] => 4,
        );
        assert_eq!(
            simplify_map(map),
            map!(
                vec!['a'] => 1,
                vec!['a', 'b'] => 2,
                vec!['c', 'b'] => 3,
                vec!['d', 'b'] => 4,
            )
        )
    }
    #[test]
    #[ignore]
    fn test_simplify_map_complex_simplify_case2() {
        let map: BTreeMap<_, u8> = map!(
            vec!['a'] => 1,
            vec!['a', 'x', 'y', 'b'] => 2,
            vec!['c', 'x', 'y', 'b'] => 3,
            vec!['d', 'y', 'x', 'b'] => 4,
            vec!['e', 'y', 'x', 'b'] => 5,
        );
        assert_eq!(
            simplify_map(map),
            map!(
                vec!['a'] => 1,
                vec!['a', 'b'] => 2,
                vec!['c', 'b'] => 3,
                vec!['d', 'b'] => 4,
                vec!['e', 'b'] => 5,
            )
        )
    }
    #[test]
    #[ignore]
    fn test_simplify_map_complex_simplify_later_elements_case() {
        let map: BTreeMap<_, u8> = map!(
            vec!['a'] => 1,
            vec!['a', 'x', 'y', 'b'] => 2,
            vec!['c', 'x', 'y', 'b'] => 3,
            vec!['a', 'y', 'x', 'b'] => 4,
            vec!['c', 'y', 'x', 'b'] => 5,
        );
        assert_eq!(
            simplify_map(map),
            map!(
                vec!['a'] => 1,
                vec!['a', 'y', 'b'] => 2,
                vec!['c', 'y', 'b'] => 3,
                vec!['a', 'x', 'b'] => 4,
                vec!['c', 'x', 'b'] => 5,
            )
        )
    }
}
