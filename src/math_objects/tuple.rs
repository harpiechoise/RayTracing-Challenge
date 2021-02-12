use crate::math_objects::utils::equal;
use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(PartialEq, Debug, Clone, Copy)]
enum TupleTypes {
    Vector,
    Point,
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    tuple_type: TupleTypes // Tuple Tupe depends of w-value
}




// Return a Tuple with w = 1.0
pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

// Return a Tuple with w = 0.0
pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

// Return a tuple type private
fn tuple(x: i32, y: i32, z: i32, w: i32) -> Tuple {
    Tuple::new(x as f64,
               y as f64,
               z as f64,
               w as f64)
}

// Check equality for tuple
impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let x = equal(self.x, other.x);
        let y = equal(self.y, other.y);
        let z = equal(self.z, other.z);
        let w = equal(self.w, other.w);
        x && y && z && w
    }
}
// Add two tuples
impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Tuple::new(self.x + other.x, 
                   self.y + other.y,
                   self.z + other.z,
                   self.w + other.w)
    }
}

// Subtract two tuples
impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Tuple::new(self.x - other.x,
                   self.y - other.y,
                   self.z - other.z,
                   self.w - other.w)
    }
}


// Negation
impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Tuple::new(-self.x, 
                   -self.y, 
                   -self.z, 
                   -self.w)
    }
}

// The basic function to construct a new tuple
impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w,
            tuple_type: Self::decide_type(w)
        }
    }

    fn decide_type(w: f64) -> TupleTypes {
        // w = 0: vector
        // w = 1: Tuple
        match equal(w, 1.0) {
            true  => TupleTypes::Point,
            false => TupleTypes::Vector
        }
    }

    pub fn scalar_multiplication(&self, value: f64) -> Tuple {
        Tuple::new(self.x * value, 
                   self.y * value, 
                   self.z * value, 
                   self.w * value)
    }

    pub fn scalar_division(&self, value: f64) -> Tuple {
        Tuple::new(self.x / value, 
                   self.y / value, 
                   self.z / value, 
                   self.w / value)
    }

    pub fn magnitude(&self) -> f64 {
        let x = self.x.powf(2.0);
        let y = self.y.powf(2.0);
        let z = self.z.powf(2.0);
        let w = self.w.powf(2.0);
        (x + y + z + w).sqrt()
    }

    pub fn int_magnitude(&self) -> i64 {
        let x = self.x.powi(2);
        let y = self.y.powi(2);
        let z = self.z.powi(2);
        let w = self.w.powi(2);
        (x + y + z + w).sqrt() as i64
    }

    pub fn normalize(&self) -> Self {
        let x = self.x / self.magnitude();
        let y = self.y / self.magnitude();
        let z = self.z / self.magnitude();
        let w = self.w / self.magnitude();
        Tuple::new(x, y, z, w)
    }

    pub fn dot(&self, other: &Self) -> Option<f64> {
        if self.tuple_type == TupleTypes::Point {
            None
        } else {
            let x = self.x * other.x;
            let y = self.y * other.y;
            let z = self.z * other.z;
            // w is 0
            Some(x + y + z + 0.0)
        }
    }

    pub fn cross(&self, other: &Self) -> Option<Tuple> {
        if self.tuple_type == TupleTypes::Point {
            None
        } else {
            let x = self.y * other.z - self.z * other.y;
            let y = self.z * other.x - self.x * other.z;
            let z = self.x * other.y - self.y * other.x;
            Some(vector(x, y, z))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn creation_make_point() {
        let v = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 1.0);
        assert_eq!(v.tuple_type, TupleTypes::Point)
    }

    #[test]
    fn creation_make_vector() {
        let v = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0);
        assert_eq!(v.tuple_type, TupleTypes::Vector)
    }

    #[test]
    fn creation_factory_point() {
        let v = point(4.0, -4.0, 3.0);
        let v1 = Tuple::new(4.0, -4.0, 3.0, 1.0);
        assert_eq!(v, v1)
    }

    #[test]
    fn creation_factory_vector() {
        let v = vector(4.0, -4.0, 3.0);
        let v1 = Tuple::new(4.0, -4.0, 3.0, 0.0);
        assert_eq!(v, v1)
    }
    #[test]
    fn operator_add_tuple() {
        let v = tuple(3, -2, 5, 1);
        let z = tuple(-2, 3, 1, 0);
        let res = tuple(1, 1, 6, 1);
        assert_eq!(v + z, res);
    }

    #[test]
    fn operator_subtract_two_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let res = vector(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, res);
    }

    #[test]
    fn operator_subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let res = point(-2.0, -4.0, -6.0); 
        assert_eq!(p - v, res);
    }

    #[test]
    fn operator_subtract_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let res = vector(-2.0, -4.0, -6.0);
        assert_eq!(v1 - v2, res);
    }

    #[test]
    fn operator_subtract_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        let res = vector(-1.0, 2.0, -3.0);
        assert_eq!(zero - v, res)
    }

    #[test]
    fn operator_tuple_negation() {
        let a = tuple(1, -2, 3, -4);
        let res = tuple(-1, 2, -3, 4);
        assert_eq!(-a, res)
    }
    
    #[test]
    fn operation_tuple_multiplication() {
        let a = tuple(1, -2, 3, -4);
        let res = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert_eq!(a.scalar_multiplication(3.5), res)
    }
    
    #[test]
    fn operation_tuple_division() {
        let a = tuple(1, -2, 3, -4);
        let res = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(a.scalar_division(2.0), res)
    }

    #[test]
    fn operation_magnitude() {
        let a = vector(0.0, 1.0, 0.0);
        assert_eq!(a.magnitude(), 1.0);
        
        let a = vector(1.0, 0.0, 0.0);
        assert_eq!(a.magnitude(), 1.0);
        
        let a = vector(0.0, 0.0, 1.0);
        assert_eq!(a.magnitude(), 1.0);
        
        let a = vector(-1.0, -2.0, -3.0);
        let res = equal(a.magnitude(), (14 as f64).sqrt());
        assert_eq!(res, true);
    }

    #[test]
    fn operation_int_magnitude() {
        let a = vector(0.0, 1.0, 0.0);
        assert_eq!(a.int_magnitude(), 1);
        
        let a = vector(1.0, 0.0, 0.0);
        assert_eq!(a.int_magnitude(), 1);
        
        let a = vector(0.0, 0.0, 1.0);
        assert_eq!(a.int_magnitude(), 1);
        
        let a = vector(-1.0, -2.0, -3.0);
        assert_eq!(a.int_magnitude(), (14 as f64).sqrt() as i64);
    }

    #[test]
    fn operation_normalize() {
        let v = vector(1.0, 2.0, 3.0);
        let n = v.normalize();
        assert!(equal(n.magnitude(), 1.0))
    }

    #[test]
    fn operation_dot_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let res = equal(a.dot(&b).unwrap(), 20.0);
        assert!(res)
    }

    #[test]
    fn operation_cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let res = vector(-1.0, 2.0, -1.0);
        let res2 = vector(1.0, -2.0, 1.0);

        assert_eq!(a.cross(&b), Some(res));
        assert_eq!(b.cross(&a), Some(res2));
    }
}