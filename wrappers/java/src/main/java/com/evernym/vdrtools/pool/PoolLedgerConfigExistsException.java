package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when attempting to create a pool configuration with the same name as one that already exists.
 */
public class PoolLedgerConfigExistsException extends IndyException
{
	private static final long serialVersionUID = 2032790158242533689L;
	private final static String message = "A pool ledger configuration already exists with the specified name.";

	/**
	 * Initializes a new PoolLedgerConfigExistsException.
	 */
	public PoolLedgerConfigExistsException()
	{
		super(message, ErrorCode.PoolLedgerConfigAlreadyExistsError.value());
	}
}