fn main() {
    let perm:[u32; 10] = [9,2,7,4,5,6,3,8,1,10];
    deal(perm)
}
fn deal(deck: [u32;10])
{
    let organized_deck:[(i32,i32);52] = [(1,1),(2,1),(3,1),(4,1),(5,1),(6,1),(7,1),(8,1),(9,1),(10,1),(11,1),(12,1),(13,1),
                    (1,2),(2,2),(3,2),(4,2),(5,2),(6,2),(7,2),(8,2),(9,2),(10,2),(11,2),(12,2),(13,2),
                    (1,3),(2,3),(3,3),(4,3),(5,3),(6,3),(7,3),(8,3),(9,3),(10,3),(11,3),(12,3),(13,3),
                    (1,4),(2,4),(3,4),(4,4),(5,4),(6,4),(7,4),(8,4),(9,4),(10,4),(11,4),(12,4),(13,4)];
    
    
    let unsort_tuple_hand_1:[(i32,i32); 5] = sort([organized_deck[(deck[0]-1) as usize], organized_deck[(deck[2]-1) as usize], organized_deck[(deck[4]-1) as usize], organized_deck[(deck[6]-1) as usize], organized_deck[(deck[8]-1) as usize]]);
    let unsort_tuple_hand_2:[(i32,i32); 5] = sort([organized_deck[(deck[1]-1) as usize], organized_deck[(deck[3]-1) as usize], organized_deck[(deck[5]-1) as usize], organized_deck[(deck[7]-1) as usize], organized_deck[(deck[9]-1) as usize]]);
    
    let hand_value_1: [i32;5] = [unsort_tuple_hand_1[0].0,unsort_tuple_hand_1[1].0,unsort_tuple_hand_1[2].0,unsort_tuple_hand_1[3].0,unsort_tuple_hand_1[4].0 ];
    let hand_value_2: [i32;5] = [unsort_tuple_hand_2[0].0,unsort_tuple_hand_2[1].0,unsort_tuple_hand_2[2].0,unsort_tuple_hand_2[3].0,unsort_tuple_hand_2[4].0 ];

    // println!("{}", (deck[0]-1) as usize)
    // for index in 0..5{
    //     println!("Hand1: ({},{}) Hand2: ({},{})\n", tuple_hand_1[index].0, tuple_hand_1[index].1, tuple_hand_2[index].0, tuple_hand_2[index].1);
    // }
    println!("{:?}", unsort_tuple_hand_1);
    println!("{:?}", unsort_tuple_hand_2);
    println!("{:?}", hand_value_1);
    println!("{:?}", hand_value_2);


    
}

//Helper Functions
fn sort<A, T>(mut array: A) -> A
    where
        A: AsMut<[T]>,
        T: Ord,
    {
        let slice = array.as_mut();
        slice.sort();

        array
    }