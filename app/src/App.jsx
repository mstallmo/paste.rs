import React from 'react';
import './App.css';
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';
import Home from './pages/Home';
import ViewPaste from './pages/ViewPaste';
import { Navbar, Nav, NavItem, NavLink } from 'reactstrap';

const App = () => (
  <Router>
    <div>
      <Navbar color={'light'}>
        <Nav>
          <NavItem className={'ml-auto'}>
            <NavLink to="/" tag={Link}>
              Home
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink to="/about" tag={Link}>
              About
            </NavLink>
          </NavItem>
          <NavItem>
            <NavLink to="/topics" tag={Link}>
              Topics
            </NavLink>
          </NavItem>
        </Nav>
      </Navbar>

      <hr />
      <Route exact path="/" component={Home} />
      <Route path="/about" component={About} />
      <Route path="/topics" component={Topics} />
      <Route path="/view/:hash" component={ViewPaste} />
    </div>
  </Router>
);

const About = () => (
  <div>
    <h2>About</h2>
  </div>
);

const Topics = () => (
  <div>
    <h2>Topics</h2>
  </div>
);

export default App;
