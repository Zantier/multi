#[derive(Clone, Debug)]
pub struct Card {
    pub id: i32,
    pub color_id: i32,
    pub amount_id: i32,
    pub shape_id: i32,
    pub fill_id: i32,
}

pub fn id_to_card(id: i32) -> Card {
    let color_id = id % 3;
    let rest = id / 3;
    let amount_id = rest % 3;
    let rest = rest / 3;
    let shape_id = rest % 3;
    let fill_id = rest / 3;

    Card {
        id,
        color_id,
        amount_id,
        shape_id,
        fill_id,
    }
}


fn get_card_id(color_id: i32, amount_id: i32, shape_id: i32, fill_id: i32) -> i32 {
    let mut id = fill_id;
    id = 3 * id + shape_id;
    id = 3 * id + amount_id;
    id = 3 * id + color_id;
    id
}


/** Get the third number, that is either the same or different to the other 2 */
fn get_third_number(num1: i32, num2: i32) -> i32 {
    if num1 == num2 { num1 } else { 3 - num1 - num2 }
}


/** Get the id of the third card that matches the given 2 cards */
pub fn get_third_card(card1: &Card, card2: &Card) -> i32 {
    let color_id = get_third_number(card1.color_id, card2.color_id);
    let amount_id = get_third_number(card1.amount_id, card2.amount_id);
    let shape_id = get_third_number(card1.shape_id, card2.shape_id);
    let fill_id = get_third_number(card1.fill_id, card2.fill_id);
    let id = get_card_id(color_id, amount_id, shape_id, fill_id);
    id
}


pub fn check_match(card1: &Card, card2: &Card, card3: &Card) -> bool {
    let check_id = get_third_card(card1, card2);
    check_id == card3.id
}
