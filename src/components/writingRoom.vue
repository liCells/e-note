<template>
  <div>
    <n-grid x-gap="6" :cols="12">
      <n-gi span="2">
        <n-menu
            v-model:value="activeFile"
            :indent="12"
            :options="files"
        />
      </n-gi>
      <n-gi span="10">
        <div>
          <div id="article" class="preview"></div>
        </div>
      </n-gi>
    </n-grid>
  </div>
</template>

<script>
import {NMenu, NGi, NGrid} from 'naive-ui'
import Vditor from 'vditor';
import 'vditor/dist/index.css';

export default {
  name: 'writingRoom',
  components: {
    NMenu,
    NGi,
    NGrid
  },
  data() {
    return {
      activeFile: '',
      files: [

      ]
    }
  },
  mounted() {
    this.init();
  },
  methods: {
    init() {
      this.content = new Vditor('article', {
        minHeight: 500,
        toolbar: [],
        mode: 'ir',
        cache: {
          enable: false,
        },
        options: {
          preview: {
            hljs: {
              lineNumber: true
            }
          }
        }
      })
    }
  }
}
</script>

<style scoped>
.preview {
  border: unset;
}
</style>
