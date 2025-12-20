<template>
  <div class="user-management">
    <div class="header">
      <h2>用户管理</h2>
      <button class="btn-primary" @click="showCreateDialog = true">
        <span class="icon">+</span>
        新建用户
      </button>
    </div>

    <div v-if="currentUser" class="current-user-card">
      <div v-if="currentUser.avatar_path" class="user-avatar-img">
        <img :src="getAvatarUrl(currentUser.avatar_path)" :alt="currentUser.username" />
      </div>
      <div v-else class="user-avatar" :style="{ backgroundColor: currentUser.avatar_color }">
        {{ currentUser.username.charAt(0) }}
      </div>
      <div class="user-info">
        <h3>当前用户</h3>
        <p class="username">{{ currentUser.username }}</p>
        <p class="created-at">创建于 {{ formatDate(currentUser.created_at) }}</p>
      </div>
      <button class="btn-guide" @click="handleResetOnboarding" title="重新查看使用引导">
        📖 查看使用引导
      </button>
    </div>

    <div class="users-list">
      <h3>所有用户</h3>
      <div class="user-cards">
        <div
          v-for="user in users"
          :key="user.id"
          class="user-card"
          :class="{ active: user.id === currentUser?.id }"
          @click="handleSwitchUser(user.id!)"
        >
          <div v-if="user.avatar_path" class="user-avatar-img">
            <img :src="getAvatarUrl(user.avatar_path)" :alt="user.username" />
          </div>
          <div v-else class="user-avatar" :style="{ backgroundColor: user.avatar_color }">
            {{ user.username.charAt(0) }}
          </div>
          <div class="user-details">
            <p class="name">{{ user.username }}</p>
            <p class="date">{{ formatDate(user.created_at) }}</p>
          </div>
          <div class="user-actions" @click.stop>
            <button class="btn-icon" @click="handleEditUser(user)" title="编辑">
              ✏️
            </button>
            <button
              class="btn-icon"
              @click="handleDeleteUser(user.id!)"
              title="删除"
              :disabled="user.id === 1"
            >
              🗑️
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create User Dialog -->
    <div v-if="showCreateDialog" class="modal-overlay" @click="showCreateDialog = false">
      <div class="modal-content" @click.stop>
        <h3>新建用户</h3>
        <form @submit.prevent="handleCreateUser">
          <div class="form-group">
            <label>用户名</label>
            <input
              v-model="newUser.username"
              type="text"
              placeholder="输入用户名"
              required
            />
          </div>
          <div class="form-group">
            <label>头像</label>
            <div class="avatar-preview">
              <div v-if="newUser.avatar_preview" class="preview-img">
                <img :src="newUser.avatar_preview" alt="预览" />
              </div>
              <div v-else class="preview-placeholder" :style="{ backgroundColor: newUser.avatar_color }">
                {{ newUser.username.charAt(0) || '?' }}
              </div>
              <button type="button" class="btn-upload" @click="handleSelectAvatar('create')">
                选择图片
              </button>
            </div>
          </div>
          <div class="form-group">
            <label>头像颜色</label>
            <div class="color-picker">
              <div
                v-for="color in colorOptions"
                :key="color"
                class="color-option"
                :style="{ backgroundColor: color }"
                :class="{ selected: newUser.avatar_color === color }"
                @click="newUser.avatar_color = color"
              />
            </div>
          </div>
          <div class="form-actions">
            <button type="button" class="btn-secondary" @click="showCreateDialog = false">
              取消
            </button>
            <button type="submit" class="btn-primary">创建</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Edit User Dialog -->
    <div v-if="showEditDialog" class="modal-overlay" @click="showEditDialog = false">
      <div class="modal-content" @click.stop>
        <h3>编辑用户</h3>
        <form @submit.prevent="handleUpdateUser">
          <div class="form-group">
            <label>用户名</label>
            <input
              v-model="editingUser.username"
              type="text"
              placeholder="输入用户名"
              required
            />
          </div>
          <div class="form-group">
            <label>头像</label>
            <div class="avatar-preview">
              <div v-if="editingUser.avatar_preview" class="preview-img">
                <img :src="editingUser.avatar_preview" alt="预览" />
              </div>
              <div v-else-if="editingUser.avatar_path" class="preview-img">
                <img :src="getAvatarUrl(editingUser.avatar_path)" alt="当前头像" />
              </div>
              <div v-else class="preview-placeholder" :style="{ backgroundColor: editingUser.avatar_color }">
                {{ editingUser.username.charAt(0) || '?' }}
              </div>
              <button type="button" class="btn-upload" @click="handleSelectAvatar('edit')">
                选择图片
              </button>
            </div>
          </div>
          <div class="form-group">
            <label>头像颜色</label>
            <div class="color-picker">
              <div
                v-for="color in colorOptions"
                :key="color"
                class="color-option"
                :style="{ backgroundColor: color }"
                :class="{ selected: editingUser.avatar_color === color }"
                @click="editingUser.avatar_color = color"
              />
            </div>
          </div>
          <div class="form-actions">
            <button type="button" class="btn-secondary" @click="showEditDialog = false">
              取消
            </button>
            <button type="submit" class="btn-primary">保存</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import {
  getAllUsers,
  getCurrentUser,
  createUser,
  switchUser,
  updateUser,
  deleteUser,
  uploadAvatar,
  getAvatarPath,
  readImageBase64,
  type User
} from '../services/database'
import { OnboardingManager, TooltipManager } from '../services/settings'

const users = ref<User[]>([])
const currentUser = ref<User | null>(null)
const showCreateDialog = ref(false)
const showEditDialog = ref(false)

// Preload avatar URLs
const avatarUrls = ref<Map<string, string>>(new Map())
const newUser = ref({
  username: '',
  avatar_color: '#3b82f6',
  avatar_preview: '',
  avatar_file_path: ''
})
const editingUser = ref<User & { avatar_preview?: string; avatar_file_path?: string }>({
  username: '',
  avatar_color: '#3b82f6',
  created_at: ''
})

const colorOptions = [
  '#3b82f6', // blue
  '#8b5cf6', // purple
  '#ec4899', // pink
  '#f59e0b', // amber
  '#10b981', // green
  '#06b6d4', // cyan
  '#f97316', // orange
  '#6366f1'  // indigo
]

async function loadUsers() {
  try {
    users.value = await getAllUsers()
    currentUser.value = await getCurrentUser()
    
    // Preload avatar URLs as base64
    const avatarPaths = [
      ...users.value.filter(u => u.avatar_path).map(u => u.avatar_path!),
      ...(currentUser.value?.avatar_path ? [currentUser.value.avatar_path] : [])
    ]
    
    for (const path of avatarPaths) {
      if (!avatarUrls.value.has(path)) {
        try {
          const fullPath = await getAvatarPath(path)
          const base64Url = await readImageBase64(fullPath)
          avatarUrls.value.set(path, base64Url)
        } catch (e) {
          console.error('Failed to load avatar:', path, e)
        }
      }
    }
  } catch (error) {
    console.error('Failed to load users:', error)
  }
}

async function handleCreateUser() {
  try {
    let avatarPath: string | undefined
    
    // If user selected an image, upload it first
    if (newUser.value.avatar_file_path) {
      // Create user with temp data to get user ID
      const userId = await createUser(newUser.value.username, newUser.value.avatar_color)
      
      // Upload avatar
      avatarPath = await uploadAvatar(userId, newUser.value.avatar_file_path)
      
      // Update user with avatar path
      await updateUser(userId, newUser.value.username, newUser.value.avatar_color, avatarPath)
    } else {
      await createUser(newUser.value.username, newUser.value.avatar_color)
    }
    
    showCreateDialog.value = false
    newUser.value = { username: '', avatar_color: '#3b82f6', avatar_preview: '', avatar_file_path: '' }
    await loadUsers()
  } catch (error) {
    console.error('Failed to create user:', error)
    alert('创建用户失败')
  }
}

async function handleSwitchUser(userId: number) {
  if (userId === currentUser.value?.id) return
  try {
    await switchUser(userId)
    await loadUsers()
    window.location.reload()
  } catch (error) {
    console.error('Failed to switch user:', error)
    alert('切换用户失败')
  }
}

function handleEditUser(user: User) {
  editingUser.value = { ...user }
  showEditDialog.value = true
}

async function handleUpdateUser() {
  try {
    let avatarPath = editingUser.value.avatar_path
    
    // If user selected a new image, upload it
    if (editingUser.value.avatar_file_path) {
      avatarPath = await uploadAvatar(editingUser.value.id!, editingUser.value.avatar_file_path)
    }
    
    await updateUser(
      editingUser.value.id!,
      editingUser.value.username,
      editingUser.value.avatar_color,
      avatarPath
    )
    showEditDialog.value = false
    await loadUsers()
  } catch (error) {
    console.error('Failed to update user:', error)
    alert('更新用户失败')
  }
}

async function handleDeleteUser(userId: number) {
  if (userId === 1) {
    alert('不能删除默认用户')
    return
  }
  if (!confirm('确定要删除此用户吗？用户的所有数据将被删除。')) {
    return
  }
  try {
    await deleteUser(userId)
    await loadUsers()
  } catch (error) {
    console.error('Failed to delete user:', error)
    alert('删除用户失败')
  }
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}

function handleResetOnboarding() {
  if (!confirm('确定要重新查看使用引导吗？这将重置所有提示状态。')) {
    return
  }
  OnboardingManager.reset()
  TooltipManager.resetAll()
  window.location.reload()
}

async function handleSelectAvatar(mode: 'create' | 'edit') {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Image',
        extensions: ['jpg', 'jpeg', 'png']
      }]
    })
    
    // Tauri v2 returns file path as string or null
    let filePath: string | null = null
    if (selected) {
      // Handle different return formats
      if (typeof selected === 'string') {
        filePath = selected
      } else if (typeof selected === 'object' && 'path' in selected) {
        filePath = (selected as { path: string }).path
      }
    }
    
    if (filePath) {
      // Use base64 for preview
      const preview = await readImageBase64(filePath)
      
      if (mode === 'create') {
        newUser.value.avatar_preview = preview
        newUser.value.avatar_file_path = filePath
      } else {
        editingUser.value.avatar_preview = preview
        editingUser.value.avatar_file_path = filePath
      }
    }
  } catch (error) {
    console.error('Failed to select avatar:', error)
    alert('选择头像失败')
  }
}

function getAvatarUrl(avatarPath: string): string {
  return avatarUrls.value.get(avatarPath) || ''
}

onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.user-management {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.header h2 {
  margin: 0;
  font-size: 1.8rem;
  color: var(--text-primary);
}

.current-user-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--card-bg);
  border-radius: 12px;
  margin-bottom: 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.user-avatar {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 1.5rem;
  font-weight: bold;
  flex-shrink: 0;
}

.user-avatar-img {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  overflow: hidden;
  flex-shrink: 0;
}

.user-avatar-img img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.user-card .user-avatar,
.user-card .user-avatar-img {
  width: 45px;
  height: 45px;
  font-size: 1.1rem;
}

.user-info h3 {
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
  color: var(--text-secondary);
  font-weight: normal;
}

.user-info .username {
  margin: 0 0 0.25rem 0;
  font-size: 1.3rem;
  font-weight: 600;
  color: var(--text-primary);
}

.user-info .created-at {
  margin: 0;
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.users-list h3 {
  margin: 0 0 1rem 0;
  font-size: 1.2rem;
  color: var(--text-primary);
}

.user-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.user-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--card-bg);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  border: 2px solid transparent;
}

.user-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.user-card.active {
  border-color: var(--primary-color);
}

.user-card .user-avatar {
  width: 45px;
  height: 45px;
  font-size: 1.1rem;
}

.user-details {
  flex: 1;
}

.user-details .name {
  margin: 0 0 0.25rem 0;
  font-size: 1rem;
  font-weight: 500;
  color: var(--text-primary);
}

.user-details .date {
  margin: 0;
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.user-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-icon {
  background: var(--primary-color);
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  padding: 0.4rem 0.6rem;
  color: white;
  transition: all 0.2s;
}

.btn-icon:hover:not(:disabled) {
  background: var(--primary-hover);
  transform: translateY(-1px);
}

.btn-icon:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--primary-hover);
}

.btn-primary .icon {
  font-size: 1.2rem;
}

.btn-secondary {
  background: var(--card-bg);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: var(--hover-bg);
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-primary);
  padding: 2rem;
  border-radius: 12px;
  width: 90%;
  max-width: 450px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.modal-content h3 {
  margin: 0 0 1.5rem 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 1rem;
  background: var(--input-bg);
  color: var(--text-primary);
}

.avatar-preview {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.preview-img,
.preview-placeholder {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  overflow: hidden;
  flex-shrink: 0;
}

.preview-img img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.preview-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 2rem;
  font-weight: bold;
}

.btn-upload {
  background: var(--primary-color);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.btn-upload:hover {
  background: var(--primary-hover);
  transform: translateY(-1px);
}

.color-picker {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.color-option {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  border: 3px solid transparent;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.selected {
  border-color: var(--text-primary);
  transform: scale(1.15);
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 2rem;
}

.btn-guide {
  background: var(--secondary-color, #667eea);
  color: white;
  border: none;
  padding: 0.625rem 1.25rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.9rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
  white-space: nowrap;
}

.btn-guide:hover {
  background: var(--secondary-hover, #5568d3);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(102, 126, 234, 0.3);
}
</style>
