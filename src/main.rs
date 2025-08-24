mod sheet;
mod aggrigation;
use sheet::*;

use crate::aggrigation::*;

fn main() {
    let mut sheet = Sheet::new(vec![
        vec![CellValue::Number(20.0), CellValue::Number(2.0), CellValue::RangeAggregation(Range::new(Pos { row: 0, col: 0 }, Pos { row: 0, col: 1 }, sum))],
        vec![CellValue::Number(3.0), CellValue::Number(4.0), CellValue::RangeAggregation(Range::new(Pos { row: 0, col: 0 }, Pos { row: 1, col: 1 }, avg))],
    ]);
    sheet.display();
}
