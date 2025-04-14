<template>
    <div class="pt-[20%] justify-items-center">
        <h1 class="text-center text-7xl h-[5rem] mb-6 ml-[5rem] bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-pink-500 tracking-light">
            {{ loginIntroMessage }}
        </h1>

        <form>
            <div class="mb-6">
                <input class="p-2 px-4 rounded-lg border border-gray-200 w-[140%] h-[3rem]" placeholder="Email or Username" type="text" name="emailOrUsername" id="emailOrUsername">
            </div>

            <div class="mb-6">
                <input class="p-2 px-4 rounded-lg border border-gray-200 w-[140%] h-[3rem]" placeholder="Password" type="password" name="password" id="password">
            </div>
        </form>

        <div>
            <Button class="ml-[5rem]" buttonText="Login"/>
        </div>
    </div>
</template>

<script setup>
/**
 * IMPORTS
 */
 import Button from '../components/Button.vue';

/**
 * PROPS
 */
 const props = defineProps({
    loginIntroMessage: {
        type: String,
        default: "Login to Muzikfy",
    }
 })
</script>

<style lang="scss" scoped>

</style>