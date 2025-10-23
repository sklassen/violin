<script>
	import { onMount } from 'svelte';
	import ViolinPlot from '$lib/ViolinPlot.svelte';
	import TimeSeriesPlot from '$lib/TimeSeriesPlot.svelte';
	import PointAndFigureChart from '$lib/PointAndFigureChart.svelte';
	import {
		calculate_violin_data,
		calculate_pnf_data,
		calculate_return_pnf,
		generate_uniform_data,
		generate_normal_data,
		generate_skewed_data,
		generate_bimodal_data,
		generate_fractal_data,
		optimize_box_size
	} from '$lib/vio-pkg/vio.js';

	/**
	/**
	 * @type {{
	 *   type: string;
	 *   initialParams: any;
	 *   rawData: number[] | Float64Array;
	 *   pnfData: { from: number; to: number; direction: 'Up' | 'Down', start_time: number, end_time: number }[];
	 *   nSamples: number;
	 * }}
	 */
	let { type, initialParams, rawData = $bindable(), pnfData = $bindable(), nSamples } = $props();

	let params = $state({ ...initialParams, boxSize: 1.0, reversal: 3, transactionCost: 3 });
	let seed = $state(Math.floor(Math.random() * 1000000));
	let generationTrigger = $state(0);

	/** @type {any} */
	let plotData = $state(null);
	/** @type {[number, number][]} */
	let plotTSData = $state([]);
	/** @type {{ from: number; to: number; direction: 'Up' | 'Down', start_time: number, end_time: number }[]} */
	let returnPnfData = $state([]);
	/** @type {{ from: number; to: number; direction: 'Up' | 'Down', start_time: number, end_time: number }[]} */
	let returnPnfUpData = $state([]);
	/** @type {{ from: number; to: number; direction: 'Up' | 'Down', start_time: number, end_time: number }[]} */
	let returnPnfDnData = $state([]);
  let title = type.charAt(0).toUpperCase() + type.slice(1) + " Distribution";

    const plotHeight = 300; // Standard height for all plots

	// This effect will re-run whenever the trigger changes, generating new rawData
	$effect(() => {
		generationTrigger; // Depend on the trigger

		try {
			switch (type) {
				case 'uniform':
					rawData = generate_uniform_data(params.min, params.max, params.ar_coeff, nSamples, seed);
					break;
				case 'normal':
					rawData = generate_normal_data(params.mean, params.std_dev, params.ar_coeff, nSamples, seed);
					break;
				case 'skewed':
					rawData = generate_skewed_data(params.mean, params.std_dev, params.skew, params.ar_coeff, nSamples, seed);
					break;
				case 'bimodal':
					rawData = generate_bimodal_data(params.mean1, params.std_dev1, params.mean2, params.std_dev2, params.weight, params.ar_coeff, nSamples, seed);
					break;
				case 'fractal':
					const fbmPath = generate_fractal_data(params.hurst, nSamples, seed);
					const increments = [];
					for (let i = 0; i < fbmPath.length - 1; i++) {
						increments.push(fbmPath[i + 1] - fbmPath[i]);
					}
					rawData = increments;
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
			plotData = calculate_violin_data(new Float64Array(rawData));

			const cumulative_sum_values = [];
			/** @type {[number, number][]} */
			const cumulative_sum_pairs = [];
			let current_sum = 0.0;
			for (let i = 0; i < rawData.length; i++) {
				current_sum += rawData[i];
				cumulative_sum_pairs.push([i, current_sum]);
				cumulative_sum_values.push(current_sum);
			}
			plotTSData = cumulative_sum_pairs;

			// Calculate P&F data
			pnfData = calculate_pnf_data(new Float64Array(cumulative_sum_values), params.boxSize, params.reversal);

		} else {
			plotData = null;
			plotTSData = [];
			pnfData = [];
		}
	});

	// This effect calculates the return P&F data whenever the primary P&F data changes
	$effect(() => {
		if (pnfData && pnfData.length > 0) {
			returnPnfData = calculate_return_pnf(pnfData, params.boxSize, params.reversal, params.transactionCost, BigInt(0));
			returnPnfUpData = calculate_return_pnf(pnfData, params.boxSize, params.reversal, params.transactionCost, BigInt(1));
			returnPnfDnData = calculate_return_pnf(pnfData, params.boxSize, params.reversal, params.transactionCost, BigInt(-1));
		} else {
			returnPnfData = [];
			returnPnfUpData = [];
			returnPnfDnData = [];
		}
	});

	function randomizeSeed() {
		seed = Math.floor(Math.random() * 1000000);
	}

	function generateData() {
		generationTrigger++;
	}

	function resetParams() {
		params = { ...initialParams, boxSize: 1.0 };
		generateData();
	}

	let optimizationDirection = $state(0);
	function runOptimizer() {
		const cumulative_sum_values = [];
		let current_sum = 0.0;
		for (let i = 0; i < rawData.length; i++) {
			current_sum += rawData[i];
			cumulative_sum_values.push(current_sum);
		}
		const bestBoxSize = optimize_box_size(new Float64Array(cumulative_sum_values), params.reversal, params.transactionCost, BigInt(optimizationDirection));
		params.boxSize = bestBoxSize;
	}

	onMount(() => {
		generateData();
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
        height: 300px;
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
    .data-panel {
        width: 100%;
        padding: 10px;
    }
    textarea {
        width: 100%;
        height: 100px;
        font-family: monospace;
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
                    <input type="number" step="0.1" bind:value={params.min}>
                </div>
                <div class="control-row">
                    <label for="max">Max</label>
                    <input type="number" step="0.1" bind:value={params.max}>
                </div>
            {:else if type === 'normal'}
                <div class="control-row">
                    <label for="mean">Mean</label>
                    <input type="number" step="0.1" bind:value={params.mean}>
                </div>
                <div class="control-row">
                    <label for="std_dev">Std Dev</label>
                    <input type="number" step="0.1" bind:value={params.std_dev}>
                </div>
            {:else if type === 'skewed'}
                 <div class="control-row">
                    <label for="mean">Mean</label>
                    <input type="number" step="0.1" bind:value={params.mean}>
                </div>
                <div class="control-row">
                    <label for="std_dev">Std Dev</label>
                    <input type="number" step="0.1" bind:value={params.std_dev}>
                </div>
                <div class="control-row">
                    <label for="skew">Skew</label>
                    <input type="number" step="0.1" bind:value={params.skew}>
                </div>
            {:else if type === 'bimodal'}
                <p>Distribution 1</p>
                <div class="control-row">
                    <label for="mean1">Mean 1</label>
                    <input type="number" step="0.1" bind:value={params.mean1}>
                </div>
                <div class="control-row">
                    <label for="std_dev1">Std Dev 1</label>
                    <input type="number" step="0.1" bind:value={params.std_dev1}>
                </div>
                <hr>
                <p>Distribution 2</p>
                <div class="control-row">
                    <label for="mean2">Mean 2</label>
                    <input type="number" step="0.1" bind:value={params.mean2}>
                </div>
                <div class="control-row">
                    <label for="std_dev2">Std Dev 2</label>
                    <input type="number" step="0.1" bind:value={params.std_dev2}>
                </div>
                 <hr>
                <div class="control-row">
                    <label for="weight">Weight (Dist 1)</label>
                    <input type="number" step="0.1" bind:value={params.weight}>
                </div>
			{:else if type === 'fractal'}
				<div class="control-row">
					<label for="hurst">Hurst</label>
					<input type="number" step="0.1" bind:value={params.hurst}>
				</div>
            {/if}
            <hr>
			{#if type !== 'fractal'}
				<div class="control-row">
					<label for="ar_coeff">Autoregression</label>
					<input type="number" step="0.1" bind:value={params.ar_coeff}>
				</div>
			{/if}
            <div class="control-row">
                <label for="boxSize">P&F Box Size</label>
                <input type="number" step="0.1" bind:value={params.boxSize}>
            </div>
            <div class="control-row">
                <label for="reversal">P&F Reversal</label>
                <input type="number" step="1" bind:value={params.reversal}>
            </div>
            <div class="control-row">
                <label for="transactionCost">P&F Transaction Cost</label>
                <input type="number" step="0.1" bind:value={params.transactionCost}>
            </div>
            <div class="control-row">
                <label for="optimizer-direction">Optimizer Direction</label>
                <select id="optimizer-direction" bind:value={optimizationDirection}>
                    <option value={0}>Both</option>
                    <option value={1}>Up</option>
                    <option value={-1}>Down</option>
                </select>
                <button onclick={runOptimizer}>Optimize Box Size</button>
            </div>
        </div>
        <div class="controls">
			<div class="control-row">
				<label for="seed">Seed</label>
				<input type="number" bind:value={seed} />
			</div>
			<div class="control-row">
				<button onclick={randomizeSeed}>Random</button>
				<button onclick={resetParams}>Reset</button>
				<button onclick={generateData}>Generate</button>
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
            <PointAndFigureChart data={pnfData} boxSize={params.boxSize} width={320} height={300} />
        {/if}
    </div>
    <div class="panel plot-panel">
        {#if returnPnfData.length > 0}
            <PointAndFigureChart data={returnPnfData} boxSize={params.boxSize} width={320} height={300} title="Total Return P&F" />
        {/if}
    </div>
    <div class="panel plot-panel">
        {#if returnPnfUpData.length > 0}
            <PointAndFigureChart data={returnPnfUpData} boxSize={params.boxSize} width={320} height={300} title="Total Return P&F Up" />
        {/if}
    </div>
    <div class="panel plot-panel">
        {#if returnPnfDnData.length > 0}
            <PointAndFigureChart data={returnPnfDnData} boxSize={params.boxSize} width={320} height={300} title="Total Return P&F Down" />
        {/if}
    </div>
    <div class="panel data-panel">
        <label for="rawData">Raw Time Series Data:</label>
        <textarea id="rawData" readonly>{rawData.join(', ')}</textarea>
    </div>
    <div class="panel data-panel">
        <label for="pnfData">Point & Figure Data:</label>
        <textarea id="pnfData" readonly>{JSON.stringify(pnfData)}</textarea>
    </div>
</div>
