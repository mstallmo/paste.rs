import React, { Component } from 'react';
import {
  Container,
  Row,
  Col,
  Button,
  FormGroup,
  Input,
  Form
} from 'reactstrap';

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
        const hash = text.split('/');
        this.props.history.push(`/view/${hash[hash.length - 1]}`);
      });
  };

  handleChange = value => {
    this.setState({ pasteText: value });
  };

  render() {
    return (
      <Container>
        <Row>
          <Col xs={12}>
            <h2 className={'header'}>Home</h2>
            <Form>
              <FormGroup>
                <Input
                  type={'textarea'}
                  value={this.state.pasteText}
                  onChange={e => this.handleChange(e.target.value)}
                  className={'paste-input'}
                  rows={8}
                />
              </FormGroup>
              <Button id={'submit-button'} onClick={() => this.onClick()}>
                Submit
              </Button>
            </Form>
            <br />
          </Col>
        </Row>
      </Container>
    );
  }
}

export default Home;
