use custom_errors::invalid_argument_error::InvalidArgumentError;
use std::cmp::Ordering;
use std::fmt;   


/// A representation of an N-dimensional point
pub struct NDPoint{
    coords: Vec<f32>,
    dim: i32
}

/// Base functions for the NDPoint
impl NDPoint{

    /// Create a new N-Dimensional point at the origin
    ///
    /// # Arguments
    /// * `dim` - The dimensionality of the point
    ///
    /// # Example
    /// 
    /// ```
    /// # use kyles_algorithm_crate::base::NDPoint;
    /// // creates a new 3-dimensional point at the origin
    /// let nd_point = NDPoint::new(3);
    /// ```
    pub fn new(dim: i32) -> Self{
        let coords = vec![0.0, 0.0, 0.0];
        NDPoint {coords, dim}
    }

    /// Create a new point from the given coordinate, given as a vector of floats.
    /// The dimensionality of the point is equal to the length of the vector
    ///
    /// # Arguments
    /// * `pt` - A vector of floats representing the coordinates for the point
    ///
    /// # Example
    /// 
    /// ```
    /// # use kyles_algorithm_crate::base::NDPoint;
    /// // creates a new 3-dimensional point at (1.5, 2.5, 3.5)
    /// let nd_point = NDPoint::from_coordinate([1.5, 2.5, 3.5]);
    /// ```
    pub fn from_coordinate(pt: Vec<f32>) -> Self {
        let dim = pt.len() as i32;

        let mut coords = Vec::new();

        for p in pt {
            coords.push(p);
        }

        NDPoint{dim: dim, coords: coords}

    }

    /// Get the dimensionality of the point
    pub fn dim(&self) -> i32 {
        return self.dim;
    }

    /// Set the coordinates of this point
    /// Returns InvalidArgumentError if the dimension of the given point is less than 1
    ///
    /// # Arguments
    /// * `pt` - A vector of floats representing the coordinates to give this point
    ///
    /// # Example
    /// ```
    /// # use kyles_algorithm_crate::base::NDPoint;
    /// // creates a new 3-dimensional point at the origin
    /// let mut nd_point = NDPoint::new(3);
    /// // Set the point to coordinates (3.2, 4.5, 6.7)
    /// nd_point.set_point([3.2, 4.5, 6.7]);
    /// ```
    pub fn set_point(&mut self, pt: Vec<f32>) -> Result<(), InvalidArgumentError> {

        let dim = pt.len() as i32;

        if dim < 1 {
            return Err(InvalidArgumentError)
        }

        let mut coords = Vec::new();

        for p in pt {
            coords.push(p);
        }

        self.dim = dim;
        self.coords = coords;

        Result::Ok(())
    }

    /// Return the ith coodinate of the point
    /// Returns InvalidArgumentError if the given index is greater than the
    /// dimensionality of the point
    ///
    /// # Arguments
    /// * `i` - The index of the coordinate to get
    ///
    /// # Example
    /// ```
    /// # use kyles_algorithm_crate::base::NDPoint;
    /// // creates a new 3-dimensional point at the origin
    /// let nd_point = NDPoint::from_coordinate([5.0, 6.0, 7.0]);
    /// let y = nd_point.idx(1);
    /// assert_eq!(y, 6.0);
    pub fn idx(&self, i: i32) -> Result<f32, InvalidArgumentError> {
        
        if i  > self.dim{
            return Err(InvalidArgumentError)
        }

        Ok(self.coords[i as usize])
    }

    /// Compares the ith coordinate of this point to another point
    /// Returns -1 if the ith coordinate of this point is smaller than that of 'other'
    /// Returns 0 if the ith coordinate of this point is equal to that of 'other'
    /// Returns 1 if the ith coordinate of this point is greater than that of 'other'
    /// Returns InvalidArgumentError if the other point has a different dimensionality or the given
    ///     dimension is greater than that of this point
    ///
    /// # Arguments
    /// * `i` - The dimension of the points to compare
    /// * `other` - Another NDPoint to compare to
    /// 
    /// # Example
    /// # use kyles_algorithm_crate::base::NDPoint;
    /// // creates a new 3-dimensional point at the origin
    /// let nd_point = NDPoint::new(3);
    /// let nd_point2 = NDPoint::from_coordinate([1.0, 1.0, 1.0]);
    /// // compare the points
    /// let res = nd_point.compare_by_dim(0, nd_point2);
    /// assert_eq(res, -1);
    pub fn compare_by_dim(&self, i: i32, other: &NDPoint) -> Result<i8, InvalidArgumentError> {

        if self.dim < i {
            return Err(InvalidArgumentError)
        } else if other.dim != self.dim {
            return Err(InvalidArgumentError)
        }

        let comp_self = self.coords[i as usize];
        let comp_other = other.coords[i as usize];

        if comp_self < comp_other {
            Ok(-1)
        }else if comp_self == comp_other {
            Ok(0)
        } else {
            Ok(1)
        }
    }

}

// NDPoint must be comparable to other NDPoints

impl Eq for NDPoint{
    
}


impl PartialEq for NDPoint{
    
    fn eq(&self, other: &NDPoint) -> bool {

        let result = self.partial_cmp(&other);

        match result {
            None => false,
            Some(r) => {
                if r == Ordering::Equal {
                    return true
                } else {
                    return false
                }
            }
        }
    }    
}

impl PartialOrd for NDPoint{
    /// Lexicographical comparison by dimension
    fn partial_cmp(&self, other: &NDPoint) -> Option<Ordering> {
        if self.dim != other.dim {
            return None
        }

        for i in 0..self.dim {
            let result = self.compare_by_dim(i, &other).unwrap();
            if result != 0 {
                if result == -1 {
                    return Some(Ordering::Less)
                }else {
                    return Some(Ordering::Greater)
                }
            }
        }

        Some(Ordering::Equal)
    }
}

impl fmt::Display for NDPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement
        write!(f, "{}", self.dim)
    }
}

#[cfg(test)]
mod ndpoint_tests {

    #[test]
    fn test_create_point() {
        use base::ndpoint::NDPoint;

        let point = NDPoint::new(4);

        assert_eq!(point.dim, 4);
        assert_eq!(point.coords, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_from_coordinate() {
        use base::ndpoint::NDPoint;

        let point = NDPoint::from_coordinate(vec![5.0, 6.0, 7.0]);

        assert_eq!(point.dim, 3);
        assert_eq!(point.coords, vec![5.0, 6.0, 7.0]);
    }

    #[test]
    fn test_set_point(){
        use base::ndpoint::NDPoint;
        use custom_errors::invalid_argument_error::InvalidArgumentError;

        let mut point = NDPoint::new(2);

        match point.set_point(vec![1.5, 2.5]) {
            Err(e) => assert!(false),
            _ => ()
        }

        assert_eq!(point.coords, vec![1.5, 2.5]);
        assert_eq!(point.dim, 2);

        match point.set_point(vec![10.0, 11.0, 12.0]) {
            Err(e) => assert!(false),
            _ => ()
        }

        assert_eq!(point.coords, vec![10.0, 11.0, 12.0]);
        assert_eq!(point.dim, 3);

        match point.set_point(vec![]) {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        
    }

    #[test]
    fn test_compare_by_dim(){
        use base::ndpoint::NDPoint;
        use custom_errors::invalid_argument_error::InvalidArgumentError;

        let point = NDPoint::new(2);
        let point2 = NDPoint::from_coordinate(vec![1.0, 2.0]);

        let result = point.compare_by_dim(0, &point2);

        if let Ok(r) = result{
            if r == 0 || r == 1 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let result2 = point.compare_by_dim(1, &point2);

        if let Ok(r) = result2{
            if r == 0 || r == 1 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let result3 = point2.compare_by_dim(0, &point);

        if let Ok(r) = result3{
            if r == -1 || r == 0 {
                assert!(false);
            }
        }else{
            assert!(false);
        }

        let result4 = point2.compare_by_dim(1, &point);

        if let Ok(r) = result4 {
            if r == -1 || r == 0 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // test equal points
        let point3 = NDPoint::new(2);

        let result5 = point3.compare_by_dim(0, &point);

        if let Ok(r) = result5 {
            if r == -1 || r == 1 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let result6 = point.compare_by_dim(0, &point3);

        if let Ok(r) = result6 {
            if r == -1 || r == 1{
                assert!(false);
            }
        } else {
            assert!(false);
        }


        // test point with different dimensionality for InvalidArgumentError
        let diff_dim_point = NDPoint::new(4);

        let result7 = diff_dim_point.compare_by_dim(0, &point);

         
        match result7 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        let result8 = diff_dim_point.compare_by_dim(1, &point);

         
        match result8 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        let result9 = diff_dim_point.compare_by_dim(2, &point);

         
        match result9 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        let result10 = diff_dim_point.compare_by_dim(3, &point);

         
        match result10 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        let result11 = point.compare_by_dim(0, &diff_dim_point);

         
        match result11 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        let result12 = point.compare_by_dim(1, &diff_dim_point);

         
        match result12 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

        let result13 = point.compare_by_dim(2, &diff_dim_point);

         
        match result13 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }

    }


    #[test]
    fn test_eq() {
        use base::ndpoint::NDPoint;

        let point = NDPoint::new(2);
        let point2 = NDPoint::from_coordinate(vec![1.0, 2.0]);

        let result = point.eq(&point2);

        if result != false {
            assert!(false);
        }

        let result2 = point2.eq(&point);

        if result != false {
            assert!(false);
        }

        let eq_point = NDPoint::from_coordinate(vec![1.0, 2.0]);

        let result3 = eq_point.eq(&point2);

        if result3 != true {
            assert!(false);
        }

        let result4 = point2.eq(&eq_point);

        if result4 != true {
            assert!(false);
        }

    }

    #[test]
    fn test_partial_cmp() {
        
        use base::ndpoint::NDPoint;
        use std::cmp::Ordering;

        let point = NDPoint::new(2);
        let point2 = NDPoint::from_coordinate(vec![1.0, 2.0]);

        let result = point.partial_cmp(&point2).unwrap();

        if result != Ordering::Less {
            assert!(false);
        }

        let result2 = point2.partial_cmp(&point).unwrap();

        if result2 != Ordering::Greater {
            assert!(false);
        }

        let equal_point = NDPoint::new(2);

        let result3 = point.partial_cmp(&equal_point).unwrap();

        if result3 != Ordering::Equal {
            assert!(false);
        }

        let higher_dim_point = NDPoint::new(4);

        let result4 = higher_dim_point.partial_cmp(&point);

        if result4 != None{
            assert!(false);
        }

        let result5 = point.partial_cmp(&higher_dim_point);

        if result5 != None {
            assert!(false);
        }
    }

    #[test]
    fn test_idx() {
        use base::ndpoint::NDPoint;
        use custom_errors::invalid_argument_error::InvalidArgumentError;

        let point = NDPoint::from_coordinate(vec![1.0, 2.0]);

        let result = point.idx(0);

        if let Ok(r) = result {
            if r != 1.0 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let result2 = point.idx(1);

        if let Ok(r) = result2 {
            if r != 2.0 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        let result3 = point.idx(3);

        match result3 {
            Ok(e) => assert!(false),
            Err(e) => assert_eq!(e.to_string(), InvalidArgumentError.to_string())
        }
    }
}