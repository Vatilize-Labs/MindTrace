import React, { createContext, useContext, useState, useEffect } from 'react';

interface WalletContextType {
  account: string | null;
  loading: boolean;
  error: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  isConnected: boolean;
  walletType: 'freighter' | null;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const StellarWalletProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [account, setAccount] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [walletType, setWalletType] = useState<'freighter' | null>(null);

  // Check if wallet is already connected on load
  useEffect(() => {
    checkWalletConnection();
  }, []);

  const checkWalletConnection = async () => {
    try {
      if (typeof window !== 'undefined' && (window as any).stellar) {
        const publicKey = await (window as any).stellar.getPublicKey();
        if (publicKey) {
          setAccount(publicKey);
          setWalletType('freighter');
        }
      }
    } catch (err) {
      console.log('No wallet connected or error:', err);
    }
  };

  const connect = async () => {
    setLoading(true);
    setError(null);

    try {
      console.log('Connect button clicked');
      console.log('Window object:', typeof window !== 'undefined');
      console.log('window.stellar available:', !!(window as any).stellar);
      console.log('All window properties:', Object.keys(window).filter(key => key.toLowerCase().includes('stellar') || key.toLowerCase().includes('freighter')));

      // Wait for Freighter to inject window.stellar
      let stellar = (window as any).stellar;
      let attempts = 0;
      const maxAttempts = 30; // 3 seconds

      while (!stellar && attempts < maxAttempts) {
        console.log(`Waiting for Freighter injection... attempt ${attempts + 1}/${maxAttempts}`);
        await new Promise(resolve => setTimeout(resolve, 100));
        stellar = (window as any).stellar;
        attempts++;
      }

      console.log('Final stellar check:', !!stellar);
      console.log('Attempts used:', attempts);

      // Check if Freighter is installed
      if (!stellar) {
        console.error('Freighter not found after waiting');
        console.log('Offering manual address entry as fallback...');

        // Fallback: Ask user to paste their Stellar address
        const address = window.prompt(
          'Freighter wallet not detected.\n\nPlease paste your Stellar address (starts with G):\n\nExample: GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX'
        );

        if (!address) {
          throw new Error('No address provided');
        }

        if (!address.startsWith('G') || address.length !== 56) {
          throw new Error('Invalid Stellar address. Must start with G and be 56 characters.');
        }

        console.log('Using manual Stellar address:', address.slice(0, 6) + '...');
        setAccount(address);
        setWalletType('freighter');
        localStorage.setItem('stellar_account', address);
        setLoading(false);
        return;
      }

      console.log('Freighter detected, requesting public key...');

      // Request public key from Freighter
      const publicKey = await stellar.getPublicKey();

      if (!publicKey) {
        throw new Error('Failed to get public key from wallet');
      }

      setAccount(publicKey);
      setWalletType('freighter');
      localStorage.setItem('stellar_account', publicKey);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to connect wallet';
      setError(errorMessage);
      console.error('Wallet connection error:', err);
    } finally {
      setLoading(false);
    }
  };

  const disconnect = () => {
    setAccount(null);
    setWalletType(null);
    setError(null);
    localStorage.removeItem('stellar_account');
  };

  const value: WalletContextType = {
    account,
    loading,
    error,
    connect,
    disconnect,
    isConnected: !!account,
    walletType,
  };

  return (
    <WalletContext.Provider value={value}>{children}</WalletContext.Provider>
  );
};

export const useStellarWallet = () => {
  const context = useContext(WalletContext);
  if (context === undefined) {
    throw new Error('useStellarWallet must be used within StellarWalletProvider');
  }
  return context;
};
