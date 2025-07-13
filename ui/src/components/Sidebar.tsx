import React from 'react';

interface SidebarProps {
  isExpanded: boolean;
  onToggle: () => void;
}

const Sidebar: React.FC<SidebarProps> = ({ isExpanded, onToggle }) => {
  return (
    <div className={`sidebar ${isExpanded ? 'expanded' : 'collapsed'}`}>
      <div className="sidebar-header">
        {isExpanded && <h2 className="sidebar-title">reStrike VTA</h2>}
        <button 
          className="toggle-btn" 
          onClick={onToggle}
          title={isExpanded ? 'Collapse sidebar' : 'Expand sidebar'}
        >
          {isExpanded ? '◀' : '▶'}
        </button>
      </div>
      
      {isExpanded && (
        <div className="sidebar-content">
          <p>Overlay Controls</p>
          <div style={{ marginTop: '1rem' }}>
            <button style={{ 
              width: '100%', 
              padding: '0.5rem', 
              marginBottom: '0.5rem',
              backgroundColor: '#3498db',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer'
            }}>
              Start Recording
            </button>
            <button style={{ 
              width: '100%', 
              padding: '0.5rem', 
              marginBottom: '0.5rem',
              backgroundColor: '#e74c3c',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer'
            }}>
              Stop Recording
            </button>
            <button style={{ 
              width: '100%', 
              padding: '0.5rem',
              backgroundColor: '#2ecc71',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer'
            }}>
              Settings
            </button>
          </div>
        </div>
      )}
      
      {isExpanded && (
        <div className="hotkey-hint">
          Press Ctrl+Shift+S to toggle
        </div>
      )}
    </div>
  );
};

export default Sidebar;