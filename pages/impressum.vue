<template>
    <div >
        <div>
            <h1>Welcome</h1>
            <h2>This page was created by: </h2>
        </div>
        <div class="flex flex-col content-center content-around">
            <Author >
                <img src="@/assets/img/ziemniak.jpg" slot="photo"> 
                <p slot="name"> Mariusz Trzeciakiewicz </p>
                <p slot="description"> Computer science student at TUB </p>
            </Author>

            <Author>
                <img src="@/assets/img/ziemniak.jpg" slot="photo">
                <p slot="name"> Karol Rogoza </p>
                <p slot="description"> Computer science student at TUB </p>
            </Author>
        </div>
    </div>
</template>

<script>
import Author from '@/components/Author.vue'

export default {
    components: {
        Author,
    }
}
</script>