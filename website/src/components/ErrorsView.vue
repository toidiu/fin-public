<template>
  <div class="errors">
    <p v-if="errors.length">
      Please fix the following errors
      <ul>
        <li v-for="error in errors">{{ error }}</li>
      </ul>
    </p>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    errors: {}
  }
};
</script>
