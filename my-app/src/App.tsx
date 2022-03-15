import React, { useEffect, useState } from 'react';
import styled from '@emotion/styled';
import './App.css';
import { PathList } from './screens';

const apiUrl = process.env.REACT_APP_API_URL;

function App() {
  const [list, setList] = useState([]);

  useEffect(() => {
    window.fetch(`${apiUrl}/index_or_content`).then(async (response) => {
      const data = await response.json();
      if (response.ok) {
        setList(data);
      } else {
        Promise.reject(data);
      }
    });
  }, []);

  return (
    <div className="App">
      <Container>
        <PathList list={list} />
      </Container>
    </div>
  );
}

const Container = styled.div`
  padding: 3.2rem;
`;

export default App;
