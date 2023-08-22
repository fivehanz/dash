/* eslint-disable-next-line */
'use client';
import './globals.css';

import { NextUIProvider } from '@nextui-org/react';

interface RootLayoutProps {
  children: React.ReactNode;
}

export default function RootLayout(props: RootLayoutProps) {
  const { children } = props;
  return <NextUIProvider>{children}</NextUIProvider>;
}
