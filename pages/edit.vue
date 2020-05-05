<template>
<div>
    <!-- Edit TODO number # -->
    <div>
        Edit TODO #
        <br>
        <br>
        <br>
    </div>
    <!-- Edit text -->
    <InputComp placeholderText="Edit TODO text">
        <p > TODO text </p>
    </InputComp>

    <!-- Edit TODO deadline -->
    <InputComp placeholderText="Edit TODO deadline">
        <p> TODO deadline </p>
    </InputComp>

    <!-- Edit %  -->
    <InputComp placeholderText="Edit TODO % left">
        <p> TODO % left </p>
    </InputComp>
</div>
</template>

<script>
import InputComp from '@/components/Input'
import NormalNavbar from '@/components/NormalNavbar'
import MobileNavbar from '@/components/MobileNavbar'

export default {
    components: {
        MobileNavbar,
        NormalNavbar,
        InputComp
    }
}
</script>