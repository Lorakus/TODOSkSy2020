<template>


    <div class="grid grid-col-6 bg-oxford-blue text-arctic-lime text-xl text-center py-2  ">
        
        <div class="grid col-start-1 col-end-2 ">
            <nuxt-link to="/" >Home           
            </nuxt-link>
        </div>

        <div class="grid col-start-2 col-end-6">
            <nuxt-link to="/add">Add</nuxt-link>
        </div>

        <div class="grid col-start-6 col-end-7">
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