
/**
 * Will return weather the year is a leap year.
 * 
 */
pub fn is_leap_year(year: u32) -> bool {
  is_exceptional_century(&year) || ( !is_century(&year) && (year % 4 == 0) )
}

fn is_exceptional_century(year: &u32) -> bool {
    year % 400 == 0
}
fn is_century(year: &u32) -> bool {
    year % 100 == 0
}
