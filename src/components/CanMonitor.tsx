import React, { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useStore } from '../store/useStore';
import { api, CanFrame } from '../services/api';
import { Activity, Pause, Play } from 'lucide-react';

export const CanMonitor: React.FC = () => {
  const { isSniffing, setSniffing, canFrames, addCanFrame } = useStore();

  useEffect(() => {
    const unlisten = listen<CanFrame>('can-frame', (event) => {
      addCanFrame(event.payload);
    });

    return () => {
      unlisten.then(f => f());
    };
  }, [addCanFrame]);

  const toggleSniffing = async () => {
    if (isSniffing) {
      await api.stopSniffing();
      setSniffing(false);
    } else {
      await api.startSniffing();
      setSniffing(true);
    }
  };

  const formatData = (data: number[]) => {
    return data.map(b => b.toString(16).padStart(2, '0').toUpperCase()).join(' ');
  };

  return (
    <div className="bg-gray-800 rounded-lg p-4 shadow-lg h-full flex flex-col">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-xl font-bold text-gray-300 flex items-center gap-2">
          <Activity size={20} />
          CAN Bus Monitor
        </h3>
        <button
          onClick={toggleSniffing}
          className={`p-2 rounded-full transition-colors ${
            isSniffing 
              ? 'bg-red-600 hover:bg-red-700 text-white' 
              : 'bg-gray-700 hover:bg-gray-600 text-gray-300'
          }`}
          title={isSniffing ? "Stop Sniffing" : "Start Sniffing"}
        >
          {isSniffing ? <Pause size={16} /> : <Play size={16} />}
        </button>
      </div>

      <div className="flex-1 overflow-y-auto bg-gray-900 rounded p-2 font-mono text-sm">
        <div className="grid grid-cols-12 gap-2 text-gray-500 border-b border-gray-700 pb-2 mb-2 text-xs uppercase">
          <div className="col-span-3">Timestamp</div>
          <div className="col-span-2">ID</div>
          <div className="col-span-7">Data Payload</div>
        </div>
        
        {canFrames.length === 0 ? (
          <div className="text-center text-gray-600 py-8">
            No frames captured. Start sniffing to see data.
          </div>
        ) : (
          canFrames.map((frame, i) => (
            <div key={`${frame.timestamp}-${i}`} className="grid grid-cols-12 gap-2 py-1 hover:bg-gray-800 transition-colors border-b border-gray-800 last:border-0">
              <div className="col-span-3 text-gray-400">
                {(frame.timestamp / 1000000).toFixed(3)}
              </div>
              <div className="col-span-2 text-blue-400 font-bold">
                0x{frame.id.toString(16).toUpperCase()}
              </div>
              <div className="col-span-7 text-green-400">
                {formatData(frame.data)}
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};
