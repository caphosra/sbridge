'use client';

import ControlButton from './controlButton';
import { useState, useEffect } from 'react';
import { listen, emit } from '@tauri-apps/api/event';

interface ISSHConfig {
  is_running: boolean;
  host: string;
  is_remote_forwarding: boolean;
  arguments: Map<string, string>;
}

export default function Home() {
  const [configs, setConfigs] = useState([] as ISSHConfig[]);

  useEffect(() => {
    (async () => {
      await listen('update-configs', (event) => {
        console.log('Hello World');
        setConfigs(event.payload as ISSHConfig[]);
      });
      await emit('reload');
    })();
  }, []);

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-between font-mono text-sm lg:flex">
        <div className="fixed left-0 top-0 flex w-full justify-center bg-white p-2">
          <ControlButton
            className="bg-blue-500"
            onClick={() => {
              emit('reload');
            }}
          >
            Reload
          </ControlButton>
          <ControlButton className="bg-green-500" onClick={() => {}}>
            Up All
          </ControlButton>
          <ControlButton className="bg-red-500" onClick={() => {}}>
            Quit All
          </ControlButton>
        </div>
        {configs
          .filter((val) => val.is_remote_forwarding)
          .map((val, index) => {
            return (
              <div
                key={index}
                className="flex h-12 m-1 rounded border-slate-300 border"
              >
                <p className="flex items-center justify-start text-black font-mono w-full text-lg m-1 ml-5">
                  {val.host}
                </p>
                <div className="w-10" />
                {val.is_running ? (
                  <button
                    className="flex items-center justify-center w-64 rounded bg-green-600 m-1"
                    onClick={() => {
                      emit('kill', val.host);
                    }}
                  >
                    <p className="text-white font-mono">Running</p>
                  </button>
                ) : (
                  <button
                    className="flex items-center justify-center w-64 rounded bg-red-600 m-1"
                    onClick={() => {
                      emit('run', val.host);
                    }}
                  >
                    <p className="text-white font-mono">Stopping</p>
                  </button>
                )}
                <button className="flex items-center justify-center w-64 rounded bg-slate-600 m-1">
                  <p className="text-white font-mono">View</p>
                </button>
              </div>
            );
          })}
        <p className="fixed bottom-0 left-0 flex w-full justify-center font-mono p-2 bg-white">
          sbridge - Sync ports over the firewall with the power of SSH
        </p>
      </div>
    </main>
  );
}
