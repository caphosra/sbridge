'use client';

import { ReactNode } from 'react';

export default function ControlButton(props: {
  className: string;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <button
      className={`flex items-center justify-center w-32 h-10 rounded m-1 ${props.className}`}
      onClickCapture={props.onClick}
    >
      <p className="text-white font-mono">
        {props.children}
      </p>
    </button>
  );
}
