package com.evernym.vdrtools.ledger;

import com.evernym.vdrtools.IndyIntegrationTestWithPoolAndSingleWallet;
import com.evernym.vdrtools.did.Did;
import com.evernym.vdrtools.did.DidResults;
import org.junit.Test;

import static org.junit.Assert.*;

public class PoolConfigRequestTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testBuildPoolConfigRequestWorks() throws Exception {
		String expectedResult = String.format("\"identifier\":\"%s\"," +
				"\"operation\":{" +
				"\"type\":\"111\"," +
				"\"writes\":false," +
				"\"force\":false" +
				"}", DID);

		String request = Ledger.buildPoolConfigRequest(DID, false, false).get();

		assertTrue(request.contains(expectedResult));
	}

	@Test
	public void testPoolConfigRequestWorks() throws Exception {
		DidResults.CreateAndStoreMyDidResult didResult = Did.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String did = didResult.getDid();

		String request = Ledger.buildPoolConfigRequest(did, false, false).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, request).get();

		request = Ledger.buildPoolConfigRequest(did, true, false).get();
		Ledger.signAndSubmitRequest(pool, wallet, did, request).get();
	}
}
