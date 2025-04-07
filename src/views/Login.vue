<template>
    <div class="pt-[20%] justify-items-center">
        <h1 class="text-center text-7xl mb-6 ml-[5rem]">Login to Muzikfy</h1>

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
</script>

<style lang="scss" scoped>

</style>