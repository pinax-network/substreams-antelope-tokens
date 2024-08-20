// import { BigInt, log } from "@graphprotocol/graph-ts";
import { Protobuf } from "as-proto/assembly";
import { toValue } from "./utils";
import { Account as AccountEntity, Token as TokenEntity, AccountBalances as AccountBalancesEntity } from "../generated/schema";
import { Events } from "./pb/antelope/eosio/token/v1/Events";
import { BalanceChange } from "./pb/antelope/eosio/token/v1/BalanceChange";
import { Create } from "./pb/antelope/eosio/token/v1/Create";
import { SupplyChange } from "./pb/antelope/eosio/token/v1/SupplyChange";
import { BigInt } from "@graphprotocol/graph-ts";

function accBalanceKey(obj: BalanceChange): string {
    return `${obj.account}:${obj.contract}:${obj.symcode}`;
}

function accountKey(obj: BalanceChange): string {
    return obj.account;
}

function handleBalanceChanges(balanceChanges: BalanceChange[]): void {
    // dump into a map to deduplicate
    const mapChanges = new Map<string, BalanceChange>();
    for (let i = 0; i < balanceChanges.length; i++) {
        const change = balanceChanges[i];
        mapChanges.set(accBalanceKey(change), change);
    }
    const values = mapChanges.values();
    for (let i = 0; i < values.length; i++) {
        const change = values[i];
        let accEntity = AccountEntity.load(accountKey(change));
        if(!accEntity){
            accEntity = new AccountEntity(accountKey(change));
            accEntity.name = change.account;
            accEntity.save();
        }

        let tokenEntity = TokenEntity.load(`${change.contract}:${change.symcode}`);
        if(!tokenEntity){
            // token creation must be handled in handleCreates
            tokenEntity = new TokenEntity(`${change.contract}:${change.symcode}`);
            tokenEntity.contract = change.contract;
            tokenEntity.symcode = change.symcode;
            tokenEntity.precision = change.precision;
            tokenEntity.holders_count = 1;

            tokenEntity.save();
        }
        else{
            tokenEntity.holders_count += 1;
            tokenEntity.save();
        }

        let accBalancesEntity = AccountBalancesEntity.load(accBalanceKey(change));
        if(!accBalancesEntity){
            accBalancesEntity = new AccountBalancesEntity(accBalanceKey(change));
            accBalancesEntity.account = accountKey(change);
            accBalancesEntity.token = `${change.contract}:${change.symcode}`;
        }
        accBalancesEntity.balance = change.balance;
        accBalancesEntity.balance_value = toValue(change.balance);
        accBalancesEntity.save();
    }
}

function handleCreates(creates: Create[]): void {
    for (let i = 0; i < creates.length; i++) {
        const create = creates[i];
        let tokenEntity = TokenEntity.load(`${create.contract}:${create.symcode}`);
        if(!tokenEntity){
            tokenEntity = new TokenEntity(`${create.contract}:${create.symcode}`);
            tokenEntity.contract = create.contract;
            tokenEntity.symcode = create.symcode;
            tokenEntity.precision = create.precision;
            tokenEntity.issuer = create.issuer;
            tokenEntity.max_supply = create.maximumSupply;
            tokenEntity.created_blocknum = BigInt.fromU64(create.blockNum);
            tokenEntity.created_tx = create.trxId;
            tokenEntity.holders_count = 0;
            tokenEntity.save();
        }
    }
}

function handleSupplyChanges(supplyChanges: SupplyChange[]): void {
    for (let i = 0; i < supplyChanges.length; i++) {
        const change = supplyChanges[i];
        let tokenEntity = TokenEntity.load(`${change.contract}:${change.symcode}`);
        if(!tokenEntity){
            tokenEntity = new TokenEntity(`${change.contract}:${change.symcode}`);
            tokenEntity.contract = change.contract;
            tokenEntity.symcode = change.symcode;
            tokenEntity.precision = change.precision;
        }
        tokenEntity.supply = change.supply;
        tokenEntity.supply_value = toValue(change.supply);
        tokenEntity.save();
    }
}

export function handleEvents(bytes: Uint8Array): void {
    const eventsProto: Events = Protobuf.decode<Events>(bytes, Events.decode);

    handleCreates(eventsProto.creates);
    handleSupplyChanges(eventsProto.supplyChanges);
    handleBalanceChanges(eventsProto.balanceChanges);

}