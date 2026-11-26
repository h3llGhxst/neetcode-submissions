
impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {


        points.select_nth_unstable_by_key(k as usize - 1, |p| p[0] * p[0] + p[1] * p[1]);
points.truncate(k as usize);

        points

    }

}