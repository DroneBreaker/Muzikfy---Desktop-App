<template>
    <div>
        <h1 class="text-center pt-[14rem] text-5xl">{{ introMessage }}</h1>
    </div>
</template>

<script setup>
/**
 * IMPORTS
*/
 import { ref, onMounted } from "vue";
 import { useRouter } from "vue-router"; // To handle navigation


/**
 * DECLARATIONS
*/
const router = useRouter();
// const greetMsg = ref("Welcome to Muzikfy."); // The message to show


/**
 * PROPS
*/
 const props = defineProps({
    introMessage: {
        type: String,
        default: 'Welcome to Muzikfy',
        required: true
    }
 })

onMounted(() => {
  setTimeout(() => {
    // After 4 seconds, navigate to the login page
    router.push("/login"); 
  }, 4000);
});
</script>

<style lang="scss" scoped>

</style>