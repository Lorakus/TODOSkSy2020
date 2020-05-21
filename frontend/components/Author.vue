<template>
    <div class="border-solid border-gray-600 bg-white  border-2  flex p-2 m-2  rounded-lg sm:rounded-full content-center h-20 sm:h-32 sm:w-3/4 lg:w-2/4 xl:w-2/4  xl:m-8">
        <div class="w-10 sm:w-16  self-center border-0 mr-4 sm:ml-8  ">
            <slot class="rounded-lg" name="photo"></slot>
        </div>

        <div class="flex flex-col content-center sm:justify-center ">
            
            <div class="text-xs mb-2 font-serif sm:text-base ">
                <slot name="name"></slot>
            </div>

            <div class="text-xs sm:text-base ">
                <slot name="description"></slot>
            </div>
        </div>
        
    </div>
</template>