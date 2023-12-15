// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IUniswapV2Pair {
    function balanceOf(address owner) external view returns (uint256);
    function approve(address spender, uint256 value) external returns (bool);
    function transfer(address to, uint value) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
    function token0() external view returns (address);
    function token1() external view returns (address);
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    function sync() external;
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external;
}

contract TaxChecker {

    event TaxSucceed(uint256 buyTax, uint256 sellTax);

    struct Config {
        address pair;
        address tokenIn;
        address token0;
        address token1;
        uint256 dexFee;
        uint112 reserve0;
        uint112 reserve1;
    }

    function getTax(
        address pair, 
        address tokenIn, 
        uint256 dexFee
    ) external returns (uint256 buyTax, uint256 sellTax) {
        
        Config memory config;
        config.pair = pair;
        config.tokenIn = tokenIn;
        config.dexFee = dexFee;
        // Get initial balance
        (config.reserve0, config.reserve1, ) = IUniswapV2Pair(config.pair).getReserves();

        if (config.tokenIn == IUniswapV2Pair(config.pair).token1()) {
            (config.reserve0, config.reserve1) = (config.reserve1, config.reserve0);
        }

        IUniswapV2Pair(config.tokenIn).transferFrom(config.pair, address(this), config.reserve0 / 10000);
        IUniswapV2Pair(config.pair).sync();

        (config.reserve0, config.reserve1,) = IUniswapV2Pair(config.pair).getReserves();

        config.token0 = config.tokenIn;
        config.token1;

        bool swapInDefault = true; // if 0 then token0 == token

        if (config.tokenIn == IUniswapV2Pair(config.pair).token1()) {
            (config.reserve0, config.reserve1) = (config.reserve1, config.reserve0);
            config.token1 = IUniswapV2Pair(config.pair).token0();
            swapInDefault = false;
        } else {
            config.token1 = IUniswapV2Pair(config.pair).token1();
        }

        uint256 taxIn = _buyTax(config, swapInDefault) + 1;
        
        (config.reserve0, config.reserve1,) = IUniswapV2Pair(config.pair).getReserves();
        if (!swapInDefault) {
            (config.reserve0, config.reserve1) = (config.reserve1, config.reserve0);
        }

        uint256 taxOut = _sellTax(config, swapInDefault);
        emit TaxSucceed(taxIn, taxOut);
        return (taxIn, taxOut);
    }

    function _buyTax(Config memory config, bool swapInDefault)
        internal
        returns (uint256 taxIn)
    {
        uint256 amountIn = IUniswapV2Pair(config.tokenIn).balanceOf(address(this));
        // Calc amountOut and transfer in amountIn to pair

        uint256 amountOut = getAmountOut(amountIn, config.reserve0, config.reserve1, config.dexFee);
        uint256 amountOutExpected = amountOut;

        IUniswapV2Pair(config.token0).transfer(config.pair, amountIn);

        // Check if this transfer was taxed
        if (IUniswapV2Pair(config.token0).balanceOf(config.pair) - config.reserve0 != amountIn) {
            // If yes then re calculate amountOut
            amountOut = getAmountOut(
                (IUniswapV2Pair(config.token0).balanceOf(config.pair) - config.reserve0), 
                config.reserve0, 
                config.reserve1, 
                config.dexFee
            );
        }

        // Do Swap
        amountOut -= 500; // Prevent Rounding error
        if (swapInDefault) {
            IUniswapV2Pair(config.pair).swap(0, amountOut, address(this), bytes(""));
        } else {
            IUniswapV2Pair(config.pair).swap(amountOut, 0, address(this), bytes(""));
        }

        uint256 difference = (amountOutExpected - IUniswapV2Pair(config.token1).balanceOf(address(this)));
        if (difference == 0) {
            taxIn = 0;
        } else {
            taxIn = uint16((difference * 10000) / amountOutExpected);
        }
    }

    function _sellTax(Config memory config, bool swapInDefault) internal returns (uint256 taxOut) {
        uint amountIn = IUniswapV2Pair(config.token1).balanceOf(address(this));

        // Calc amountOut and transfer in amountIn to pair
        uint amountOut = getAmountOut(amountIn, config.reserve1, config.reserve0, config.dexFee);
        uint amountOutExpected = amountOut;
        IUniswapV2Pair(config.token1).transfer(config.pair, amountIn);

        // Check if this transfer was taxed
            if (IUniswapV2Pair(config.token1).balanceOf(config.pair) - config.reserve1 != amountIn) {
                amountOut = getAmountOut(
                    (IUniswapV2Pair(config.token1).balanceOf(config.pair) - config.reserve1), 
                    config.reserve1, 
                    config.reserve0,
                    config.dexFee
                );
        }

        // Do Swap
        amountOut -= 5; // Prevent Rounding error
        if (swapInDefault) {
            IUniswapV2Pair(config.pair).swap(amountOut, 0, address(this), bytes(""));
        } else { 
            IUniswapV2Pair(config.pair).swap(0 , amountOut, address(this), bytes(""));
        }
        
        uint difference = (amountOutExpected - IUniswapV2Pair(config.token0).balanceOf(address(this)));
        if (difference == 0) {
            taxOut = 0;
        } else {
            taxOut = uint16((difference * 10000) / amountOutExpected);
        }
    }

    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut, uint256 dexFee)
        internal
        pure
        returns (uint256)
    {
        uint256 amountInWithFee = amountIn * dexFee;
        return amountInWithFee * reserveOut / ((reserveIn * 1000) + amountInWithFee) + 1;
    }
}
