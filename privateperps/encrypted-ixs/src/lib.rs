use arcis::*;

#[encrypted]
mod circuits {
    use arcis::*;

    #[instruction]
    pub fn compute_pnl(
        size_ctxt: Enc<Shared, u64>,
        leverage_ctxt: Enc<Shared, u64>,
        entry_price_ctxt: Enc<Shared, u64>,
        current_price_ctxt: Enc<Shared, u64>,
    ) -> i64 {
        let size = size_ctxt.to_arcis();
        let leverage = leverage_ctxt.to_arcis();
        let entry_price = entry_price_ctxt.to_arcis();
        let current_price = current_price_ctxt.to_arcis();

        let notional = size * leverage;
        let price_diff = current_price as i64 - entry_price as i64;
        let pnl = (notional as i64 * price_diff) / entry_price as i64;
        pnl.reveal()
    }
}
