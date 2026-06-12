/* eslint-disable @typescript-eslint/no-explicit-any */
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';
import { StellarWalletProvider } from './context/StellarWalletContext';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <div className='h-screen w-screen'>
    <StellarWalletProvider>
      <App />
    </StellarWalletProvider>
  </div>
);
