import { useEffect } from 'react';
import { useStellarWallet } from '@context/StellarWalletContext';

export const useInitGame = () => {
  const { account } = useStellarWallet();

  useEffect(() => {
    if (account) {
      // Initialize game for Stellar account
      // For Stellar/Soroban, game records will be fetched from contract state
      // This is handled separately in game components via Soroban RPC
      console.log('Game initialized for account:', account);
    }
  }, [account]);
};
