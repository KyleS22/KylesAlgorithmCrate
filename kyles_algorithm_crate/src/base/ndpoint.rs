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
        let coords = vec![0.0, 0.0, 0.0];
        NDPoint {coords, dim: 3}
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
        Result::Err(InvalidArgumentError)
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
    pub fn idx(&self, i: u32) -> f32{
        0.0
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
    pub fn compare_by_dim(&self, i: u32, other: &NDPoint) -> Result<u32, InvalidArgumentError> {
        Result::Err(InvalidArgumentError)
    }

    // Private functions

    /// Copies the contents of the given vector into the point's coordinate vector
    /// Returns InvalidArgumentError if the vector's length
    /// is not equal to the point's dimensionality
    ///
    /// # Arguments
    /// * `ar` - The vector to copy into the point's coordinate vector
    ///
    fn copy_array(ar: Vec<f32>) -> Result<(), InvalidArgumentError>{
        // TODO: implement
        Result::Err(InvalidArgumentError)
    }

}

// NDPoint must be comparable to other NDPoints

impl Eq for NDPoint{
    
}

impl Ord for NDPoint{
    
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Less  
    }
}

impl PartialEq for NDPoint{
    
    fn eq(&self, other: &NDPoint) -> bool {
        false
    }    
}

impl PartialOrd for NDPoint{
    fn partial_cmp(&self, other: &NDPoint) -> Option<Ordering> {
        Some(self.cmp(other))
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

        if let Ok(r) = result{
            if r == 0 || r == 1 {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        // TODO: test point2.compare by dim

        // TODO: test equal points

        // TODO: test point with different dimensionality for InvalidArgumentError
        

    }

    #[test]
    fn test_cmp(){
        assert!(false);
    }

    #[test]
    fn test_eq() {
        assert!(false);
    }

    #[test]
    fn test_partial_cmp() {
        assert!(false);
    }
}