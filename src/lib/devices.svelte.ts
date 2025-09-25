import { Context, type ResourceReturn } from "runed";

const devicesResourceCtx = new Context<
  ResourceReturn<string[], unknown, false>
>("devices_resource_ctx");

export { devicesResourceCtx };
