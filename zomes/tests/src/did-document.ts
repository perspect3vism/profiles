const did_document_secp256k1 = {
    "keyAgreement": [
      {
        "usage": "signing",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#keyAgreement",
        "type": "X25519KeyAgreementKey2019",
        "publicKeyBase58": "ENpfk9K9J6uss5qu6BrAszioE732mYCobmMPSpvB3faM",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      }
    ],
    "service": [
      {
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#openid",
        "serviceEndpoint": "https://openid.example.com/",
        "type": "OpenIdConnectVersion1.0Service"
      }
    ],
    "publicKey": [
      {
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#primary",
        "usage": "signing",
        "publicKeyHex": "02411cc661e9c14265edbd70a01e06a7238ae09752a7d52b78a6729e28c1d19ffe",
        "type": "Secp256k1VerificationKey2018",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "type": "Secp256k1VerificationKey2018",
        "usage": "recovery",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#recovery",
        "publicKeyHex": "02c00982681081372cbb941cd2c9745908316e1373ac333479f0deabcad0e9d574",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "type": "Ed25519VerificationKey2018",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv",
        "usage": "signing",
        "publicKeyBase58": "atEBuHypSkQx7486xT5FUkoBLqvNcWyNK2Xz9EPjdMy",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "usage": "signing",
        "publicKeyPem": "-----BEGIN PUBLIC KEY\nMIIBCgKCAQEAvzoCEC2rpSpJQaWZbUmlsDNwp83Jr4fi6KmBWIwnj1MZ6CUQ7rBa\nsuLI8AcfX5/10scSfQNCsTLV2tMKQaHuvyrVfwY0dINk+nkqB74QcT2oCCH9XduJ\njDuwWA4xLqAKuF96FsIes52opEM50W7/W7DZCKXkC8fFPFj6QF5ZzApDw2Qsu3yM\nRmr7/W9uWeaTwfPx24YdY7Ah+fdLy3KN40vXv9c4xiSafVvnx9BwYL7H1Q8NiK9L\nGEN6+JSWfgckQCs6UUBOXSZdreNN9zbQCwyzee7bOJqXUDAuLcFARzPw1EsZAyjV\ntGCKIQ0/btqK+jFunT2NBC8RItanDZpptQIDAQAB\nEND PUBLIC KEY-----\r\n",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#delegate",
        "type": "Secp256k1VerificationKey2018",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "type": "Ed25519VerificationKey2018",
        "publicKeyJwk": {
          "y": "36uMVGM7hnw-N6GnjFcihWE3SkrhMLzzLCdPMXPEXlA",
          "kty": "EC",
          "x": "dWCvM4fTdeM0KmloF57zxtBPXTOythHPMm1HCLrdd3A",
          "kid": "JUvpllMEYUZ2joO59UNui_XYDqxVqiFLLAJ8klWuPBw",
          "crv": "secp256k1"
        },
        "usage": "signing",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#key-JUvpllMEYUZ2joO59UNui_XYDqxVqiFLLAJ8klWuPBw",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      }
    ],
    "assertionMethod": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv"
    ],
    "@context": "https://w3id.org/did/v1",
    "authentication": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv",
      {
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#authentication",
        "type": "Ed25519VerificationKey2018",
        "usage": "signing",
        "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      }
    ],
    "capabilityDelegation": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv"
    ],
    "capabilityInvocation": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv"
    ],
    "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A",
    "signed_agent": ""
};

const did_docuemnt_ed25519 = {
    "keyAgreement": [
      {
        "usage": "signing",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#keyAgreement",
        "type": "X25519KeyAgreementKey2019",
        "publicKeyBase58": "ENpfk9K9J6uss5qu6BrAszioE732mYCobmMPSpvB3faM",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      }
    ],
    "service": [
      {
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#openid",
        "serviceEndpoint": "https://openid.example.com/",
        "type": "OpenIdConnectVersion1.0Service"
      }
    ],
    "publicKey": [
      {
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#primary",
        "usage": "signing",
        "publicKeyHex": "02411cc661e9c14265edbd70a01e06a7238ae09752a7d52b78a6729e28c1d19ffe",
        "type": "Secp256k1VerificationKey2018",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "type": "Secp256k1VerificationKey2018",
        "usage": "recovery",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#recovery",
        "publicKeyHex": "02c00982681081372cbb941cd2c9745908316e1373ac333479f0deabcad0e9d574",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "type": "Ed25519VerificationKey2018",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv",
        "usage": "signing",
        "publicKeyBase58": "atEBuHypSkQx7486xT5FUkoBLqvNcWyNK2Xz9EPjdMy",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "usage": "signing",
        "publicKeyPem": "-----BEGIN PUBLIC KEY\nMIIBCgKCAQEAvzoCEC2rpSpJQaWZbUmlsDNwp83Jr4fi6KmBWIwnj1MZ6CUQ7rBa\nsuLI8AcfX5/10scSfQNCsTLV2tMKQaHuvyrVfwY0dINk+nkqB74QcT2oCCH9XduJ\njDuwWA4xLqAKuF96FsIes52opEM50W7/W7DZCKXkC8fFPFj6QF5ZzApDw2Qsu3yM\nRmr7/W9uWeaTwfPx24YdY7Ah+fdLy3KN40vXv9c4xiSafVvnx9BwYL7H1Q8NiK9L\nGEN6+JSWfgckQCs6UUBOXSZdreNN9zbQCwyzee7bOJqXUDAuLcFARzPw1EsZAyjV\ntGCKIQ0/btqK+jFunT2NBC8RItanDZpptQIDAQAB\nEND PUBLIC KEY-----\r\n",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#delegate",
        "type": "Secp256k1VerificationKey2018",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      },
      {
        "type": "Ed25519VerificationKey2018",
        "publicKeyJwk": {
          "y": "36uMVGM7hnw-N6GnjFcihWE3SkrhMLzzLCdPMXPEXlA",
          "kty": "EC",
          "x": "dWCvM4fTdeM0KmloF57zxtBPXTOythHPMm1HCLrdd3A",
          "kid": "JUvpllMEYUZ2joO59UNui_XYDqxVqiFLLAJ8klWuPBw",
          "crv": "secp256k1"
        },
        "usage": "signing",
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#key-JUvpllMEYUZ2joO59UNui_XYDqxVqiFLLAJ8klWuPBw",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      }
    ],
    "assertionMethod": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv"
    ],
    "@context": "https://w3id.org/did/v1",
    "authentication": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv",
      {
        "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#authentication",
        "type": "Ed25519VerificationKey2018",
        "usage": "signing",
        "publicKeyBase58": "H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV",
        "controller": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A"
      }
    ],
    "capabilityDelegation": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv"
    ],
    "capabilityInvocation": [
      "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A#edv"
    ],
    "id": "did:elem:ropsten:EiAS3mqC4OLMKOwcz3ItIL7XfWduPT7q3Fa4vHgiCfSG2A",
    "signed_agent": ""
};

export { did_document };