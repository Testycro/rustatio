<script>
  import Card from '$lib/components/ui/card.svelte';
  import Checkbox from '$lib/components/ui/checkbox.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Label from '$lib/components/ui/label.svelte';

  let {
    progressiveRatesEnabled,
    targetUploadRate,
    targetDownloadRate,
    progressiveDurationHours,
    isRunning,
    onUpdate,
  } = $props();

  // Local state - initialized in $effect to avoid warnings
  let localProgressiveRatesEnabled = $state();
  let localTargetUploadRate = $state();
  let localTargetDownloadRate = $state();
  let localProgressiveDurationHours = $state();

  // Update local state when props change (only if different to prevent loops)
  $effect(() => {
    if (localProgressiveRatesEnabled !== progressiveRatesEnabled)
      localProgressiveRatesEnabled = progressiveRatesEnabled;
    if (localTargetUploadRate !== targetUploadRate) localTargetUploadRate = targetUploadRate;
    if (localTargetDownloadRate !== targetDownloadRate)
      localTargetDownloadRate = targetDownloadRate;
    if (localProgressiveDurationHours !== progressiveDurationHours)
      localProgressiveDurationHours = progressiveDurationHours;
  });

  function updateValue(key, value) {
    if (onUpdate) {
      onUpdate({ [key]: value });
    }
  }
</script>

<Card class="p-5">
  <h2 class="mb-4 text-primary text-xl font-semibold">📈 Progressive Rates</h2>
  <div class="flex flex-col gap-3">
    <div class="flex items-center gap-3 p-2 bg-muted rounded-md">
      <Checkbox
        id="progressive-enabled"
        checked={localProgressiveRatesEnabled}
        disabled={isRunning}
        onchange={checked => {
          localProgressiveRatesEnabled = checked;
          updateValue('progressiveRatesEnabled', checked);
        }}
      />
      <Label
        for="progressive-enabled"
        class="cursor-pointer text-sm text-muted-foreground font-medium"
        >Enable Progressive Adjustment</Label
      >
    </div>

    {#if localProgressiveRatesEnabled}
      <div class="flex flex-col gap-2 mt-2">
        <div class="flex items-center gap-2 p-2 bg-muted rounded-md">
          <Label for="targetUpload" class="min-w-[70px] text-xs text-muted-foreground font-semibold"
            >↑ Target</Label
          >
          <Input
            id="targetUpload"
            type="number"
            bind:value={localTargetUploadRate}
            disabled={isRunning}
            min="0"
            step="0.1"
            class="flex-1 max-w-[100px] h-9"
            oninput={() => updateValue('targetUploadRate', localTargetUploadRate)}
          />
          <span class="text-xs text-muted-foreground font-semibold min-w-[40px]">KB/s</span>
        </div>

        <div class="flex items-center gap-2 p-2 bg-muted rounded-md">
          <Label
            for="targetDownload"
            class="min-w-[70px] text-xs text-muted-foreground font-semibold">↓ Target</Label
          >
          <Input
            id="targetDownload"
            type="number"
            bind:value={localTargetDownloadRate}
            disabled={isRunning}
            min="0"
            step="0.1"
            class="flex-1 max-w-[100px] h-9"
            oninput={() => updateValue('targetDownloadRate', localTargetDownloadRate)}
          />
          <span class="text-xs text-muted-foreground font-semibold min-w-[40px]">KB/s</span>
        </div>

        <div class="flex items-center gap-2 p-2 bg-muted rounded-md">
          <Label
            for="progressiveDuration"
            class="min-w-[70px] text-xs text-muted-foreground font-semibold">Duration</Label
          >
          <Input
            id="progressiveDuration"
            type="number"
            bind:value={localProgressiveDurationHours}
            disabled={isRunning}
            min="0.1"
            max="48"
            step="0.1"
            class="flex-1 max-w-[100px] h-9"
            oninput={() => updateValue('progressiveDurationHours', localProgressiveDurationHours)}
          />
          <span class="text-xs text-muted-foreground font-semibold min-w-[40px]">hrs</span>
        </div>
      </div>
    {/if}
  </div>
</Card>
