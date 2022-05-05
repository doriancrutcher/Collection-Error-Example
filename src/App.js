import "regenerator-runtime/runtime";
import React from "react";
import { login, logout } from "./utils";
import "./global.css";
import "bootstrap/dist/css/bootstrap.min.css";

import { Container, Row, Card, Alert } from "react-bootstrap";
import RustStruct from "./assets/RustStruct.png";
import MethodPic from "./assets/Method.png";

import getConfig from "./config";
const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function App() {
  return (
    <div>
      <Container>
        <Row>
          {" "}
          <button onClick={login}>login</button>
          <button onClick={logout}>logout</button>
        </Row>
        <Row style={{ marginTop: "10px" }}>
          {" "}
          <Card>
            <Card.Title>Error</Card.Title>
            <Card.Body>
              The Error Message Received is as follows:
              {`json-rpc-provider.js:116 Uncaught (in promise) Error: Querying [object Object] failed: wasm execution failed with error: FunctionCallError(HostError(GuestPanic { panic_msg: "Cannot deserialize value with Borsh" })).
{`}
            </Card.Body>
          </Card>
        </Row>
        <Row style={{ marginTop: "10vh" }}>
          <Card>
            <Card.Title>To Reproduce Error</Card.Title>
            <Card.Body>
              <Container>
                <Row>1.Login</Row>
                <Row>
                  2. Open the Inspection tool on your browser and navigate to
                  the console
                </Row>
                <Row> 3. copy and paste this into your terminal</Row>
                <Row>
                  <Alert>{`await contract.add_candidate_pair({prompt:'test', name_1: 'bob', name_2:'rob'})`}</Alert>
                </Row>
              </Container>
            </Card.Body>
          </Card>
        </Row>
        <Row>
          <Card>
            <Card.Title>
              Here is a look into the rust code related to this methods
            </Card.Title>
            <Card.Body>
              <img src={RustStruct} style={{ width: "70vw" }} />
              <img
                src={MethodPic}
                style={{ width: "70vw", marginTop: "10vh" }}
              />
            </Card.Body>
          </Card>
        </Row>
      </Container>
    </div>
  );
}
