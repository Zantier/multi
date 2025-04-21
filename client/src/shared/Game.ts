export const TIMEOUT_MS = 10000

export type Card = {
  id: number,
  colorId: number,
  amountId: number,
  shapeId: number,
  fillId: number,
}


export function idToCard(id: number): Card {
  const colorId = id % 3;
  let rest = Math.floor(id / 3);
  const amountId = rest % 3;
  rest = Math.floor(rest / 3);
  const shapeId = rest % 3;
  const fillId = Math.floor(rest / 3);

  return {
    id,
    colorId,
    amountId,
    shapeId,
    fillId,
  };
}


function getCardId(colorId: number, amountId: number, shapeId: number, fillId: number) {
  let id = fillId;
  id = 3 * id + shapeId;
  id = 3 * id + amountId;
  id = 3 * id + colorId;
  return id;
}


/** Get the third number, that is either the same or different to the other 2 */
function getThirdNumber(num1: number, num2: number): number {
  return num1 === num2 ? num1 : 3 - num1 - num2;
}


/** Get the id of the third card that matches the given 2 cards */
export function getThirdCard(card1: Card, card2: Card): number {
  const colorId = getThirdNumber(card1.colorId, card2.colorId);
  const amountId = getThirdNumber(card1.amountId, card2.amountId);
  const shapeId = getThirdNumber(card1.shapeId, card2.shapeId);
  const fillId = getThirdNumber(card1.fillId, card2.fillId);
  const id = getCardId(colorId, amountId, shapeId, fillId);
  return id;
}


export function checkMatch(card1: Card, card2: Card, card3: Card) {
  const checkId = getThirdCard(card1, card2);
  return checkId === card3.id;
}
