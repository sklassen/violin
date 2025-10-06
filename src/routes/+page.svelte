<script>
	import { onMount } from 'svelte';
	import init from '$lib/vio-pkg/vio.js';
	import InteractivePlot from '$lib/InteractivePlot.svelte';
	import TimeSeriesPlot from '$lib/TimeSeriesPlot.svelte';

	/** @type {Error | null} */
	let error = null;
	let initialized = false;

	// State to hold the data for all plots
	let plotDataStore = {
		uniform: null,
		normal: null,
		skewed: null,
		bimodal: null
	};

	function handlePlotUpdate(event) {
		const { type, data } = event.detail;
		plotDataStore[type] = data;
		plotDataStore = plotDataStore; // Trigger reactivity
	}

	// Initial parameters for each plot type
	const uniformParams = { min: -2, max: 2 };
	const normalParams = { mean: 0, std_dev: 1 };
	const skewedParams = { mean: 0, std_dev: 1, skew: 5 };
	const bimodalParams = { mean1: -3, std_dev1: 1, mean2: 3, std_dev2: 1, weight: 0.5 };

	onMount(async () => {
		try {
			console.log('WASM module initializing...');
			await init();
			console.log('WASM module initialized.');
			initialized = true;
		} catch (/** @type {any} */ e) {
			console.error('Error during WASM execution:', e);
			error = e;
		}
	});
</script>

<style>
	main {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
		justify-content: center;
		padding: 20px;
	}
	h1 {
		width: 100%;
		text-align: center;
	}
	.plot-group {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

<h1>Interactive Svelte + Rust (WASM) Violin Plot Demo</h1>

{#if error}
	<p style="color: red;">Error: {error.message}</p>
{:else if !initialized}
	<p>Initializing WASM module...</p>
{:else}
	<main>
		<div class="plot-group">
			<InteractivePlot type="uniform" initialParams={uniformParams} on:update={handlePlotUpdate} />
			{#if plotDataStore.uniform?.cumulative_sum}
				<TimeSeriesPlot data={plotDataStore.uniform.cumulative_sum} />
			{/if}
		</div>
		<div class="plot-group">
			<InteractivePlot type="normal" initialParams={normalParams} on:update={handlePlotUpdate} />
			{#if plotDataStore.normal?.cumulative_sum}
				<TimeSeriesPlot data={plotDataStore.normal.cumulative_sum} />
			{/if}
		</div>
		<div class="plot-group">
			<InteractivePlot type="skewed" initialParams={skewedParams} on:update={handlePlotUpdate} />
			{#if plotDataStore.skewed?.cumulative_sum}
				<TimeSeriesPlot data={plotDataStore.skewed.cumulative_sum} />
			{/if}
		</div>
		<div class="plot-group">
			<InteractivePlot type="bimodal" initialParams={bimodalParams} on:update={handlePlotUpdate} />
			{#if plotDataStore.bimodal?.cumulative_sum}
				<TimeSeriesPlot data={plotDataStore.bimodal.cumulative_sum} />
			{/if}
		</div>
	</main>
{/if}