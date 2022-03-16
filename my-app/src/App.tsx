import React, { useEffect, useState } from 'react';
import styled from '@emotion/styled';
import './App.css';
import { PathList } from './screens';

const apiUrl = process.env.REACT_APP_API_URL;

function App() {
  const [list, setList] = useState([]);

  useEffect(() => {
    window.fetch(`${apiUrl}/root_path`).then(async (response) => {
      const data = await response.json();
      if (response.ok) {
        setList(data);
      }
    });
  }, []);

  return (
    <div className="App">
      <Container>
        <PathList list={list} setList={setList} />
      </Container>
    </div>
  );
}

const Container = styled.div`
  padding: 3.2rem;
`;

const Row = styled.div<{
  gap?: number | boolean;
  between?: boolean;
  marginBottom?: number;
}>`
  display: flex;
  align-items: center;
  justify-content: ${(props) => (props.between ? 'space-between' : undefined)};
  margin_bottom: ${(props) => props.marginBottom + 'rem'};
  > * {
    margin-top: 0 !important;
    margin-bottom: 0 !important;
    margin-right: ${(props) =>
      typeof props.gap === 'number'
        ? props.gap + 'rem'
        : props.gap
        ? '2rem'
        : undefined};
  }
`;

const Header = styled(Row)`
  padding: 3.2rem;
  box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.1);
  z-index: 1;
`;

const HeaderLeft = styled.div``;
const HeaderRight = styled.div``;
const HeaderMiddler = styled.div``;

export default App;
