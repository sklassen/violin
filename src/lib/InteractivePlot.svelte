<script>
	import { onMount, createEventDispatcher } from 'svelte';
  import * as d3 from 'd3';
	import ViolinPlot from '$lib/ViolinPlot.svelte';
	import TimeSeriesPlot from '$lib/TimeSeriesPlot.svelte';
	import {
		calculate_violin_data,
		generate_uniform_data,
		generate_normal_data,
		generate_skewed_data,
		generate_bimodal_data
	} from '$lib/vio-pkg/vio.js';

	export let type;
	export let initialParams;

	const dispatch = createEventDispatcher();

	let params = { ...initialParams };
	let plotData = null;
    let title = type.charAt(0).toUpperCase() + type.slice(1) + " Distribution";

	const n_samples = 300;

	function updatePlot() {
		let rawData;
		try {
			switch (type) {
				case 'uniform':
					rawData = generate_uniform_data(params.min, params.max, n_samples);
					break;
				case 'normal':
					rawData = generate_normal_data(params.mean, params.std_dev, n_samples);
					break;
				case 'skewed':
					rawData = generate_skewed_data(params.mean, params.std_dev, params.skew, n_samples);
					break;
				case 'bimodal':
					rawData = generate_bimodal_data(params.mean1, params.std_dev1, params.mean2, params.std_dev2, params.weight, n_samples);
					break;
			}

            if (rawData && rawData.length > 0) {
			    plotData = calculate_violin_data(rawData);
				dispatch('update', { type: type, data: plotData });
            }
		} catch (e) {
			console.error(`Error generating data for ${type}:`, e);
            plotData = null;
			dispatch('update', { type: type, data: null });
		}
	}

    function handleParamChange(paramName, value) {
        params[paramName] = value;
        params = params; // This reassignment is key to triggering reactivity
    }

	onMount(() => {
		updatePlot();
	});

    // Reactive statement to update plot when params change
    $: params, updatePlot();

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
    <div class="controls">
        {#if type === 'uniform'}
            <div class="control-row">
                <label for="min">Min</label>
                <input type="range" id="min" min="-10" max="10" step="0.1" value={params.min} on:input={(e) => handleParamChange('min', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.min} on:input={(e) => handleParamChange('min', e.currentTarget.valueAsNumber)}>
            </div>
            <div class="control-row">
                <label for="max">Max</label>
                <input type="range" id="max" min="-10" max="10" step="0.1" value={params.max} on:input={(e) => handleParamChange('max', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.max} on:input={(e) => handleParamChange('max', e.currentTarget.valueAsNumber)}>
            </div>
        {:else if type === 'normal'}
            <div class="control-row">
                <label for="mean">Mean</label>
                <input type="range" id="mean" min="-5" max="5" step="0.1" value={params.mean} on:input={(e) => handleParamChange('mean', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.mean} on:input={(e) => handleParamChange('mean', e.currentTarget.valueAsNumber)}>
            </div>
            <div class="control-row">
                <label for="std_dev">Std Dev</label>
                <input type="range" id="std_dev" min="0.1" max="5" step="0.1" value={params.std_dev} on:input={(e) => handleParamChange('std_dev', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.std_dev} on:input={(e) => handleParamChange('std_dev', e.currentTarget.valueAsNumber)}>
            </div>
        {:else if type === 'skewed'}
             <div class="control-row">
                <label for="mean">Mean</label>
                <input type="range" id="mean" min="-5" max="5" step="0.1" value={params.mean} on:input={(e) => handleParamChange('mean', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.mean} on:input={(e) => handleParamChange('mean', e.currentTarget.valueAsNumber)}>
            </div>
            <div class="control-row">
                <label for="std_dev">Std Dev</label>
                <input type="range" id="std_dev" min="0.1" max="5" step="0.1" value={params.std_dev} on:input={(e) => handleParamChange('std_dev', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.std_dev} on:input={(e) => handleParamChange('std_dev', e.currentTarget.valueAsNumber)}>
            </div>
            <div class="control-row">
                <label for="skew">Skew</label>
                <input type="range" id="skew" min="-10" max="10" step="0.1" value={params.skew} on:input={(e) => handleParamChange('skew', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.skew} on:input={(e) => handleParamChange('skew', e.currentTarget.valueAsNumber)}>
            </div>
        {:else if type === 'bimodal'}
            <p>Distribution 1</p>
            <div class="control-row">
                <label for="mean1">Mean 1</label>
                <input type="range" id="mean1" min="-10" max="10" step="0.1" value={params.mean1} on:input={(e) => handleParamChange('mean1', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.mean1} on:input={(e) => handleParamChange('mean1', e.currentTarget.valueAsNumber)}>
            </div>
            <div class="control-row">
                <label for="std_dev1">Std Dev 1</label>
                <input type="range" id="std_dev1" min="0.1" max="5" step="0.1" value={params.std_dev1} on:input={(e) => handleParamChange('std_dev1', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.std_dev1} on:input={(e) => handleParamChange('std_dev1', e.currentTarget.valueAsNumber)}>
            </div>
            <hr>
            <p>Distribution 2</p>
            <div class="control-row">
                <label for="mean2">Mean 2</label>
                <input type="range" id="mean2" min="-10" max="10" step="0.1" value={params.mean2} on:input={(e) => handleParamChange('mean2', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.mean2} on:input={(e) => handleParamChange('mean2', e.currentTarget.valueAsNumber)}>
            </div>
            <div class="control-row">
                <label for="std_dev2">Std Dev 2</label>
                <input type="range" id="std_dev2" min="0.1" max="5" step="0.1" value={params.std_dev2} on:input={(e) => handleParamChange('std_dev2', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.std_dev2} on:input={(e) => handleParamChange('std_dev2', e.currentTarget.valueAsNumber)}>
            </div>
             <hr>
            <div class="control-row">
                <label for="weight">Weight (Dist 1)</label>
                <input type="range" id="weight" min="0" max="1" step="0.01" value={params.weight} on:input={(e) => handleParamChange('weight', e.currentTarget.valueAsNumber)}>
                <input type="number" value={params.weight} on:input={(e) => handleParamChange('weight', e.currentTarget.valueAsNumber)}>
            </div>
        {/if}
    </div>

    {#if plotData}
        <ViolinPlot data={plotData} title={title} width={320} height={350} />
				<TimeSeriesPlot data={d3.cumsum(plotData)} />
    {:else}
        <p>Generating plot...</p>
    {/if}
</div>
