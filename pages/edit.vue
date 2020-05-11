<template>
<div class="bg-verdigris min-h-screen pt-2 pb-16 ">
    <!-- Edit TODO number # -->
    <Title title="Edit TODO #" />
    <!-- Edit text -->
    <div class="flex flex-col sm:items-center ">
        <InputComp  placeholderText="Edit TODO text">
            <p> TODO text </p>
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
</div>
</template>

<script>
import InputComp from '@/components/Input'
import NormalNavbar from '@/components/NormalNavbar'
import MobileNavbar from '@/components/MobileNavbar'
import Title from '@/components/Title'

export default {
    components: {
        MobileNavbar,
        NormalNavbar,
        InputComp,
        Title
    }
}
</script>