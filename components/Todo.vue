<template>
    <div class="flex bg-gray-100 rounded py-3 mt-3  justify-between">

        <div>
            <p class="pl-5 text-lg">{{todo_name}}</p>
        </div>

        <div>
            <p>{{time}}</p>
        </div>

        <div class=" ml-2">
            <nuxt-link to="/edit" class="  rounded ">
            <button class="bg-blue-500 px-3 rounded mr-2">edit</button>
            </nuxt-link>
            <button class="bg-red-600 px-3 rounded mr-2">x</button>
        </div>
    </div>
</template>



<script>
export default {
    props: {
        todo_name: {
            name: String,
            required: true,
        },

        time:{
            name:String,
        }
    }
}
</script>