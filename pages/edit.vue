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
    <Input>
        <p> TODO text </p>
    </Input>

    <!-- Edit TODO deadline -->
    <Input>
        <p> TODO deadline </p>
    </Input>

    <!-- Edit %  -->
    <Input>
        <p> TODO % left </p>
    </Input>
</div>
</template>

<script>
import Input from '@/components/Input'

export default {
    components: {
        Input
    }
}
</script>