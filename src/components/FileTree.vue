<script setup lang="ts">
import { computed } from 'vue';
import FileTreeNode from './FileTreeNode.vue';

interface TreeNode {
  name: string;
  fullPath: string;
  isDirectory: boolean;
  isExpanded: boolean;
  children: Record<string, TreeNode>;
}

const props = defineProps<{
  paths: string[];
}>();

const emit = defineEmits<{
  (e: 'delete', fullPath: string): void;
}>();

// 将 flat paths 构建成树状对象
const root = computed(() => {
  const tree: Record<string, TreeNode> = {};
  
  // 确保按字典序或者层级显示比较好，但路径一般已经是排好序的
  const sortedPaths = [...props.paths].sort();

  sortedPaths.forEach(path => {
    const parts = path.split('/');
    let currentLevel = tree;
    let currentPath = '';

    parts.forEach((part, index) => {
      currentPath = currentPath ? `${currentPath}/${part}` : part;
      const isDirectory = index < parts.length - 1;
      
      if (!currentLevel[part]) {
        currentLevel[part] = {
          name: part,
          fullPath: currentPath,
          isDirectory,
          isExpanded: true, // 默认展开
          children: {}
        };
      } else if (isDirectory && !currentLevel[part].isDirectory) {
          // 在极少数情况下，可能是覆盖导致的异常，确保类型为目录
          currentLevel[part].isDirectory = true;
      }
      currentLevel = currentLevel[part].children;
    });
  });

  return tree;
});

function handleDelete(fullPath: string) {
  emit('delete', fullPath);
}
</script>

<template>
  <div class="file-tree w-full h-full overflow-y-auto overflow-x-hidden text-sm text-slate-300 pr-1 pb-2">
    <div v-if="paths.length === 0" class="text-slate-500 text-xs italic p-4 text-center">
        暂无依赖文件
    </div>
    <FileTreeNode
      v-for="node in root"
      :key="node.fullPath"
      :node="node"
      @delete="handleDelete"
    />
  </div>
</template>

<style scoped>
.file-tree::-webkit-scrollbar {
  width: 6px;
}
.file-tree::-webkit-scrollbar-track {
  background: transparent;
}
.file-tree::-webkit-scrollbar-thumb {
  background: #334155;
  border-radius: 3px;
}
.file-tree::-webkit-scrollbar-thumb:hover {
  background: #475569;
}
</style>
