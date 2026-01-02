import { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { authApi } from '../api/auth'

interface User {
  id: string
  email: string
  first_name?: string
  last_name?: string
}

interface AuthContextType {
  user: User | null
  isAuthenticated: boolean
  login: (email: string, password: string) => Promise<void>
  register: (email: string, password: string, firstName?: string, lastName?: string) => Promise<void>
  logout: () => void
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null)

  useEffect(() => {
    const token = localStorage.getItem('token')
    if (token) {
      authApi.getCurrentUser()
        .then((userData) => setUser(userData))
        .catch(() => {
          localStorage.removeItem('token')
          localStorage.removeItem('refreshToken')
        })
    }
  }, [])

  const login = async (email: string, password: string) => {
    const response = await authApi.login({ email, password })
    localStorage.setItem('token', response.token)
    localStorage.setItem('refreshToken', response.refresh_token)
    setUser(response.user)
  }

  const register = async (email: string, password: string, firstName?: string, lastName?: string) => {
    const response = await authApi.register({ email, password, first_name: firstName, last_name: lastName })
    localStorage.setItem('token', response.token)
    localStorage.setItem('refreshToken', response.refresh_token)
    setUser(response.user)
  }

  const logout = () => {
    localStorage.removeItem('token')
    localStorage.removeItem('refreshToken')
    setUser(null)
  }

  return (
    <AuthContext.Provider
      value={{
        user,
        isAuthenticated: !!user,
        login,
        register,
        logout,
      }}
    >
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}

