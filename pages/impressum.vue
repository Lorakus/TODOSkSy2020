<template>
    <div>
        <div>
            <h1>Welcome</h1>
            <h2>This page was created by: </h2>
        </div>
        <Author>
            <p slot="photo"> Photo </p>
            <p slot="name"> Mariusz Trzeciakiewicz </p>
            <p slot="description"> Computer science student at TUB </p>
        </Author>

        <Author>
            <p slot="photo"> Photo </p>
            <p slot="name"> Karol Rogoza </p>
            <p slot="description"> Computer science student at TUB </p>
        </Author>
    </div>
</template>

<script>
//import Author from '@/components/Author.vue'

export default {
    components: {
        

    }
}
</script>