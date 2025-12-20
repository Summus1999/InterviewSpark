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
      <div class="user-avatar" :style="{ backgroundColor: currentUser.avatar_color }">
        {{ currentUser.username.charAt(0) }}
      </div>
      <div class="user-info">
        <h3>当前用户</h3>
        <p class="username">{{ currentUser.username }}</p>
        <p class="created-at">创建于 {{ formatDate(currentUser.created_at) }}</p>
      </div>
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
          <div class="user-avatar" :style="{ backgroundColor: user.avatar_color }">
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
import {
  getAllUsers,
  getCurrentUser,
  createUser,
  switchUser,
  updateUser,
  deleteUser,
  type User
} from '../services/database'

const users = ref<User[]>([])
const currentUser = ref<User | null>(null)
const showCreateDialog = ref(false)
const showEditDialog = ref(false)
const newUser = ref({
  username: '',
  avatar_color: '#3b82f6'
})
const editingUser = ref<User>({
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
  } catch (error) {
    console.error('Failed to load users:', error)
  }
}

async function handleCreateUser() {
  try {
    await createUser(newUser.value.username, newUser.value.avatar_color)
    showCreateDialog.value = false
    newUser.value = { username: '', avatar_color: '#3b82f6' }
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
    await updateUser(
      editingUser.value.id!,
      editingUser.value.username,
      editingUser.value.avatar_color
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
  background: none;
  border: none;
  cursor: pointer;
  font-size: 1.2rem;
  padding: 0.25rem;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.btn-icon:hover:not(:disabled) {
  opacity: 1;
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
</style>
