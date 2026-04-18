import React, { useState } from 'react';
import FileNavigation from './FileNavigation';
import { api, Config } from '../api/tauri';

function Home() {
  const [showConfig, setShowConfig] = useState<boolean>(false);
  const [config, setConfig] = useState<Config>({
    bucket: '',
    region: '',
    access_key: '',
    secret_key: '',
    endpoint: ''
  });
  const [configError, setConfigError] = useState<string>('');

  const handleConfigSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setConfigError('');
    
    try {
      await api.initDataManager(config);
      setShowConfig(false);
    } catch (err) {
      setConfigError(err as string);
    }
  };

  const handleConfigChange = (field: keyof Config, value: string) => {
    setConfig(prev => ({ ...prev, [field]: value }));
  };

  return (
    <div className="home">
      <div className="home-header">
        <h1>sync-bfu</h1>
        <p>Welcome to the storage management interface.</p>
        <button 
          onClick={() => setShowConfig(!showConfig)}
          className="config-toggle-btn"
        >
          ⚙️ {showConfig ? 'Hide' : 'Show'} Configuration
        </button>
      </div>

      {showConfig && (
        <div className="config-panel">
          <h2>S3 Configuration</h2>
          <form onSubmit={handleConfigSubmit} className="config-form">
            <div className="form-group">
              <label>Bucket:</label>
              <input
                type="text"
                value={config.bucket}
                onChange={(e) => handleConfigChange('bucket', e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label>Region:</label>
              <input
                type="text"
                value={config.region}
                onChange={(e) => handleConfigChange('region', e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label>Access Key:</label>
              <input
                type="password"
                value={config.access_key}
                onChange={(e) => handleConfigChange('access_key', e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label>Secret Key:</label>
              <input
                type="password"
                value={config.secret_key}
                onChange={(e) => handleConfigChange('secret_key', e.target.value)}
                required
              />
            </div>
            <div className="form-group">
              <label>Endpoint (optional):</label>
              <input
                type="text"
                value={config.endpoint}
                onChange={(e) => handleConfigChange('endpoint', e.target.value)}
              />
            </div>
            {configError && <div className="error">{configError}</div>}
            <button type="submit" className="submit-btn">Connect</button>
          </form>
        </div>
      )}

      <div className="home-content">
        <FileNavigation />
      </div>
    </div>
  );
}

export default Home;
