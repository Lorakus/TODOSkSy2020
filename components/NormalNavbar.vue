<template>


    <div class="grid grid-col-6">
        <div class="grid col-start-1 col-end-2">
            <nuxt-link to="/" class="bg-red-500">Home </nuxt-link>
        </div>

        <div class="grid col-start-7 col-end-7">
            <nuxt-link to="/add">Add</nuxt-link>
        </div>
    </div>

</template>

<script>
export default {
    data () {
        return {
            isOpen: false,
        }
    },
}
</script>