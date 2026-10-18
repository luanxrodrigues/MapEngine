<div class="overlay">
    <div class="Container">

        <div class="heading">
            <h4 class="headingtext nametext">Name</h4>
            <div class="timeInfoHeader">
                        <h4 class="headingtext">Age</h4>
                        <h4 class="headingtext">LastEdited</h4>
                        <button class="headingimage" onclick={OnClose}>
                         <img src="/icons/Xclose.svg" alt="Fechar" />
                        </button>
            </div>
        </div>
        <div class="divider"></div>

        <div class="entrycontainer">
            {#each maps as map}
                <div class="map">
                    <h4 class="mapinfo">{map}</h4>
                </div>
            {/each}
        </div>
    </div>
</div>
<script lang="ts">
    import {invoke} from "@tauri-apps/api/core"
    let maps : string = $state("")
    $inspect(maps).with(console.trace)
    HandleMapLoad()

    let { OnClose }: { OnClose: () => void } = $props();

    console.log("Onclose: ", OnClose)

    async function HandleMapLoad()
    {
        try{
            invoke<string>('list_maps')
                .then((val) => {
                    console.log(val)
                })
        }
        catch(err)
        {
            console.log(err)
            return;
        }
    }
    

</script>
<style>

.nametext{
    margin:auto;
    margin-left: 20%;
}
button{
    padding: 0px;
    margin: 0px;
    border: none;
    border-radius: none;
    background: none;
}
h4{
    color: rgb(243, 220, 93);
    margin: 0px;
}
.headingtext{
    height: min-content;
    width: fit-content;
    padding-right: 10%;
}
.headingimage{
    height: min-content;
    width: fit-content;
    min-width: 20px;
    min-height: 20px;
}


.Container{
    display: flex;
    flex-direction: column;
    border-radius: 10px;
}
.heading{
    display:grid;
    height: 10%;
    grid-template-columns: 1fr 4fr 1fr;
}
img {
    width: 80% !important;
    height: 80% !important;
    max-width: 40px;
    margin: auto;
}
.divider{
    width: 100%;
    height: 1px;
    background-color: rgba(255, 255, 255, 0.356);
}
.heading * {
    margin-top:auto;
    margin-bottom: auto;
}
.timeInfoHeader{
    display: flex;
    justify-content: space-between;
    height: min-content;
    align-items: center;
    margin-right:1% ;
    position: relative;
    right: 0px;
    left: auto;
    grid-column-start: 3;
    grid-column-end: 4;
    min-width: 170px;
}
.overlay {
    position: fixed;
    inset: 0;
    background-color: rgba(0, 0, 0, 0.445);

    display: flex;
    justify-content: center;
    align-items: center;

    z-index: 1000;
}

.Container {
    width: 90vw;
    height: 90vh;

    background: #202321;
    position: relative;
    z-index: 1;

    overflow: auto;
    max-width: 1360px;
}
</style> 