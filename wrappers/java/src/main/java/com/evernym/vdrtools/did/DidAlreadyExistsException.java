package com.evernym.vdrtools.did;

import com.evernym.vdrtools.ErrorCode;
import com.evernym.vdrtools.IndyException;

/**
 * Exception thrown when an anoncreds accumulator is full.
 */
public class DidAlreadyExistsException extends IndyException
{
	private static final long serialVersionUID = -6792822612990030627L;
	private final static String message = "The anoncreds accumulator is full.";

	/**
	 * Initializes a new DidAlreadyExistsError.
	 */
	public DidAlreadyExistsException()
	{
		super(message, ErrorCode.DidAlreadyExistsError.value());
	}
}