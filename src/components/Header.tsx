import { useStellarWallet } from '../context/StellarWalletContext';
import Button from './Button';
import { useGameStore } from '@state/gameStore';
import { useNavigate } from 'react-router-dom';
import whoIsYourGuyImg from '../assets/logo.png'

// Helper to shorten address
const shortenAddress = (address: string) => {
  if (!address) return '';
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
};

export const AppHeader = () => {
  const { account, disconnect, loading } = useStellarWallet();
  const navigate = useNavigate();

  return (
    <div className='flex w-full items-stretch justify-between gap-5  p-4'>
      {account ? (
        <>
          <button
            onClick={() => {
              useGameStore.getState().clearFlowStores();
              navigate('/');
            }}
          >
           <img
            loading='lazy'
            src={whoIsYourGuyImg}
            className='aspect-[2.95] w-[161px] max-w-full self-stretch overflow-hidden object-cover object-center'
            style={{ maxHeight: '100%', maxWidth: '100%' }}
/>

          </button>
          <Button
            size='sm'
            color='white'
            className='w-fit'
            onClick={disconnect}
            disabled={loading}
          >
            {shortenAddress(account)}
          </Button>
        </>
      ) : (
        <div className='w-full self-stretch' />
      )}
    </div>
  );
};
