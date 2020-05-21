<template>
        <h1 class="text-arctic-lime font-bold text-center mt-10 text-5xl">{{title}}</h1>
</template>

<script>
export default {
    props: {
        title: {
            name: String,
            required: true,
        },
    }
}
</script>