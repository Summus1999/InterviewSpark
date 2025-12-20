<template>
  <div class="tooltip-wrapper">
    <div
      @mouseenter="handleHover"
      @mouseleave="handleLeave"
      class="tooltip-target"
    >
      <slot></slot>
    </div>
    
    <transition name="tooltip-fade">
      <div
        v-if="showTooltip"
        :class="['tooltip-bubble', positionClass]"
        @click.stop
      >
        <div class="tooltip-content">
          {{ content }}
        </div>
        <div v-if="allowDismiss" class="tooltip-actions">
          <button @click="handleDismiss" class="dismiss-btn">
            不再显示
          </button>
          <button @click="closeTooltip" class="close-btn">
            知道了
          </button>
        </div>
        <div :class="['tooltip-arrow', arrowPosition]"></div>
      </div>
    </transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { TooltipManager } from '../services/settings'

interface Props {
  content: string
  position?: 'top' | 'bottom' | 'left' | 'right'
  tooltipId: string
  allowDismiss?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  position: 'top',
  allowDismiss: true
})

const showTooltip = ref(false)
const isDismissed = ref(false)

onMounted(() => {
  isDismissed.value = TooltipManager.isDismissed(props.tooltipId)
})

const positionClass = computed(() => `position-${props.position}`)
const arrowPosition = computed(() => {
  switch (props.position) {
    case 'top': return 'arrow-bottom'
    case 'bottom': return 'arrow-top'
    case 'left': return 'arrow-right'
    case 'right': return 'arrow-left'
    default: return 'arrow-bottom'
  }
})

const handleHover = () => {
  if (!isDismissed.value) {
    showTooltip.value = true
  }
}

const handleLeave = () => {
  showTooltip.value = false
}

const closeTooltip = () => {
  showTooltip.value = false
}

const handleDismiss = () => {
  TooltipManager.dismiss(props.tooltipId)
  isDismissed.value = true
  showTooltip.value = false
}
</script>

<style scoped>
.tooltip-wrapper {
  position: relative;
  display: inline-block;
}

.tooltip-target {
  display: inline-block;
}

.tooltip-bubble {
  position: absolute;
  z-index: 1001;
  background: #2d3748;
  color: white;
  padding: 0.75rem 1rem;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  max-width: 280px;
  font-size: 0.9rem;
  line-height: 1.5;
}

.position-top {
  bottom: calc(100% + 12px);
  left: 50%;
  transform: translateX(-50%);
}

.position-bottom {
  top: calc(100% + 12px);
  left: 50%;
  transform: translateX(-50%);
}

.position-left {
  right: calc(100% + 12px);
  top: 50%;
  transform: translateY(-50%);
}

.position-right {
  left: calc(100% + 12px);
  top: 50%;
  transform: translateY(-50%);
}

.tooltip-content {
  margin-bottom: 0.5rem;
}

.tooltip-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
  padding-top: 0.5rem;
  border-top: 1px solid rgba(255, 255, 255, 0.2);
}

.dismiss-btn,
.close-btn {
  padding: 0.3rem 0.75rem;
  border: none;
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.dismiss-btn {
  background: transparent;
  color: #cbd5e0;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.dismiss-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.close-btn {
  background: #667eea;
  color: white;
}

.close-btn:hover {
  background: #5a67d8;
}

.tooltip-arrow {
  position: absolute;
  width: 0;
  height: 0;
  border-style: solid;
}

.arrow-bottom {
  bottom: -6px;
  left: 50%;
  transform: translateX(-50%);
  border-width: 6px 6px 0 6px;
  border-color: #2d3748 transparent transparent transparent;
}

.arrow-top {
  top: -6px;
  left: 50%;
  transform: translateX(-50%);
  border-width: 0 6px 6px 6px;
  border-color: transparent transparent #2d3748 transparent;
}

.arrow-right {
  right: -6px;
  top: 50%;
  transform: translateY(-50%);
  border-width: 6px 0 6px 6px;
  border-color: transparent transparent transparent #2d3748;
}

.arrow-left {
  left: -6px;
  top: 50%;
  transform: translateY(-50%);
  border-width: 6px 6px 6px 0;
  border-color: transparent #2d3748 transparent transparent;
}

.tooltip-fade-enter-active,
.tooltip-fade-leave-active {
  transition: opacity 0.2s ease;
}

.tooltip-fade-enter-from,
.tooltip-fade-leave-to {
  opacity: 0;
}
</style>
