package com.evernym.vdrtools.ledger;

import com.evernym.vdrtools.IndyIntegrationTest;
import org.json.JSONObject;
import org.junit.Test;

public class GetRevocRegDefRequestTest extends LedgerIntegrationTest {

	@Test
	public void testBuildGetRevocRegDefRequestWorks() throws Exception {
		JSONObject expectedResult = new JSONObject()
				.put("operation", new JSONObject()
						.put("type", "115")
						.put("id", revRegDefId)
				);

		String request = Ledger.buildGetRevocRegDefRequest(IndyIntegrationTest.DID, revRegDefId).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}
}
