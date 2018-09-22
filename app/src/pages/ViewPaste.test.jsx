import React from 'react';
import { shallow } from 'enzyme';
import ViewPaste from './ViewPaste';
import { mockFetch } from '../mockFetch';

describe('View Paste Page', () => {
  let component;
  const testHash = 'EGQRz2R6mK';
  let testText = 'This is my test paste text';
  beforeEach(() => {
    global.fetch = mockFetch({ textResult: testText });
    component = shallow(<ViewPaste hash={testHash} />);
  });

  afterEach(() => {
    global.fetch.mockReset();
  });

  it('should render the page without crashing', () => {
    expect(component.find('.header').text()).toEqual('Paste');
  });

  it('should take the call the API with the hash property', () => {
    expect(fetch).toHaveBeenCalledWith(`/api/${testHash}`);
  });

  it('should add the returned text to the component state', done => {
    setTimeout(() => {
      expect(component.state('pasteText')).toEqual(testText);
      done();
    });
  });

  it('should render the paste text to the page', done => {
    setTimeout(() => {
      expect(component.find('.text-area').text()).toEqual(testText);
      done();
    });
  });
});
