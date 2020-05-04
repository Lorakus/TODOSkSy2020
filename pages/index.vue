<template>
  <div class=" min-h-screen bg-verdigris pt-1 pb-2">
    
    <Title title="TODO List"/>


    <div class="sm:w-max-400px m-auto mb-24">
      <!-- add new TODO input -->
      <div class="flex mt-6 mx-10 mb-12">
        <input type="text" placeholder="New TODO" id="todo-input" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2" />
        <button class="bg-pale-cerulean hover:bg-oxford-blue text-arctic-lime  hover:text-flame rounded uppercase py-2 px-6 appearance-none focus:outline-none transform duration-500">
          add
        </button>
      </div>
      <!-- TODOs -->
      <Todo todo_name="test 1" time="30min"/>
      <Todo todo_name="test 2" time="60min"/>
      <Todo todo_name="test 3" time="15min"/>
      <Todo todo_name="test 4" time="2h"/>
      <Todo todo_name="test 4" time="2h"/>
      <Todo todo_name="test 4" time="2h"/>
      <Todo todo_name="test 4" time="2h"/>

      
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

