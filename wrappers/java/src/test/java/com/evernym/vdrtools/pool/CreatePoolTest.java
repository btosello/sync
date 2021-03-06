package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.IndyIntegrationTest;
import com.evernym.vdrtools.utils.PoolUtils;
import org.junit.Test;

import java.io.File;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;

public class CreatePoolTest extends IndyIntegrationTest {

	@Test
	public void testCreatePoolWorksForNullConfig() throws Exception {
		File file = new File("testCreatePoolWorks.txn");
		file.deleteOnExit();
		assertTrue(file.createNewFile());
		PoolUtils.writeTransactions(file);

		Pool.createPoolLedgerConfig("testCreatePoolWorks", null).get();
	}

	@Test
	public void testCreatePoolWorksForConfigJSON() throws Exception {
		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("testCreatePoolWorks", createPoolLedgerConfigJSONParameter.toJson()).get();
	}

	@Test
	public void testCreatePoolWorksForTwice() throws Exception {
		thrown.expectCause(isA(PoolLedgerConfigExistsException.class));

		File genesisTxnFile = PoolUtils.createGenesisTxnFile("genesis.txn");

		PoolJSONParameters.CreatePoolLedgerConfigJSONParameter createPoolLedgerConfigJSONParameter
				= new PoolJSONParameters.CreatePoolLedgerConfigJSONParameter(genesisTxnFile.getAbsolutePath());
		Pool.createPoolLedgerConfig("pool1", createPoolLedgerConfigJSONParameter.toJson()).get();
		Pool.createPoolLedgerConfig("pool1", createPoolLedgerConfigJSONParameter.toJson()).get();
	}
}
