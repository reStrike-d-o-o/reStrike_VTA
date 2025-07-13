import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('renders reStrike VTA Overlay heading', () => {
  render(<App />);
  const linkElement = screen.getByText(/reStrike VTA Overlay/i);
  expect(linkElement).toBeInTheDocument();
});