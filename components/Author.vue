<template>
    <div class=" border-solid border-gray-600 bg-red-100 border-4 w-1/3 flex">
        <div class = "w-24 h-32 " >
            <slot  name="photo"></slot>
        </div>
        <div class="">
            <slot name="name"></slot>
        </div>
        <div>
            <slot name="description"></slot>
        </div>
    </div>
</template>