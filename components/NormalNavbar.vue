<template>

    <div class="bg-gray-200 text-gray-900 flex-row">

        <div>
            <nuxt-link to="/" class="bg-red-500">Home </nuxt-link>
        </div>

        <div>
            <nuxt-link to="/add">Add</nuxt-link>

        </div>

        <div>
            <nuxt-link to="/edit" class="bg-red-500">Edit</nuxt-link>

        </div>

        <div>
            <nuxt-link to="/impressum">About</nuxt-link>
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