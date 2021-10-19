// This macro helps us implement math operators on Vector3
// in such a way that it handles binary operators on any
// combination of Vec3, &Vec3 and f64.
#[macro_export]
macro_rules! impl_binary_operations {
    // $VectorType is something like `Vec3`
    // $Operation is something like `Add`
    // $op_fn is something like `add`
    // $op_symbol is something like `+`
    ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
      // Implement a + b where a and b are both of type &VectorType.
      // Lower down we'll implement cases where either a or b - or both
      // - are values by forwarding through to this implementation.
      impl<'a, 'b> $Operation<&'a $VectorType> for &'b $VectorType {
        type Output = $VectorType;
        fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
          $VectorType {
            x: self.x $op_symbol other.x,
            y: self.y $op_symbol other.y,
            z: self.z $op_symbol other.z,
          }
        }
      }

      // Implement a + b for the cases...
      //
      //   a: $VectorType,  b: &$VectorType
      //   a: &$VectorType, b: $VectorType
      //   a: $VectorType, b: $VectorType
      //
      // In each case we forward through to the implementation above.
      impl $Operation<$VectorType> for $VectorType {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self, other: $VectorType) -> $VectorType {
          &self $op_symbol &other
        }
      }

      impl<'a> $Operation<&'a $VectorType> for $VectorType {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
          &self $op_symbol other
        }
      }

      impl<'a> $Operation<$VectorType> for &'a $VectorType {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self, other: $VectorType) -> $VectorType {
          self $op_symbol &other
        }
      }

      // Implement a + b where a is type &$VectorType and b is type f64
      impl<'a> $Operation<f64> for &'a $VectorType {
        type Output = $VectorType;

        fn $op_fn(self, other: f64) -> $VectorType {
          $VectorType {
            x: self.x $op_symbol other,
            y: self.y $op_symbol other,
            z: self.z $op_symbol other
          }
        }
      }

      // Implement a + b where...
      //
      // a is $VectorType and b is f64
      // a is f64 and b is $VectorType
      // a is f64 and b is &$VectorType
      //
      // In each case we forward the logic to the implementation
      // above.
      impl $Operation<f64> for $VectorType {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self, other: f64) -> $VectorType {
          &self $op_symbol other
        }
      }

      impl $Operation<$VectorType> for f64 {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self, other: $VectorType) -> $VectorType {
          &other $op_symbol self
        }
      }

      impl<'a> $Operation<&'a $VectorType> for f64 {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
          other $op_symbol self
        }
      }
    };
  }

// It also implements unary operators like - a where a is of
// type Vec3 or &Vec3.
#[macro_export]
macro_rules! impl_unary_operations {
    // $VectorType is something like `Vec3`
    // $Operation is something like `Neg`
    // $op_fn is something like `neg`
    // $op_symbol is something like `-`
    ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {

      // Implement the unary operator for references
      impl<'a> $Operation for &'a $VectorType {
        type Output = $VectorType;

        fn $op_fn(self) -> $VectorType {
          $VectorType {
            x: $op_symbol self.x,
            y: $op_symbol self.y,
            z: $op_symbol self.z,
          }
        }
      }

      // Have the operator on values forward through to the implementation
      // above
      impl $Operation for $VectorType {
        type Output = $VectorType;

        #[inline]
        fn $op_fn(self) -> $VectorType {
          $op_symbol &self
        }
      }
    };
  }

// Implement add-assignment operators like a += b where a and
// b is either &Vec3 or Vec3 (in this case a is always of type
// &mut Vec3).
#[macro_export]
macro_rules! impl_op_assign {
    // $VectorType is something like `Vec3`
    // $OperationAssign is something like `AddAssign`
    // $op_fn is something like `add_assign`
    // $op_symbol is something like `+=`
    ($VectorType:ident $OperationAssign:ident $op_fn:ident $op_symbol:tt) => {
      // Implement $OperationAssign for RHS &Vec3
      impl<'a> $OperationAssign<&'a $VectorType> for $VectorType {
        fn $op_fn(&mut self, other: &'a $VectorType) {
          *self = $VectorType {
            x: self.x $op_symbol other.x,
            y: self.y $op_symbol other.y,
            z: self.z $op_symbol other.z,
          };
        }
      }

      // Implement $OperationAssign for RHS Vec3 by forwarding through to the
      // implementation above
      impl $OperationAssign for $VectorType {
        #[inline]
        fn $op_fn(&mut self, other: $VectorType) {
          *self = *self $op_symbol &other
        }
      }
    };
  }
