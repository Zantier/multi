<script lang="ts">
  import { onMount } from "svelte";
  import { drawCard, type CardOutline } from "./draw";
  import type { Card } from "./shared/Game";

  let showHelp = false;

  let canvasExample11: HTMLCanvasElement;
  let canvasExample12: HTMLCanvasElement;
  let canvasExample13: HTMLCanvasElement;
  let canvasExample21: HTMLCanvasElement;
  let canvasExample22: HTMLCanvasElement;
  let canvasExample23: HTMLCanvasElement;
  let canvasExample31: HTMLCanvasElement;
  let canvasExample32: HTMLCanvasElement;
  let canvasExample33: HTMLCanvasElement;

  let CANVAS_SIZE = 150;
  let cardAngles = new Array(50).fill(0).map(_ => { return Math.random() * 2 * Math.PI; } );

  onMount(() => {
    let ctxExample11: CanvasRenderingContext2D = canvasExample11.getContext('2d')!;
    let ctxExample12: CanvasRenderingContext2D = canvasExample12.getContext('2d')!;
    let ctxExample13: CanvasRenderingContext2D = canvasExample13.getContext('2d')!;
    let ctxExample21: CanvasRenderingContext2D = canvasExample21.getContext('2d')!;
    let ctxExample22: CanvasRenderingContext2D = canvasExample22.getContext('2d')!;
    let ctxExample23: CanvasRenderingContext2D = canvasExample23.getContext('2d')!;
    let ctxExample31: CanvasRenderingContext2D = canvasExample31.getContext('2d')!;
    let ctxExample32: CanvasRenderingContext2D = canvasExample32.getContext('2d')!;
    let ctxExample33: CanvasRenderingContext2D = canvasExample33.getContext('2d')!;
    canvasExample11.width = CANVAS_SIZE;
    canvasExample11.height = CANVAS_SIZE;
    canvasExample12.width = CANVAS_SIZE;
    canvasExample12.height = CANVAS_SIZE;
    canvasExample13.width = CANVAS_SIZE;
    canvasExample13.height = CANVAS_SIZE;
    canvasExample21.width = CANVAS_SIZE;
    canvasExample21.height = CANVAS_SIZE;
    canvasExample22.width = CANVAS_SIZE;
    canvasExample22.height = CANVAS_SIZE;
    canvasExample23.width = CANVAS_SIZE;
    canvasExample23.height = CANVAS_SIZE;
    canvasExample31.width = CANVAS_SIZE;
    canvasExample31.height = CANVAS_SIZE;
    canvasExample32.width = CANVAS_SIZE;
    canvasExample32.height = CANVAS_SIZE;
    canvasExample33.width = CANVAS_SIZE;
    canvasExample33.height = CANVAS_SIZE;

    function drawCard2(ctx: CanvasRenderingContext2D, index: number, outline: CardOutline, card: Card) {
      ctx.fillStyle = '#181818';
      ctx.fillRect(0, 0, CANVAS_SIZE, CANVAS_SIZE);
      drawCard(ctx, CANVAS_SIZE/2, CANVAS_SIZE/2, card, cardAngles[index], CANVAS_SIZE/2 * 0.9, outline);
    }

    function draw() {
      drawCard2(ctxExample11, 0, 'correct', { id: 0,
        colorId: 0,
        amountId: 0,
        fillId: 0,
        shapeId: 1,
      });
      drawCard2(ctxExample12, 1, 'correct', { id: 0,
        colorId: 0,
        amountId: 2,
        fillId: 1,
        shapeId: 1,
      });
      drawCard2(ctxExample13, 2, 'correct', { id: 0,
        colorId: 0,
        amountId: 1,
        fillId: 2,
        shapeId: 1,
      });

      drawCard2(ctxExample21, 3, 'correct', { id: 0,
        colorId: 0,
        amountId: 2,
        fillId: 2,
        shapeId: 0,
      });
      drawCard2(ctxExample22, 4, 'correct', { id: 0,
        colorId: 1,
        amountId: 1,
        fillId: 2,
        shapeId: 1,
      });
      drawCard2(ctxExample23, 5, 'correct', { id: 0,
        colorId: 2,
        amountId: 0,
        fillId: 2,
        shapeId: 2,
      });

      drawCard2(ctxExample31, 6, 'wrong', { id: 0,
        colorId: 2,
        amountId: 1,
        fillId: 0,
        shapeId: 1,
      });
      drawCard2(ctxExample32, 7, 'wrong', { id: 0,
        colorId: 2,
        amountId: 1,
        fillId: 1,
        shapeId: 2,
      });
      drawCard2(ctxExample33, 8, 'wrong', { id: 0,
        colorId: 2,
        amountId: 1,
        fillId: 2,
        shapeId: 1,
      });

      requestAnimationFrame(draw);
    }

    requestAnimationFrame(draw);
  });
</script>

<svelte:window on:load={() => showHelp = true}/>
<button class="help" class:show-help={showHelp} on:click={() => showHelp = false}>
  <div class="help2" class:show-help={showHelp}>
    <div class="help-header">Trifix</div>
    <div class="help-p">
      This is an online multiplayer version of the card game "Trifix", also known as "Set".
    </div>
    <div class="help-p">
      To play with other people, you simply join a room with the same name.
    </div>
    <div class="help-p">
      The aim of the game is to select 3 cards that match in the following way:
      <ul>
        <li>The shapes must be <span class="same">ALL THE SAME</span>, or <span class="different">ALL DIFFERENT</span></li>
        <li>The number of shapes must be <span class="same">ALL THE SAME</span>, or <span class="different">ALL DIFFERENT</span></li>
        <li>The colours must be <span class="same">ALL THE SAME</span>, or <span class="different">ALL DIFFERENT</span></li>
        <li>The fills must be <span class="same">ALL THE SAME</span>, or <span class="different">ALL DIFFERENT</span></li>
      </ul>
    </div>
    <div class="help-p">
      <div class="example-title">
        Example 1: <span class="same">MATCH</span>
      </div>
      <canvas bind:this={canvasExample11}></canvas>
      <canvas bind:this={canvasExample12}></canvas>
      <canvas bind:this={canvasExample13}></canvas>
      <ul>
        <li>The shapes are <span class="same">ALL THE SAME</span></li>
        <li>The number of shapes are <span class="different">ALL DIFFERENT</span></li>
        <li>The colours are <span class="same">ALL THE SAME</span></li>
        <li>The fills are <span class="different">ALL DIFFERENT</span></li>
      </ul>
    </div>
    <div class="help-p">
      <div class="example-title">
        Example 2: <span class="same">MATCH</span>
      </div>
      <canvas bind:this={canvasExample21}></canvas>
      <canvas bind:this={canvasExample22}></canvas>
      <canvas bind:this={canvasExample23}></canvas>
      <ul>
        <li>The shapes are <span class="different">ALL DIFFERENT</span></li>
        <li>The number of shapes are <span class="different">ALL DIFFERENT</span></li>
        <li>The colours are <span class="different">ALL DIFFERENT</span></li>
        <li>The fills are <span class="same">ALL THE SAME</span></li>
      </ul>
    </div>
    <div class="help-p">
      <div class="example-title">
        Example 3: <span class="bad">NO MATCH</span>
      </div>
      <canvas bind:this={canvasExample31}></canvas>
      <canvas bind:this={canvasExample32}></canvas>
      <canvas bind:this={canvasExample33}></canvas>
      <ul>
        <li>The shapes are <span class="bad">PARTIALLY THE SAME</span></li>
        <li>The number of shapes are <span class="same">ALL THE SAME</span></li>
        <li>The colours are <span class="same">ALL THE SAME</span></li>
        <li>The fills are <span class="different">ALL DIFFERENT</span></li>
      </ul>
    </div>
  </div>
</button>

<style>
  .help {
    cursor: pointer;
    padding: 0;
    text-align: left;
    background-color: transparent;
    border: none;
    position: fixed;
    display: flex;
    justify-content: center;
    align-items: center;
    top: 0;
    width: 100%;
    height: 100svh;
    pointer-events: none;
    /* Prevent the whole screen flashing blue in Chrome */
    -webkit-tap-highlight-color: transparent;
  }
  .help.show-help {
    pointer-events: all;
  }
  .help2 {
    position: relative;
    background-color: #303030;
    border: solid 0.2rem #a0a0a0;
    border-radius: 1rem;
    max-width: 40rem;
    height: calc(100svh - 1rem);
    transition: opacity 0.5s, top 0.5s;
    opacity: 0;
    top: -10rem;
    overflow: auto;
  }
  .help2.show-help {
    opacity: 1;
    top: 0;
  }
  .help-header {
    text-align: center;
    font-size: 1.5rem;
    padding-top: 0.5rem;
    padding-bottom: 0.5rem;
  }
  .help-p {
    padding: 0.5rem;
  }

  .example-title {
    font-weight: 800;
    font-size: 1.2rem;
  }

  .same {
    color: hsl(118, 45%, 50%);
  }

  .different {
    color: hsl(237, 30%, 60%);
  }

  .bad {
    color: hsl(187, 39%, 56%);
  }
</style>
