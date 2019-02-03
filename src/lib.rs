pub fn add_float(a: f32, b: f32) -> f32 {
  return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_float(1.0, 2.1), 3.1);
    }
}
