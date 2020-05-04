<template>
  <div class="overflow-scroll bg-verdigris">
    <NormalNavbar class="hidden md:grid" />
    <nuxt />
    <MobileNavbar class="md:hidden bottom-auto sm:bottom-0 absolute w-full"/>
  </div>
</template>


<script>
import NormalNavbar from '@/components/NormalNavbar'
import MobileNavbar from '@/components/MobileNavbar'


export default {
  components:{
    NormalNavbar,
    MobileNavbar
  },
  data() {
    return {
    }
  },
}
</script>

