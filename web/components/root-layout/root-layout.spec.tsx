import { render } from '@testing-library/react';

import RootLayout from './root-layout';

describe('RootLayout', () => {
  it('should render successfully', () => {
    const { baseElement } = render(<RootLayout />);
    expect(baseElement).toBeTruthy();
  });
});
