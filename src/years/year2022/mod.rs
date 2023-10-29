pub mod day01;
pub mod day02;
#[cfg(test)]
mod tests {
   mod year2022 {
        crate::test_test_existence!(year2022, day01);
        crate::test_test_existence!(year2022, day02);
    }
}
