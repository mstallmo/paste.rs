import React, { Component } from 'react';

class ViewPaste extends Component {
  constructor(props) {
    super(props);

    this.state = {
      pasteText: ''
    };
  }

  componentDidMount() {
    fetch(`/api/v1/${this.props.match.params.hash}`)
      .then(res => res.text())
      .then(paste => {
        this.setState({ pasteText: paste });
      });
  }

  render() {
    return (
      <div>
        <h2 className={'header'}>{'Paste'}</h2>
        <div className={'text-area'}>{this.state.pasteText}</div>
      </div>
    );
  }
}

export default ViewPaste;
