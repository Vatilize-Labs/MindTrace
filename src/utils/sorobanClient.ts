import { SorobanRpc } from '@stellar/js-sdk';

// Configure your Soroban RPC server
const SOROBAN_RPC_URL =
  import.meta.env.VITE_SOROBAN_RPC_URL || 'https://soroban-testnet.stellar.org';
const CONTRACT_ID =
  import.meta.env.VITE_CONTRACT_ID || '';

export const sorobanClient = new SorobanRpc.Server(SOROBAN_RPC_URL);

export const getContractId = (): string => {
  if (!CONTRACT_ID) {
    throw new Error('CONTRACT_ID not configured. Set VITE_CONTRACT_ID in .env');
  }
  return CONTRACT_ID;
};

// Helper functions for interacting with the contract
export const shortenAddress = (address: string): string => {
  if (!address) return '';
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
};

export const getCurrentNetwork = () => {
  return {
    name: 'testnet',
    passphrase: 'Test SDF Network ; September 2015',
    rpcUrl: SOROBAN_RPC_URL,
  };
};

// Type definitions for contract interactions
export interface GameProposedEvent {
  game_id: string;
  challenger: string;
  opponent: string;
  wager: string;
}

export interface GameState {
  id: string;
  challenger: string;
  opponent: string;
  challenger_wager: string;
  opponent_wager: string;
  challenger_answer: number;
  opponent_answer: number;
  total_pot: string;
  state: number;
  created_at: string;
  accepted_at: string;
}

export const GAME_STATES = {
  PROPOSED: 1,
  WAGER_MATCHED: 2,
  ACCEPTED: 3,
  REVEALED: 4,
  FINISHED: 5,
  TIMEOUT: 6,
  RENEGED: 0,
} as const;
