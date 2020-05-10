<template>
    <div class="bg-verdigris h-screen">
      <div class="bg-verdigris py-2">

        <Title title="Add New TODO"/>

        <div class=" w-full sm:w-max-400px m-auto">
          <div class="flex flex-col mt-2 mx-1 sm:mx-8 mb-12">

            <div class="my-2">
              <label for="" class="text-xl text-white">TODO Name:</label>

              <input type="text" placeholder="New TODO" id="todo-input" class="shadow appearance-none border rounded w-full py-2 px-6 mr-2 text-gray-700 leading-tight focus:outline-none  focus:shadow-outline-arctic-lime px-2" />
            </div>

            <div class="my-2 flex flex-col ">
                <label for="" class="text-xl text-white">Procent: {{procent}}</label>
                <input v-model="procent" class="rounded-lg overflow-hidden appearance-none bg-gray-400 h-10" type="range" min="0" max="100" step="1" @change="changeProcent"/>
            </div>

            <div class="flex my-2 justify-center mb-3  sm:justify-between ">
              <div class="flex flex-col my-1 text-center ">
                <label for="" class="text-xl text-white">Deadline:</label>
                <input type="text" placeholder="date">
              </div>

              <div class="hidden sm:flex ">
                <button class="my-8 w-64 bg-pale-cerulean hover:bg-blue-500 text-arctic-lime  hover:text-black rounded uppercase py-2 px-6 appearance-none focus:outline-none transform duration-500">
                  add
                </button>
              </div>
            </div> 

            <div class="flex justify-center sm:hidden">

              <button class=" w-64 bg-pale-cerulean hover:bg-blue-500 text-arctic-lime  hover:text-black rounded uppercase py-2 px-6 appearance-none focus:outline-none transform duration-500">
                add
              </button>
            </div>

          </div>
        </div>
        </div>
    </div>
</template>


<script>
import Title from '@/components/Title';

export default {
    components:{
        Title,
    },
    data() {
      return {
        time1: null,
        procent: 50,
      }
    },
    methods: {
        changeProcent() {
            this.$emit('changeProcent', this.procent);
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