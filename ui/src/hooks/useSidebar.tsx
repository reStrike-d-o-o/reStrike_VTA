import { useState, useEffect, useCallback } from 'react';

export interface SidebarState {
  isExpanded: boolean;
}

export const useSidebar = () => {
  const [isExpanded, setIsExpanded] = useState<boolean>(false);

  const toggleSidebar = useCallback(() => {
    setIsExpanded(prev => !prev);
  }, []);

  const expandSidebar = useCallback(() => {
    setIsExpanded(true);
  }, []);

  const collapseSidebar = useCallback(() => {
    setIsExpanded(false);
  }, []);

  // Global hotkey listener
  useEffect(() => {
    const handleKeyPress = (event: KeyboardEvent) => {
      // Toggle sidebar on Ctrl+Shift+S (or Cmd+Shift+S on Mac)
      if ((event.ctrlKey || event.metaKey) && event.shiftKey && event.key === 'S') {
        event.preventDefault();
        toggleSidebar();
      }
    };

    document.addEventListener('keydown', handleKeyPress);
    
    return () => {
      document.removeEventListener('keydown', handleKeyPress);
    };
  }, [toggleSidebar]);

  return {
    isExpanded,
    toggleSidebar,
    expandSidebar,
    collapseSidebar
  };
};