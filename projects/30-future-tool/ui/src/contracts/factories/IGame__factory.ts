/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Interface, type ContractRunner } from "ethers";
import type { IGame, IGameInterface } from "../IGame";

const _abi = [
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "address",
        name: "_by",
        type: "address",
      },
    ],
    name: "Joined",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "uint8",
        name: "_shotIndex",
        type: "uint8",
      },
    ],
    name: "ShotFired",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "uint8",
        name: "_shipId",
        type: "uint8",
      },
    ],
    name: "ShotLanded",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "address",
        name: "_by",
        type: "address",
      },
    ],
    name: "Started",
    type: "event",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "address",
        name: "_by",
        type: "address",
      },
    ],
    name: "Won",
    type: "event",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
    ],
    name: "gameState",
    outputs: [
      {
        internalType: "address[2]",
        name: "_participants",
        type: "address[2]",
      },
      {
        internalType: "uint256[2]",
        name: "_boards",
        type: "uint256[2]",
      },
      {
        internalType: "uint256",
        name: "_turn",
        type: "uint256",
      },
      {
        internalType: "uint256[2]",
        name: "_hits",
        type: "uint256[2]",
      },
      {
        internalType: "address",
        name: "_winner",
        type: "address",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "_boardHash",
        type: "uint256",
      },
      {
        internalType: "uint256[2]",
        name: "a",
        type: "uint256[2]",
      },
      {
        internalType: "uint256[2][2]",
        name: "b",
        type: "uint256[2][2]",
      },
      {
        internalType: "uint256[2]",
        name: "c",
        type: "uint256[2]",
      },
    ],
    name: "joinGame",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "_turnShotIndex",
        type: "uint256",
      },
    ],
    name: "playFirstTurn",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "_gameId",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "_hitShipId",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "_prevTurnShotIndex",
        type: "uint256",
      },
      {
        internalType: "uint256",
        name: "_turnShotIndex",
        type: "uint256",
      },
      {
        internalType: "uint256[2]",
        name: "a",
        type: "uint256[2]",
      },
      {
        internalType: "uint256[2][2]",
        name: "b",
        type: "uint256[2][2]",
      },
      {
        internalType: "uint256[2]",
        name: "c",
        type: "uint256[2]",
      },
    ],
    name: "playTurn",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "_boardHash",
        type: "uint256",
      },
      {
        internalType: "uint256[2]",
        name: "a",
        type: "uint256[2]",
      },
      {
        internalType: "uint256[2][2]",
        name: "b",
        type: "uint256[2][2]",
      },
      {
        internalType: "uint256[2]",
        name: "c",
        type: "uint256[2]",
      },
    ],
    name: "startGame",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
] as const;

export class IGame__factory {
  static readonly abi = _abi;
  static createInterface(): IGameInterface {
    return new Interface(_abi) as IGameInterface;
  }
  static connect(address: string, runner?: ContractRunner | null): IGame {
    return new Contract(address, _abi, runner) as unknown as IGame;
  }
}