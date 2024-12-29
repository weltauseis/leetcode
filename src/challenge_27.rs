/*
27. Remove Element

Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

-   Change the array nums such that the first k elements of nums contain the elements which are not equal to val.
    The remaining elements of nums are not important as well as the size of nums.

-   Return k.
*/

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = *&nums.len();

    loop {
        // iterate through the array to find a match
        let mut found_match = false;
        for i in 0..k {
            // if we found one
            if nums[i] == val {
                found_match = true;
                // swap it with the last one
                let tmp = nums[k - 1];
                nums[k - 1] = nums[i];
                nums[i] = tmp;
                // decrement k bc don't need to care about that element anymore
                k = k - 1;
                break;
            }
        }

        // if no match was found, all work is done
        if !found_match {
            break;
        }
    }

    return k as i32;
}
