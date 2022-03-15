import React from 'react';
import { Anchor } from 'antd';

const { Link } = Anchor;

const handleClick = (
  e: React.MouseEvent<HTMLElement>,
  link: {
    title: React.ReactNode;
    href: string;
  }
) => {
  let a = document.createElement('a');
  a.href = link.href;
  a.click();
};

export const Demo = () => {
  return (
    <Anchor onClick={handleClick}>
      <Link href="index_or_content" title="click it" />
    </Anchor>
  );
};
