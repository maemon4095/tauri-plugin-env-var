import { parse } from "$std/flags/mod.ts";
import { Builder, BuilderOptions } from "https://raw.githubusercontent.com/maemon4095/tauri-deno-builder/main/src/mod.ts";

const args = parse(Deno.args, {
    boolean: ["dev"],
});
const is_dev = args.dev;
const mode = args._[0];

const commonOptions: BuilderOptions = {
    denoConfigPath: "./deno.json",
    staticResources: ["public"]
};

const options: BuilderOptions = is_dev ? commonOptions : {
    ...commonOptions,
    minifySyntax: true,
    dropLabels: ["DEV"]
};

const builder = new Builder(options);

switch (mode) {
    case "serve": {
        await builder.serve();
        break;
    }
    case "build": {
        await builder.build();
        break;
    }
}