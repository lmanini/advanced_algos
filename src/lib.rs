mod sorting {

    pub fn insertion_sort(a: &mut [i128]) {
        for i in 1..a.len() {
            let mut j = i;
            while j > 0 && a[j - 1] > a[j] {
                a.swap(j - 1, j);
                j = j - 1;
            }
        }
    }

    pub fn merge_sort(a: &mut [i128]) {
        let len = a.len();
        if len <= 1 {
            return;
        }
        let half = a.len() / 2;

        merge_sort(&mut a[0..half]);
        merge_sort(&mut a[half..len]);

        let mut y: Vec<i128> = a.to_vec();

        merge(&a[0..half], &a[half..len], &mut y[..]);

        a.copy_from_slice(&y)
    }

    fn merge(a1: &[i128], a2: &[i128], y: &mut [i128]) {
        assert_eq!(a1.len() + a2.len(), y.len());
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < a1.len() && j < a2.len() {
            if a1[i] < a2[j] {
                y[k] = a1[i];
                i += 1;
                k += 1;
            } else {
                y[k] = a2[j];
                j += 1;
                k += 1;
            }
        }

        if i < a1.len() {
            y[k..].copy_from_slice(&a1[i..]);
        }
        if j < a2.len() {
            y[k..].copy_from_slice(&a2[j..]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting;

    #[test]
    fn test_insertion_sort() {
        let mut v: [i128; 10] = [23, 14, -43, 0, 54, -515, 2123, 32, 341, -999];
        let sorted_v = [-999, -515, -43, 0, 14, 23, 32, 54, 341, 2123];

        sorting::insertion_sort(&mut v);
        assert_eq!(v, sorted_v);
    }

    #[test]
    fn test_merge_sort() {
        let mut v: [i128; 10] = [23, 14, -43, 0, 54, -515, 2123, 32, 341, -999];
        let sorted_v = [-999, -515, -43, 0, 14, 23, 32, 54, 341, 2123];

        sorting::merge_sort(&mut v);
        assert_eq!(v, sorted_v);
    }
}
