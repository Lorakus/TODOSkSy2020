<template>
    <div class=" h-screen bg-verdigris">
      <div class="bg-verdigris py-6">


          <h1 class="text-arctic-lime font-bold text-center mt-10 text-5xl">TODO LIST</h1>


          <div class="mt-6 mx-10 flex">
            <input type="text" placeholder="New TODO" id="todo-input" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2" />
            <button class="bg-pale-cerulean hover:bg-oxford-blue text-arctic-lime  hover:text-flame rounded uppercase py-2 px-6 appearance-none focus:outline-none">
              add
            </button>
          </div>

          <div class="mt-8 mx-10">
            <ul class="todos flex-row" id="todos">
              <li class="bg-white border inline-block rounded w-auto py-2 px-2 w-full flex ">
                todo first
                <button class="bg-red-400 px-2 hover:bg-red-500 text-white font-bold ml-auto rounded-sm">
                  x
                </button>
              </li>
              <li class="bg-white border inline-block rounded w-auto py-2 px-2 w-full flex mt-2">
                todo second
                <button class="bg-red-400 px-2 hover:bg-red-500 text-white font-bold ml-auto rounded-sm">
                  x
                </button>
              </li>
              <li class="bg-white border inline-block rounded w-auto py-2 px-2 w-full flex mt-2">
                todo third
                <button class="bg-red-400 px-2 hover:bg-red-500 text-white font-bold ml-auto rounded-sm">
                  x
                </button>
              </li>
            </ul>
          </div>
        </div>
    </div>
</template>




<script lang="ts">
import Vue from 'vue'
export default Vue.extend({
  components: {
  }
})
</script>

