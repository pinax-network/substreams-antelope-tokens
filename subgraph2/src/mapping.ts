// import { BigInt, log } from "@graphprotocol/graph-ts";
import { Protobuf } from "as-proto/assembly";
import { toValue } from "./utils";
import { Account as AccountEntity, Token as TokenEntity, AccountBalances as AccountBalancesEntity } from "../generated/schema";
import { Events } from "./pb/antelope/eosio/token/v1/Events";
import { BalanceChange } from "./pb/antelope/eosio/token/v1/BalanceChange";

function tokenKey(obj: BalanceChange): string {
    return `${obj.contract}:${obj.symcode}`;
}

function accBalanceKey(obj: BalanceChange): string {
    return `${obj.account}:${obj.contract}:${obj.symcode}`;
}

function accountKey(obj: BalanceChange): string {
    return obj.account;
}

export function handleEvents(bytes: Uint8Array): void {
    const eventsProto: Events = Protobuf.decode<Events>(bytes, Events.decode);

    const balanceChanges = new Map<string, BalanceChange>();
    for (let i = 0; i < eventsProto.balanceChanges.length; i++) {
        const change = eventsProto.balanceChanges[i];
        balanceChanges.set(accBalanceKey(change), change);
    }
    const values = balanceChanges.values();
    for (let i = 0; i < values.length; i++) {
        const change = values[i];
        let accEntity = AccountEntity.load(accountKey(change));
        if(!accEntity){
            accEntity = new AccountEntity(accountKey(change));
            accEntity.name = change.account;
            accEntity.save();
        }

        let tokenEntity = TokenEntity.load(tokenKey(change));
        if(!tokenEntity){
            tokenEntity = new TokenEntity(tokenKey(change));
            tokenEntity.contract = change.contract;
            tokenEntity.symcode = change.symcode;
            tokenEntity.precision = change.precision;
            tokenEntity.issuer = change.account;
            tokenEntity.supply = change.balance;
            tokenEntity.max_supply = change.balance;
            tokenEntity.supply_value = toValue(change.balance);
            tokenEntity.save();
        }

        let accBalancesEntity = AccountBalancesEntity.load(accBalanceKey(change));
        if(!accBalancesEntity){
            accBalancesEntity = new AccountBalancesEntity(accBalanceKey(change));
            accBalancesEntity.account = accountKey(change);
            accBalancesEntity.token = tokenKey(change);
        }
        accBalancesEntity.balance = change.balance;
        accBalancesEntity.balance_value = toValue(change.balance);
        accBalancesEntity.save();
    }

}