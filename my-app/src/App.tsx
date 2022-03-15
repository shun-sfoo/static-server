import React, { useEffect, useState } from 'react';
import styled from '@emotion/styled';
import './App.css';
import { Demo } from './demo';

function App() {
  return (
    <div className="App">
      <Container>
        <Demo />
      </Container>
    </div>
  );
}

const Container = styled.div`
  padding: 3.2rem;
`;

export default App;
