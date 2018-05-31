<template>
  <div class="ImageDisplay">
    <div
      v-for="image in imageData"
      :key="image"
      class="ImageDisplay-imageContainer"
    >
      <img
        :src="image"
        class="ImageDisplay-image"
      >
    </div>
  </div>
</template>

<script>
export default {
  name: 'ImageDisplay',
  data() {
    return {
      imageData: {},
    };
  },

  mounted() {
    this.fetchJson();
  },

  methods: {
    async fetchJson() {
      const res = await fetch('http://localhost:8000/test', { method: 'GET' });
      const json = await res.json();
      const data = json.map(url => `http://localhost:8000/${url}`);
      this.imageData = data;
    },
  },
};
</script>

<style scoped>
.ImageDisplay {
  display: flex;
  flex-flow: row wrap;
  width: 100vw;
}

.ImageDisplay-imageContainer {
  object-fit: cover;
  width: 100%;
}

.ImageDisplay-image {
  width: 50vw;
}

h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
