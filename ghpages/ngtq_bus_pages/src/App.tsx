import React from 'react';
import logo from './logo.svg';
import Heading from "./components/Heading"
import './App.css';
import PageBody from './components/PageBody';

function App() {
  return (
    <div className="App">
      <Heading currentPage='About' />
      <PageBody currentPage='About' />
    </div>
  );
}

export default App;
