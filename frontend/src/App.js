import React from 'react';
import './App.css';
import Rant from './Rant.js';

class App extends React.Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1>RANTS</h1>
          <div className='container'>
            <Rant />
          </div>
        </header>
      </div>
    );
  }
}

export default App;
