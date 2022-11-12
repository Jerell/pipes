use length::{Length, LengthUnits};

pub mod length;

#[cfg(test)]
mod tests {
    use super::*;

    mod length {
        use super::*;

        #[test]
        fn units() {
            let l0 = Length::new(3.0, LengthUnits::M);

            assert_eq!(l0.m(), 3.0);
            assert_eq!(l0.km(), 0.003);
            assert_eq!(l0.mm(), 3000.0);
            assert_eq!(l0.inch(), 118.11024);
        }

        #[test]
        fn pythag() {
            let l0 = Length::new(3.0, LengthUnits::M);
            let l1 = Length::new(4.0, LengthUnits::M);

            let diag = Length::pythag(l0, l1);

            assert_eq!(diag.m(), 5.0);
        }
    }
}
