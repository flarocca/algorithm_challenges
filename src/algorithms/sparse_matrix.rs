use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
pub enum Error {
    RowOutOfIndex,
    ColumnOutOfIndex,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Item {
    row: usize,
    col: usize,
}

impl Item {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
pub struct SparseMatrix<T> {
    row_max_index: usize,
    col_max_index: usize,
    data: HashMap<Item, T>,
}

impl<T> SparseMatrix<T> {
    // TODO: transpose()
    // TODO: from_dense()
    // TODO: add
    // TODO: Matrix operations

    pub fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            panic!("Rows and Cols must be greater than 0.");
        }

        SparseMatrix {
            row_max_index: rows - 1,
            col_max_index: cols - 1,
            data: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, row: usize, col: usize) -> Result<Option<&T>, Error> {
        self.check_index_boundaries(row, col)?;

        Ok(self.data.get(&Item::new(row, col)))
    }

    pub fn insert(&mut self, row: usize, col: usize, item: T) -> Result<(), Error> {
        self.check_index_boundaries(row, col)?;

        self.data.insert(Item::new(row, col), item);

        Ok(())
    }

    pub fn remove(&mut self, row: usize, col: usize) -> Result<Option<T>, Error> {
        self.check_index_boundaries(row, col)?;

        Ok(self.data.remove(&Item::new(row, col)))
    }

    pub fn nnz(&self) -> usize {
        self.data.len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.row_max_index + 1, self.col_max_index + 1)
    }

    pub fn iter(&self) -> SparseMatrixIterator<T> {
        SparseMatrixIterator::from(self)
    }

    pub fn to_dense(&self) -> Vec<Vec<Option<&T>>> {
        let (rows, cols) = self.shape();

        let mut dense_matrix = vec![vec![None; cols]; rows];

        for (item, value) in &self.data {
            dense_matrix[item.row][item.col] = Some(value);
        }

        dense_matrix
    }

    fn check_index_boundaries(&self, row: usize, col: usize) -> Result<(), Error> {
        if row > self.row_max_index {
            return Err(Error::RowOutOfIndex);
        }

        if col > self.col_max_index {
            return Err(Error::ColumnOutOfIndex);
        }

        Ok(())
    }
}

impl<T: PartialEq> PartialEq for SparseMatrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.row_max_index == other.row_max_index
            && self.col_max_index == other.col_max_index
            && self.data == other.data
    }
}

impl<T: Clone> Clone for SparseMatrix<T> {
    fn clone(&self) -> Self {
        Self {
            row_max_index: self.row_max_index,
            col_max_index: self.col_max_index,
            data: self.data.clone(),
        }
    }
}

impl<T> Index<(usize, usize)> for SparseMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match self.get(index.0, index.1) {
            Ok(Some(item)) => item,
            Ok(None) => panic!("No item found in the matrix"),
            Err(Error::RowOutOfIndex) => panic!("Row out of bounds"),
            Err(Error::ColumnOutOfIndex) => panic!("Column out of bounds"),
        }
    }
}

pub struct SparseMatrixIterator<'a, T> {
    matrix: &'a SparseMatrix<T>,
    actual: Item,
}

impl<'a, T> From<&'a SparseMatrix<T>> for SparseMatrixIterator<'a, T> {
    fn from(matrix: &'a SparseMatrix<T>) -> Self {
        SparseMatrixIterator {
            matrix,
            actual: Default::default(),
        }
    }
}

impl<T: Clone> Iterator for SparseMatrixIterator<'_, T> {
    type Item = (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        let (rows, cols) = self.matrix.shape();
        let (max_row, max_col) = (rows - 1, cols - 1);

        while self.actual.row <= max_row && self.actual.col <= max_col {
            let current_row = self.actual.row;
            let current_col = self.actual.col;
            let item = self.matrix.get(current_row, current_col).unwrap();

            if self.actual.row < max_row {
                self.actual.row += 1;
            } else if self.actual.row == max_row && self.actual.col < max_col {
                self.actual.row = 0;
                self.actual.col += 1;
            } else if self.actual.row == max_row && self.actual.col == max_col {
                self.actual.row += 1;
                self.actual.col += 1;
            }

            if let Some(item) = item {
                return Some((current_row, current_col, item.clone()));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let matrix: SparseMatrix<i32> = SparseMatrix::new(3, 3);

        assert_eq!(matrix.row_max_index, 2);
        assert_eq!(matrix.col_max_index, 2);

        assert!(matrix.data.is_empty());
    }

    #[test]
    fn test_matrix_insertion_within_boundaries() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.insert(1, 1, "o".to_owned());

        assert!(result.is_ok());
        assert!(!matrix.data.is_empty());
    }

    #[test]
    fn test_matrix_insertion_outside_row_boundaries() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.insert(3, 0, "o".to_owned());

        assert!(matches!(result, Err(Error::RowOutOfIndex)));
        assert!(matrix.data.is_empty());
    }

    #[test]
    fn test_matrix_insertion_outside_column_boundaries() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.insert(2, 3, "o".to_owned());

        assert!(matches!(result, Err(Error::ColumnOutOfIndex)));
        assert!(matrix.data.is_empty());
    }

    #[test]
    fn test_matrix_get_existent_item() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix.insert(2, 2, "o".to_owned());

        let result = matrix.get(2, 2);

        assert!(matches!(result, Ok(Some(item)) if item == "o"));
    }

    #[test]
    fn test_matrix_get_non_existent_item() {
        let matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.get(2, 2);

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn test_matrix_get_outside_row_boundaries() {
        let matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.get(3, 0);

        assert!(matches!(result, Err(Error::RowOutOfIndex)));
    }

    #[test]
    fn test_matrix_get_outside_column_boundaries() {
        let matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.get(2, 3);

        assert!(matches!(result, Err(Error::ColumnOutOfIndex)));
    }

    #[test]
    fn test_matrix_is_empty() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        assert!(matrix.is_empty());

        let _ = matrix.insert(2, 2, "o".to_owned());

        assert!(!matrix.is_empty());
    }

    #[test]
    fn test_matrix_remove_existent_item() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix.insert(2, 2, "o".to_owned());

        let result = matrix.remove(2, 2);

        assert!(matches!(result, Ok(Some(item)) if item == *"o"));
    }

    #[test]
    fn test_matrix_remove_non_existent_item() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.remove(2, 2);

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn test_matrix_remove_outside_row_boundaries() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.remove(3, 0);

        assert!(matches!(result, Err(Error::RowOutOfIndex)));
    }

    #[test]
    fn test_matrix_remove_outside_column_boundaries() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let result = matrix.remove(2, 3);

        assert!(matches!(result, Err(Error::ColumnOutOfIndex)));
    }

    #[test]
    fn test_matrix_non_zero_values() {
        let (rows, cols) = (3, 4);
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(rows, cols);

        assert_eq!(matrix.nnz(), 0);

        let _ = matrix.insert(0, 0, "o".to_owned());
        let _ = matrix.insert(1, 0, "o".to_owned());

        assert_eq!(matrix.nnz(), 2);
    }

    #[test]
    fn test_matrix_shape() {
        let (rows, cols) = (3, 4);
        let matrix: SparseMatrix<String> = SparseMatrix::new(rows, cols);

        assert_eq!(matrix.shape(), (rows, cols));
    }

    #[test]
    fn test_matrix_equality() {
        let mut matrix_a: SparseMatrix<String> = SparseMatrix::new(3, 3);
        let mut matrix_b: SparseMatrix<String> = SparseMatrix::new(3, 3);

        assert_eq!(matrix_a, matrix_b);

        let _ = matrix_a.insert(0, 0, "o".to_owned());
        let _ = matrix_b.insert(0, 0, "o".to_owned());

        assert_eq!(matrix_a, matrix_b);

        let _ = matrix_b.insert(0, 1, "o".to_owned());

        assert_ne!(matrix_a, matrix_b);
    }

    #[test]
    fn test_matrix_clone() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        assert_eq!(matrix, matrix.clone());

        let _ = matrix.insert(0, 0, "o".to_owned());

        assert_eq!(matrix, matrix.clone());
    }

    #[test]
    fn test_matrix_access_by_index() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix.insert(0, 0, "o".to_owned());

        assert_eq!(matrix[(0, 0)], "o");
    }

    #[test]
    #[should_panic(expected = "Row out of bounds")]
    fn test_matrix_access_by_out_of_range_row_panics() {
        let matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix[(3, 0)].to_owned();
    }

    #[test]
    #[should_panic(expected = "Column out of bounds")]
    fn test_matrix_access_by_out_of_range_column_panics() {
        let matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix[(0, 3)].to_owned();
    }

    #[test]
    #[should_panic(expected = "No item found in the matrix")]
    fn test_matrix_access_non_existent_item_panics() {
        let matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix[(0, 0)].to_owned();
    }

    #[test]
    fn test_matrix_iter() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix.insert(0, 0, "a".to_owned());
        let _ = matrix.insert(1, 1, "b".to_owned());
        let _ = matrix.insert(2, 0, "c".to_owned());

        let mut iterator = matrix.iter();

        assert!(
            matches!(iterator.next(), Some((row, col, item)) if row == 0 && col == 0 && item == "a")
        );
        assert!(
            matches!(iterator.next(), Some((row, col, item)) if row == 2 && col == 0 && item == "c")
        );
        assert!(
            matches!(iterator.next(), Some((row, col, item)) if row == 1 && col == 1 && item == "b")
        );
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_matrix_to_dense() {
        let mut matrix: SparseMatrix<String> = SparseMatrix::new(3, 3);

        let _ = matrix.insert(0, 0, "a".to_owned());
        let _ = matrix.insert(1, 1, "b".to_owned());
        let _ = matrix.insert(2, 2, "c".to_owned());

        let dense_matrix = matrix.to_dense();

        assert_eq!(dense_matrix[0][0], Some(&"a".to_owned()));
        assert_eq!(dense_matrix[1][1], Some(&"b".to_owned()));
        assert_eq!(dense_matrix[2][2], Some(&"c".to_owned()));
        assert_eq!(dense_matrix[0][1], None);
        assert_eq!(dense_matrix[0][2], None);
        assert_eq!(dense_matrix[1][0], None);
        assert_eq!(dense_matrix[1][2], None);
        assert_eq!(dense_matrix[2][0], None);
        assert_eq!(dense_matrix[2][1], None);
    }
}
