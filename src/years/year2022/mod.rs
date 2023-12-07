pub mod day01;
pub mod day03;
pub mod day02;
pub mod day05;
pub mod day04;
#[cfg(test)]
mod tests {
   mod year2022 {
        crate::test_test_existence!(year2022, day01);
        crate::test_test_existence!(year2022, day03);
        crate::test_test_existence!(year2022, day02);
        crate::test_test_existence!(year2022, day05);
        crate::test_test_existence!(year2022, day04);
    }
}
