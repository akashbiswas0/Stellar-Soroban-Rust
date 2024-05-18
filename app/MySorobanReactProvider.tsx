import React from "react";
import { SorobanReactProvider } from "@soroban-react/core";
import { testnet } from "@soroban-react/chains";
import { freighter } from "@soroban-react/freighter";
import type { ChainMetadata, Connector } from "@soroban-react/types";

import deployments from "";

const chains: ChainMetadata[] = [testnet];
const connectors: Connector[] = [freighter()];

export default function MySorobanReactProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <SorobanReactProvider
      chains={chains}
      appName={"Tossing App"}
      activeChain={testnet}
      connectors={connectors}
      deployments={deployments}
    >
      {children}
    </SorobanReactProvider>
  );
}
