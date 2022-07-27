<template>
  <div class="body">
    <n-grid x-gap="6" :cols="36">
      <n-gi span="1">
        标签
      </n-gi>
      <n-gi span="35">
          <n-dynamic-tags v-model:value="tags" @update:value="updateTag">
            <template #trigger="{ activate, disabled }">
              <n-button
                  id="addTag"
                  size="small"
                  type="primary"
                  dashed
                  :disabled="disabled"
                  @click="activate()"
              >
                <template #icon>
                  <n-icon>
                    <Add />
                  </n-icon>
                </template>
                添加
              </n-button>
            </template>
          </n-dynamic-tags>
      </n-gi>
    </n-grid>

    <div id="article" class="preview"></div>
  </div>
</template>

<script>
import {NDivider, NDynamicTags, NGi, NGrid, NIcon, NButton, NCard} from 'naive-ui'
import {Add} from '@vicons/ionicons5';
import Vditor from 'vditor';
import 'vditor/dist/index.css';

export default {
  name: 'editor',
  components: {
    NDivider,
    NDynamicTags,
    NGi,
    NGrid,
    Add,
    NIcon,
    NButton,
    NCard,
  },
  data() {
    return {
      tags: [],
      content: '',
    }
  },
  mounted() {
    this.initVditor();
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
    },
    updateTag(val) {
      let set = new Set();
      val.forEach(item => {
        set.add(item.trim());
      })
      this.tags = [...set];
      this.autoClickTag();
    },
    autoClickTag() {
      setTimeout(() => {
        document.getElementById('addTag').click()
      }, 50)
    }
  }
}
</script>

<style scoped>
.body {
  margin-top: 1%;
  margin-right: 1%;
}
.preview {
  border: unset;
}
#article {
  margin-top: 1%;
}
</style>
