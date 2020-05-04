<template>
  <div class="bg-verdigris">
    <MobileNavbar id="mobile-navbar" class="md:hidden w-full bottom-0"/>
    <NormalNavbar class="hidden md:grid" />
    <nuxt/>
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

<style>

#mobile-nabar{
  width: 100%;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: none;
  box-sizing: border-box;
}

</style>

