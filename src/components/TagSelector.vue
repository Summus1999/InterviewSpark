<!--
  TagSelector.vue - Tag selector for individual questions
  
  Features:
  - Display assigned tags
  - Add/remove tags from question
  - Dropdown selection
-->
<template>
  <div class="tag-selector">
    <div class="assigned-tags">
      <span
        v-for="tag in assignedTags"
        :key="tag.id"
        class="tag-chip"
        :style="{ backgroundColor: tag.color }"
      >
        {{ tag.name }}
        <button @click="removeTag(tag.id!)" class="remove-btn">&times;</button>
      </span>
      <button
        v-if="!showDropdown"
        @click="showDropdown = true"
        class="add-tag-btn"
      >
        + 添加标签
      </button>
    </div>

    <div v-if="showDropdown" class="dropdown">
      <div class="dropdown-content">
        <div
          v-for="tag in availableTags"
          :key="tag.id"
          class="dropdown-item"
          @click="addTag(tag)"
        >
          <span class="tag-preview" :style="{ backgroundColor: tag.color }">
            {{ tag.name }}
          </span>
        </div>
        <div v-if="availableTags.length === 0" class="no-tags">
          暂无可用标签
        </div>
      </div>
      <button @click="showDropdown = false" class="close-dropdown">关闭</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import type { QuestionTag } from '../services/database'
import { getAllTags, getTagsForQuestion, addTagToQuestion, removeTagFromQuestion } from '../services/database'

const props = defineProps<{
  questionId: number
}>()

const emit = defineEmits<{
  (e: 'tagsChanged', tags: QuestionTag[]): void
}>()

const allTags = ref<QuestionTag[]>([])
const assignedTags = ref<QuestionTag[]>([])
const showDropdown = ref(false)

const availableTags = computed(() => {
  const assignedIds = new Set(assignedTags.value.map(t => t.id))
  return allTags.value.filter(t => !assignedIds.has(t.id))
})

onMounted(async () => {
  await loadData()
})

watch(() => props.questionId, async () => {
  await loadData()
})

const loadData = async () => {
  try {
    const [tags, assigned] = await Promise.all([
      getAllTags(),
      getTagsForQuestion(props.questionId)
    ])
    allTags.value = tags
    assignedTags.value = assigned
  } catch (error) {
    console.error('Failed to load tags:', error)
  }
}

const addTag = async (tag: QuestionTag) => {
  try {
    await addTagToQuestion(props.questionId, tag.id!)
    assignedTags.value.push(tag)
    showDropdown.value = false
    emit('tagsChanged', assignedTags.value)
  } catch (error) {
    console.error('Failed to add tag:', error)
  }
}

const removeTag = async (tagId: number) => {
  try {
    await removeTagFromQuestion(props.questionId, tagId)
    assignedTags.value = assignedTags.value.filter(t => t.id !== tagId)
    emit('tagsChanged', assignedTags.value)
  } catch (error) {
    console.error('Failed to remove tag:', error)
  }
}

// Expose refresh method
const refresh = async () => {
  await loadData()
}

defineExpose({ refresh })
</script>

<style scoped>
.tag-selector {
  position: relative;
}

.assigned-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.4rem;
  align-items: center;
}

.tag-chip {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0.2rem 0.5rem;
  color: white;
  border-radius: 10px;
  font-size: 0.75rem;
  font-weight: 500;
}

.remove-btn {
  background: transparent;
  border: none;
  color: white;
  font-size: 1rem;
  line-height: 1;
  cursor: pointer;
  padding: 0;
  margin-left: 0.2rem;
  opacity: 0.8;
}

.remove-btn:hover {
  opacity: 1;
}

.add-tag-btn {
  padding: 0.2rem 0.5rem;
  background: var(--bg-secondary, #f0f0f0);
  border: 1px dashed var(--border-color, #ccc);
  border-radius: 10px;
  font-size: 0.75rem;
  color: var(--text-secondary, #666);
  cursor: pointer;
  transition: all 0.2s;
}

.add-tag-btn:hover {
  background: var(--border-color, #e0e0e0);
  border-style: solid;
}

.dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 100;
  min-width: 180px;
  margin-top: 0.3rem;
  background: var(--card-bg, #fff);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.dropdown-content {
  max-height: 200px;
  overflow-y: auto;
  padding: 0.5rem;
}

.dropdown-item {
  padding: 0.4rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.2s;
}

.dropdown-item:hover {
  background: var(--bg-secondary, #f8f9ff);
}

.tag-preview {
  display: inline-block;
  padding: 0.2rem 0.6rem;
  color: white;
  border-radius: 10px;
  font-size: 0.8rem;
}

.no-tags {
  padding: 0.8rem;
  text-align: center;
  color: var(--text-secondary, #999);
  font-size: 0.85rem;
}

.close-dropdown {
  width: 100%;
  padding: 0.5rem;
  background: var(--bg-secondary, #f0f0f0);
  border: none;
  border-top: 1px solid var(--border-color, #e0e0e0);
  border-radius: 0 0 6px 6px;
  font-size: 0.8rem;
  color: var(--text-secondary, #666);
  cursor: pointer;
}

.close-dropdown:hover {
  background: var(--border-color, #e0e0e0);
}
</style>
