// 'use client';

import { Navbar } from 'flowbite-react';

/* eslint-disable-next-line */
export interface NavBarProps {}

export function NavBar(props: NavBarProps) {
  return (
    <Navbar fluid={true} rounded={true}>
      <Navbar.Brand href="/#">
        <span className="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
          Flowbite
        </span>
      </Navbar.Brand>
      <Navbar.Toggle />
      <Navbar.Collapse>
        <Navbar.Link href="/" active={true}>
          Home
        </Navbar.Link>
        <Navbar.Link href="/">About</Navbar.Link>
        <Navbar.Link href="/">Services</Navbar.Link>
        <Navbar.Link href="/">Pricing</Navbar.Link>
      </Navbar.Collapse>
    </Navbar>
  );
}

export default NavBar;
