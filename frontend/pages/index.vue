<template>
  <div class=" min-h-screen bg-verdigris pt-1 pb-2">
    
    <Title title="TODO List" class=""/>

    <div class="sm:w-max-400px m-auto mb-24">
      <!--TODO component 3props, todo_name, date, procent -->
      
      <Todo v-for="(todo,idx) in todos" v-bind:key="idx" :todo_name="todo.title" :date="todo.deadline" :procent="todo.procent" :id="todo.id"/>
    
      
    </div>
  </div>
</template>




<script>
import Title from "@/components/Title";
import Todo from "@/components/Todo";
import Vue from "vue";

// import service to make api call
import TodoService from "@/services/TodoService.js";

export default Vue.extend({
  data() {
    return {
      todos: []
    };
  },
  components: {
    Title,
    Todo
  },
  created() {
    const response = TodoService.getTodos().then(response => {
      this.todos = response.data;
    });
  }
});
</script>

