(function() {var implementors = {};
implementors["sc_client_api"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sc_client_api/trait.HeaderBackend.html\" title=\"trait sc_client_api::HeaderBackend\">HeaderBackend</a>&lt;Block&gt; for <a class=\"struct\" href=\"sc_client_api/in_mem/struct.Blockchain.html\" title=\"struct sc_client_api::in_mem::Blockchain\">Blockchain</a>&lt;Block&gt;","synthetic":false,"types":["sc_client_api::in_mem::Blockchain"]}];
implementors["sc_client_db"] = [{"text":"impl&lt;Block:&nbsp;<a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>&gt; <a class=\"trait\" href=\"sp_blockchain/backend/trait.HeaderBackend.html\" title=\"trait sp_blockchain::backend::HeaderBackend\">HeaderBackend</a>&lt;Block&gt; for <a class=\"struct\" href=\"sc_client_db/struct.BlockchainDb.html\" title=\"struct sc_client_db::BlockchainDb\">BlockchainDb</a>&lt;Block&gt;","synthetic":false,"types":["sc_client_db::BlockchainDb"]}];
implementors["sc_service"] = [{"text":"impl&lt;B, E, Block, RA&gt; <a class=\"trait\" href=\"sp_blockchain/backend/trait.HeaderBackend.html\" title=\"trait sp_blockchain::backend::HeaderBackend\">HeaderBackend</a>&lt;Block&gt; for <a class=\"struct\" href=\"sc_service/client/struct.Client.html\" title=\"struct sc_service::client::Client\">Client</a>&lt;B, E, Block, RA&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"sc_client_api/call_executor/trait.CallExecutor.html\" title=\"trait sc_client_api::call_executor::CallExecutor\">CallExecutor</a>&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;RA: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":false,"types":["sc_service::client::client::Client"]},{"text":"impl&lt;B, E, Block, RA&gt; <a class=\"trait\" href=\"sp_blockchain/backend/trait.HeaderBackend.html\" title=\"trait sp_blockchain::backend::HeaderBackend\">HeaderBackend</a>&lt;Block&gt; for &amp;<a class=\"struct\" href=\"sc_service/client/struct.Client.html\" title=\"struct sc_service::client::Client\">Client</a>&lt;B, E, Block, RA&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"sc_client_api/backend/trait.Backend.html\" title=\"trait sc_client_api::backend::Backend\">Backend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"sc_client_api/call_executor/trait.CallExecutor.html\" title=\"trait sc_client_api::call_executor::CallExecutor\">CallExecutor</a>&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sp_runtime/traits/trait.Block.html\" title=\"trait sp_runtime::traits::Block\">BlockT</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;RA: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":false,"types":["sc_service::client::client::Client"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()