<template>
    <div class="" style="height: 88vh;">
        <div>
            <h1 class="text-center m-2 font-semibold">Welcome!</h1>
            <h2 class="m-2">This page was created by: </h2>
        </div>
        <div class="flex flex-col content-center content-around">
            <!-- rounded dzila tylko poza komponentem, nie wiem czemu -->
            <Author >
                <img class="rounded-lg" src="@/assets/img/ziemniak.jpg" slot="photo"> 
                <p slot="name"> Mariusz Trzeciakiewicz </p>
                <p slot="description"> Computer science student at TUB </p>
            </Author>

            <Author>
                <img class="rounded-lg" src="@/assets/img/ziemniak.jpg" slot="photo">
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