/**
 * Unified error handler for API calls
 * Provides user-friendly error messages and offline handling
 */

// Error code enumeration
export enum ErrorCode {
  UNKNOWN = 'UNKNOWN',
  NETWORK_ERROR = 'NETWORK_ERROR',
  TIMEOUT = 'TIMEOUT',
  INVALID_API_KEY = 'INVALID_API_KEY',
  UNAUTHORIZED = 'UNAUTHORIZED',
  RATE_LIMIT = 'RATE_LIMIT',
  SERVER_ERROR = 'SERVER_ERROR',
  SERVICE_UNAVAILABLE = 'SERVICE_UNAVAILABLE',
}

export interface ApiError {
  code: ErrorCode
  message: string
  isRetryable: boolean
  isOffline: boolean
}

/**
 * Parse error from invoke call
 */
export function parseApiError(error: unknown): ApiError {
  const errorStr = String(error).toLowerCase()

  // Check if offline
  const isOffline =
    !navigator.onLine ||
    errorStr.includes('network') ||
    errorStr.includes('connection') ||
    errorStr.includes('fetch')

  // Check if retryable
  const isRetryable =
    isOffline ||
    errorStr.includes('timeout') ||
    errorStr.includes('500') ||
    errorStr.includes('502') ||
    errorStr.includes('503') ||
    errorStr.includes('504')

  // Generate user-friendly message
  let message = '操作失败，请重试'

  if (isOffline) {
    message = '网络连接失败，请检查网络设置'
  } else if (errorStr.includes('api key')) {
    message = 'API 密钥无效，请检查配置'
  } else if (errorStr.includes('timeout')) {
    message = '请求超时，请稍后重试'
  } else if (errorStr.includes('401') || errorStr.includes('unauthorized')) {
    message = '认证失败，请检查 API 配置'
  } else if (errorStr.includes('429')) {
    message = 'API 调用频率过高，请稍后再试'
  } else if (errorStr.includes('500') || errorStr.includes('503')) {
    message = '服务暂时不可用，请稍后重试'
  }

  return {
    code: extractErrorCode(errorStr),
    message,
    isRetryable,
    isOffline
  }
}

function extractErrorCode(errorStr: string): ErrorCode {
  // Extract and map to error code enum
  if (errorStr.includes('network') || errorStr.includes('connection') || errorStr.includes('fetch')) {
    return ErrorCode.NETWORK_ERROR
  } else if (errorStr.includes('timeout')) {
    return ErrorCode.TIMEOUT
  } else if (errorStr.includes('api key')) {
    return ErrorCode.INVALID_API_KEY
  } else if (errorStr.includes('401') || errorStr.includes('unauthorized')) {
    return ErrorCode.UNAUTHORIZED
  } else if (errorStr.includes('429')) {
    return ErrorCode.RATE_LIMIT
  } else if (errorStr.includes('500')) {
    return ErrorCode.SERVER_ERROR
  } else if (errorStr.includes('503') || errorStr.includes('504')) {
    return ErrorCode.SERVICE_UNAVAILABLE
  }
  
  return ErrorCode.UNKNOWN
}

/**
 * Show error toast notification
 */
export function showErrorToast(error: ApiError, duration = 3000) {
  // Use browser notification API for simple toast
  // In production, consider using a UI library like Element Plus
  if ('Notification' in window && Notification.permission === 'granted') {
    new Notification('错误', {
      body: error.message,
      icon: '/error-icon.png'
    })
  } else {
    // Fallback: create custom toast element
    const toast = document.createElement('div')
    toast.className = 'error-toast'
    toast.textContent = error.message
    toast.style.cssText = `
      position: fixed;
      top: 20px;
      right: 20px;
      background: #f56565;
      color: white;
      padding: 12px 20px;
      border-radius: 6px;
      box-shadow: 0 4px 12px rgba(0,0,0,0.15);
      z-index: 9999;
      animation: slideIn 0.3s ease-out;
    `

    document.body.appendChild(toast)

    setTimeout(() => {
      toast.style.animation = 'slideOut 0.3s ease-out'
      setTimeout(() => document.body.removeChild(toast), 300)
    }, duration)
  }
}

/**
 * Handle API call with error handling
 */
export async function withErrorHandling<T>(
  operation: () => Promise<T>,
  options?: {
    showToast?: boolean
    onError?: (error: ApiError) => void
  }
): Promise<T | null> {
  try {
    return await operation()
  } catch (error) {
    const apiError = parseApiError(error)

    if (options?.showToast !== false) {
      showErrorToast(apiError)
    }

    if (options?.onError) {
      options.onError(apiError)
    }

    console.error('API Error:', apiError)
    return null
  }
}
