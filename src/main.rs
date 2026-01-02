use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {

    let crankshaft :Decimal = dec!(57.881234);
    let bearing :Decimal = dec!(57.929012);
    let gap :Decimal = bearing - crankshaft;

    let gap_cent :Decimal = gap.round_dp(2);
    let gap_mil :Decimal = gap.round_dp(3);

    println!("Original Gap: {}", gap);
    println!("Gap with 2 decimal places: {}", gap_cent);
    println!("Gap with 3 decimal places: {}", gap_mil);

}