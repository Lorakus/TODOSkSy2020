<template>
  <div>
    <NormalNavbar class="hidden md:block" />
    <nuxt />
    <MobileNavbar class="md:hidden bottom-0 absolute w-full"/>
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

