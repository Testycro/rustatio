<script>
  let { isRunning, nextUpdateIn } = $props();
</script>

<section class="text-center my-4">
  {#if isRunning}
    <div
      class="inline-block bg-muted border-2 border-border rounded-lg px-6 py-3 text-muted-foreground text-base"
    >
      Next automatic update in: <strong class="text-xl text-primary font-bold ml-1"
        >{nextUpdateIn}s</strong
      >
    </div>
  {/if}
</section>
