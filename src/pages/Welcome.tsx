import { useStellarWallet } from '../context/StellarWalletContext';
import rightImageSrc from '../assets/logo.png';
import leftImageSrc from '../assets/who_your_guy.jpeg';
import bottomImageSrc from '../assets/new_guy.png';
import { useNavigate } from 'react-router-dom';
import { useEffect } from 'react';
import Button from '@components/Button.js';

export const Welcome = () => {
  const navigate = useNavigate();
  const { account, loading, connect, error } = useStellarWallet();

  useEffect(() => {
    if (account) {
      navigate('/');
    }
  }, [account, navigate]);

  return (
    <div className='flex h-full w-full items-stretch justify-between'>
      <div className='relative flex h-full w-full flex-col items-center justify-center'>
        <img
          src={rightImageSrc}
          alt='Top-right Alex'
          className='fixed right-0 top-0 h-full max-h-[18rem] max-w-[50%] object-contain'
        />
        <img
          src={leftImageSrc}
          alt='Left Alex'
          className='fixed left-0 top-1/4 h-full max-h-[20rem] max-w-[50%] -translate-y-20 object-contain'
        />
        <h1 className='text-24xl z-10 max-w-full overflow-visible whitespace-nowrap text-center font-extrabold leading-[40.56px] tracking-tight text-primary-white'>
          WHO'S
          <br />
          YOUR GUY?
        </h1>
        <p className='z-10 mb-8 mt-8 max-w-[400px] text-center text-base font-bold tracking-tight text-primary-white'>
          A thrilling game on Stellar Soroban - challenge friends and win rewards!
        </p>
        {error && (
          <p className='z-10 mb-4 max-w-[400px] text-center text-sm font-semibold text-red-400'>
            {error}
          </p>
        )}
        <Button
          className='max-w-[250px]'
          onClick={connect}
          color='yellow'
          disabled={loading}
        >
          {loading ? 'Connecting...' : 'Play!'}
        </Button>
        <p className='z-10 mt-6 text-center text-xs text-gray-400'>
          Requires Freighter Wallet
          <br />
          <a
            href='https://www.freighter.app/'
            target='_blank'
            rel='noopener noreferrer'
            className='text-blue-400 hover:underline'
          >
            Install Freighter
          </a>
        </p>
        <img
          src={bottomImageSrc}
          alt='Bottom Alex'
          className='center -translate-y-100 fixed bottom-0 h-full max-h-[12rem] w-3/5 max-w-[35%] transform object-contain'
        />
      </div>
    </div>
  );
};
