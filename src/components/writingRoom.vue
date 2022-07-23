<template>
  <div>
    <n-grid x-gap="6" :cols="12">
      <n-gi span="2">

        <n-menu
            v-model:value="activateTheTag"
            :indent="12"
            :options="tags"
            @click="activateTheFile = null; theBodyShow = 'store'"
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
          <div id="article" class="preview" v-if="theBodyShow === 'vditor'"></div>
          <div v-else-if="theBodyShow === 'store'">store</div>
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
import globalConfig from "@/conf";

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
      theBodyShow: null,
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
  created() {
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
      this.theBodyShow = 'vditor';
    },
  }
}
</script>

<style scoped>
.preview {
  border: unset;
}
</style>
