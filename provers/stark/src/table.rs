use crate::frame::Frame;
use itertools::Itertools;
use lambdaworks_math::field::{
    element::FieldElement,
    traits::{IsField, IsSubFieldOf},
};

/// A two-dimensional Table holding field elements, arranged in a row-major order.
/// This is the basic underlying data structure used for any two-dimensional component in the
/// the STARK protocol implementation, such as the `TraceTable` and the `EvaluationFrame`.
/// Since this struct is a representation of a two-dimensional table, all rows should have the same
/// length.
#[derive(Clone, Default, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Table<F: IsField> {
    pub data: Vec<FieldElement<F>>,
    pub width: usize,
    pub height: usize,
}

/// A view of a contiguos subset of rows of a table.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct TableView<'t, F: IsField> {
//     pub data: &'t [FieldElement<F>],
//     pub table_row_idx: usize,
//     pub width: usize,
//     pub height: usize,
// }

// /// A pair of tables corresponding to evaluations of the main and auxiliary traces.
// /// It supports main and auxiliary tables taking values in different fields.
// /// Both tables must have the same number of rows.
// #[derive(Clone, Default, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
// pub struct EvaluationTable<F: IsSubFieldOf<E>, E: IsField> {
//     pub(crate) main_table: Table<F>,
//     pub(crate) aux_table: Table<E>,
//     pub(crate) step_size: usize,
// }

impl<'t, F: IsField> Table<F> {
    /// Crates a new Table instance from a one-dimensional array in row major order
    /// and the intended width of the table.
    pub fn new(data: Vec<FieldElement<F>>, width: usize) -> Self {
        // Check if the intented width is 0, used for creating an empty table.
        if width == 0 {
            return Self {
                data: Vec::new(),
                width,
                height: 0,
            };
        }

        // Check that the one-dimensional data makes sense to be interpreted as a 2D one.
        debug_assert!(crate::debug::validate_2d_structure(&data, width));
        let height = data.len() / width;

        Self {
            data,
            width,
            height,
        }
    }

    /// Creates a Table instance from a vector of the intended columns.
    pub fn from_columns(columns: Vec<Vec<FieldElement<F>>>) -> Self {
        if columns.is_empty() {
            return Self::new(Vec::new(), 0);
        }
        let height = columns[0].len();

        // Check that all columns have the same length for integrity
        debug_assert!(columns.iter().all(|c| c.len() == height));

        let width = columns.len();
        let mut data = Vec::with_capacity(width * height);

        for row_idx in 0..height {
            for column in columns.iter() {
                data.push(column[row_idx].clone());
            }
        }

        Self::new(data, width)
    }

    /// Returns a vector of vectors of field elements representing the table rows
    pub fn rows(&self) -> Vec<Vec<FieldElement<F>>> {
        self.data.chunks(self.width).map(|r| r.to_vec()).collect()
    }

    /// Given a row index, returns a reference to that row as a slice of field elements.
    pub fn get_row(&self, row_idx: usize) -> &[FieldElement<F>] {
        let row_offset = row_idx * self.width;
        &self.data[row_offset..row_offset + self.width]
    }

    /// Given a row index, returns a mutable reference to that row as a slice of field elements.
    pub fn get_row_mut(&mut self, row_idx: usize) -> &mut [FieldElement<F>] {
        let n_cols = self.width;
        let row_offset = row_idx * n_cols;
        &mut self.data[row_offset..row_offset + n_cols]
    }

    /// Given a slice of field elements representing a row, appends it to
    /// the end of the table.
    pub fn append_row(&mut self, row: &[FieldElement<F>]) {
        debug_assert_eq!(row.len(), self.width);
        self.data.extend_from_slice(row);
        self.height += 1
    }

    /// Returns a reference to the last row of the table
    pub fn last_row(&self) -> &[FieldElement<F>] {
        self.get_row(self.height - 1)
    }

    /// Returns a vector of vectors of field elements representing the table
    /// columns
    pub fn columns(&self) -> Vec<Vec<FieldElement<F>>> {
        (0..self.width)
            .map(|col_idx| {
                (0..self.height)
                    .map(|row_idx| self.data[row_idx * self.width + col_idx].clone())
                    .collect()
            })
            .collect()
    }

    /// Given row and column indexes, returns the stored field element in that position of the table.
    pub fn get(&self, row: usize, col: usize) -> &FieldElement<F> {
        let idx = row * self.width + col;
        &self.data[idx]
    }

    /// Given a step size, converts the given table into a `Frame`.
    pub fn into_frame(&'t self, main_trace_columns: usize, step_size: usize) -> Frame<'t, F, F> {
        debug_assert!(self.height % step_size == 0);
        let steps = (0..self.height)
            .step_by(step_size)
            .map(|initial_row_idx| {
                let end_row_idx = initial_row_idx + step_size;

                let mut step_main_data: Vec<&'t [FieldElement<F>]> = Vec::new();
                let mut step_aux_data: Vec<&'t [FieldElement<F>]> = Vec::new();

                (initial_row_idx..end_row_idx).for_each(|row_idx| {
                    let row = self.get_row(row_idx);
                    step_main_data.push(&row[..main_trace_columns]);
                    step_aux_data.push(&row[main_trace_columns..]);
                });

                TableView::new(step_main_data, step_aux_data)
            })
            .collect();

        Frame::new(steps)
    }

    // /// Returns the values of the tables as a single list of columns containing both main and
    // /// auxiliary tables. The first `self.n_main_cols()` are the columns of the main trace and the
    // /// rest are the auxiliary table columns.
    // pub fn columns(&self) -> Vec<Vec<FieldElement<F>>> {
    //     let mut columns = self.main_table.columns();
    //     let aux_columns = self.aux_table.columns();
    //     columns.extend(aux_columns);
    //     columns
    // }
}

/// A view of a contiguos subset of rows of a table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableView<'t, F, E>
where
    E: IsField,
    F: IsSubFieldOf<F>,
{
    pub data: Vec<&'t [FieldElement<F>]>,
    pub aux_data: Vec<&'t [FieldElement<E>]>,
    // pub width: usize,
    // pub height: usize,
}

impl<'t, F, E> TableView<'t, F, E>
where
    E: IsField,
    F: IsSubFieldOf<F>,
{
    pub fn new(
        data: Vec<&'t [FieldElement<F>]>,
        aux_data: Vec<&'t [FieldElement<E>]>,
        // _width: usize,
        // _height: usize,
    ) -> Self {
        Self {
            data,
            aux_data,
            // width,
            // height,
        }
    }

    pub fn get_main_evaluation_element(&self, row: usize, col: usize) -> &FieldElement<F> {
        &self.data[row][col]
    }

    pub fn get_aux_evaluation_element(&self, row: usize, col: usize) -> &FieldElement<E> {
        &self.aux_data[row][col]
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::Felt252;

//     #[test]
//     fn get_rows_slice_works() {
//         let data: Vec<Felt252> = (0..=11).map(Felt252::from).collect();
//         let table = Table::new(data, 3);

//         let slice = table.table_view(1, 2);
//         let expected_data: Vec<Felt252> = (3..=8).map(Felt252::from).collect();

//         assert_eq!(slice.data, expected_data);
//     }
// }
