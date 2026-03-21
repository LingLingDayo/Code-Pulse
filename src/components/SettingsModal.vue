<script setup lang="ts">
import { ref } from "vue";

// Props from App.vue
const props = defineProps<{
  show: boolean;
  maxDepth: number;
  generateTree: boolean;
  autoGenerate: boolean;
}>();

const emit = defineEmits<{
  (e: "update:show", value: boolean): void;
  (e: "update:maxDepth", value: number): void;
  (e: "update:generateTree", value: boolean): void;
  (e: "update:autoGenerate", value: boolean): void;
}>();

// 此处为组件内部维护的其他状态示例
const ignoreExts = ref(".git, node_modules, dist");
const customPrompt = ref("");
const parseMode = ref("normal"); // radio
const includedTypes = ref(["vue", "ts"]); // checkbox

function close() {
  emit("update:show", false);
}

function updateMaxDepth(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:maxDepth", parseInt(target.value, 10));
}

function updateGenerateTree(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:generateTree", target.checked);
}

function updateAutoGenerate(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:autoGenerate", target.checked);
}
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm">
    <div class="bg-slate-800 border border-slate-700 rounded-2xl shadow-xl w-full max-w-2xl max-h-[90vh] flex flex-col transform transition-all">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-slate-700 flex justify-between items-center bg-slate-800/50 shrink-0">
        <h3 class="text-xl font-bold text-slate-100 flex items-center">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2 text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          偏好设置 (Settings)
        </h3>
        <button @click="close" class="text-slate-400 hover:text-slate-200 transition-colors p-1 rounded-md hover:bg-slate-700">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Body -->
      <div class="p-6 overflow-y-auto space-y-8 flex-1 custom-scrollbar">
        <!-- 基础配置 -->
        <div class="space-y-4">
          <h4 class="text-sm font-semibold text-blue-400 uppercase tracking-widest border-b border-slate-700/50 pb-2">基础设置 (Basic Options)</h4>
          
          <!-- 此为：滑块 (Slider) -->
          <div class="space-y-2">
            <label for="modal-depth" class="block text-sm font-semibold text-slate-300">递归解析深度 (Recursive Parsing Depth)</label>
            <p class="text-xs text-slate-500 pb-1">设置文件解析依赖扫描的层级数。设置得越高，包含的相关文件越多。</p>
            <div class="flex items-center space-x-3">
              <input 
                id="modal-depth" 
                type="range" 
                :value="maxDepth"
                @input="updateMaxDepth"
                min="0" 
                max="10"
                class="flex-1 w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-blue-500"
              />
              <span class="text-lg font-mono font-bold text-blue-400 w-8 text-center">{{ maxDepth }}</span>
            </div>
          </div>

          <!-- 此为：输入框 (Input) -->
          <div class="space-y-2 mt-4">
            <label class="block text-sm font-semibold text-slate-300">忽略后缀或目录 (Ignore Patterns)</label>
            <p class="text-xs text-slate-500 pb-1">通过英文逗号分隔，匹配的目录或文件将不被解析。</p>
            <input 
              type="text" 
              v-model="ignoreExts"
              class="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow"
              placeholder=".git, node_modules, dist"
            />
          </div>

          <!-- 此为：文本框 (Textarea) -->
          <div class="space-y-2 mt-4">
            <label class="block text-sm font-semibold text-slate-300">自定义提示词首部 (Custom Prompt Header)</label>
            <p class="text-xs text-slate-500 pb-1">可以在生成的上下文前面插入所需的引导信息。</p>
            <textarea 
              v-model="customPrompt"
              rows="3"
              class="w-full bg-slate-900 border border-slate-600 rounded-lg px-3 py-2 text-sm text-slate-200 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow resize-y custom-scrollbar"
              placeholder="请输入自定义提示词..."
            ></textarea>
          </div>
        </div>

        <!-- 高级与显示 -->
        <div class="space-y-4 pt-4">
          <h4 class="text-sm font-semibold text-purple-400 uppercase tracking-widest border-b border-slate-700/50 pb-2">解析选项 (Parse Options)</h4>
          
          <!-- 此为：开关 (Switch) - 文件树 -->
          <label class="flex items-center justify-between cursor-pointer p-3 bg-slate-900/50 rounded-lg border border-slate-700/50 hover:bg-slate-900/70 transition-colors">
            <div class="flex flex-col">
              <span class="text-sm font-semibold text-slate-200">顶部生成文件树结构</span>
              <span class="text-xs text-slate-500 mt-0.5">结果中最开头将包含解析目录的层级树状图。</span>
            </div>
            <div class="relative">
              <input type="checkbox" :checked="generateTree" @change="updateGenerateTree" class="sr-only peer">
              <div class="w-11 h-6 bg-slate-600 rounded-full peer peer-checked:bg-blue-500 peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all"></div>
            </div>
          </label>

          <!-- 此为：开关 (Switch) - 自动生成 -->
          <label class="flex items-center justify-between cursor-pointer p-3 bg-slate-900/50 rounded-lg border border-slate-700/50 hover:bg-slate-900/70 transition-colors">
            <div class="flex flex-col">
              <span class="text-sm font-semibold text-slate-200">选择文件后立即解析</span>
              <span class="text-xs text-slate-500 mt-0.5">如果关闭，在拖拽或选择路径后需要手动点击“生成”按钮。</span>
            </div>
            <div class="relative">
              <input type="checkbox" :checked="autoGenerate" @change="updateAutoGenerate" class="sr-only peer">
              <div class="w-11 h-6 bg-slate-600 rounded-full peer peer-checked:bg-blue-500 peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:border-slate-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all"></div>
            </div>
          </label>

          <!-- 此为：单选 (Radio) -->
          <div class="space-y-2 mt-4 inline-block w-full">
            <label class="block text-sm font-semibold text-slate-300 mb-2">默认解析模式 (Parse Mode)</label>
            <div class="flex gap-6">
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="radio" value="normal" v-model="parseMode" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">普通模式</span>
              </label>
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="radio" value="strict" v-model="parseMode" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">严格模式</span>
              </label>
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="radio" value="smart" v-model="parseMode" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">智能过滤</span>
              </label>
            </div>
          </div>

          <!-- 此为：多选 (Checkbox) -->
          <div class="space-y-2 mt-4 inline-block w-full">
            <label class="block text-sm font-semibold text-slate-300 mb-2">特殊偏好的文件类型 (Included Prefs)</label>
            <div class="flex flex-wrap gap-6">
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="checkbox" value="vue" v-model="includedTypes" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 rounded focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">.vue</span>
              </label>
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="checkbox" value="ts" v-model="includedTypes" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 rounded focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">.ts</span>
              </label>
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="checkbox" value="rs" v-model="includedTypes" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 rounded focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">.rs</span>
              </label>
              <label class="flex items-center space-x-2 cursor-pointer">
                <input type="checkbox" value="json" v-model="includedTypes" class="w-4 h-4 text-blue-600 bg-slate-700 border-slate-600 rounded focus:ring-blue-500 focus:ring-1">
                <span class="text-sm text-slate-300">.json</span>
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="px-6 py-4 bg-slate-900/50 border-t border-slate-700 flex justify-end shrink-0">
        <button 
          @click="close" 
          class="px-5 py-2 bg-blue-600 hover:bg-blue-500 text-white font-semibold rounded-lg shadow-md transition-colors"
        >
          完成 (Done)
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #334155;
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #475569;
}
</style>
