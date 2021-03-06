package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempting to use a wallet with a pool other than the pool the wallet was created for.
 */
public class WrongWalletForPoolException extends IndyException
{
	private static final long serialVersionUID = -8931044806844925321L;
	private final static String message = "The wallet specified is not compatible with the open pool.";

	/**
	 * Initializes a new WrongWalletForPoolException.
	 */
	public WrongWalletForPoolException()
	{
		super(message, ErrorCode.WalletIncompatiblePoolError.value());
	}
}
