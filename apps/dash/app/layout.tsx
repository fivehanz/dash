import RootLayout from '../components/root-layout/root-layout';

export const metadata = {
  title: 'Next.js',
  description: 'Generated by Next.js',
};

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body>
        <RootLayout>{children}</RootLayout>
      </body>
    </html>
  );
}