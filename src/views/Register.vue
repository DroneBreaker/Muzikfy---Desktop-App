<template>
    <div class="pt-[5%] justify-items-center">
        <h1 class="text-center text-6xl h-[5rem] mb-4 ml-[5rem] bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-pink-500 tracking-light">
            {{ registerMessage }}
        </h1>

        <form>
            <div class="mb-6">
                <input class="p-2 px-4 rounded-lg border border-gray-200 w-[140%] h-[3rem]" placeholder="Full Name" type="text" name="fullName" id="fullName">
            </div>

            <div class="mb-6">
                <input class="p-2 px-4 rounded-lg border border-gray-200 w-[140%] h-[3rem]" placeholder="Username" type="text" name="username" id="username">
            </div>

            <div class="mb-6">
                <input class="p-2 px-4 rounded-lg border border-gray-200 w-[140%] h-[3rem]" placeholder="Password" type="password" name="password" id="password">
            </div>

            <div class="mb-6">
                <input class="p-2 px-4 rounded-lg border border-gray-200 w-[140%] h-[3rem]" placeholder="Confirm Password" type="password" name="confirmPassword" id="confirmPassword">
            </div>
        </form>

        <p class="mb-4 text-sm ml-18">
            {{ gotAccountMsg }}
            <RouterLink to="/login">Log in.</RouterLink>
        </p>

        <div>
            <Button class="ml-[5rem]" buttonText="Register"/>
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
    registerMessage: {
        type: String,
        default: "Create an Account",
    },
    gotAccountMsg: {
        type: String,
        default: "Already have an Account?"
    }
 })
</script>

<style lang="scss" scoped>

</style>