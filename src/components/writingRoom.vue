<template>
  <div>
    <n-grid x-gap="6" :cols="12">
      <n-gi span="2">

        <n-menu
            v-model:value="activateTheTag"
            :indent="12"
            :options="tags"
            @click="activateTheFile = null"
        />

        <n-divider/>

        <n-menu
            v-model:value="activateTheFile"
            :indent="12"
            :options="files"
            @click="activateTheTag = null"
        />
      </n-gi>
      <n-gi span="10">
        <div>
          <div id="article" class="preview" v-if="vditorShow"></div>
        </div>
      </n-gi>
    </n-grid>
  </div>
</template>

<script>
import {h} from 'vue'
import {NMenu, NGi, NGrid, NDivider, NIcon} from 'naive-ui'
import {Library, Settings} from '@vicons/ionicons5';
import Vditor from 'vditor';
import 'vditor/dist/index.css';
function renderIcon(icon) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

export default {
  name: 'writingRoom',
  components: {
    NMenu,
    NGi,
    NGrid,
    NDivider,
    NIcon,
  },
  data() {
    return {
      activateTheFile: null,
      activateTheTag: null,
      vditorShow: false,
      files: [
        {
          label: 'testVal',
          key: 'testVal'
        }
      ],
      tags: [
        {
          label: 'Store',
          key: 'Store',
          icon: renderIcon(Library)
        },
        {
          label: 'Settings',
          key: 'Settings',
          icon: renderIcon(Settings)
        },
      ]
    }
  },
  methods: {
    initVditor() {
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
      });
      this.vditorShow = true;
    },
  }
}
</script>

<style scoped>
.preview {
  border: unset;
}
</style>
