// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    println!( "fuck clippy3");
    /* 
    let my_option: Option<()> = None;
    if let Some( fuck) = my_option {
        my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let my_empty_vec = vec![1, 2, 3, 4, 5].clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    let temp = value_a;
    // Let's swap these two!
    value_a = value_b;
    value_b = temp;
    println!("value a: {}; value b: {}", value_a, value_b);
    */
}
