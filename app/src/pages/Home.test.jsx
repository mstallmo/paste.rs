import React from 'react';
import { shallow } from 'enzyme';
import Home from './Home';

describe('Home Page', () => {
  describe('render', () => {
    let component;
    beforeAll(() => {
      component = shallow(<Home />);
    });

    it('Renders without crashing', () => {
      expect(component.find('.header').text()).toEqual('Home');
    });

    it('should render a textarea for input', () => {
      expect(component.find('.paste-input').exists()).toEqual(true);
    });

    it('should have a button for submitting a new paste', () => {
      expect(component.find('#submit-button').exists()).toEqual(true);
    });
  });

  describe('functionality', () => {
    let component;
    beforeAll(() => {
      component = shallow(<Home />);
    });

    it('should pass', () => {
      expect(true).toEqual(true);
    });
  });
});
