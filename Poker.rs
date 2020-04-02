fn main() {

    let perm:[u32; 10] = [40,17,49,18,50,19,51,20,52,21] ;
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
    
    //handArrays 
    let hand_value_1: [i32;5] = [unsort_tuple_hand_1[0].0,unsort_tuple_hand_1[1].0,unsort_tuple_hand_1[2].0,unsort_tuple_hand_1[3].0,unsort_tuple_hand_1[4].0 ];
    let hand_value_2: [i32;5] = [unsort_tuple_hand_2[0].0,unsort_tuple_hand_2[1].0,unsort_tuple_hand_2[2].0,unsort_tuple_hand_2[3].0,unsort_tuple_hand_2[4].0 ];

    //tupleArray 
    let suit_value_1: [i32;5] = [unsort_tuple_hand_1[0].1,unsort_tuple_hand_1[1].1,unsort_tuple_hand_1[2].1,unsort_tuple_hand_1[3].1,unsort_tuple_hand_1[4].1];
    let suit_value_2: [i32;5] = [unsort_tuple_hand_2[0].1,unsort_tuple_hand_2[1].1,unsort_tuple_hand_2[2].1,unsort_tuple_hand_2[3].1,unsort_tuple_hand_2[4].1];

    //handVectors 
    let mut vector_hand_value_1: Vec<i32> = hand_value_1.to_vec();
    let mut vector_hand_value_2: Vec<i32> = hand_value_2.to_vec();




    // println!("{:?}", is_one_pair(hand_value_1));
    // println!("{:?}", vector_hand_value_1);
    println!("{:?}", hand_value_1);
    println!("{:?}", is_royalflush(unsort_tuple_hand_1));

}

/**
 * OnePair
 * Priority: 2 
 * Input: Pass in an immutable array 
 * Output: Integer i32
*/
 fn is_one_pair(hand: [i32;5]) -> i32{
    let mut copy_vector = hand.to_vec();
    copy_vector.dedup();
    if ((copy_vector).len() == 4){
        return(2);
    }
    else{
        return(0);
    }
}

/**
 * TwoPair/ThreeOfAKind
 * Priority: 3, 4
 * Input: Pass in an immutable array 
 * Output: Integer i32
 * 
*/
fn is_two_pair_or_three_of_a_kind(hand: [i32;5]) -> (i32) {
    let mut copy_vector = hand.to_vec();
    let mut another = hand.to_vec();
    another.dedup(); 
    if (another.len()==3){
        for index in (0..5){
            if (index + 2 <= 5)
                && (copy_vector[index]==copy_vector[index+1])
                    && (copy_vector[index+1] == copy_vector[index+2]){
                        return(4);
                    }
        }
        return(3);
    }
    return(0);
}

/**
 * is_straight
 * Priority: 5
 * Input: Pass in an immutable array 
 * Output: Integer i32
 * 
*/
fn is_straight(hand: [i32;5]) -> i32 {
    let mut copy_vector = hand.to_vec();
    let head = copy_vector[0];
    if ((head == 1) && (copy_vector[1]==10 && copy_vector[2]==11 && copy_vector[3] == 12 && copy_vector[4]==13)) {
            return(5);
    }
    if ((head == 1) && (copy_vector[1]==2 && copy_vector[2]==3 && copy_vector[3] == 4 && copy_vector[4]==5)) {
        return(5);
    }
    if ((copy_vector[4] == (copy_vector[3]+1)) && (copy_vector[4] == (copy_vector[2]+2)) && (copy_vector[4]== (copy_vector[1]+3)) && (copy_vector[4] == (copy_vector[0]+4))){
        return(5); 
    }
    else {
        return(0);
    }
}

/**
 * is_Flush
 * Priority: 6
 * Input: Pass in an immutable array of suitHands 
 * Output: Integer i32
 * 
*/
fn is_flush(hand: [i32;5]) -> i32{ 
    let mut copy_vector = hand.to_vec();
    if ((copy_vector[1]==copy_vector[0]) && (copy_vector[2]==copy_vector[0]) && (copy_vector[3] == copy_vector[0]) && (copy_vector[4]==copy_vector[0])) {
        return(6);
    }
    else {
        return(0);
    }
}

/**
 * is_Fullhouse 
 * Priority: 7
 * Input: Pass in an immutable array of hands
 * Output: Integer i32
 * Note: two pair flag to differentiate between fullhouse and 4 of a kind 
*/
fn is_fullhouse(hand: [i32;5]) -> i32 { 
    let mut copy_vector = hand.to_vec();
    let mut another = hand.to_vec();
    another.dedup(); 
    if(is_two_pair_or_three_of_a_kind(hand)==3){
        if (another.len()==2){
            return(7)
        }
    }
    return(0)
}
/**
 * is_Four_of_a_kind 
 * Priority: 8
 * Input: Pass in an immutable array of hands
 * Output: Integer i32
 * Note: ifFlush is true  
*/
fn is_four_of_a_kind(hand: [i32;5]) -> i32 { 
    let mut copy_vector = hand.to_vec();
    let mut another = hand.to_vec();
    another.dedup(); 
    if(is_two_pair_or_three_of_a_kind(hand)==3){
        return(3);
    }
    else if (another.len()==2){
        return(8);
    }
    return(0)
}

/**
 * is_straight_flush
 * Priority: 9
 * Input: Pass in an immutable array of tuples
 * Output: Integer i32
 * Note: ifFlush is true, isStraight is true  
*/
fn is_straightflush(unsort_tuple_hand: [(i32,i32); 5]) -> i32 { 
    let hand_value: [i32;5] = [unsort_tuple_hand[0].0,unsort_tuple_hand[1].0,unsort_tuple_hand[2].0,unsort_tuple_hand[3].0,unsort_tuple_hand[4].0 ];
    let suit_value: [i32;5] = [unsort_tuple_hand[0].1,unsort_tuple_hand[1].1,unsort_tuple_hand[2].1,unsort_tuple_hand[3].1,unsort_tuple_hand[4].1 ];

    if (is_flush(suit_value)==6){
        if (is_straight(hand_value)==5){
            return(9);
        }
        else {
            return(0);
        }
    }
    else{
        return(0);
    }
}
/**
 * is_royal_flush
 * Priority: 10
 * Input: Pass in an immutable array of tuples 
 * Output: Integer i32
 * Note: ifFlush is true  and pattern match to 1,10,11,12,13
*/
fn is_royalflush(unsort_tuple_hand: [(i32,i32); 5]) -> i32 { 
    let copy_vector: [i32;5] = [unsort_tuple_hand[0].0,unsort_tuple_hand[1].0,unsort_tuple_hand[2].0,unsort_tuple_hand[3].0,unsort_tuple_hand[4].0 ];
    let suit_value: [i32;5] = [unsort_tuple_hand[0].1,unsort_tuple_hand[1].1,unsort_tuple_hand[2].1,unsort_tuple_hand[3].1,unsort_tuple_hand[4].1 ];
    let head = copy_vector[0];

    if (is_flush(suit_value)==6){
        if ((head == 1) && (copy_vector[1]==10 && copy_vector[2]==11 && copy_vector[3] == 12 && copy_vector[4]==13)) {
                return(10);
        }
        else{
            return(0);
        }
    }
    else{
        return(0);
    }
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