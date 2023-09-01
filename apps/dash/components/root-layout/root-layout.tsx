/* eslint-disable-next-line */
import './globals.css';

interface RootLayoutProps {
  children: React.ReactNode;
}

export default function RootLayout(props: RootLayoutProps) {
  const { children } = props;
  return <main>{children}</main>;
}
