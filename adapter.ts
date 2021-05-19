import type Address from "@perspect3vism/ad4m/Address";
import type Agent from "@perspect3vism/ad4m/Agent";
import type Expression from "@perspect3vism/ad4m/Expression";
import type {
  ExpressionAdapter,
  PublicSharing,
} from "@perspect3vism/ad4m/Language";
import type LanguageContext from "@perspect3vism/ad4m-language-context/LanguageContext";
import type { default as HolochainLanguageDelegate } from "@perspect3vism/ad4m-language-context/Holochain/HolochainLanguageDelegate";
import type AgentService from "@perspect3vism/ad4m/AgentService";
import { DNA_NICK } from "./dna";

class ProfilePutAdapter implements PublicSharing {
  #agent: AgentService;
  #DNA: HolochainLanguageDelegate;

  constructor(context: LanguageContext) {
    this.#agent = context.agent;
    this.#DNA = context.Holochain as HolochainLanguageDelegate;
  }

  async createPublic(shortForm: object): Promise<Address> {
    const orderedShortFormData = Object.keys(shortForm)
      .sort()
      .reduce((obj, key) => {
        obj[key] = shortForm[key];
        return obj;
      }, {});
    const expression = this.#agent.createSignedExpression(orderedShortFormData);
    console.log("Posting expression", expression);
    const res = await this.#DNA.call(
      DNA_NICK,
      "did-profiles",
      "create_profile",
      expression
    );
    return expression.author.did;
  }
}

export default class ProfileAdapter implements ExpressionAdapter {
  #DNA: HolochainLanguageDelegate;

  putAdapter: PublicSharing;

  constructor(context: LanguageContext) {
    this.#DNA = context.Holochain as HolochainLanguageDelegate;
    this.putAdapter = new ProfilePutAdapter(context);
  }

  async get(address: Address): Promise<void | Expression> {
    console.log("Getting expression with address", address);
    const expression = await this.#DNA.call(
      DNA_NICK,
      "did-profiles",
      "get_profile",
      address
    );
    if (expression != null) {
      var cloneRes = Object.assign({}, expression);
      delete cloneRes.proof;
      delete cloneRes.timestamp;
      let ad4mExpression: Expression = {
        author: {
          did: address,
        } as Agent,
        proof: expression.proof,
        timestamp: expression.timestamp,
        data: cloneRes
      }
      return ad4mExpression
    } else {
      return null;
    }
  }
}
