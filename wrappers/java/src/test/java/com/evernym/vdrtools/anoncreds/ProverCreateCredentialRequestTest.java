package com.evernym.vdrtools.anoncreds;

import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.wallet.WalletItemNotFoundException;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;

import java.util.concurrent.ExecutionException;

public class ProverCreateCredentialRequestTest extends AnoncredsIntegrationTest {

	@Test
	public void testProverCreateAndStoreCredentialReqWorks() throws Exception {
	}

	@Test
	public void testProverCreateAndStoreCredentialReqWorksForInvalidCredentialOffer() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String credentialOffer = String.format("{\"issuer_did\":\"%s\"}", issuerDid);

		Anoncreds.proverCreateCredentialReq(wallet, proverDid, credentialOffer, issuer1gvtCredDef, masterSecretId).get();
	}

	@Test
	public void testProverCreateAndStoreCredentialReqWorksForInvalidMasterSecret() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(WalletItemNotFoundException.class));

		Anoncreds.proverCreateCredentialReq(wallet, proverDid, issuer1GvtCredOffer, issuer1gvtCredDef, masterSecretId + "a").get();
	}
}
