<template>
<div class="bg-verdigris min-h-screen pt-2 pb-16 ">
    <!-- Edit TODO number # -->
    <Title title="Edit TODO " />
    <div class="flex flex-col sm:items-center ">
        
        <!-- Edit text -->
        <div class="mb-4 sm:w-3/4 ">
            <div class="mb-2">
                <label class="  font-bold text-xl text-white" >
                    TODO Text
                </label>
            </div>
        <input class=" bg-gray-200  border rounded w-full py-2 px-3 sm:h-16"  v-model="todo.title" >
      
        </div>

        <!-- Edit TODO deadline -->
        <div class="mb-4 sm:w-3/4 ">
            <div class="mb-2">
                <label class="  font-bold text-xl text-white" >
                    TODO deadline
                </label>
            </div>
        <input class=" bg-gray-200  border rounded w-full py-2 px-3 sm:h-16"  v-model="todo.deadline" >
      
        </div>

        <!-- Edit %  -->
        <div class="mb-4 sm:w-3/4 ">
            <div class="mb-2">
                <label class="  font-bold text-xl text-white" >
                    TODO % left
                </label>
            </div>
        <input class=" bg-gray-200  border rounded w-full py-2 px-3 sm:h-16"  v-model="todo.procent" >
      
        </div>

        <button @click="saveEditedTodo()"> Save </button>
    </div>
</div>
</template>

<script>
//import InputComp from '@/components/Input'
import NormalNavbar from "@/components/NormalNavbar";
import MobileNavbar from "@/components/MobileNavbar";
import Title from "@/components/Title";

// import service to make api calls
import TodoService from "@/services/TodoService";

export default {
  components: {
    MobileNavbar,
    NormalNavbar,
    Title
  },
  data() {
    return {
      todo: {
        title: String,
        procent: Number,
        deadline: String
      }
    };
  },
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
  created() {
    console.log(this.todo);
    this.todo.title = this.todo_name;
    this.todo.deadline = this.date;
    this.todo.procent = this.procent;
  },
  methods: {
    //save edited todo

    saveEditedTodo: function() {
      this.todo.procent = Number(this.todo.procent);
      TodoService.putTodo(this.todo, this.id);
      this.$router.push("/");
    }
  }
};
</script>