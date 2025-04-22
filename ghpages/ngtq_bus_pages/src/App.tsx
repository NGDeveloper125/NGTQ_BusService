import React from 'react';
import logo from './logo.svg';
import Heading from "./components/Heading"
import './App.css';
import PageBody from './components/PageBody';
import Footer from './components/Footer';

function App() {
  return (
    <div className="App">
      <Heading currentPage='About' />
      <PageBody currentPage='About' />
      <Footer />
    </div>
  );
}

export default App;
