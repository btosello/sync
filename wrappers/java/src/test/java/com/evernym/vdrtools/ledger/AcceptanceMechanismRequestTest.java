package com.evernym.vdrtools.ledger;

import com.evernym.vdrtools.IndyIntegrationTest;
import org.json.JSONObject;
import org.junit.Test;


public class AcceptanceMechanismRequestTest extends IndyIntegrationTest {

	@Test
	public void testBuildAcceptanceMechanismRequest() throws Exception {
		JSONObject aml = new JSONObject()
				.put("acceptance mechanism label 1", "some acceptance mechanism description 1");

		String version = "1.0.0";
		String amlContext = "some context";

		JSONObject expectedResult = new JSONObject()
				.put("identifier", DID)
				.put("operation",
						new JSONObject()
								.put("type", "5")
								.put("aml", aml)
								.put("version", version)
								.put("amlContext", amlContext)
				);

		String request = Ledger.buildAcceptanceMechanismsRequest(DID, aml.toString(), version, amlContext).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}

	@Test
	public void testBuildGetAcceptanceMechanismRequestForTimestamp() throws Exception {
		int timestamp = 123456789;

		JSONObject expectedResult = new JSONObject()
				.put("operation",
						new JSONObject()
								.put("type", "7")
								.put("timestamp", timestamp)
				);

		String request = Ledger.buildGetAcceptanceMechanismsRequest(null, timestamp, null).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}

	@Test
	public void testBuildGetAcceptanceMechanismRequestForVersion() throws Exception {
		String version = "1.0.0";

		JSONObject expectedResult = new JSONObject()
				.put("operation",
						new JSONObject()
								.put("type", "7")
								.put("version", version)
				);

		String request = Ledger.buildGetAcceptanceMechanismsRequest(null, -1, version).get();

		assert (new JSONObject(request).toMap().entrySet()
				.containsAll(
						expectedResult.toMap().entrySet()));
	}
}