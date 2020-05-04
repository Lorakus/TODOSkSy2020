<template>
    <div class=" border-solid border-gray-600  border-2 w-1/3 flex p-4 m-2 rounded-full">
        <div class="flex flex-col content-center p-4">
            <div class="">
                <slot name="name"></slot>
            </div>
            <div>
                <slot name="description"></slot>
            </div>
        </div>
    </div>
</template>