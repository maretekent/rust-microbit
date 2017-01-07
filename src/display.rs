use pins::*;
use gpio::PinNumber;

const XY_TO_PINS: [(PinNumber, PinNumber);5*5] = [ 
    //        (0, y)          (1, y)          (2, y)          (3, y)          (4, y)
    // y = 0
              (ROW_1, COL_1), (ROW_2, COL_4), (ROW_1, COL_2), (ROW_2, COL_5), (ROW_1, COL_3),
    // y = 1
              (ROW_3, COL_4), (ROW_3, COL_5), (ROW_3, COL_6), (ROW_3, COL_7), (ROW_3, COL_8),
    // y = 2
              (ROW_2, COL_2), (ROW_1, COL_9), (ROW_2, COL_3), (ROW_3, COL_9), (ROW_2, COL_1), 
    // y = 3
              (ROW_1, COL_8), (ROW_1, COL_7), (ROW_1, COL_6), (ROW_1, COL_5), (ROW_1, COL_4),
    // y = 4
              (ROW_3, COL_3), (ROW_2, COL_7), (ROW_3, COL_1), (ROW_2, COL_6), (ROW_3, COL_2)
];

pub fn map_coords(x: u8, y: u8) -> (PinNumber, PinNumber) {
    XY_TO_PINS[(x + 5*y) as usize]
}
