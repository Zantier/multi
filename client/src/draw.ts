import type { Card } from "./shared/Game";

export const COLORS = [
  '64,192,64','64,180,240','64,64,240',
];
export type CardOutline = 'normal' | 'correct' | 'wrong';


const patterns = [
  createPattern(COLORS[0]),
  createPattern(COLORS[1]),
  createPattern(COLORS[2]),
];

export function drawCard(
  ctx: CanvasRenderingContext2D, x: number, y: number, card: Card, angle: number,
  cardRadius: number, outline: CardOutline
) {
  switch (outline) {
    case "normal":
      ctx.strokeStyle = '#404040';
      break;
    case "correct":
      ctx.strokeStyle = 'rgb(64,192,64)';
      break;
    case "wrong":
      ctx.strokeStyle = 'rgb(64,64,240)';
      break;
  }
  ctx.lineWidth = 5;

  ctx.beginPath();
  ctx.arc(x, y, cardRadius, 0, 2*Math.PI);
  ctx.stroke();

  const amount = card.amountId + 1;
  const color = COLORS[card.colorId];

  ctx.lineWidth = 4;
  ctx.fillStyle = card.fillId === 0 ? `rgb(${color})` : card.fillId === 1 ? 'rgba(0,0,0,0)' : patterns[card.colorId];
  ctx.strokeStyle =  `rgb(${color})`;
  ctx.save();
  ctx.translate(x, y);
  ctx.rotate(angle);
  const radius = 15;
  for (let i = 0; i < amount; i++) {
    const amountRadius = amount === 1 ? 0 : cardRadius * 0.45;
    ctx.save();
    ctx.rotate(i * 2 * Math.PI / amount);
    ctx.translate(amountRadius, 0);
    ctx.beginPath();
    if (card.shapeId === 0) {
      // Circle
      ctx.arc(0, 0, radius, 0, 2*Math.PI);
    } else if (card.shapeId === 1) {
      // Triangle
      ctx.moveTo(-1.3*radius, 0);
      ctx.lineTo(1.3*radius, 0);
      ctx.lineTo(0, 1.6*radius);
      ctx.closePath();
    } else {
      // Pentagon
      ctx.moveTo(-radius, -0.6*radius);
      ctx.lineTo(0.5*radius, -1.0*radius);
      ctx.lineTo(1.4*radius, 0);
      ctx.lineTo(0.5*radius, 1.0*radius);
      ctx.lineTo(-radius, 0.6*radius);
      ctx.closePath();
    }
    ctx.fill();
    ctx.stroke();
    ctx.restore();
  }

  ctx.restore();
}


function createPattern(color: string) {
  const patternCanvas = document.createElement('canvas');
  const patternContext = patternCanvas.getContext('2d')!;
  const width = 6;
  patternCanvas.width = width;
  patternCanvas.height = width;
  patternContext.fillStyle = `rgb(${color})`;
  patternContext.strokeStyle = `rgb(${color})`;
  patternContext.lineWidth = 1;
  patternContext.beginPath();
  for (let i = 0; i < 2; i++) {
    patternContext.moveTo(-0.5 * width + i * width, 0);
    patternContext.lineTo(0.5 * width + i * width, width);
  }
  patternContext.arc(0, 0, 50, 0, 0.5 * Math.PI);
  patternContext.stroke();

  const pattern = patternContext.createPattern(patternCanvas, "repeat")!;
  return pattern;
}
