<script>
  import Card from '$lib/components/ui/card.svelte';
  import Label from '$lib/components/ui/label.svelte';
  import Input from '$lib/components/ui/input.svelte';
  import Select from '$lib/components/ui/select.svelte';
  import Checkbox from '$lib/components/ui/checkbox.svelte';

  let {
    clients,
    clientVersions,
    selectedClient,
    selectedClientVersion,
    port,
    uploadRate,
    downloadRate,
    completionPercent,
    initialUploaded,
    updateIntervalSeconds,
    randomizeRates,
    randomRangePercent,
    isRunning,
    onUpdate,
  } = $props();

  // Local state for form values
  let localSelectedClient = $state();
  let localSelectedClientVersion = $state();
  let localPort = $state();
  let localUploadRate = $state();
  let localDownloadRate = $state();
  let localCompletionPercent = $state();
  let localInitialUploaded = $state();
  let localUpdateIntervalSeconds = $state();
  let localRandomizeRates = $state();
  let localRandomRangePercent = $state();

  // Track if we're currently editing to prevent external updates from interfering
  let isEditing = $state(false);
  let editTimeout;

  // Update local state when props change (only when not actively editing)
  $effect(() => {
    if (!isEditing) {
      localSelectedClient = selectedClient;
      localSelectedClientVersion = selectedClientVersion;
      localPort = port;
      localUploadRate = uploadRate;
      localDownloadRate = downloadRate;
      localCompletionPercent = completionPercent;
      localInitialUploaded = initialUploaded;
      localUpdateIntervalSeconds = updateIntervalSeconds;
      localRandomizeRates = randomizeRates;
      localRandomRangePercent = randomRangePercent;
    }
  });

  // Helper to call onUpdate and manage editing state
  function updateValue(key, value) {
    isEditing = true;
    clearTimeout(editTimeout);

    if (onUpdate) {
      onUpdate({ [key]: value });
    }

    // Clear editing flag after a short delay
    editTimeout = setTimeout(() => {
      isEditing = false;
    }, 100);
  }
</script>

<Card class="p-6">
  <h2 class="mb-5 text-primary text-2xl font-semibold">⚙️ Configuration</h2>

  <!-- Client Settings -->
  <div class="mb-5">
    <h3
      class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3 pb-2 border-b border-border"
    >
      Client
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="flex flex-col gap-2">
        <Label for="client">Type</Label>
        <Select
          id="client"
          bind:value={localSelectedClient}
          disabled={isRunning}
          onchange={() => updateValue('selectedClient', localSelectedClient)}
        >
          {#each clients as client (client.id)}
            <option value={client.id}>{client.name}</option>
          {/each}
        </Select>
      </div>

      <div class="flex flex-col gap-2">
        <Label for="clientVersion">Version</Label>
        <Select
          id="clientVersion"
          bind:value={localSelectedClientVersion}
          disabled={isRunning}
          onchange={() => updateValue('selectedClientVersion', localSelectedClientVersion)}
        >
          {#each clientVersions[localSelectedClient] || [] as version (version)}
            <option value={version}>{version}</option>
          {/each}
        </Select>
      </div>

      <div class="flex flex-col gap-2">
        <Label for="port">Port</Label>
        <Input
          id="port"
          type="number"
          bind:value={localPort}
          disabled={isRunning}
          min="1024"
          max="65535"
          oninput={() => updateValue('port', localPort)}
        />
      </div>
    </div>
  </div>

  <!-- Transfer Rates -->
  <div class="mb-5">
    <h3
      class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3 pb-2 border-b border-border"
    >
      Transfer Rates
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="flex flex-col gap-2">
        <Label for="upload">↑ Upload (KB/s)</Label>
        <Input
          id="upload"
          type="number"
          bind:value={localUploadRate}
          disabled={isRunning}
          min="0"
          step="0.1"
          oninput={() => updateValue('uploadRate', localUploadRate)}
        />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="download">↓ Download (KB/s)</Label>
        <Input
          id="download"
          type="number"
          bind:value={localDownloadRate}
          disabled={isRunning}
          min="0"
          step="0.1"
          oninput={() => updateValue('downloadRate', localDownloadRate)}
        />
      </div>
    </div>
  </div>

  <!-- Initial State -->
  <div class="mb-5">
    <h3
      class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3 pb-2 border-b border-border"
    >
      Initial State
    </h3>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="flex flex-col gap-2">
        <Label for="completion">Completion %</Label>
        <Input
          id="completion"
          type="number"
          bind:value={localCompletionPercent}
          disabled={isRunning}
          min="0"
          max="100"
          oninput={() => updateValue('completionPercent', localCompletionPercent)}
        />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="initialUp">Initial Uploaded (MB)</Label>
        <Input
          id="initialUp"
          type="number"
          bind:value={localInitialUploaded}
          disabled={isRunning}
          min="0"
          oninput={() => updateValue('initialUploaded', localInitialUploaded)}
        />
      </div>

      <div class="flex flex-col gap-2">
        <Label for="updateInterval">Update Interval (sec)</Label>
        <Input
          id="updateInterval"
          type="number"
          bind:value={localUpdateIntervalSeconds}
          disabled={isRunning}
          min="1"
          max="300"
          step="1"
          oninput={() => updateValue('updateIntervalSeconds', localUpdateIntervalSeconds)}
        />
      </div>
    </div>
  </div>

  <!-- Randomization -->
  <div class="mb-0">
    <div class="flex items-center gap-3 mb-4">
      <Checkbox
        id="randomize"
        checked={localRandomizeRates}
        disabled={isRunning}
        onchange={checked => {
          localRandomizeRates = checked;
          updateValue('randomizeRates', checked);
        }}
      />
      <Label for="randomize" class="cursor-pointer font-medium"
        >Randomize rates for realistic behavior</Label
      >
    </div>

    {#if localRandomizeRates}
      <div class="bg-muted p-5 rounded-lg border border-border">
        <div class="flex justify-between items-center mb-3">
          <Label for="randomRange">Random Range</Label>
          <span
            class="text-lg font-bold text-primary px-3 py-1 bg-background rounded-md border border-primary"
            >±{localRandomRangePercent}%</span
          >
        </div>
        <input
          id="randomRange"
          type="range"
          bind:value={localRandomRangePercent}
          disabled={isRunning}
          min="1"
          max="50"
          step="1"
          class="w-full h-2 rounded bg-primary appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-6 [&::-webkit-slider-thumb]:h-6 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-white [&::-webkit-slider-thumb]:border-4 [&::-webkit-slider-thumb]:border-primary [&::-webkit-slider-thumb]:shadow-lg [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-thumb]:transition-transform [&::-webkit-slider-thumb]:hover:scale-110 [&::-moz-range-thumb]:w-6 [&::-moz-range-thumb]:h-6 [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:bg-white [&::-moz-range-thumb]:border-4 [&::-moz-range-thumb]:border-primary [&::-moz-range-thumb]:shadow-lg [&::-moz-range-thumb]:cursor-pointer [&::-moz-range-thumb]:transition-transform [&::-moz-range-thumb]:hover:scale-110 [&::-moz-range-track]:bg-transparent disabled:opacity-50 disabled:cursor-not-allowed"
          oninput={() => updateValue('randomRangePercent', localRandomRangePercent)}
        />
        <div class="flex justify-between mt-2">
          <span class="text-xs text-muted-foreground">±1%</span>
          <span class="text-xs text-muted-foreground">±50%</span>
        </div>
      </div>
    {/if}
  </div>
</Card>
