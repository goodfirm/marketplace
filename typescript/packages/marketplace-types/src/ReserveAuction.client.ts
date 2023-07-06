/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.28.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Uint128, Decimal, InstantiateMsg, Coin, ExecuteMsg, QueryMsg, QueryOptionsForMinReservePriceOffset, MinReservePriceOffset, QueryOptionsForAuctionKeyOffset, AuctionKeyOffset, NullableAuction, Addr, Timestamp, Uint64, Auction, HighBid, ArrayOfAuction, Config, HaltManager, HaltWindow, ArrayOfCoin } from "./ReserveAuction.types";
export interface ReserveAuctionReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  haltManager: () => Promise<HaltManager>;
  minReservePrices: ({
    queryOptions
  }: {
    queryOptions?: QueryOptionsForMinReservePriceOffset;
  }) => Promise<ArrayOfCoin>;
  auction: ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }) => Promise<NullableAuction>;
  auctionsBySeller: ({
    queryOptions,
    seller
  }: {
    queryOptions?: QueryOptionsForAuctionKeyOffset;
    seller: string;
  }) => Promise<ArrayOfAuction>;
  auctionsByEndTime: ({
    endTime,
    queryOptions
  }: {
    endTime: number;
    queryOptions?: QueryOptionsForAuctionKeyOffset;
  }) => Promise<ArrayOfAuction>;
}
export class ReserveAuctionQueryClient implements ReserveAuctionReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.haltManager = this.haltManager.bind(this);
    this.minReservePrices = this.minReservePrices.bind(this);
    this.auction = this.auction.bind(this);
    this.auctionsBySeller = this.auctionsBySeller.bind(this);
    this.auctionsByEndTime = this.auctionsByEndTime.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  haltManager = async (): Promise<HaltManager> => {
    return this.client.queryContractSmart(this.contractAddress, {
      halt_manager: {}
    });
  };
  minReservePrices = async ({
    queryOptions
  }: {
    queryOptions?: QueryOptionsForMinReservePriceOffset;
  }): Promise<ArrayOfCoin> => {
    return this.client.queryContractSmart(this.contractAddress, {
      min_reserve_prices: {
        query_options: queryOptions
      }
    });
  };
  auction = async ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }): Promise<NullableAuction> => {
    return this.client.queryContractSmart(this.contractAddress, {
      auction: {
        collection,
        token_id: tokenId
      }
    });
  };
  auctionsBySeller = async ({
    queryOptions,
    seller
  }: {
    queryOptions?: QueryOptionsForAuctionKeyOffset;
    seller: string;
  }): Promise<ArrayOfAuction> => {
    return this.client.queryContractSmart(this.contractAddress, {
      auctions_by_seller: {
        query_options: queryOptions,
        seller
      }
    });
  };
  auctionsByEndTime = async ({
    endTime,
    queryOptions
  }: {
    endTime: number;
    queryOptions?: QueryOptionsForAuctionKeyOffset;
  }): Promise<ArrayOfAuction> => {
    return this.client.queryContractSmart(this.contractAddress, {
      auctions_by_end_time: {
        end_time: endTime,
        query_options: queryOptions
      }
    });
  };
}
export interface ReserveAuctionInterface extends ReserveAuctionReadOnlyInterface {
  contractAddress: string;
  sender: string;
  createAuction: ({
    collection,
    duration,
    reservePrice,
    sellerFundsRecipient,
    tokenId
  }: {
    collection: string;
    duration: number;
    reservePrice: Coin;
    sellerFundsRecipient?: string;
    tokenId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  updateReservePrice: ({
    collection,
    reservePrice,
    tokenId
  }: {
    collection: string;
    reservePrice: Coin;
    tokenId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  cancelAuction: ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  placeBid: ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  settleAuction: ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class ReserveAuctionClient extends ReserveAuctionQueryClient implements ReserveAuctionInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createAuction = this.createAuction.bind(this);
    this.updateReservePrice = this.updateReservePrice.bind(this);
    this.cancelAuction = this.cancelAuction.bind(this);
    this.placeBid = this.placeBid.bind(this);
    this.settleAuction = this.settleAuction.bind(this);
  }

  createAuction = async ({
    collection,
    duration,
    reservePrice,
    sellerFundsRecipient,
    tokenId
  }: {
    collection: string;
    duration: number;
    reservePrice: Coin;
    sellerFundsRecipient?: string;
    tokenId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_auction: {
        collection,
        duration,
        reserve_price: reservePrice,
        seller_funds_recipient: sellerFundsRecipient,
        token_id: tokenId
      }
    }, fee, memo, funds);
  };
  updateReservePrice = async ({
    collection,
    reservePrice,
    tokenId
  }: {
    collection: string;
    reservePrice: Coin;
    tokenId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_reserve_price: {
        collection,
        reserve_price: reservePrice,
        token_id: tokenId
      }
    }, fee, memo, funds);
  };
  cancelAuction = async ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      cancel_auction: {
        collection,
        token_id: tokenId
      }
    }, fee, memo, funds);
  };
  placeBid = async ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      place_bid: {
        collection,
        token_id: tokenId
      }
    }, fee, memo, funds);
  };
  settleAuction = async ({
    collection,
    tokenId
  }: {
    collection: string;
    tokenId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      settle_auction: {
        collection,
        token_id: tokenId
      }
    }, fee, memo, funds);
  };
}