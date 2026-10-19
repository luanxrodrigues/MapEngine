<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Loading from "../components/menu/loading.svelte"

  let name = $state("");
  let greetMsg = $state("");
  let LoadingScreen = $state(false)

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  function ToggleLoadingScreen()
  {
    LoadingScreen = !LoadingScreen;
    console.log("Change")
  }
  function OpenSettings()
  {

  }
  function CloseApp()
  {
    
  }
  console.log(  invoke('greet',{
    name:"Luan"
  }))
</script>

<main class="container">


<div class="body">
      <div class="sidebar">
    <div class="tag">
      <h1 class="title">MapEngine</h1>
      <h2 class="version">Pre-Alpha</h2>
    </div>
      <ul>
        <li class="mainButton"><button onclick={() => {ToggleLoadingScreen()}}>Load Map</button></li>
        <li class="mainButton"><button>Settings</button></li>
        <li class="mainButton"><button>Exit</button></li>
      </ul>
    </div>
    <div class="fade">

    </div>
</div>
{#if LoadingScreen==true}
  <Loading OnClose={ToggleLoadingScreen}></Loading>
{/if}
</main>

<style>
    @font-face{
    font-family: ArtGothic;
    src: url("fonts/ArtGothic.otf");
    font-weight: normal;
    font-style: normal;
    }
  .body{
    width: 100%;
    height: 100%;
    display: flex;
    --background-color:#202321;
    color: white;
    background:url("/test.png");
    background-repeat: no-repeat;
    background-size: cover;
    box-shadow: 0 0 400px rgba(0,0,0,1) inset;
    position: relative;
  }
  .sidebar{
    display: flex;
    flex-direction: column;
    list-style: none;
    background-color: var(--background-color);
    height: 100vh;
    width: 20%;
    min-width: 300px;
  }
  .fade{
    width: 5%;
    height: 100vh;
    background: linear-gradient(90deg,rgba(32, 35, 33, 1) 12%, rgba(32, 35, 33, 0) 100%);
  }
  ul{
    list-style: none;
    margin-top: auto;
    height: 30%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 0px;
    width: 80%;
    align-self:center ;
  }
  li::after{
    content: "";
    height: 2px;
    width: 100%;
    background-color: rgba(240, 248, 255, 0.199);
    display: block;
    margin: 1%;
    display: flex;

  }
  li{
    background: none;
    align-items: center;
    justify-content: center;
    display: flex;
    flex-direction: column;
    height: 33%;
    padding: 0px;
  }
  .tag{
    width: 70%;
    padding: 5%;
    display: flex;
    flex-direction: column;
    margin-top: 5%;
    margin-left: 5%;
    max-width: 230px;
  }
  button{
    background:none;
    color: white;
    border: none;
    width: 100%;
    height: 100%;
  }
  button:hover{
    cursor: pointer;
    background-color: rgba(240, 248, 255, 0.096);
  }
  .title{
    font-family: ArtGothic;
    font-size: 3rem;
  }
  .version{
    align-self: flex-end;
  }
  .tag * {
    margin: 0px;
  }
  
</style>
