import React, { Component } from 'react';

class Home extends Component {
  constructor(props) {
    super(props);

    this.state = {
      pasteText: ''
    };
  }

  onClick = () => {
    fetch('/api/v1', {
      method: 'POST',
      body: this.state.pasteText,
      headers: {
        'Content-Type': 'text/plain'
      }
    })
      .then(res => res.text())
      .then(text => {
        console.log(text);
        const hash = text.split('/');
        this.props.history.push(`/view/${hash[hash.length - 1]}`);
      });
  };

  handleChange = value => {
    this.setState({ pasteText: value });
  };

  render() {
    return (
      <div>
        <h2 className={'header'}>Home</h2>
        <textarea
          className={'paste-input'}
          onChange={e => this.handleChange(e.target.value)}
          value={this.state.pasteText}
        />
        <br />
        <button id={'submit-button'} onClick={() => this.onClick()}>
          Submit
        </button>
      </div>
    );
  }
}

export default Home;
