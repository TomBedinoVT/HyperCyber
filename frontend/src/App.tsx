import { Routes, Route, Navigate, useSearchParams, useNavigate } from 'react-router-dom'
import { useEffect } from 'react'
import { useAuth } from './hooks/useAuth'
import Login from './pages/Login'
import Dashboard from './pages/Dashboard'
import Entities from './pages/Entities'
import RGPDRegister from './pages/RGPDRegister'
import RGPDRequests from './pages/RGPDRequests'
import RGPDBreaches from './pages/RGPDBreaches'
import Layout from './components/Layout'

function PrivateRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated } = useAuth()
  return isAuthenticated ? <>{children}</> : <Navigate to="/login" />
}

function OIDCCallback() {
  const [searchParams] = useSearchParams()
  const navigate = useNavigate()
  const token = searchParams.get('token')
  const refreshToken = searchParams.get('refresh_token')

  useEffect(() => {
    if (token && refreshToken) {
      localStorage.setItem('token', token)
      localStorage.setItem('refreshToken', refreshToken)
      navigate('/')
      window.location.reload()
    } else {
      navigate('/login')
    }
  }, [token, refreshToken, navigate])

  return <div>Connexion en cours...</div>
}

function App() {
  return (
    <Routes>
      <Route path="/login" element={<Login />} />
      <Route path="/auth/callback" element={<OIDCCallback />} />
      <Route
        path="/"
        element={
          <PrivateRoute>
            <Layout />
          </PrivateRoute>
        }
      >
        <Route index element={<Dashboard />} />
        <Route path="entities" element={<Entities />} />
        <Route path="rgpd/register" element={<RGPDRegister />} />
        <Route path="rgpd/requests" element={<RGPDRequests />} />
        <Route path="rgpd/breaches" element={<RGPDBreaches />} />
      </Route>
    </Routes>
  )
}

export default App

