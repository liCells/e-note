<template>
  <router-view></router-view>
</template>

<script>
export default {
  data() {
    return {

    }
  },
  created() {
    this.check();
  },
  methods: {
    check() {
      let val = localStorage.getItem("mainFolder");
      if (val == null || val === '') {
        this.$router.push("chooseFolder");
      } else {
        this.$router.push("writingRoom");
      }
    }
  }
}
</script>

<style>
</style>
