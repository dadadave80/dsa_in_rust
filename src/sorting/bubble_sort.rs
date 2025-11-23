pub fn _bubble_sort(mut nums: Vec<u16>) -> Vec<u16> {
    let mut swapping = true;
    let mut end = nums.len();
    while swapping {
        swapping = false;
        for i in 1..end {
            if nums[i - 1] > nums[i] {
                (nums[i - 1], nums[i]) = (nums[i], nums[i - 1]);
                swapping = true;
            }
        }
        end -= 1;
    }
    nums
}

#[cfg(test)]
mod bubble_sort_test {
    use super::*;

    #[test]
    fn sort_first() {
        let input: Vec<u16> = vec![5, 7, 3, 6, 8];
        let expected: Vec<u16> = vec![3, 5, 6, 7, 8];

        assert_eq!(expected, _bubble_sort(input));
    }

    #[test]
    fn sort_second() {
        let input: Vec<u16> = vec![8, 100, 250, 99, 63, 510];
        let expected: Vec<u16> = vec![8, 63, 99, 100, 250, 510];

        assert_eq!(expected, _bubble_sort(input));
    }

    #[test]
    fn sort_fail() {
        let input: Vec<u16> = vec![2, 1];
        let expected: Vec<u16> = vec![1, 2];

        assert_eq!(expected, _bubble_sort(input));
    }
}
