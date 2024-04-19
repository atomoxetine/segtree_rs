#[cfg(test)]
mod tests {
    use std::ops::Range;

    use rand::{rngs::StdRng, Rng, SeedableRng};
    use segtree_rs::StaticSegtree;

    #[test]
    fn static_segtree() {
        let mut rng: StdRng = StdRng::seed_from_u64(0);
        const N: usize = 1024;
        const M: usize = 16;
        const VAL_RANGE: Range<i32> = -512..512;
        const MERGE_FN: fn(&Vec<i32>, &Vec<i32>) -> Vec<i32> =
            |a, b| [a.to_owned(), b.to_owned()].concat();

        // generating non-commutative on sum dataset
        let mut data: Vec<Vec<i32>> = Vec::with_capacity(N);
        unsafe { data.set_len(N) };
        data.fill_with(|| {
            let mut inner = Vec::with_capacity(rng.gen_range(0..M));
            unsafe { inner.set_len(inner.capacity()) };
            inner.fill_with(|| rng.gen_range(VAL_RANGE));

            inner
        });

        let mut segtree = StaticSegtree::from_vec(&data, MERGE_FN, Vec::new());

        dbg!(&data);
        dbg!(&segtree);

        let data_query = data.iter().fold(Vec::new(), |a, b| MERGE_FN(&a, &b));
        let tree_query = segtree.query(0, N - 1);

        assert_eq!(data_query, tree_query);

        // perform N queries
        for _ in 0..N {
            if rng.gen_bool(0.5) {
                let index = rng.gen_range(0..N);
                let mut value = Vec::with_capacity(rng.gen_range(0..M));
                unsafe { value.set_len(value.capacity()) }
                value.fill_with(|| rng.gen_range(VAL_RANGE));

                data[index] = value.clone();
                segtree.set(index, value);

                assert_eq!(data[index], *segtree.get(index))
            } else {
                let l = rng.gen_range(0..N);
                let r = rng.gen_range(l..N);
                let data_query = data[l..=r]
                    .iter()
                    .fold(Vec::new(), |a, b| MERGE_FN(&a, &b));
                let tree_query = segtree.query(l, r);

                assert_eq!(data_query, tree_query);
            }
        }
    }
}
