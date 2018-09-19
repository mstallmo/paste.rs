import React from 'react';
import { shallow } from 'enzyme';
import Home from './Home';

describe('Home Page', () => {
  let component;
  beforeAll(() => {
    component = shallow(<Home />);
  });

  it('Renders without crashing', () => {
    expect(component.text()).toEqual('Home');
  });
});
