import { Orchestrator, Config, InstallAgentsHapps, TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from "@holochain/tryorama";
import path from "path";
//import { did_document_secp256k1, did_document_ed25519 } from "did-document";

// Set up a Conductor configuration using the handy `Conductor.config` helper.
// Read the docs for more on configuration.
const network = {
    transport_pool: [{
      type: TransportConfigType.Proxy,
      sub_transport: {type: TransportConfigType.Quic},
      proxy_config: {
        type: ProxyConfigType.LocalProxyServer,
        proxy_accept_config: ProxyAcceptConfig.AcceptAll
      }
    }],
    bootstrap_service: "https://bootstrap.holo.host"
};
const conductorConfig = Config.gen({network});
//const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const socialContext = path.join(__dirname, '../../profiles.dna.gz')

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [socialContext] // contains 1 dna, the "social-context" dna
  ]
]

// Instatiate your test's orchestrator.
// It comes loaded with a lot default behavior which can be overridden, including:
// * custom conductor startup
// * custom test result reporting
// * scenario middleware, including integration with other test harnesses
const orchestrator = new Orchestrator()

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

orchestrator.registerScenario("create a profile and get it", async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_profiles]] = await alice.installAgentsHapps(installation);
    const [[bob_profiles]] = await bob.installAgentsHapps(installation);

    //Test creating a profile with secp256k1
    let createProfile = await alice_profiles.cells[0].call(
        "did-profiles",
        "create_profile",
        {did: "did:elem:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A", signed_agent: "asignedagent", profile: {name: "Alice"}}
    );

    //Test creating a profile with ed25519
    let createProfileBob = await bob_profiles.cells[0].call(
        "did-profiles",
        "create_profile",
        {did: "did:elem:EiAC3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A", signed_agent: "asignedagent", profile: {name: "Bob"}}
    );
});

const report = orchestrator.run();

// Note: by default, there will be no report
console.log(report);