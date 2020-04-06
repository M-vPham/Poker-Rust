use std::collections::HashSet;

fn main() {

    let perm:[u32; 10] = [40,17,49,18,50,19,51,20,52,21] ;
    three_tiebreaker([12,12,1,12,4], [3,8,8,8,3]);
}

fn deal(deck: [u32;10]) -> [&'static str;5]
{
    let organized_deck:[(i32,i32);52] = [(1,1),(2,1),(3,1),(4,1),(5,1),(6,1),(7,1),(8,1),(9,1),(10,1),(11,1),(12,1),(13,1),
                    (1,2),(2,2),(3,2),(4,2),(5,2),(6,2),(7,2),(8,2),(9,2),(10,2),(11,2),(12,2),(13,2),
                    (1,3),(2,3),(3,3),(4,3),(5,3),(6,3),(7,3),(8,3),(9,3),(10,3),(11,3),(12,3),(13,3),
                    (1,4),(2,4),(3,4),(4,4),(5,4),(6,4),(7,4),(8,4),(9,4),(10,4),(11,4),(12,4),(13,4)];
    let actual_deck:[&str;52] =   ["1C","2C","3C","4C","5C","6C","7C","8C","9C","10C","11C","12C","13C",
                                "1D","2D","3D","4D","5D","6D","7D","8D","9D","10D","11D","12D","13D",
                                "1H","2H","3H","4H","5H","6H","7H","8H","9H","10H","11H","12H","13H",
                                "1S","2S","3S","4S","5S","6S","7S","8S","9S","10S","11S","12S","13S"];
    
    let unsort_tuple_hand_1:[(i32,i32); 5] = sort([organized_deck[(deck[0]-1) as usize], organized_deck[(deck[2]-1) as usize], organized_deck[(deck[4]-1) as usize], organized_deck[(deck[6]-1) as usize], organized_deck[(deck[8]-1) as usize]]);
    let unsort_tuple_hand_2:[(i32,i32); 5] = sort([organized_deck[(deck[1]-1) as usize], organized_deck[(deck[3]-1) as usize], organized_deck[(deck[5]-1) as usize], organized_deck[(deck[7]-1) as usize], organized_deck[(deck[9]-1) as usize]]);
    
    let hand1_indices: [u32;5] = sort([deck[0],deck[2] ,deck[4],deck[6],deck[8]]);
    let hand2_indices: [u32;5] = sort([deck[1],deck[3],deck[5],deck[7],deck[9]]);

    //handArrays 
    let hand_value_1: [i32;5] = [unsort_tuple_hand_1[0].0,unsort_tuple_hand_1[1].0,unsort_tuple_hand_1[2].0,unsort_tuple_hand_1[3].0,unsort_tuple_hand_1[4].0 ];
    let hand_value_2: [i32;5] = [unsort_tuple_hand_2[0].0,unsort_tuple_hand_2[1].0,unsort_tuple_hand_2[2].0,unsort_tuple_hand_2[3].0,unsort_tuple_hand_2[4].0 ];

    //tupleArray 
    let suit_value_1: [i32;5] = [unsort_tuple_hand_1[0].1,unsort_tuple_hand_1[1].1,unsort_tuple_hand_1[2].1,unsort_tuple_hand_1[3].1,unsort_tuple_hand_1[4].1];
    let suit_value_2: [i32;5] = [unsort_tuple_hand_2[0].1,unsort_tuple_hand_2[1].1,unsort_tuple_hand_2[2].1,unsort_tuple_hand_2[3].1,unsort_tuple_hand_2[4].1];

    //handVectors 
    let mut vector_hand_value_1: Vec<i32> = hand_value_1.to_vec();
    let mut vector_hand_value_2: Vec<i32> = hand_value_2.to_vec();

    //actualHands 
    let actual_hand1: [&str;5] = ([actual_deck[(hand1_indices[0]-1) as usize], actual_deck[(hand1_indices[1]-1) as usize], actual_deck[(hand1_indices[2]-1) as usize], actual_deck[(hand1_indices[3]-1) as usize], actual_deck[(hand1_indices[4]-1) as usize]]);
    let actual_hand2: [&str;5] = ([actual_deck[(hand2_indices[0]-1) as usize], actual_deck[(hand2_indices[1]-1) as usize], actual_deck[(hand2_indices[2]-1) as usize], actual_deck[(hand2_indices[3]-1) as usize], actual_deck[(hand2_indices[4]-1) as usize]]);
    
    // priorities 
    let priority1 = check_priority(unsort_tuple_hand_1, hand_value_1); 
    let priority2 = check_priority(unsort_tuple_hand_2, hand_value_2);
    
    if (priority1 > priority2){
        return(sort(actual_hand1));
    }
    else if (priority2 > priority1){
        return(sort(actual_hand2));
    }
    else{
        return(["NOPE", "NOPE", "DO", "TIE", "BREAKER"]);
    }
}


/**
 * OnePair
 * Priority: 2 
 * Input: Pass in an immutable hand 
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
 * Input: Pass in an immutable hand 
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
 * Input: Pass in an immutable hand 
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
 * Input: Pass in an immutable array of tuple 
 * Output: Integer i32
 * 
*/
fn is_flush(tuple_hand:[(i32,i32); 5]) -> i32{ 
    let copy_vector: [i32;5] = [tuple_hand[0].1,tuple_hand[1].1,tuple_hand[2].1,tuple_hand[3].1,tuple_hand[4].1 ];
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

    if (is_flush(unsort_tuple_hand)==6){
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

    if (is_flush(unsort_tuple_hand)==6){
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
//EVALUATING PRIORITY 
//will return the priority of one hand, must call this one both the hands 
fn check_priority(tuple_hand:[(i32,i32); 5], hand: [i32;5]) -> i32 {
    if(is_royalflush(tuple_hand)==10){
        return(10);
    }
    else if (is_straightflush(tuple_hand)==9){
        return(9); 
    }
    else if (is_four_of_a_kind(hand)==8){
        return(8);
    }
    else if (is_fullhouse(hand)==7){
        return(7);
    }
    else if (is_flush(tuple_hand))==6{
        return(6);
    }
    else if(is_straight(hand)==5){
        return(5);
    }
    else if(is_two_pair_or_three_of_a_kind(hand)==4){
        return(4);
    }
    else if(is_two_pair_or_three_of_a_kind(hand)==3){
        return(3);
    }
    else if(is_one_pair(hand)==2){
        return(2);
    }
    else{
        return(1); 
    }
}


//TIEBREAKERS 
/**
 * HighCard (5 CARDS TAKE'S )
 * Priority: ? 
 * Input: two hands 
 * Output: Integer i32
 * Note: Pass in two tuples that are ONLY HIGH CARDS, if there's a onePair 
 * 
 * 
*/
fn is_high_card(hand_1: [(i32,i32); 5], hand_2:[(i32,i32); 5]){ //-> &'a i32
    //handArrays 
    let hand_value_1: [i32;5] = ([hand_1[0].0,hand_1[1].0,hand_1[2].0,hand_1[3].0,hand_1[4].0 ]);
    let hand_value_2: [i32;5] = ([hand_2[0].0,hand_2[1].0,hand_2[2].0,hand_2[3].0,hand_2[4].0 ]);

    let mut a: HashSet<i32>= hand_value_1.to_vec().into_iter().collect();
    let mut b: HashSet<i32> = hand_value_2.to_vec().into_iter().collect();

    let mut a_unique = a.difference(&b).collect::<Vec<&i32>>();
    let mut b_unique = b.difference(&a).collect::<Vec<&i32>>();
    // println!("{:?}", a_unique);
    // println!("{:?}", b_unique);
    let high_1 = get_max_rank_vector(a_unique);
    let high_2 = get_max_rank_vector(b_unique);

    println!("HighCard: {:?} \nHighCard: {:?}", high_1, high_2 );

    if (high_1 > high_2){
        println!("HAND1");
    }
    else if (high_2>high_1){
        println!("HAND2");
    }
    else {
        return(suit_breaker_high_hand(hand_1, hand_2));
    }

}
// unique numbers 
fn suit_breaker_high_hand(hand_1: [(i32,i32); 5], hand_2:[(i32,i32); 5]){
        let hand_value_1: [i32;5] = sort([hand_1[0].0,hand_1[1].0,hand_1[2].0,hand_1[3].0,hand_1[4].0 ]);
        let hand_value_2: [i32;5] = sort([hand_2[0].0,hand_2[1].0,hand_2[2].0,hand_2[3].0,hand_2[4].0 ]);
        //highHand 
        let high_1 = get_max_rank_list(hand_value_1);
        let high_2 = get_max_rank_list(hand_value_2);
        //maxTuple 
        let mut high_suit_1: (i32) = 0;
        let mut high_suit_2: (i32) = 0;
    
        for i in (0..5){
            if (hand_1[i].0 == high_1){
                high_suit_1 = hand_1[i].1;
            }
            if (hand_2[i].0 == high_2){
                high_suit_2 = hand_2[i].1;
            }
        }
        if high_suit_1 > high_suit_2 {
            println!("HAND1");
        }
        else {
            println!("HAND2");
        }
}

fn one_pair_tiebreaker(tuple_1:[(i32,i32);5], tuple_2:[(i32,i32);5]) {
    let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13];
    let mut pair_1:i32 = 0;
    let mut pair_2:i32 = 0;

    for rank in ranks.iter(){ 
		if (hand_1.iter().filter(|&x| x == rank).count()) == 2{
			pair_1 = *rank; 
        }
        if (hand_2.iter().filter(|&x| x == rank).count()) == 2{
            pair_2 = *rank;
        }
    }
    if (pair_1 > pair_2) {
        println!("HAND1");
    }
    else if (pair_2 > pair_1) {
        println!("HAND2"); 
    }
    else {
        println!("TIEBREAKER BITCH");
    }
}

fn three_tiebreaker(hand_1:[i32;5],hand_2:[i32;5]) {
    let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13];
    let mut three_1:i32 = 0;
    let mut three_2:i32 = 0;

    for rank in ranks.iter(){ 
		if (hand_1.iter().filter(|&x| x == rank).count()) == 3{
			three_1 = *rank; 
        }
        if (hand_2.iter().filter(|&x| x == rank).count()) == 3{
            three_2 = *rank;
        }
    }
    
    if (three_1 > three_2) {
        println!("HAND1");
    }
    else {
        println!("HAND2");
    }
}

fn full_tiebreaker(hand_1:[i32;5],hand_2:[i32;5] ) {
    let ranks:[i32;13] = [1,2,3,4,5,6,7,8,9,10,11,12,13];
    let mut three_1:i32 = 0;
    let mut three_2:i32 = 0;

    for rank in ranks.iter(){ 
		if (hand_1.iter().filter(|&x| x == rank).count()) == 3{
			three_1 = *rank; 
        }
        if (hand_2.iter().filter(|&x| x == rank).count()) == 3{
            three_2 = *rank;
        }
    }
    
    if (three_1 > three_2) {
        println!("HAND1");
    }
    else {
        println!("HAND2");
    }

}


fn four_tiebreaker(tuple_1:[(i32,i32);5], tuple_2:[(i32,i32);5]) {
    //sortedTupleHands
    let t1 = sort(tuple_1);
    let t2 = sort(tuple_2);

    //fourOfAKindCard value
    let four_1 = tuple_1[2].0;
    let four_2 = tuple_2[2].0;
    
    if (four_1 > four_2){
        println!("HAND1");
    }
    else {
        println!("HAND2");
    }
}



fn straight_tiebreaker( tuple_1:[(i32,i32);5], tuple_2:[(i32,i32);5])  {
    //sortedTupleHands
    let t1 = sort(tuple_1);
    let t2 = sort(tuple_2);
    //sort hand
    let hand_1: [i32;5] = sort([tuple_1[0].0,tuple_1[1].0,tuple_1[2].0,tuple_1[3].0,tuple_1[4].0 ]);
    let hand_2: [i32;5] = sort([tuple_2[0].0,tuple_2[1].0,tuple_2[2].0,tuple_2[3].0,tuple_2[4].0 ]);
    // maximum hands not including Aces
    let max_1 = (hand_1[4]);
    let max_2 = (hand_2[4]);
    //max Tuple
    let max_t1 = (tuple_1[4]);
    let max_t2 = (tuple_2[4]);
    //ace Hand 
    let ace_suit_1 = tuple_1[0].1;
    let ace_suit_2 = tuple_2[0].1; 

    if(max_1==13 && max_2==13){
        //tiebreak with ace suit/ or the suit of 9
        if (ace_suit_1 > ace_suit_2){
            println!("HAND1"); 
        }
        else {
            println!("HAND2"); 
        }
    }
    else if (max_1==max_2){
        if(max_t1.1 > max_t2.1){
            println!("HAND1");
        }
        else{
            println!("HAND2");
        }
    }
    else {
        if (max_1 > max_2){
            println!("HAND1");
        }
        else {
            println!("HAND2");
        }
    }

    println!("{:?}", t1);
}

fn royal_tiebreaker(tuple_1:[(i32,i32);5], tuple_2:[(i32,i32);5]){
    let hand_1_suit = tuple_1[0].1;
    let hand_2_suit = tuple_2[0].1;
    if (hand_1_suit > hand_2_suit){
        println!("HAND1");
    }
    else {
        println!("HAND2");
    }
}


fn tiebreaker(tuple_hand_1:[(i32,i32); 5], tuple_hand_2:[(i32,i32); 5], hand_1:[i32;5], hand_2:[i32;5], priority:i32){ //-> i32 
    match priority{
        10 => royal_tiebreaker(tuple_hand_1, tuple_hand_2),
        9 => straight_tiebreaker(tuple_hand_1, tuple_hand_2), 
        8 => four_tiebreaker(tuple_hand_1, tuple_hand_2), 
        7 => full_tiebreaker(hand_1, hand_2),
        6 => is_high_card(tuple_hand_1, tuple_hand_2), 
        5 => straight_tiebreaker(tuple_hand_1, tuple_hand_2),
        4 => three_tiebreaker(hand_1, hand_2),
    //     3 => two_pair_tiebreaker(tuple_hand_1, tuple_hand_2), 
    //     2 => one_pair_tiebreaker(tuple_hand_1, tuple_hand_2), 
        _ => is_high_card(tuple_hand_1, tuple_hand_2)
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


fn get_max_rank_vector(hand: Vec<&i32>) -> i32{
    let mut max_rank:i32 = 1; 
    for card_value in hand.iter(){
        if card_value > &&max_rank {
            max_rank = **card_value;
        }
    }
    return max_rank;
}

fn get_max_rank_list(hand:[i32;5]) -> i32{
    let mut max_rank:i32 = 1; 
    for card_value in hand.iter(){
        if card_value > &max_rank {
            max_rank = *card_value;
        }
    }
    return max_rank;
}
