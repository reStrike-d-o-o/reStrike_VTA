import React from 'react';
import Sidebar from './components/Sidebar';
import Overlay from './components/Overlay';
import { useSidebar } from './hooks/useSidebar';

function App() {
  const { isExpanded, toggleSidebar } = useSidebar();

  return (
    <div className="app">
      <Sidebar isExpanded={isExpanded} onToggle={toggleSidebar} />
      <div className="main-content">
        <h1>reStrike VTA Overlay</h1>
        <p>
          Press <strong>Ctrl+Shift+S</strong> to toggle the sidebar.
        </p>
        <p>
          Sidebar is currently: <strong>{isExpanded ? 'Expanded' : 'Collapsed'}</strong>
        </p>
        <Overlay />
      </div>
    </div>
  );
}

export default App;
