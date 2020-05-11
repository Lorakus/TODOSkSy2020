<template>
  <div class=" min-h-screen bg-verdigris pt-1 pb-2">
    
    <Title title="TODO List" class=""/>

    <div class="sm:w-max-400px m-auto mb-24">
      <!--TODO component 3props, todo_name, date, procent -->
      <Todo todo_name="Put together an epic puzzle." date="05.02.2020"  procent="22 %"/>
      <Todo todo_name="Watch a rom-com marathon." date="06.12.2021"  procent="75 %"/>
      <Todo todo_name="Sing along to some Disney songs." date="06.12.2021"  procent="23 %"/>
      <Todo todo_name="Get a workout in." date="06.12.2021" procent="90 %"/>
      <Todo todo_name="Write a letter." date="06.12.2021" procent="10 %"/>
      <Todo todo_name="Call a friend." date="06.12.2021" procent="40 %"/>
      <Todo todo_name="Have a dance party." date="06.12.2021" procent="66 %"/>

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
    Todo,
  }
})
</script>

