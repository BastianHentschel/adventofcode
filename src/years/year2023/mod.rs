pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
#[cfg(test)]
mod tests {
   mod year2023 {
        crate::test_test_existence!(year2023, day01);
        crate::test_test_existence!(year2023, day02);
        crate::test_test_existence!(year2023, day03);
        crate::test_test_existence!(year2023, day04);
        crate::test_test_existence!(year2023, day05);
        crate::test_test_existence!(year2023, day06);
        crate::test_test_existence!(year2023, day07);
        crate::test_test_existence!(year2023, day08);
    }
}
