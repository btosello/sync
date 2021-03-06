package com.evernym.vdrtools.demo;

import com.evernym.vdrtools.IndyIntegrationTestWithPoolAndSingleWallet;
import com.evernym.vdrtools.did.Did;
import com.evernym.vdrtools.did.DidJSONParameters;
import com.evernym.vdrtools.did.DidResults;
import com.evernym.vdrtools.ledger.Ledger;
import org.junit.Test;


public class ReplaceKeysDemoTest extends IndyIntegrationTestWithPoolAndSingleWallet {

	@Test
	public void testReplaceKeysDemoWorks() throws Exception {
		// 1. Create My Did
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();
		String myDid = result.getDid();
		String myVerkey = result.getVerkey();

		// 2. Create Their Did from Trustee1 seed
		DidJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		DidResults.CreateAndStoreMyDidResult createTheirDidResult = Did.createAndStoreMyDid(wallet, theirDidJson.toJson()).get();
		String trusteeDid = createTheirDidResult.getDid();

		// 3. Build and send Nym Request
		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		// 4. Start replacing of keys
		String newVerkey = Did.replaceKeysStart(wallet, myDid, "{}").get();

		// 5. Build and send Nym Request with new key
		nymRequest = Ledger.buildNymRequest(myDid, myDid, newVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, nymRequest).get();

		// 6. Apply replacing of keys
		Did.replaceKeysApply(wallet, myDid).get();

		// 7. Send schema request
		String schemaRequest = Ledger.buildSchemaRequest(myDid, SCHEMA_DATA).get();
		Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();
	}

	@Test
	public void testReplaceKeysWithoutNymTransaction() throws Exception {
		// 1. Create My Did
		DidResults.CreateAndStoreMyDidResult result = Did.createAndStoreMyDid(wallet, "{}").get();
		String myDid = result.getDid();
		String myVerkey = result.getVerkey();

		// 2. Create Their Did from Trustee1 seed
		DidJSONParameters.CreateAndStoreMyDidJSONParameter theirDidJson =
				new DidJSONParameters.CreateAndStoreMyDidJSONParameter(null, TRUSTEE_SEED, null, null);

		DidResults.CreateAndStoreMyDidResult createTheirDidResult = Did.createAndStoreMyDid(wallet, theirDidJson.toJson()).get();
		String trusteeDid = createTheirDidResult.getDid();

		// 3. Build and send Nym Request
		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, null).get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		// 4. Start replacing of keys
		Did.replaceKeysStart(wallet, myDid, "{}").get();

		// 5. Apply replacing of keys
		Did.replaceKeysApply(wallet, myDid).get();

		// 6. Send schema request
		String schemaRequest = Ledger.buildSchemaRequest(myDid, SCHEMA_DATA).get();
		String response = Ledger.signAndSubmitRequest(pool, wallet, myDid, schemaRequest).get();
		checkResponseType(response,"REQNACK" );
	}
}
