import type Expression from "@perspect3vism/ad4m/Expression";
import type Agent from "@perspect3vism/ad4m/Agent";
import type { GetByAuthorAdapter } from "@perspect3vism/ad4m/Language";
import type LanguageContext from "@perspect3vism/ad4m-language-context/LanguageContext";
import type { default as HolochainLanguageDelegate } from "@perspect3vism/ad4m-language-context/Holochain/HolochainLanguageDelegate";
import { DNA_NICK } from "./dna";

export default class ProfileAuthorAdapter implements GetByAuthorAdapter {
  #DNA: HolochainLanguageDelegate;

  constructor(context: LanguageContext) {
    this.#DNA = context.Holochain as HolochainLanguageDelegate;
  }

  //Question: For this author; assuming we resolved with profile DHT; how do we know which agent to use if they have multiple listed?
  //might not be a clear 1:1 mapping for did to agents
  ///Get expressions authored by a given Agent/Identity
  async getByAuthor(
    author: Agent,
    count: number,
    page: number
  ): Promise<void | Expression[]> {
    //TODO: resolve did
    const expression = await this.#DNA.call(
      DNA_NICK,
      "did-profiles",
      "get_profile",
      author.did
    );
    if (res != null) {
      var cloneRes = Object.assign({}, expression);
      delete cloneRes.proof;
      delete cloneRes.timestamp;
      let ad4mExpression: Expression = {
        author: {
          did: author.did,
        } as Agent,
        proof: expression.proof,
        timestamp: expression.timestamp,
        data: cloneRes.data
      }
      return [ad4mExpression]
    } else {
      return null
    }
  }
}
