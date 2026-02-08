<script setup lang="ts" generic="T extends any, O extends any">
import packageJson from "../../package.json";

defineOptions({
  name: "IndexPage",
});

const mode = import.meta.env.MODE;
const baseUrl =
  mode === "prod" ? import.meta.env.VITE_API_BASE : "http://localhost:8001/api";

const viteVersion = packageJson.devDependencies.vite;

const response = ref<Record<string, any>>({});

const router = useRouter();
const route = ref("");

function go() {
  router.push(`/hi/${route.value}`);
}

async function ping() {
  const resp = await fetch(`${baseUrl}/ping`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      code: 200,
      message: "Hello from Axum!",
    }),
  });

  const respText = await resp.text();
  response.value = JSON.parse(respText);
}
</script>

<template>
  <div>
    <div i-carbon-campsite inline-block text-4xl />
    <p>
      <a
        rel="noreferrer"
        href="https://github.com/Vincent-the-gamer/vitesse-superslim"
        target="_blank"
      >
        Vitesse Superslim
      </a>
    </p>
    <p>
      <em text-sm op75
        >Opinionated Vite starter template, but a superslim version. (*╹▽╹*)</em
      >
    </p>
    <h3 op75 m-1>Vite version: {{ viteVersion }}</h3>

    <div py-4 />

    <input
      v-model="route"
      type="text"
      placeholder="Enter route"
      h-8
      border-rd-2
      p-2
      border="~ 3 pink"
      color-black
    />

    <div flex="~ gap-2 justify-center" mt-3>
      <button btn @click="go">Go</button>
      <button btn @click="ping">Ping!</button>
    </div>

    <div mt-4 font-size-8>
      {{ response.message }}
    </div>
  </div>
</template>
