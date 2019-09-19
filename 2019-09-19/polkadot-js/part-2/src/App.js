import { ApiPromise, WsProvider } from '@polkadot/api';
import keyring from '@polkadot/ui-keyring';
import React, { useState, useEffect } from 'react';
import { Container, Dimmer, Loader} from 'semantic-ui-react';

import Balances from './Balances';
import NodeInfo from './NodeInfo';
import Transfer from './Transfer';
import 'semantic-ui-css/semantic.min.css'

 export default function App () {
  const [api, setApi] = useState();
  const [apiReady, setApiReady] = useState();
  // const WS_PROVIDER = 'ws://127.0.0.1:9944';
  const WS_PROVIDER = 'wss://dev-node.substrate.dev:9944';

  useEffect(() => {
    const provider = new WsProvider(WS_PROVIDER);

    ApiPromise.create({provider})
      .then((api) => {
        setApi(api);
        api.isReady.then(() => setApiReady(true));
      })
      .catch((e) => console.error(e));
  }, []);

  useEffect(() => {
    keyring.loadAll({
      isDevelopment: true
    });
  },[]);



  const loader = function (text){
    return (
      <Dimmer active>
        <Loader size='small'>{text}</Loader>
      </Dimmer>
    );
  };

  if(!apiReady){
    return loader('Connecting to the blockchain')
  }

  return (
    <Container>
      <NodeInfo
        api={api}
      />
      <Transfer
        api={api}
        keyring={keyring}
      />
      <Balances
        keyring={keyring}
        api={api}
      />
    </Container>
  );
}
