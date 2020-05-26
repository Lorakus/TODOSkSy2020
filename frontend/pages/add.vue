<template>
    <div class="bg-verdigris h-screen">
      <div class="bg-verdigris py-2">
        <Title title="Add New TODO"/>


        <!-- add Item form -->
        <form class=" w-full sm:w-max-400px m-auto">
          <div class="flex flex-col mt-2 mx-1 sm:mx-8 mb-12">
            
            <!-- todo name maxlenght 160 -->
            <div class="my-2">
              <label for="" class="text-xl text-white">TODO Name:</label>
              <input v-model="todo.title" type="text" maxlength="160" placeholder="New TODO" id="todo-input" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2" />
            </div>

            <!-- input procent range 1-100% -->
            <div class="my-2">
              <label for="" class="text-xl text-white">TODO %:</label>
              <input v-model="todo.procent" type="number" maxlength="160" placeholder="%" id="todo-input" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2" />
            </div>

            <!-- input date -->
            <div class="flex my-2 justify-center mb-3  sm:justify-between ">
              <div class="flex flex-col my-1 text-center sm:text-left">
                <label for="" class="text-xl text-white">Deadline:</label>
                <input v-model="todo.deadline" type="text" placeholder="date: DD/MM/YYYY" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2">
              </div>
              <!-- button for PC -->
              <div class="hidden sm:flex ">
                <button @click="addTodo" class="my-8 w-64 bg-pale-cerulean hover:bg-blue-500 text-arctic-lime  hover:text-black rounded uppercase py-2 px-6 appearance-none focus:outline-none transform duration-500">
                  add
                </button>
              </div>
            </div> 
            <!-- button mobile -->
            <div class="flex justify-center sm:hidden">
              <button @click="addTodo" class=" w-64 bg-pale-cerulean hover:bg-blue-500 text-arctic-lime  hover:text-black rounded uppercase py-2 px-6 appearance-none focus:outline-none transform duration-500">
                add
              </button>
            </div>

          </div>
        </form>
      </div>
    </div>
</template>


<script>
import Title from '@/components/Title';

import TodoService from '@/services/TodoService'

export default {
    components:{
        Title,
    },
    data() {
      return {
        time1: null,
        procent: 50,
        todo: {
          title: '',
          procent: Number,
          deadline: ''
        }
      }
    },
    methods: {
        changeProcent() {
            this.$emit('changeProcent', this.procent);
        },
        addTodo(){
          TodoService.addTodo(this.todo)
          // .then(()=>{
          //   this.$router.push("/")
          // })
        }
    }
}
</script>

<style>
    @media screen and (-webkit-min-device-pixel-ratio: 0) {
        input[type="range"]::-webkit-slider-thumb {
            width: 15px;
            -webkit-appearance: none;
  			appearance: none;
            height: 15px;
            cursor: ew-resize;
            background: #FFF;
            box-shadow: -405px 0 0 400px rgb(14, 221, 211);
            border-radius: 50%;
        }
    }
</style>