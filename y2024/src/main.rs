mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

fn format_solution(day: usize, solution: (isize, isize)) {
    println!("Day: {:0>2}", day);
    println!("\t1:\t{}", solution.0);
    println!("\t2:\t{}", solution.1);
    println!();
}

fn main() {
    format_solution(1, d01::solution());
    format_solution(2, d02::solution());
    format_solution(3, d03::solution());
    format_solution(4, d04::solution());
    format_solution(5, d05::solution());
}
