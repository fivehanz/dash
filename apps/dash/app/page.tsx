'use client';

import NavBar from '../components/nav-bar/nav-bar';
import { Alert } from 'flowbite-react';

const Index = () => {
  return (
    <section>
      <NavBar />
      <div className="my-4 px-4">
        <Alert color="info">Alert!</Alert>
        <h1 className="text-5xl">hello, hanz!</h1>
      </div>
    </section>
  );
};

export default Index;
