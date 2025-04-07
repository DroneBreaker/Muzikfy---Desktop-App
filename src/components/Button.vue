<template>
    <button class="h-[3rem] w-[8rem] rounded-md !bg-gray-100 text-sm !text-black cursor-pointer">{{ buttonText }}</button>
</template>

<script setup>
/**
 * PROPS
 */
 const props = defineProps({
    buttonText: {
        type: String,
        default: 'Button'
    }
 })
</script>

<style lang="scss" scoped>

</style>