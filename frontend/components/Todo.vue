<template>
    <div class="flex bg-gray-100 rounded py-3 mt-3 justify-between flex-col mx-1 ">
    <!-- TODO name -->
     <div class=" flex justify-center text-center mb-2 border-gray-300 border-b-2">
            <p class="pl-5 text-lg">{{todo_name}}</p>
    </div>
    
    <!-- date and procent -->
    <div class="flex flex-row justify-around sm: justify-around order-gray-300 border-b-2 sm:border-b-0">

        <div>
            <p class="text-blue-500 font-bold">{{date}}</p>
        </div>

        <div>
            <p class="text-green-500 font-bold">{{procent}} done</p>
        </div>

        <!-- edit and x button -->
        <div class="ml-2  flex-row justify-center mt-2 sm:mt-0 hidden sm:flex ">
            <nuxt-link :to="{ name: 'edit',params: {id:id,todo_name:todo_name, procent:procent, date:date} }" class="  rounded " >
            <button class="bg-blue-500 px-3 rounded mr-2" >edit</button>
            </nuxt-link>
            <button class="bg-red-600 px-3 rounded mr-2" @click="deleteTodo(id)">x</button>
        </div>

    </div>
        <div class="ml-2 flex flex-row  justify-evenly mt-2 sm:mt-0 sm:hidden  ">
            <nuxt-link :to="{ name: 'edit',params: {id:id,todo_name:todo_name, procent:procent, date:date} }" class="rounded " >
            <button class="bg-blue-500 px-3 rounded mr-2" >edit</button>
            </nuxt-link>
            <button class="bg-red-600 px-3 rounded mr-2" @click="deleteTodo(id)">x</button>
        </div>
    </div>
</template>

<script>
// to get api call
import TodoService from "@/services/TodoService";

export default {
  props: {
    id: {
      id: Number,
      required: true
    },
    todo_name: {
      name: String,
      required: true
    },

    date: {
      date: String
    },
    procent: {
      procent: Number
    }
  },
  methods: {
    deleteTodo: function(id) {
      this.$axios.$delete("http://localhost:8081/todos/" + id);
      // this.$router.push("/");
      //   TodoService.deleteTodo(id);
      window.location.reload(true);
    }
  }
};
</script>