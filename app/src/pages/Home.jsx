import React, { Component } from 'react';
import {
  Grid,
  Row,
  Col,
  Button,
  FormGroup,
  FormControl
} from 'react-bootstrap';

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
      <Grid>
        <Row>
          <Col md={12}>
            <h2 className={'header'}>Home</h2>
            <form>
              <FormGroup>
                <FormControl
                  componentClass={'textarea'}
                  value={this.state.pasteText}
                  onChange={e => this.handleChange(e.target.value)}
                  className={'paste-input'}
                  rows={'8'}
                />
              </FormGroup>
            </form>
            <br />
            <Button id={'submit-button'} onClick={() => this.onClick()}>
              Submit
            </Button>
          </Col>
        </Row>
      </Grid>
    );
  }
}

export default Home;
