<template>
  <div class="bg-verdigris" >
    <NormalNavbar class="hidden md:grid" />
    <nuxt />
    <MobileNavbar class="md:hidden absolute bottom-0 w-full" style="position: fixed; bottom: 0px" />
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

