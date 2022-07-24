# m-note

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

- [x] 指定库路径
- [x] 构建页面框架
- [ ] store
  - [ ] 存储知识点
  - [ ] 精确匹配
  - [ ] 模糊匹配
  - [ ] 关键词配置 @关键词@
  - [ ] 指定标签
  - [ ] 展示知识点
- [ ] Settings 系统配置
- [ ] 文章
  - [ ] 编写时可匹配
  - [ ] 右侧展示关联知识点
  - [ ] 分享 可选择是否要自动补充知识点到文章结尾
  - [ ] 匹配知识点
    - [ ] Enter触发关联
    - [ ] Shift + Enter 拷贝整个知识点到当前位置
    - [ ] Control + Shift + Enter 拷贝整个知识点到当前位置, 并删除原知识点

> 知识库用来存储知识点, 均为短文或几句话
>
> 真正编写文章时, 通过输入～呼出匹配精确知识点, 输入@呼出模糊匹配知识点

### 预想效果

> 左侧为整体菜单, 中间为编辑器, 右侧含有多个tab, 一个是匹配文章tag的知识点,
> 一个是文章中搜索到后设置关联的知识点
