fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: Option<u32> = Some(0);
    for &num in nums {
        sum = sum.and_then(|acc| acc.checked_add(num));
    }
    sum
}

fn main() {
    let nums1 = [1, 2, 3, 4, 5];
    let nums2 = [std::u32::MAX, std::u32::MAX];

    println!("sum of {:?} is {:?}", nums1, sum_u32(&nums1));
    println!("sum of {:?} is {:?}", nums2, sum_u32(&nums2));
}
