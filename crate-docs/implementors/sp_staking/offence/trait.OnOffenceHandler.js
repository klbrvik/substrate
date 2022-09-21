(function() {var implementors = {};
implementors["pallet_staking"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"pallet_staking/pallet/trait.Config.html\" title=\"trait pallet_staking::pallet::Config\">Config</a>&gt; <a class=\"trait\" href=\"sp_staking/offence/trait.OnOffenceHandler.html\" title=\"trait sp_staking::offence::OnOffenceHandler\">OnOffenceHandler</a>&lt;&lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>&lt;T as <a class=\"trait\" href=\"pallet_session/pallet/trait.Config.html\" title=\"trait pallet_session::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"pallet_session/pallet/trait.Config.html#associatedtype.ValidatorId\" title=\"type pallet_session::pallet::Config::ValidatorId\">ValidatorId</a>, &lt;T as <a class=\"trait\" href=\"pallet_session/historical/pallet/trait.Config.html\" title=\"trait pallet_session::historical::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"pallet_session/historical/pallet/trait.Config.html#associatedtype.FullIdentification\" title=\"type pallet_session::historical::pallet::Config::FullIdentification\">FullIdentification</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt; for <a class=\"struct\" href=\"pallet_staking/pallet/struct.Pallet.html\" title=\"struct pallet_staking::pallet::Pallet\">Pallet</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"pallet_session/pallet/trait.Config.html\" title=\"trait pallet_session::pallet::Config\">Config</a>&lt;ValidatorId = &lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"pallet_session/historical/pallet/trait.Config.html\" title=\"trait pallet_session::historical::pallet::Config\">Config</a>&lt;FullIdentification = <a class=\"struct\" href=\"pallet_staking/struct.Exposure.html\" title=\"struct pallet_staking::Exposure\">Exposure</a>&lt;&lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>, <a class=\"type\" href=\"pallet_staking/type.BalanceOf.html\" title=\"type pallet_staking::BalanceOf\">BalanceOf</a>&lt;T&gt;&gt;, FullIdentificationOf = <a class=\"struct\" href=\"pallet_staking/struct.ExposureOf.html\" title=\"struct pallet_staking::ExposureOf\">ExposureOf</a>&lt;T&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"associatedtype\" href=\"pallet_session/pallet/trait.Config.html#associatedtype.SessionHandler\" title=\"type pallet_session::pallet::Config::SessionHandler\">SessionHandler</a>: <a class=\"trait\" href=\"pallet_session/trait.SessionHandler.html\" title=\"trait pallet_session::SessionHandler\">SessionHandler</a>&lt;&lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"associatedtype\" href=\"pallet_session/pallet/trait.Config.html#associatedtype.SessionManager\" title=\"type pallet_session::pallet::Config::SessionManager\">SessionManager</a>: <a class=\"trait\" href=\"pallet_session/trait.SessionManager.html\" title=\"trait pallet_session::SessionManager\">SessionManager</a>&lt;&lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"associatedtype\" href=\"pallet_session/pallet/trait.Config.html#associatedtype.ValidatorIdOf\" title=\"type pallet_session::pallet::Config::ValidatorIdOf\">ValidatorIdOf</a>: <a class=\"trait\" href=\"sp_runtime/traits/trait.Convert.html\" title=\"trait sp_runtime::traits::Convert\">Convert</a>&lt;&lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&lt;T as <a class=\"trait\" href=\"frame_system/pallet/trait.Config.html\" title=\"trait frame_system::pallet::Config\">Config</a>&gt;::<a class=\"associatedtype\" href=\"frame_system/pallet/trait.Config.html#associatedtype.AccountId\" title=\"type frame_system::pallet::Config::AccountId\">AccountId</a>&gt;&gt;,&nbsp;</span>","synthetic":false,"types":["pallet_staking::pallet::pallet::Pallet"]}];
implementors["sp_staking"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()