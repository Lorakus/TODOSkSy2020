<template>
<div class="sm:w-3/4 ">
    <div class="mb-4 ">
        <div class="mb-2">
            <label class=" text-sm font-bold" >
                <slot/>
            </label>
        </div>
      <input class=" bg-gray-200  border rounded w-full py-2 px-3 sm:h-16" :placeholder = "placeholderText" >
      
    </div>
</div>
</template>

<script>
export default {
    props: {
        placeholderText: {
            type: String,
            required: true
        }
    }
}
</script>