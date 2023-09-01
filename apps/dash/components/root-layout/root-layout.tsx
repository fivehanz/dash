/* eslint-disable-next-line */
'use client';
import './globals.css';

interface RootLayoutProps {
  children: React.ReactNode;
}

export default function RootLayout(props: RootLayoutProps) {
  const { children } = props;
  return <main>{children}</main>;
}
