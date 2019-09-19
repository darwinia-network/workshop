import { ApiPromise, WsProvider } from '@polkadot/api';
import React, { useState, useEffect } from 'react';
import { Container, Dimmer, Loader} from 'semantic-ui-react';

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
      Connected
    </Container>
  );
}
