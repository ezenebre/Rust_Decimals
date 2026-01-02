use rust_decimal::Decimal;

fn main() {

    let crankshaft_string :&str = "57.881234";
    let bearing_string :&str = "57.929012";
    let crankshaft :Decimal = Decimal::from_str_exact(crankshaft_string).unwrap();
    let bearing :Decimal = Decimal::from_str_exact(bearing_string).unwrap();
    let gap :Decimal = bearing - crankshaft;

    let gap_cent :Decimal = gap.round_dp(2);
    let gap_mil :Decimal = gap.round_dp(3);

    println!("Original Gap: {}", gap);
    println!("Gap with 2 decimal places: {}", gap_cent);
    println!("Gap with 3 decimal places: {}", gap_mil);

}