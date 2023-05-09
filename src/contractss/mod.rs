use ethers::prelude::abigen;
pub mod constants;

abigen!(
    PriceNativeCurrencyChainInUSD,
    r#"[
    latestAnswer() public view virtual override returns (int256 answer)
]"#,
);

abigen!(
    ETH_ROUTER,
    r#"[
    swapETH(uint16 _dstChainId,address payable _refundAddress,bytes calldata _toAddress,uint256 _amountLD,uint256 _minAmountLD) external payable
]"#,
);

abigen!(
    ROUTER,
    "/home/lprm/github/stargate_bot/src/contractss/router.json",
    methods {
        Struct(uint256 , uint256, bytes ) as LzTxObj;
    };
);
