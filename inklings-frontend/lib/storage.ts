// Local storage utilities for progress tracking
const STORAGE_PREFIX = "inlinks_"

export const storage = {
  // User progress
  setProgress: (userId: string, exerciseId: string, data: any) => {
    const key = `${STORAGE_PREFIX}progress_${userId}_${exerciseId}`
    localStorage.setItem(key, JSON.stringify(data))
  },

  getProgress: (userId: string, exerciseId: string) => {
    const key = `${STORAGE_PREFIX}progress_${userId}_${exerciseId}`
    const data = localStorage.getItem(key)
    return data ? JSON.parse(data) : null
  },

  getAllProgress: (userId: string) => {
    const prefix = `${STORAGE_PREFIX}progress_${userId}_`
    const progress: Record<string, any> = {}
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i)
      if (key?.startsWith(prefix)) {
        const exerciseId = key.replace(prefix, "")
        progress[exerciseId] = JSON.parse(localStorage.getItem(key) || "{}")
      }
    }
    return progress
  },

  // User auth
  setUser: (user: any) => {
    const key = `${STORAGE_PREFIX}user`
    localStorage.setItem(key, JSON.stringify(user))
  },

  getUser: () => {
    const key = `${STORAGE_PREFIX}user`
    const data = localStorage.getItem(key)
    return data ? JSON.parse(data) : null
  },

  clearUser: () => {
    const key = `${STORAGE_PREFIX}user`
    localStorage.removeItem(key)
  },

  // Wallet
  setWallet: (wallet: any) => {
    const key = `${STORAGE_PREFIX}wallet`
    localStorage.setItem(key, JSON.stringify(wallet))
  },

  getWallet: () => {
    const key = `${STORAGE_PREFIX}wallet`
    const data = localStorage.getItem(key)
    return data ? JSON.parse(data) : null
  },
}
