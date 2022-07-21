<template>
  <div>
    <n-layout has-sider style="margin-top: 10%;">
      <n-tooltip :show="showTip" placement="bottom">
        <template #trigger>
            <n-input v-model:value="folder" type="text" placeholder="请输入文件夹路径" @click="showTip = false" class="chooseFolder"/>
        </template>
        <span>请检查下文件夹路径...</span>
      </n-tooltip>
      <n-button type="primary" class="chooseBtn" @click="chooseFolder">
        确认
      </n-button>
    </n-layout>
  </div>
</template>

<script>
import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
import { NInput, NButton, NLayout, NTooltip } from 'naive-ui'

export default {
  name: 'chooseFolder',
  components: {
    NInput,
    NButton,
    NLayout,
    NTooltip
  },
  data() {
    return {
      folder: '',
      showTip: false,
    }
  },
  methods: {
    async readDir(folder) {
      this.showTip = false;
      try {
        await readDir(folder, { dir: BaseDirectory.App, recursive: false });
        localStorage.setItem("mainFolder", folder)
        this.$router.push('writingRoom');
      } catch (e) {
        this.showTip = true;
      }
    },
    chooseFolder() {
      this.readDir(this.folder);
    }
  }
}
</script>

<style scoped>
.chooseFolder {
  margin: 3% 3% 30px 20%;
  min-width: 300px;
}
.chooseBtn {
  margin: 3% 20% 30px 0;
  min-width: 80px;
}
</style>
