import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [name, setName] = useState('');
  const [greetMsg, setGreetMsg] = useState('');

  async function greet() {
    try {
      const message = await invoke('greet', { name });
      setGreetMsg(message);
    } catch (error) {
      console.error('Error calling greet:', error);
    }
  }

  async function startUdpServer() {
    try {
      await invoke('start_udp_server');
      console.log('UDP server started');
    } catch (error) {
      console.error('Error starting UDP server:', error);
    }
  }

  async function connectObs() {
    try {
      await invoke('connect_obs');
      console.log('OBS connection initiated');
    } catch (error) {
      console.error('Error connecting to OBS:', error);
    }
  }

  async function checkLicense() {
    try {
      await invoke('check_license');
      console.log('License check initiated');
    } catch (error) {
      console.error('Error checking license:', error);
    }
  }

  return (
    <div style={{ padding: '20px', fontFamily: 'Arial, sans-serif' }}>
      <h1>reStrike VTA Overlay</h1>
      
      <div style={{ marginBottom: '20px' }}>
        <input
          placeholder="Enter a name..."
          value={name}
          onChange={(e) => setName(e.target.value)}
          style={{ marginRight: '10px', padding: '5px' }}
        />
        <button onClick={greet} style={{ padding: '5px 10px' }}>
          Greet
        </button>
        {greetMsg && <p>{greetMsg}</p>}
      </div>

      <div style={{ marginBottom: '20px' }}>
        <h2>Core Plugins</h2>
        <div style={{ display: 'flex', gap: '10px' }}>
          <button onClick={startUdpServer} style={{ padding: '10px' }}>
            Start UDP Server
          </button>
          <button onClick={connectObs} style={{ padding: '10px' }}>
            Connect OBS
          </button>
          <button onClick={checkLicense} style={{ padding: '10px' }}>
            Check License
          </button>
        </div>
      </div>

      <div>
        <p>Open the browser console to see plugin status messages.</p>
        {/* TODO: Add overlay UI components */}
      </div>
    </div>
  );
}

export default App;
