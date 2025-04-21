<script lang="ts">
  import Game from './Game.svelte';
  import Help from './Help.svelte';

  type State = 'first' | 'viewing' | 'joining' | 'starting' | 'game';

  let basePath = import.meta.env.BASE_URL;
  let roomId = window.location.pathname.slice(basePath.length);
  let roomStarted: boolean | undefined = undefined;
  let state: State = roomId === '' ? 'first' : 'viewing';
  let socket: WebSocket | undefined = undefined;
  let disconnectTimerId: number | undefined = undefined;
  let playerName: string = localStorage.getItem('name') ?? '';
  let players: any[] = [];
  let roomData: any;
  let shutdownTime: number | null;
  let startTime = 0;

  $: {
    localStorage.setItem("name", playerName);
  }

  $: {
    if (roomId.length > 8) {
      roomId = roomId.slice(0, 8);
    }
  }

  $: {
    if (playerName.length > 8) {
      playerName = playerName.slice(0, 8);
    }
  }

  $: {
    if (state === 'viewing' && !goodWebSocket(socket)) {
      let heartbeatId = 0;
      socket = new WebSocket(import.meta.env.VITE_WEBSOCKET_ADDRESS);
      socket.addEventListener('open', (ev: Event) => {
        heartbeatId = setInterval(() => {
          send({
            type: 'heartbeat',
          });
        }, 10000);
        send({
          type: 'view-room',
          id: roomId,
        });
      });
      socket.addEventListener('close', (ev: Event) => {
        clearInterval(heartbeatId);
        window.history.replaceState(null, '', basePath);
        state = 'first';
        disconnectTimerId = setTimeout(() => {
          disconnectTimerId = undefined;
        }, 2000);
      });
      socket.addEventListener('message', (ev: MessageEvent) => {
        const data = JSON.parse(ev.data);
        switch (data.type) {
          case 'reject-join-room':
            if (state === 'joining') {
              state = 'viewing';
            }
            break;
          case 'update-game':
            state = 'game';
            players = data.players;
            roomData = data;
            shutdownTime = roomData.shutdownTime === null ? null : roomData.shutdownTime + Date.now();
            startTime = roomData.startTime + Date.now();
            const now = Date.now();
            for (let player of players) {
              player.timeout += now;
            }
            break;
          case 'update-players':
            players = data.players;
            roomStarted = data.started;
            break;
        }
      });
    }
  }

  function send(obj: any) {
    socket?.send(JSON.stringify(obj));
  }

  function goodWebSocket(socket: WebSocket | undefined) {
    if (socket === undefined) {
      console.log('socket: undefined');
      return false;
    }

    console.log(`socket: ${socket.readyState}`);
    return socket.readyState === socket.CONNECTING || socket.readyState === socket.OPEN;
  }

  function onViewRoomKeyup(ev: KeyboardEvent) {
    if (ev.key === 'Enter') {
      onViewRoom();
    }
  }

  function onViewRoom() {
    if (roomId === '') {
      return;
    }

    window.history.replaceState(null, '', basePath + roomId);
    state = 'viewing';
    players = [];
    roomStarted = undefined;
  }

  function onRoomBack() {
    if (socket !== undefined) {
      socket.close();
    }
  }

  function onJoinRoomKeyup(ev: KeyboardEvent) {
    if (ev.key === 'Enter') {
      onJoinRoom();
    }
  }

  function onJoinRoom() {
    if (playerName === '') {
      return;
    }

    send({
      type: 'join-room',
      name: playerName,
    });
    state = 'joining';
  }

  function onLeaveRoom() {
    send({
      type: 'leave-room',
    });
    state = 'viewing';
  }

  function onStartKeyup(ev: KeyboardEvent) {
    if (ev.key === 'Enter') {
      onStart();
    }
  }

  function onStart() {
    send({
      type: 'start-game',
      name: playerName,
    });
    state = 'starting'
  }
</script>

<div id="root">
  {#if state === 'first'}
    <div>
      <input bind:value={roomId} on:keyup={onViewRoomKeyup}/>
    </div>
    <div>
      <button on:click={onViewRoom} disabled={disconnectTimerId !== undefined}>View Room</button>
    </div>
    {#if disconnectTimerId !== undefined}
      <div class="disconnected">
        Disconnected
      </div>
    {/if}
  {:else if state === 'viewing' || state === 'joining' || state === 'starting'}
    <div>
      <button on:click={onRoomBack}>❮</button>
      <h1 style="display:inline; vertical-align:middle">Room {roomId}</h1>
      {#if roomStarted}
        <span class="started">[started]</span>
      {/if}
      {#if roomStarted === false}
        <span class="not-started">[not started]</span>
      {/if}
    </div>
    {#if state === 'viewing'}
      <div>
        Name: <input bind:value={playerName} on:keyup={onJoinRoomKeyup}/>
      </div>
      <div>
        <button on:click={onJoinRoom}>Join Room</button>
      </div>
    {/if}
    {#if (state === 'joining' || state === 'starting') && players.length > 0 && players[0].name === playerName}
      <div>
        <button on:click={onStart} disabled={state === 'starting'}>Start</button>
      </div>
    {/if}
    <div class="players">
      <h3>Players</h3>
      {#each players as player}
        <div class:player-disconnected={!player.connected}>
          {player.name}
        </div>
      {/each}
    </div>
  {:else if state === 'game'}
    <Game players={players}
      roomData={roomData}
      shutdownTime={shutdownTime}
      startTime={startTime}
      socketParam={socket} onLeaveRoom={onLeaveRoom}/>
  {/if}
</div>
<Help/>

<style>
  #root {
    padding: 0.5rem;
  }

  h1, h3 {
    margin: 0;
  }
  div, h1, h3 {
    padding: 0.5rem;
  }

  input {
    width: 10rem;
  }

  .disconnected {
    color: #ad4040;
  }
  .player-disconnected {
    background-color: #4a0909;
  }

  .players {
    background-color: #282828;
  }

  .started, .not-started {
    font-size: 1.5rem;
  }
  .started {
    color: #86ca72;
  }
  .not-started {
    color: #a95555;
  }
</style>
