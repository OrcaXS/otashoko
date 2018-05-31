<template>
  <div class="home">
    <ImageDisplay />
  </div>
</template>

<script>
// @ is an alias to /src
import ImageDisplay from '@/components/ImageDisplay';

export default {
  name: 'Home',
  components: {
    ImageDisplay,
  },
};
</script>
