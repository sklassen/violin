<script>
	import { onMount } from 'svelte';
	import ViolinPlot from '$lib/ViolinPlot.svelte';
	import TimeSeriesPlot from '$lib/TimeSeriesPlot.svelte';
	import PointAndFigureChart from '$lib/PointAndFigureChart.svelte';
	import {
		calculate_violin_data,
		calculate_pnf_data,
		generate_uniform_data,
		generate_normal_data,
		generate_skewed_data,
		generate_bimodal_data
	} from '$lib/vio-pkg/vio.js';

	let { type, initialParams } = $props();

	let params = $state({ ...initialParams, boxSize: 1.0 });
	let rawData = $state([]);
	let plotData = $state(null);
	let plotTSData = $state([]);
	let pnfData = $state([]);
    let title = type.charAt(0).toUpperCase() + type.slice(1) + " Distribution";

	const n_samples = 300;
    const plotHeight = 200; // Standard height for all plots

	// This effect will re-run whenever params change, generating new rawData
	$effect(() => {
		try {
			switch (type) {
				case 'uniform':
					rawData = generate_uniform_data(params.min, params.max, params.ar_coeff, n_samples);
					break;
				case 'normal':
					rawData = generate_normal_data(params.mean, params.std_dev, params.ar_coeff, n_samples);
					break;
				case 'skewed':
					rawData = generate_skewed_data(params.mean, params.std_dev, params.skew, params.ar_coeff, n_samples);
					break;
				case 'bimodal':
					rawData = generate_bimodal_data(params.mean1, params.std_dev1, params.mean2, params.std_dev2, params.weight, params.ar_coeff, n_samples);
					break;
			}
		} catch (e) {
			console.error(`Error generating data for ${type}:`, e);
            rawData = [];
		}
	});

	// This effect calculates plot data whenever rawData or boxSize changes
	$effect(() => {
		if (rawData && rawData.length > 0) {
			plotData = calculate_violin_data(rawData);

			const cumulative_sum_values = [];
			const cumulative_sum_pairs = [];
			let current_sum = 0.0;
			for (let i = 0; i < rawData.length; i++) {
				current_sum += rawData[i];
				cumulative_sum_pairs.push([i, current_sum]);
				cumulative_sum_values.push(current_sum);
			}
			plotTSData = cumulative_sum_pairs;

			// Calculate P&F data
			pnfData = calculate_pnf_data(cumulative_sum_values, params.boxSize, 3);

		} else {
			plotData = null;
			plotTSData = [];
			pnfData = [];
		}
	});

</script>

<style>
    .interactive-plot {
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 16px;
        margin: 10px;
        width: 350px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .panel {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
    .controls-panel {
        min-height: 320px; /* Ensures all control panels have same height */
        justify-content: flex-start;
    }
    .plot-panel {
        min-height: 220px; /* Ensures all plot panels have same height */
    }
    .controls {
        width: 100%;
    }
    .control-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 8px;
    }
    label {
        margin-right: 10px;
    }
    input {
        width: 60px;
    }
</style>

<div class="interactive-plot">
    <div class="panel controls-panel">
        <div class="controls">
            {#if type === 'uniform'}
                <div class="control-row">
                    <label for="min">Min</label>
                    <input type="range" id="min" min="-10" max="10" step="0.1" bind:value={params.min}>
                    <input type="number" bind:value={params.min}>
                </div>
                <div class="control-row">
                    <label for="max">Max</label>
                    <input type="range" id="max" min="-10" max="10" step="0.1" bind:value={params.max}>
                    <input type="number" bind:value={params.max}>
                </div>
            {:else if type === 'normal'}
                <div class="control-row">
                    <label for="mean">Mean</label>
                    <input type="range" id="mean" min="-5" max="5" step="0.1" bind:value={params.mean}>
                    <input type="number" bind:value={params.mean}>
                </div>
                <div class="control-row">
                    <label for="std_dev">Std Dev</label>
                    <input type="range" id="std_dev" min="0.1" max="5" step="0.1" bind:value={params.std_dev}>
                    <input type="number" bind:value={params.std_dev}>
                </div>
            {:else if type === 'skewed'}
                 <div class="control-row">
                    <label for="mean">Mean</label>
                    <input type="range" id="mean" min="-5" max="5" step="0.1" bind:value={params.mean}>
                    <input type="number" bind:value={params.mean}>
                </div>
                <div class="control-row">
                    <label for="std_dev">Std Dev</label>
                    <input type="range" id="std_dev" min="0.1" max="5" step="0.1" bind:value={params.std_dev}>
                    <input type="number" bind:value={params.std_dev}>
                </div>
                <div class="control-row">
                    <label for="skew">Skew</label>
                    <input type="range" id="skew" min="-10" max="10" step="0.1" bind:value={params.skew}>
                    <input type="number" bind:value={params.skew}>
                </div>
            {:else if type === 'bimodal'}
                <p>Distribution 1</p>
                <div class="control-row">
                    <label for="mean1">Mean 1</label>
                    <input type="range" id="mean1" min="-10" max="10" step="0.1" bind:value={params.mean1}>
                    <input type="number" bind:value={params.mean1}>
                </div>
                <div class="control-row">
                    <label for="std_dev1">Std Dev 1</label>
                    <input type="range" id="std_dev1" min="0.1" max="5" step="0.1" bind:value={params.std_dev1}>
                    <input type="number" bind:value={params.std_dev1}>
                </div>
                <hr>
                <p>Distribution 2</p>
                <div class="control-row">
                    <label for="mean2">Mean 2</label>
                    <input type="range" id="mean2" min="-10" max="10" step="0.1" bind:value={params.mean2}>
                    <input type="number" bind:value={params.mean2}>
                </div>
                <div class="control-row">
                    <label for="std_dev2">Std Dev 2</label>
                    <input type="range" id="std_dev2" min="0.1" max="5" step="0.1" bind:value={params.std_dev2}>
                    <input type="number" bind:value={params.std_dev2}>
                </div>
                 <hr>
                <div class="control-row">
                    <label for="weight">Weight (Dist 1)</label>
                    <input type="range" id="weight" min="0" max="1" step="0.01" bind:value={params.weight}>
                    <input type="number" bind:value={params.weight}>
                </div>
            {/if}
            <hr>
            <div class="control-row">
                <label for="ar_coeff">Autoregression</label>
                <input type="range" id="ar_coeff" min="0" max="0.99" step="0.01" bind:value={params.ar_coeff}>
                <input type="number" bind:value={params.ar_coeff}>
            </div>
            <div class="control-row">
                <label for="boxSize">P&F Box Size</label>
                <input type="range" id="boxSize" min="0.1" max="10" step="0.1" bind:value={params.boxSize}>
                <input type="number" bind:value={params.boxSize}>
            </div>
        </div>
    </div>

    <div class="panel plot-panel">
        {#if plotData}
            <ViolinPlot data={plotData} title={title} width={320} height={plotHeight} />
        {:else}
            <p>Generating plot...</p>
        {/if}
    </div>
    <div class="panel plot-panel">
        {#if plotTSData.length > 0}
            <TimeSeriesPlot data={plotTSData} width={320} height={plotHeight} />
        {/if}
    </div>
    <div class="panel plot-panel">
        {#if pnfData.length > 0}
            <PointAndFigureChart data={pnfData} boxSize={params.boxSize} width={320} height={plotHeight} />
        {/if}
    </div>
</div>