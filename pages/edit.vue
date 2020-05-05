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
    <InputComp>
        <p > TODO text </p>
    </InputComp>

    <!-- Edit TODO deadline -->
    <InputComp>
        <p> TODO deadline </p>
    </InputComp>

    <!-- Edit %  -->
    <InputComp>
        <p> TODO % left </p>
    </InputComp>
</div>
</template>

<script>
import InputComp from '@/components/Input'

export default {
    components: {
        InputComp
    }
}
</script>