package com.evernym.vdrtools;

import com.evernym.vdrtools.did.Did;
import com.evernym.vdrtools.did.DidResults;
import com.evernym.vdrtools.ledger.Ledger;
import com.evernym.vdrtools.pool.Pool;
import com.evernym.vdrtools.utils.PoolUtils;
import com.evernym.vdrtools.wallet.Wallet;
import org.json.JSONObject;
import org.junit.After;
import org.junit.Before;

import static org.junit.Assert.assertTrue;


public class IndyIntegrationTestWithPoolAndSingleWallet extends IndyIntegrationTest {

	public Pool pool;
	public Wallet wallet;

	@Before
	public void createPoolAndWallet() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		pool = Pool.openPoolLedger(poolName, null).get();

		Wallet.createWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();
		this.wallet = Wallet.openWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();
	}

	@After
	public void deletePoolAndWallet() throws Exception {
		pool.closePoolLedger().get();
		wallet.closeWallet().get();
		Wallet.deleteWallet(WALLET_CONFIG, WALLET_CREDENTIALS).get();
	}

	protected void checkResponseType(String response, String expectedType) {
		assertTrue(compareResponseType(response, expectedType));
	}

	protected boolean compareResponseType(String response, String expectedType) {
		JSONObject res = new JSONObject(response);
		return expectedType.equals(res.getString("op"));
	}

	protected String createStoreAndPublishDidFromTrustee() throws Exception {
		DidResults.CreateAndStoreMyDidResult trusteeDidResult = Did.createAndStoreMyDid(wallet, TRUSTEE_IDENTITY_JSON).get();
		String trusteeDid = trusteeDidResult.getDid();

		DidResults.CreateAndStoreMyDidResult myDidResult = Did.createAndStoreMyDid(wallet, "{}").get();
		String myDid = myDidResult.getDid();
		String myVerkey = myDidResult.getVerkey();

		String nymRequest = Ledger.buildNymRequest(trusteeDid, myDid, myVerkey, null, "TRUSTEE").get();
		Ledger.signAndSubmitRequest(pool, wallet, trusteeDid, nymRequest).get();

		return myDid;
	}
}
