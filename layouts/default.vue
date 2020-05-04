<template>
  <div class="overflow-scroll bg-verdigris">
    <NormalNavbar class="hidden md:grid" />
    <MobileNavbar class="md:hidden absolute w-full bottom-0"/>
    <nuxt />
    
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

