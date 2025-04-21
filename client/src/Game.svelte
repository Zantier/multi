<script lang="ts">
  import { onMount } from "svelte";
  import { drawCard, type CardOutline } from "./draw";
  import { checkMatch, idToCard, TIMEOUT_MS, type Card } from "./shared/Game";

  const CANVAS_WIDTH = 600;
  const CARD_GAP = 10;
  const CARDS_PADDING = 20;
  const CARDS_PER_ROW = 4;
  const CARDS_PER_COL = 3;
  const CARD_RADIUS = ((CANVAS_WIDTH - 2*CARDS_PADDING + CARD_GAP) / CARDS_PER_ROW - CARD_GAP) / 2;
  const CANVAS_HEIGHT = 2 * CARDS_PADDING + 2 * CARDS_PER_COL * CARD_RADIUS + (CARDS_PER_COL + 1) * CARD_GAP;
  const WAIT_SECONDS = 3;

  export let players: any;
  export let roomData: any;
  export let shutdownTime: number | null;
  let shutdownSeconds: number = 0;
  export let startTime: number;
  export let socketParam: WebSocket | undefined;
  export let onLeaveRoom: () => void;
  $: socket = socketParam!;
  let basePath = import.meta.env.BASE_URL;
  let roomId = window.location.pathname.slice(basePath.length);
  let canvas: HTMLCanvasElement;
  $: cards = roomData.cards.map((id: number) => idToCard(id));
  let cardAttributes = new Array(12).fill(undefined).map(x => { return { angle: Math.random() * 2 * Math.PI }; } );
  let selectedCards: number[] = [];
  let timeoutEnd = 0;

  function drawCard2(ctx: CanvasRenderingContext2D, x: number, y: number, index: number, card: Card, angle: number) {
    let outline: CardOutline = 'normal';
    ctx.strokeStyle = '#404040';
    ctx.lineWidth = 5;
    for (let wrong of roomData.wrong) {
      if (wrong.cards.includes(index)) {
        outline = 'wrong';
      }
    }
    for (let correct of roomData.correct) {
      if (correct.cards.includes(index)) {
        outline = 'correct';
      }
    }
    drawCard(ctx, x, y, card, angle, CARD_RADIUS, outline);
  }

  onMount(() => {
    let ctx: CanvasRenderingContext2D = canvas.getContext('2d')!;
    canvas.width = CANVAS_WIDTH;
    canvas.height = CANVAS_HEIGHT;
    resize();
    function draw() {
      ctx.fillStyle = '#181818';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      const timeLeft = Math.ceil((startTime - Date.now()) / 1000);
      if (timeLeft > 0) {
        ctx.fillStyle = '#a0a0a0';
        ctx.font = '200px arial'
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(`${timeLeft}`, canvas.width/2, canvas.height/2);
        requestAnimationFrame(draw);
        return;
      }

      const timeoutDiff = Math.ceil((timeoutEnd - Date.now()) / 1000)
      if (timeoutDiff > 0) {
        ctx.fillStyle = '#a0a0a0';
        ctx.font = '200px arial'
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(`${timeoutDiff}`, canvas.width/2, canvas.height/2);
      }

      if (roomData.gameOver) {
        ctx.fillStyle = '#a0a0a0';
        ctx.font = '100px arial'
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText('Game Over', canvas.width/2, canvas.height/2);
      }

      for (let i = 0; i < CARDS_PER_COL * CARDS_PER_ROW; i++) {
        const x = i % 4 * (2 * CARD_RADIUS + CARD_GAP) + CARDS_PADDING + CARD_RADIUS;
        const y = Math.floor(i / 4) * (2 * CARD_RADIUS + CARD_GAP) + CARDS_PADDING + CARD_RADIUS;
        if (selectedCards.includes(i)) {
          ctx.fillStyle = '#303030';
          ctx.beginPath();
          ctx.arc(x, y, CARD_RADIUS, 0, 2*Math.PI);
          ctx.fill();
        }
        drawCard2(ctx, x, y, i, cards[i], cardAttributes[i].angle);
      }

      requestAnimationFrame(draw);
    }
    requestAnimationFrame(draw);
  });

  function send(obj: any) {
    socket.send(JSON.stringify(obj));
  }

  function mouseDown(ev: MouseEvent) {
    if (timeoutEnd - Date.now() > 0 || roomData.gameOver) {
      return;
    }

    const offsetX = (ev.clientX - dynamicSize.left) / dynamicSize.width * canvas.width;
    const offsetY = (ev.clientY - dynamicSize.top) / dynamicSize.height * canvas.height;

    let cardX = Math.floor((offsetX - CARDS_PADDING + 0.5 * CARD_GAP) / (2 * CARD_RADIUS + CARD_GAP));
    let cardY = Math.floor((offsetY - CARDS_PADDING + 0.5 * CARD_GAP) / (2 * CARD_RADIUS + CARD_GAP));
    if (cardX >= 0 && cardX < CARDS_PER_ROW && cardY >= 0 && cardY < CARDS_PER_COL) {
      let cardIndex = cardY * CARDS_PER_ROW + cardX;
      let tmpIndex = selectedCards.indexOf(cardIndex);
      if (tmpIndex === -1) {
        selectedCards.push(cardIndex);
      } else {
        selectedCards.splice(tmpIndex, 1);
      }

      if (selectedCards.length === 3) {
        const isMatch = checkMatch(cards[selectedCards[0]], cards[selectedCards[1]], cards[selectedCards[2]]);
        if (!isMatch) {
          timeoutEnd = Date.now() + TIMEOUT_MS;
        }
        let data = {
          type: 'pick-cards',
          cards: selectedCards,
        };
        send(data);
        selectedCards = [];
      }
    }
  }

  let dynamicSize = { left: 0, top: 0, width: 0, height: 0 };
  function resize() {
    const boundingBox = canvas.getBoundingClientRect();
    // CSS "object-fit: contain" is visual. The bounding box is still stretched.
    dynamicSize = {
      left: boundingBox.left,
      top: boundingBox.top,
      width: boundingBox.width,
      height: boundingBox.height,
    };
    if (dynamicSize.width > dynamicSize.height * canvas.width / canvas.height) {
      const oldWidth = dynamicSize.width;
      dynamicSize.width = dynamicSize.height * canvas.width / canvas.height;
      dynamicSize.left += (oldWidth - dynamicSize.width) / 2;
    } else {
      const oldHeight = dynamicSize.height;
      dynamicSize.height = dynamicSize.width * canvas.height / canvas.width;
      dynamicSize.top += (oldHeight - dynamicSize.height) / 2;
    }
  }

  function getTimeoutString(timeout: number) {
    const timeDiff = Math.ceil((timeout - Date.now()) / 1000);
    if (timeDiff > 0) {
      return timeDiff;
    }

    return '';
  }

  // Constantly re-render the timeout
  setInterval(() => {
    players = players;
    if (shutdownTime !== null) {
      shutdownSeconds = Math.ceil((shutdownTime - Date.now()) / 1000);
    }
  }, 100);
</script>

<svelte:window on:resize={resize} on:scroll={resize}/>
<div>
  <button style="margin-left: 0" on:click={onLeaveRoom}>❮</button>
  Room {roomId}
</div>
<canvas bind:this={canvas} on:mousedown={mouseDown}></canvas>
<div class="shutdown">
  {#if shutdownTime !== null}
    Server shutdown in: {shutdownSeconds}
  {/if}
</div>
<table>
  <tr>
    <td>Name</td>
    <td>Score</td>
    <td>Timeout</td>
    <td>Connected</td>
  </tr>
  {#each players as player, playerIndex}
    <tr>
      <td
        class:correct={roomData.correct.some(x => x.player === playerIndex)}
        class:wrong={roomData.wrong.some(x => x.player === playerIndex)}
        >{player.name}</td>
      <td>
        {player.score}
        {#if player.minusScore > 0}
          (-{player.minusScore})
        {/if}
      </td>
      <td>{getTimeoutString(player.timeout)}</td>
      <td>{player.connected}</td>
    </tr>
  {/each}
</table>

<style>
  canvas {
    max-width: 100%;
    max-height: 100lvh;
  }

  tr:first-child {
    font-weight: 800;
  }
  td {
    padding-right: 2rem;
  }
  td:last-child {
    padding-right: 0;
  }

  .correct {
    color: black;
    background-color: rgb(64,192,64);
  }
  .wrong {
    color: black;
    background-color: rgb(100,100,240);
  }
  .shutdown {
    color: rgb(100,100,240);
    font-size: 2rem;
  }
</style>
