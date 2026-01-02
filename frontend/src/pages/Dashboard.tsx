import { useQuery } from '@tanstack/react-query'
import { entitiesApi } from '../api/entities'
import { rgpdApi } from '../api/rgpd'
import { catalogueApi } from '../api/catalogue'
import './Dashboard.css'

export default function Dashboard() {
  const { data: entities } = useQuery({
    queryKey: ['entities'],
    queryFn: () => entitiesApi.list(),
  })

  const { data: registerEntries } = useQuery({
    queryKey: ['rgpd-register'],
    queryFn: () => rgpdApi.getRegister(),
  })

  const { data: accessRequests } = useQuery({
    queryKey: ['rgpd-requests'],
    queryFn: () => rgpdApi.listAccessRequests(),
  })

  const { data: breaches } = useQuery({
    queryKey: ['rgpd-breaches'],
    queryFn: () => rgpdApi.listBreaches(),
  })

  const { data: endpoints } = useQuery({
    queryKey: ['catalogue-endpoints'],
    queryFn: () => catalogueApi.listEndpoints(),
  })

  const { data: licenseKeys } = useQuery({
    queryKey: ['catalogue-license-keys'],
    queryFn: () => catalogueApi.listLicenseKeys(),
  })

  const { data: softwareVersions } = useQuery({
    queryKey: ['catalogue-software-versions'],
    queryFn: () => catalogueApi.listSoftwareVersions(),
  })

  const { data: encryptionAlgorithms } = useQuery({
    queryKey: ['catalogue-encryption-algorithms'],
    queryFn: () => catalogueApi.listEncryptionAlgorithms(),
  })

  const pendingRequests = accessRequests?.filter((r) => r.status === 'pending').length || 0
  const activeBreaches = breaches?.filter((b) => b.status !== 'resolved').length || 0

  return (
    <div className="dashboard">
      <h1>Tableau de bord</h1>
      <div className="stats-grid">
        <div className="stat-card">
          <h3>Entités</h3>
          <p className="stat-number">{entities?.length || 0}</p>
        </div>
        <div className="stat-card">
          <h3>Registre RGPD</h3>
          <p className="stat-number">{registerEntries?.length || 0}</p>
        </div>
        <div className="stat-card">
          <h3>Demandes en attente</h3>
          <p className="stat-number">{pendingRequests}</p>
        </div>
        <div className="stat-card">
          <h3>Écarts actifs</h3>
          <p className="stat-number">{activeBreaches}</p>
        </div>
        <div className="stat-card">
          <h3>Endpoints</h3>
          <p className="stat-number">{endpoints?.length || 0}</p>
        </div>
        <div className="stat-card">
          <h3>Clés de licences</h3>
          <p className="stat-number">{licenseKeys?.length || 0}</p>
        </div>
        <div className="stat-card">
          <h3>Versions de logiciels</h3>
          <p className="stat-number">{softwareVersions?.length || 0}</p>
        </div>
        <div className="stat-card">
          <h3>Algorithmes de cryptage</h3>
          <p className="stat-number">{encryptionAlgorithms?.length || 0}</p>
        </div>
      </div>
    </div>
  )
}

