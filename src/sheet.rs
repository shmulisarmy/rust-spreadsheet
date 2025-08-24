use std::{cell::{Cell, RefCell}, cmp::Ordering, collections::{hash_map::DefaultHasher, HashSet}, hash::{Hash, Hasher}, vec};





#[derive(PartialEq, Clone, Copy)]
pub struct Pos{
    pub row: usize,
    pub col: usize
}




pub struct Range {
    pub start: Pos,
    pub end: Pos,
    cached_value: Cell<f64>,
    is_cached: Cell<bool>,
    agg: Aggregation
}




impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for Range {}

impl Hash for Range {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.row.hash(state);
        self.start.col.hash(state);
        self.end.row.hash(state);
        self.end.col.hash(state);
    }
}


impl  Range {
    pub fn new(start: Pos, end: Pos, agg: Aggregation) -> Range {
        Range { start, end, cached_value: Cell::new(0.0), is_cached: Cell::new(false), agg }
    }
}



pub type Aggregation = fn(values: Vec<f64>) -> f64;



type ObserverSheet<'sheet> = Vec<Vec<RefCell<Vec<&'sheet Range>>>>;
pub struct Sheet<'sheet> {
    pub rows: Vec<Vec<CellValue>>   ,
    pub observer_sheet: ObserverSheet<'sheet>    
}

impl<'sheet> Sheet<'sheet> {
    pub fn new(starting_rows: Vec<Vec<CellValue>>) -> Sheet<'sheet> {
        let observer_sheet: ObserverSheet = vec![vec![RefCell::new(Vec::new()); starting_rows[0].len()]; starting_rows.len()];
        Sheet { rows: starting_rows, observer_sheet }
    }
    
    pub fn get_value(&'sheet self, pos: Pos, observer: Option<&'sheet Range>, observer_sheet: Option<& ObserverSheet<'sheet>>) -> f64 {
        if let Some(observer) = observer && let Some(observer_sheet) = observer_sheet {
            if !observer_sheet[pos.row][pos.col].borrow().contains(&observer) {
                observer_sheet[pos.row][pos.col].borrow_mut().push(observer);
            }
        }
        match &self.rows[pos.row][pos.col] {
            CellValue::Number(num) => *num,
            CellValue::RangeAggregation(range) => self.get_range_value(range)
        }
    }


    pub fn get_range_value(&'sheet self , range: &'sheet Range) -> f64 {
        if !range.is_cached.get() {
            let collected_value = self.collect_range_values(range);
            let agg_result = (range.agg)(collected_value);
            range.cached_value.set(agg_result);
            range.is_cached.set(true);
        }
        range.cached_value.get()
    }
    
    pub fn display(&'sheet self) {
        for (row_index, row) in self.rows.iter().enumerate() {
            for (col_index, _cell) in row.iter().enumerate() {
                print!("{} ", self.get_value(Pos{row: row_index, col: col_index}, None, None));
            }
            println!();
        }
    }

    pub fn collect_range_values(&'sheet self, range: &'sheet Range) -> Vec<f64> {
        let mut values = Vec::new();
        for row in range.start.row..=range.end.row {
            for col in range.start.col..=range.end.col {
                values.push(self.get_value(Pos{row, col}, Some(range), Some(&self.observer_sheet)));
            }
        }
        values
    }
}


pub enum CellValue {
    Number(f64),
    RangeAggregation(Range)    
}