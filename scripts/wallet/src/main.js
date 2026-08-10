/**
 * Wallet Integration for ORE App
 *
 * Supports external wallets (Phantom, Backpack, MWA, etc.) via @solana/wallet-adapter-react
 */

import React, { useEffect, useMemo, useCallback } from 'react';
import ReactDOM from 'react-dom/client';
import { WalletProvider, useWallet } from '@solana/wallet-adapter-react';
import { SolanaMobileWalletAdapterWalletName } from '@solana-mobile/wallet-standard-mobile';
import { VersionedTransaction } from '@solana/web3.js';
import bs58 from 'bs58';
import * as buffer from 'buffer';

// Polyfill Buffer for browser
window.Buffer = buffer.Buffer;

// Empty wallets array - WalletProvider will auto-detect standard wallets (including MWA)
const wallets = [];

/**
 * Main Wallet Component
 */
export const Wallet = () => {
  return (
    <WalletProvider wallets={wallets} autoConnect={true}>
      <WalletBridge />
    </WalletProvider>
  );
};

/**
 * Bridge component that connects wallet functionality to Dioxus
 */
function WalletBridge() {
  const {
    wallets: externalWallets,
    publicKey,
    select,
    connect: walletAdapterConnect,
    disconnect: walletAdapterDisconnect,
    signTransaction,
    signMessage,
    connected,
  } = useWallet();

  // ============================================================
  // Expose External Wallets to Dioxus
  // ============================================================

  useEffect(() => {
    console.log('[Wallet] Detected wallets:', externalWallets.map(w => w.adapter.name));

    window.WalletAdapters = externalWallets.map((wallet) => ({
      name: wallet.adapter.name,
      icon: wallet.adapter.icon,
    }));
  }, [externalWallets]);

  // ============================================================
  // Dispatch Wallet State to Dioxus
  // ============================================================

  useEffect(() => {
    if (publicKey) {
      console.log('[Wallet] Connected:', publicKey.toBase58());
      const pubkeyBytes = bs58.decode(publicKey.toBase58());
      window.dispatchEvent(new CustomEvent('wallet-pubkey', {
        detail: {
          pubkey: Array.from(pubkeyBytes),
        }
      }));
    } else if (!connected) {
      console.log('[Wallet] No wallet connected');
      window.dispatchEvent(new CustomEvent('wallet-pubkey', {
        detail: {
          pubkey: null,
        }
      }));
    }
  }, [publicKey, connected]);

  // ============================================================
  // Window Interface: Connection Functions
  // ============================================================

  useEffect(() => {
    // Connect external wallet (including MWA)
    window.WalletConnect = async (walletName) => {
      console.log('[Wallet] Selecting wallet:', walletName);
      try {
        select(walletName);

        // MWA requires explicit connect() call after select()
        const isMWA = walletName === SolanaMobileWalletAdapterWalletName;
        if (isMWA) {
          console.log('[Wallet] MWA detected, calling connect()...');
          await walletAdapterConnect();
        }
      } catch (err) {
        console.error('[Wallet] Connect error:', err.message || String(err));
        throw err;
      }
    };

    // Disconnect
    window.WalletDisconnect = async () => {
      console.log('[Wallet] Disconnect');
      try {
        await walletAdapterDisconnect();
      } catch (err) {
        console.error('[Wallet] Disconnect error:', err.message || String(err));
      }
    };
  }, [select, walletAdapterConnect, walletAdapterDisconnect]);

  // ============================================================
  // Window Interface: Signing Functions
  // ============================================================

  useEffect(() => {
    // Sign transaction
    window.WalletSignTransaction = async (msg) => {
      console.log('[Wallet] Sign transaction');
      try {
        if (!connected || !signTransaction) {
          throw new Error('No wallet connected');
        }

        const tx = VersionedTransaction.deserialize(
          Buffer.from(msg.b64, 'base64')
        );
        const signed = await signTransaction(tx);
        return Buffer.from(signed.serialize()).toString('base64');
      } catch (err) {
        console.error('[Wallet] Sign transaction error:', err.message || String(err));
        throw err;
      }
    };

    // Sign message
    window.WalletSignMessage = async (msg) => {
      console.log('[Wallet] Sign message');
      try {
        if (!connected || !signMessage) {
          throw new Error('No wallet connected');
        }

        const message = Buffer.from(msg.b64, 'base64');
        const sig = await signMessage(message);
        return Buffer.from(sig).toString('base64');
      } catch (err) {
        console.error('[Wallet] Sign message error:', err.message || String(err));
        throw err;
      }
    };
  }, [connected, signTransaction, signMessage]);

  return null;
}

// ============================================================
// Mount Logic
// ============================================================

let walletRoot = null;

function MountWalletAdapter() {
  if (walletRoot) {
    console.log('[Wallet] Already mounted');
    return;
  }

  const container = document.getElementById('ore-wallet-adapter');
  if (!container) {
    console.error('[Wallet] Container #ore-wallet-adapter not found');
    return;
  }

  console.log('[Wallet] Mounting...');
  walletRoot = ReactDOM.createRoot(container);
  walletRoot.render(<Wallet />);
}

window.MountWalletAdapter = MountWalletAdapter;
