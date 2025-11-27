import React, { useEffect, useState } from 'react';
import { useStore } from '../store/useStore';

export const CarAnimation: React.FC = () => {
  const { rpm, speed, throttlePosition, isRunning } = useStore();
  const [wheelRotation, setWheelRotation] = useState(0);
  const [shake, setShake] = useState(0);

  // Animate wheels based on speed
  useEffect(() => {
    if (!isRunning || speed === 0) return;

    const interval = setInterval(() => {
      setWheelRotation(prev => (prev + speed / 10) % 360);
    }, 50);

    return () => clearInterval(interval);
  }, [speed, isRunning]);

  // Add subtle shake effect at high RPM
  useEffect(() => {
    if (rpm > 4000) {
      const shakeAmount = Math.min((rpm - 4000) / 3000, 1) * 2;
      setShake(shakeAmount);
    } else {
      setShake(0);
    }
  }, [rpm]);

  // Determine car state
  const isMoving = speed > 5;
  const isHighSpeed = speed > 80;
  const isAccelerating = throttlePosition > 50;

  // Car body color based on state
  const getCarColor = () => {
    if (!isRunning) return '#4B5563'; // Gray when stopped
    if (isHighSpeed) return '#EF4444'; // Red at high speed
    if (isAccelerating) return '#F59E0B'; // Amber when accelerating
    return '#3B82F6'; // Blue normal
  };

  // Exhaust smoke based on throttle
  const getExhaustOpacity = () => {
    if (!isRunning) return 0;
    return Math.min(throttlePosition / 100, 0.6);
  };

  return (
    <div className="fixed bottom-6 right-6 bg-gray-800/90 backdrop-blur-sm border border-gray-700 rounded-lg p-4 shadow-2xl">
      <div className="text-xs text-gray-400 mb-2 font-semibold">VEHICLE STATUS</div>

      <svg
        width="200"
        height="120"
        viewBox="0 0 200 120"
        className="transition-transform duration-100"
        style={{
          transform: shake > 0 ? `translate(${Math.sin(Date.now() / 50) * shake}px, ${Math.cos(Date.now() / 50) * shake}px)` : 'none'
        }}
      >
        {/* Road */}
        <rect x="0" y="90" width="200" height="30" fill="#1F2937" />
        <rect x="0" y="95" width="200" height="2" fill="#374151" strokeDasharray="10,5" />

        {/* Exhaust smoke */}
        {isRunning && (
          <>
            <circle
              cx="35"
              cy="75"
              r="8"
              fill="#9CA3AF"
              opacity={getExhaustOpacity() * 0.3}
              className="animate-pulse"
            />
            <circle
              cx="30"
              cy="70"
              r="12"
              fill="#9CA3AF"
              opacity={getExhaustOpacity() * 0.2}
              className="animate-pulse"
              style={{ animationDelay: '0.3s' }}
            />
          </>
        )}

        {/* Car Body */}
        <g transform="translate(50, 40)">
          {/* Main body */}
          <rect
            x="0"
            y="20"
            width="100"
            height="30"
            rx="4"
            fill={getCarColor()}
            stroke="#1F2937"
            strokeWidth="2"
            className="transition-colors duration-300"
          />

          {/* Roof/Cabin */}
          <path
            d="M 20 20 L 30 5 L 70 5 L 80 20 Z"
            fill={getCarColor()}
            stroke="#1F2937"
            strokeWidth="2"
            className="transition-colors duration-300"
          />

          {/* Windows */}
          <rect x="32" y="8" width="18" height="12" fill="#60A5FA" opacity="0.6" />
          <rect x="52" y="8" width="18" height="12" fill="#60A5FA" opacity="0.6" />

          {/* Headlights */}
          <circle
            cx="95"
            cy="35"
            r="3"
            fill={isRunning ? '#FCD34D' : '#374151'}
            className="transition-colors duration-300"
          />
          <circle
            cx="95"
            cy="42"
            r="3"
            fill={isRunning ? '#FCD34D' : '#374151'}
            className="transition-colors duration-300"
          />

          {/* Tail lights */}
          <circle
            cx="5"
            cy="35"
            r="2"
            fill={isRunning && speed < 5 ? '#EF4444' : '#7F1D1D'}
            className="transition-colors duration-300"
          />
          <circle
            cx="5"
            cy="42"
            r="2"
            fill={isRunning && speed < 5 ? '#EF4444' : '#7F1D1D'}
            className="transition-colors duration-300"
          />

          {/* Front Wheel */}
          <g transform="translate(75, 50)">
            <circle cx="0" cy="0" r="12" fill="#1F2937" stroke="#4B5563" strokeWidth="2" />
            <circle cx="0" cy="0" r="8" fill="#374151" />
            <g transform={`rotate(${wheelRotation})`}>
              <line x1="-6" y1="0" x2="6" y2="0" stroke="#6B7280" strokeWidth="2" />
              <line x1="0" y1="-6" x2="0" y2="6" stroke="#6B7280" strokeWidth="2" />
            </g>
          </g>

          {/* Rear Wheel */}
          <g transform="translate(25, 50)">
            <circle cx="0" cy="0" r="12" fill="#1F2937" stroke="#4B5563" strokeWidth="2" />
            <circle cx="0" cy="0" r="8" fill="#374151" />
            <g transform={`rotate(${wheelRotation})`}>
              <line x1="-6" y1="0" x2="6" y2="0" stroke="#6B7280" strokeWidth="2" />
              <line x1="0" y1="-6" x2="0" y2="6" stroke="#6B7280" strokeWidth="2" />
            </g>
          </g>
        </g>

        {/* Speed lines when moving fast */}
        {isHighSpeed && (
          <g opacity="0.4">
            <line x1="20" y1="50" x2="40" y2="50" stroke="#60A5FA" strokeWidth="2" className="animate-pulse" />
            <line x1="15" y1="60" x2="35" y2="60" stroke="#60A5FA" strokeWidth="2" className="animate-pulse" style={{ animationDelay: '0.1s' }} />
            <line x1="10" y1="70" x2="30" y2="70" stroke="#60A5FA" strokeWidth="2" className="animate-pulse" style={{ animationDelay: '0.2s' }} />
          </g>
        )}
      </svg>

      {/* Status indicators */}
      <div className="flex gap-2 mt-2 text-xs">
        <div className={`flex items-center gap-1 px-2 py-1 rounded ${isRunning ? 'bg-green-900/50 text-green-400' : 'bg-gray-700 text-gray-500'}`}>
          <div className={`w-2 h-2 rounded-full ${isRunning ? 'bg-green-400 animate-pulse' : 'bg-gray-500'}`} />
          {isRunning ? 'ENGINE ON' : 'STOPPED'}
        </div>

        {isMoving && (
          <div className="px-2 py-1 rounded bg-blue-900/50 text-blue-400 flex items-center gap-1">
            <span>▶</span>
            {isHighSpeed ? 'HIGH SPEED' : 'CRUISING'}
          </div>
        )}

        {isAccelerating && isRunning && (
          <div className="px-2 py-1 rounded bg-amber-900/50 text-amber-400 flex items-center gap-1">
            <span>⚡</span>
            ACCEL
          </div>
        )}
      </div>
    </div>
  );
};
