package com.evernym.vdrtools.vdr;

import com.evernym.vdrtools.IndyException;
import com.evernym.vdrtools.IndyJava;
import com.evernym.vdrtools.LibIndy;
import com.evernym.vdrtools.ParamGuard;
import com.sun.jna.Callback;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;


public class VDR extends IndyJava.API implements AutoCloseable {

    private final int vdrHandle;

    private VDR(int vdrHandle) {
        this.vdrHandle = vdrHandle;
    }

    public int getVdrHandle() {
        return vdrHandle;
    }

    private static Callback stringCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, String str) {
            CompletableFuture<String> future = (CompletableFuture<String>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            String result = str;
            future.complete(result);
        }
    };

    private static Callback voidCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err) {

            CompletableFuture<Void> future = (CompletableFuture<Void>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            Void result = null;
            future.complete(result);
        }
    };

    private static Callback createVdrCb = new Callback() {

        @SuppressWarnings({"unused", "unchecked"})
        public void callback(int xcommand_handle, int err, int vdr_handle) {

            CompletableFuture<VDR> future = (CompletableFuture<VDR>) removeFuture(xcommand_handle);
            if (!checkResult(future, err)) return;

            VDR result = new VDR(vdr_handle);
            future.complete(result);
        }
    };

    public static CompletableFuture<VDR> createVDR() throws IndyException {
        CompletableFuture<VDR> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int result = LibIndy.api.indy_vdr_create(
                commandHandle,
                createVdrCb
        );

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<Void> registerIndyLedger(
            VDR vdr,
            String namespaceList,
            String genesisTxnData,
            String taaConfig
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");
        ParamGuard.notNull(genesisTxnData, "genesisTxnData");
        ParamGuard.notNull(taaConfig, "taaConfig");

        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.indy_vdr_register_indy_ledger(
                commandHandle,
                handle,
                namespaceList,
                genesisTxnData,
                taaConfig,
                voidCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<Void> registerCheqdLedger(
            VDR vdr,
            String namespaceList,
            String chainId,
            String nodeAddrList
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");
        ParamGuard.notNull(chainId, "chainId");
        ParamGuard.notNull(nodeAddrList, "nodeAddrList");

        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.indy_vdr_register_cheqd_ledger(
                commandHandle,
                handle,
                namespaceList,
                chainId,
                nodeAddrList,
                voidCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> ping(
            VDR vdr,
            String namespaceList
    ) throws IndyException {
        ParamGuard.notNull(namespaceList, "namespaceList");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.indy_vdr_ping(
                commandHandle,
                handle,
                namespaceList,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<Void> cleanup(
            VDR vdr
    ) throws IndyException {
        CompletableFuture<Void> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();
        int result = LibIndy.api.indy_vdr_cleanup(
                commandHandle,
                handle,
                voidCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> resolveDID(
            VDR vdr,
            String fqDID,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqDID, "fqDID");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.indy_vdr_resolve_did(
                commandHandle,
                handle,
                fqDID,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> resolveSchema(
            VDR vdr,
            String fqSchema,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqSchema, "fqSchema");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.indy_vdr_resolve_schema(
                commandHandle,
                handle,
                fqSchema,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }

    private static CompletableFuture<String> resloveCredDef(
            VDR vdr,
            String fqCredDef,
            String cacheOptions
    ) throws IndyException {
        ParamGuard.notNull(fqCredDef, "fqCredDef");
        ParamGuard.notNull(cacheOptions, "cacheOptions");

        CompletableFuture<String> future = new CompletableFuture<>();
        int commandHandle = addFuture(future);

        int handle = vdr.getVdrHandle();

        int result = LibIndy.api.indy_vdr_resolve_cred_def(
                commandHandle,
                handle,
                fqCredDef,
                cacheOptions,
                stringCb);

        checkResult(future, result);

        return future;
    }


    public CompletableFuture<Void> registerIndyLedger(
            String namespaceList,
            String genesisTxnData,
            String taaConfig
    ) throws IndyException {
        return registerIndyLedger(this, namespaceList, genesisTxnData, taaConfig);
    }


    public CompletableFuture<Void> registerCheqdLedger(
            String namespaceList,
            String chainId,
            String nodeAddrList
    ) throws IndyException {
        return registerCheqdLedger(this, namespaceList, chainId, nodeAddrList);
    }

    public CompletableFuture<String> ping(
            String namespaceList
    ) throws IndyException {
        return ping(this, namespaceList);
    }

    public CompletableFuture<Void> cleanup() throws IndyException {
        return cleanup(this);
    }

    public CompletableFuture<String> resolveDID(
            String fqDID,
            String cacheOptions
    ) throws IndyException {
        return resolveDID(this, fqDID, cacheOptions);
    }

    public CompletableFuture<String> resolveSchema(
            String fqSchema,
            String cacheOptions
    ) throws IndyException {
        return resolveSchema(this, fqSchema, cacheOptions);
    }

    public CompletableFuture<String> resloveCredDef(
            String fqCredDef,
            String cacheOptions
    ) throws IndyException {
        return resloveCredDef(this, fqCredDef, cacheOptions);
    }

    @Override
    public void close() throws InterruptedException, ExecutionException, IndyException {
        cleanup().get();
    }
}
