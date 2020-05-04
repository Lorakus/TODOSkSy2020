<template>
    <div class=" border-solid border-gray-600  border-2  flex p-2 m-2  rounded-lg content-center h-20 ">
        <div class="w-10  self-center border-0 mr-4 ">
            <slot class="rounded-lg" name="photo"></slot>
        </div>
        <div class="flex flex-col content-center ">
            <div class="text-xs mb-2 font-serif">
                <slot name="name"></slot>
            </div>
            <div class="text-xs ">
                <slot name="description"></slot>
            </div>
        </div>
    </div>
</template>