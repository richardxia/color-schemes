/// Helper methods for nalgebra traits.
use nalgebra::{Scalar, Vector3};

pub trait Vector3OptionExt<T>
where
    T: Scalar,
{
    fn sequence_option(self) -> Option<Vector3<T>>;
}

impl<T> Vector3OptionExt<T> for Vector3<Option<T>>
where
    T: Scalar,
{
    fn sequence_option(self) -> Option<Vector3<T>> {
        let opt0 = self[0];
        let opt1 = self[1];
        let opt2 = self[2];
        match (opt0, opt1, opt2) {
            (Some(t0), Some(t1), Some(t2)) => Some(Vector3::new(t0, t1, t2)),
            _ => None,
        }
    }
}

pub trait Vector3ResultExt<T, E>
where
    T: Scalar,
    E: Scalar,
{
    fn sequence_result(self) -> Result<Vector3<T>, E>;
}

impl<T, E> Vector3ResultExt<T, E> for Vector3<Result<T, E>>
where
    T: Scalar,
    E: Scalar,
{
    fn sequence_result(self) -> Result<Vector3<T>, E> {
        let res0 = self[0];
        let res1 = self[1];
        let res2 = self[2];
        match (res0, res1, res2) {
            (Ok(t0), Ok(t1), Ok(t2)) => Ok(Vector3::new(t0, t1, t2)),
            (Err(e), _, _) => Err(e),
            (_, Err(e), _) => Err(e),
            (_, _, Err(e)) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_option_some() {
        assert_eq!(
            Vector3::new(Some(1), Some(2), Some(3)).sequence_option(),
            Some(Vector3::new(1, 2, 3))
        )
    }

    #[test]
    fn test_sequence_option_none() {
        assert_eq!(Vector3::new(Some(1), None, Some(3)).sequence_option(), None)
    }

    #[test]
    fn test_sequence_result_ok() {
        assert_eq!(
            Vector3::<Result<u8, ()>>::new(Ok(1), Ok(2), Ok(3)).sequence_result(),
            Ok(Vector3::new(1, 2, 3))
        )
    }

    #[test]
    fn test_sequence_result_err_multiple() {
        assert_eq!(
            Vector3::new(Ok(1), Err(2), Err(3)).sequence_result(),
            Err(2)
        )
    }
}
