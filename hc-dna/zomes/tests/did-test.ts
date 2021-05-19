import { Orchestrator, Config, InstallAgentsHapps, TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from "@holochain/tryorama";
import { create } from "lodash";
import path from "path";
//import { did_document_secp256k1, did_document_ed25519 } from "did-document";

const conductorConfig = Config.gen();

const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [path.join("../../workdir/agent-profiles.dna")]
  ]
]

const orchestrator = new Orchestrator()

orchestrator.registerScenario("create a profile and get it", async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);

    // install your happs into the coductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_profiles]] = await alice.installAgentsHapps(installation);
    //const [[bob_profiles]] = await bob.installAgentsHapps(installation);

    //Test creating a profile with secp256k1
    let createProfile = await alice_profiles.cells[0].call(
        "did-profiles",
        "create_profile",
        {
          author: { did: 'did:key:zQ3shNgAH1yUW21P5se2jnkRW1PtoyaS8SrGRr8LBhLJw1SY7' },
          timestamp: '2021-05-19T14:04:02.366Z',
          data: {
            '@context': {"foaf": "http://xmlns.com/foaf/0.1/"},
            profile: { 'foaf:AccountName': 'Alice', '@type': 'foaf:OnlineAccount' },
            signed_agent: 'NA'
          },
          proof: {
            key: '#zQ3shNgAH1yUW21P5se2jnkRW1PtoyaS8SrGRr8LBhLJw1SY7',
            signature: 'cdfc9802db7213255f04f581b089334fe171fe6bee2ee9d21a71b3d8bac974cc56059bdb07619e319b9d782edc224be80655c8f3b7c994f1a0e6e300dc3ee679'
          }
        }
    );
    t.ok(createProfile);

    let getProfile = await alice_profiles.cells[0].call(
      "did-profiles",
      "get_profile",
      "did:key:zQ3shNgAH1yUW21P5se2jnkRW1PtoyaS8SrGRr8LBhLJw1SY7"
    );
    console.log("Got did profile", getProfile);
    t.ok(getProfile);
    t.equal(getProfile["data"]["profile"]["foaf:AccountName"], "Alice");

    // //Test creating a profile with ed25519
    // let createProfileBob = await bob_profiles.cells[0].call(
    //     "did-profiles",
    //     "create_profile",
    //     {did: "did:elem:EiAC3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A", signed_agent: "asignedagent", profile: {name: "Bob"}}
    // );
    // t.ok(createProfileBob);
});

const report = orchestrator.run();

// Note: by default, there will be no report
console.log(report);