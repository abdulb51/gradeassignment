/*
By: <Your Name Here>
Date: 2026-11-03
Program Details: <Program Description Here>
*/

use std::io;
 
fn main() {
    // Create a 2D vector
    let marks: Vec<Vec<i32>> = vec![vec![80, 75, 90, 85], vec![65, 70, 85, 90], vec![90, 95, 88, 100], vec![70, 80, 75, 85], vec![100, 100, 100, 100]];

   
println! ("Marks for each student:");

println! ("which student's marks would you like to see? (0-4)");


    // Calculate each student's average
   


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
 
println! ("Student {} marks: {:?}", input.trim().parse::<usize>().unwrap_or(0) + 1, marks[input.trim().parse::<usize>().unwrap_or(0)]);

        let mut total = 0;
    for mark in 0..marks[input.trim().parse::<usize>().unwrap_or(0)].len() {
            total += marks[input.trim().parse::<usize>().unwrap_or(0)][mark];
        }
        let aver = total as f32 / marks[input.trim().parse::<usize>().unwrap_or(0)].len() as f32;
        println!("Student {} - Average: {}", input.trim().parse::<usize>().unwrap_or(0) + 1, aver);

    



    // for student in 0..marks.len() {
    //     let mut total: i32 = 0;
    //     for mark in 0..marks[student].len() {
    //         total += marks[student][mark];
    //     }
    //     let aver = total as f32 / marks[student].len() as f32;
    //     println!("Student {} - Average: {}", student + 1, aver);
    // }


//  io::stdin().read_line(&mut input).expect("Failed to read line");
// if input.trim() == "2" {

//     let mut total: i32 = 0;

// }



    // for student in 0..marks.len() {
    //     let mut total: i32 = 0;
    //     for mark in 0..marks[student].len() {
    //         total += marks[student][mark];
    //     }
    //     let aver = total as f32 / marks[student].len() as f32;
    //     println!("Student {} - Average: {}", student + 1, aver);
    // }




    // // Calculate the class average
    // let mut class_total: i32 = 0;

    // for student in 0..marks.len() {
    //     for mark in 0..marks[student].len() {
    //         class_total += marks[student][mark];
    //     }
    // }
    // let class_average = class_total as f32 / (marks.len() * marks[0].len()) as f32;
    // println!("Class Average: {}", class_average); 
    
    
    
    // Display all marks (last)
    // for student in 0..marks.len() {
    //     for mark in 0..marks[student].len() {
    //         println!("Student {} - Mark {}: {}", student + 1, mark + 1, marks[student][mark]);
    //     }
    // }
    

}
