## Introduction

The spark market SDK is an interface which enables calls upon the Orderbook contract.

It exposes a method for each function in the ABI in addition to some utility methods.

## API

### Utility methods

#### Deploy

The deploy method creates a new contract and deploys it to the network. It returns a wrapper for the contract instance which enables users to interact with the contract.

### New

The new method enables interaction with an existing / deployed contract without having to redeploy.

### ID

The ID method returns the contract ID

### With Account

The account method switches the user (equivalent to msg.sender in Solidity) who makes contract calls

### Contract methods

#### Create Market

The method creates a new market for a specified asset ID

#### Open Order

The method opens a new order for the contract caller

#### Cancel Order

The method cancels an existing order for the contract caller

#### Match Orders

The method matches two orders, a sell order and a buy order, for any users

#### Get Market ID

The method returns the ID for the market of the asset

#### Market Exists

The method returns whether a market exists for an asset

#### Order by ID

The method returns information about an order for a specific order ID

#### Orders by trader

The method returns all orders for a user
