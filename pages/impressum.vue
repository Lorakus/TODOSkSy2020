<template>
    <div  >
        <div>
            <h1 class=" pt-20 text-center  font-semibold sm:text-xl md:text-2xl">Welcome!</h1>
            <h2 class="m-2 md:text-2xl sm:text-xl xl:ml-8">This page was created by: </h2>
        </div>
        <div class="flex flex-col content-center content-around" style="margin-bottom: 16vh">
            <!-- rounded dzila tylko poza komponentem, nie wiem czemu -->
            <Author >
                <img class="rounded-lg  h-16 sm:h-20" src="@/assets/img/teletubis_zielony.jpg" slot="photo"> 
                <p slot="name"> Mariusz Trzeciakiewicz </p>
                <p slot="description"> Computer science student at TU Berlin </p>
            </Author>

            <Author>
                <img class="rounded-lg" src="@/assets/img/teletubis_czerwony.jpg" slot="photo">
                <p slot="name"> Karol Rogoza </p>
                <p slot="description"> Computer science student at TU Berlin </p>
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