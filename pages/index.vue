<template>
    <div class="h-screen bg-verdigris">
      <div class="bg-verdigris py-6">

        <Title title="TODO List"/>
        <!-- <h1 class="text-arctic-lime font-bold text-center mt-10 text-5xl">TODO List</h1> -->

        <div class="sm:w-max-400px m-auto">

          <div class="mt-6 mx-10 flex mb-8">
            <input type="text" placeholder="New TODO" id="todo-input" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2" />
            <button class="bg-pale-cerulean hover:bg-oxford-blue text-arctic-lime  hover:text-flame rounded uppercase py-2 px-6 appearance-none focus:outline-none transform duration-500  ">
              add
            </button>
          </div>
          
          <Todo todo_name="test 1" time="30min"/>
          <Todo todo_name="test 2" time="60min"/>
          <Todo todo_name="test 3" time="15min"/>
          <Todo todo_name="test 4" time="2h"/>
          <Todo todo_name="test 5" time="25h"/>


        </div>
        
      </div>
    </div>
</template>




<script>
import Title from '@/components/Title'
import Todo from '@/components/Todo'
import Vue from 'vue'


export default Vue.extend({
  components: {
    Title,
    Todo
  }
})
</script>

