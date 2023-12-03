pub mod day01;
pub mod day02;
#[cfg(test)]
mod tests {
   mod year2023 {
        crate::test_test_existence!(year2023, day01);
        crate::test_test_existence!(year2023, day02);
    }
}
