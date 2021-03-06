package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when requesting a value from a wallet that does not contain the specified key.
 */
public class WalletItemNotFoundException extends IndyException
{
	private static final long serialVersionUID = 667964860056778208L;
	private final static String message = "No value with the specified key exists in the wallet from which it was requested.";

	/**
	 * Initializes a new WalletItemNotFoundException.
	 */
	public WalletItemNotFoundException() {
		super(message, ErrorCode.WalletItemNotFound.value());
	}
}
