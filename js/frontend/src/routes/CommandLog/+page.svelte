<script lang="ts">
  import gsap from "gsap";
  import { onDestroy, onMount, untrack } from "svelte";
  import { GetAppsState } from "$lib/AppState.svelte";
  import type {
    AppCommand,
    CmdrLog,
    CommandConfig,
    EventPayload,
    UnListen,
    UserEvent,
  } from "@tsck/lib";
  import { invokePayload, invokePayloadWithCallback, listen } from "$lib";
  import CmdButton from "$lib/components/CmdButton.svelte";
  import { GetRootState } from "$lib/RootState.svelte";
  const ctx = GetRootState();
  let {
    onConnected,
    splashScreen,
  }: {
    onConnected: () => void;
    splashScreen: boolean;
  } = $props();

  let activeIndex = $state(0);

  let cliAppCommand: AppCommand[] = $state([]);
  let allLogs: { name: string; log: string }[] = $state([
    { name: "comfyui", log: "Initiate..." },
    {
      name: "whatsapp",
      log: `Initiate...`,
    },
  ]);

  let LIMIT: number = $state(10);
  let logListener: UnListen = $state();
  let logRegListener: UnListen = $state();
  let logContainer: HTMLDivElement | undefined = $state();
  let connected = $state(false);

  let parkContainer: HTMLDivElement | undefined = $state();
  let buttonContainer: HTMLDivElement | undefined = $state();
  function updateCliCommand(newValue: AppCommand[]) {
    cliAppCommand = newValue;
  }

  function requestCommandConfig() {
    invokePayloadWithCallback<UserEvent, CommandConfig>(
      { type: "RequestCommand" },
      (error, result) => {
        console.log("COMMAND", error);
        if (error) return;
        LIMIT = result?.log_limit ?? 30;
        if (result?.commands) {
          updateCliCommand(result?.commands);
        }
      },
    );
  }

  onMount(() => {
    requestCommandConfig();
    logListener = listen<EventPayload, CmdrLog>(
      "tsck::event|EVENTPAYLOAD::COMMAND",
      (event) => {
        if (!event) return;
        switch (event.log_type) {
          case "Pid":
            const cmd = cliAppCommand.find(
              (c: AppCommand) => c.name == event.app_name,
            );
            if (cmd && event.pid) cmd.pid = event.pid;
            console.log(cmd);
            break;
          case "Stderr":
          case "Stdout":
            if (event.content)
              allLogs.push({ name: event.app_name, log: event.content });
            if (allLogs.length > LIMIT) {
              allLogs.splice(0, allLogs.length - LIMIT);
              if (logContainer)
                gsap.to(logContainer, {
                  scrollTop: logContainer?.scrollHeight,
                  duration: 0.1,
                  ease: "power2.out",
                });
            }
            break;
        }
      },
    );
  });
  function checkPid() {
    untrack(() => {
      const empty_pids = cliAppCommand
        .map((c) => c.pid)
        .filter((n) => n == 0).length;
      const app_len = cliAppCommand.length;
      if (empty_pids == app_len) {
        connected = false;
        resetButtons();
      } else {
        if (empty_pids == 0) {
          connected = true;
          animateButtons();
          if (onConnected) {
            if (splashScreen) {
              onConnected();
            }
          }
        }
      }
    });
  }
  $effect(() => {
    cliAppCommand;
    checkPid();
  });

  onDestroy(() => {
    if (logListener) logListener();
  });

  let x = 0;
  let scale = 1;
  // -------------------------
  // COLLAPSED → TOP RIGHT
  // -------------------------
  function animateButtons() {
    if (!buttonContainer || !parkContainer) return;

    x = 0; // reset spacing
    scale = 1;

    Array.from(buttonContainer.children).forEach((el: Element) => {
      const node = el as HTMLDivElement;
      node.style.transformOrigin = "top right";

      gsap.to(node, {
        scale,
        x: -x,
        y: 0,
        clearProps: "all",
        ease: "elastic.out(0.3, 1)",
        duration: 0.5,
        onComplete: () => {
          const r = node.getBoundingClientRect();
          x += r.width * scale + 5;
        },
      });
    });
  }

  // -------------------------
  // EXPAND → CENTER STACK
  // -------------------------
  function resetButtons() {
    if (!buttonContainer) return;
    if (scale > 1) return;
    const doc = { width: window.innerWidth, height: window.innerHeight };
    x = 0;
    Array.from(buttonContainer.children).forEach((el: Element, index) => {
      const node = el as HTMLDivElement;

      gsap.set(node, { clearProps: "all" });

      const r = node.getBoundingClientRect();

      const xx = doc.width / 2 - r.x - r.width / 2;
      const yy = doc.height / 2 - r.y - r.height / 2;
      scale = 1.4;
      console.log("animating", scale);
      gsap.to(node, {
        scale,
        x: xx,
        maxWidth: "70%",
        y: yy + index * (r.height + 20),
        duration: 0.5,
        ease: "power2.out",
      });
    });
  }

  // -------------------------
  // STATE DECISION
  // -------------------------
  function init() {
    console.log("init called with scale: ", scale);
    if (cliAppCommand.some((c) => c.pid === 0)) {
      setTimeout(() => {
        resetButtons();
      }, 100);
    } else {
      animateButtons();
    }
  }
</script>

{#if cliAppCommand}
  <div class="flex h-full w-full flex-col overflow-hidden">
    <div bind:this={parkContainer} class="flex w-full bg-base-200"></div>

    <div
      bind:this={buttonContainer}
      class="flex flex-row justify-center gap-2 p-2"
    >
      {#each cliAppCommand as cli, idx}
        <CmdButton
          onPlay={() => {
            let payload: UserEvent =
              cli.pid > 0
                ? {
                    type: "KillCommand",
                    value: cli.name,
                  }
                : {
                    type: "RunCommand",
                    value: cli.name,
                  };

            invokePayload<UserEvent>(payload);
            setTimeout(requestCommandConfig, 1000);
          }}
          onClick={() => {
            activeIndex = idx;
          }}
          {cli}
        />
      {/each}
    </div>

    <div
      bind:this={logContainer}
      class="w-full flex-1 overflow-hidden overflow-y-auto"
    >
      {#each allLogs as log}
        <div
          class="flex w-full flex-row gap-2 border border-transparent border-b-base-200 px-2 py-1 font-jetbrains text-[8px] font-normal"
        >
          <span
            class="badge w-1/7 shrink-0 font-jetbrains badge-xs text-[8px] uppercase {log.name ==
            'whatsapp'
              ? 'badge-success'
              : 'badge-error'}"
          >
            {log.name}
          </span>
          <span>
            {log.log}
          </span>
        </div>
      {/each}
    </div>
  </div>
{/if}
