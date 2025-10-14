<script>
	import * as tf from '@tensorflow/tfjs';
	import PointAndFigureChart from '$lib/PointAndFigureChart.svelte';
	import ViolinPlot from '$lib/ViolinPlot.svelte';
	import TimeSeriesPlot from '$lib/TimeSeriesPlot.svelte';
	import ConfusionMatrix from '$lib/ConfusionMatrix.svelte';
	import { calculate_violin_data, calculate_pnf_data } from '$lib/vio-pkg/vio.js';

	/**
	 * @typedef {{
	 *   id: number;
	 *   type: string;
	 *   params: any;
	 *   rawData: number[];
	 *   pnfData: { from: number; to: number; direction: 'Up' | 'Down' }[];
	 * }[]} Plots
	 */

	/** @type {{ plots: Plots }} */
	let { plots } = $props();
	/** @type {tf.LayersModel | null} */
	let model = $state(null);
	let trainingStatus = $state('Not started');
	/** @type {string | null} */
	let predictionResult = $state(null);
	let epochs = $state(20);
	let batchSize = $state(32);
	let trainingProgress = $state(0);
	/** @type {{ loss: number; acc: number; mae: number } | null} */
	let finalTrainingStats = $state(null);
	/** @type {number[][] | null} */
	let confusionMatrix = $state(null);

	let isTraining = $state(false);
	let stopTrainingFlag = false;

	function resetModel() {
		if (model) {
			tf.dispose(model);
		}
		model = null;
		trainingStatus = 'Not started';
		predictionResult = null;
		trainingProgress = 0;
		finalTrainingStats = null;
		isTraining = false;
		stopTrainingFlag = false;
	}

	function stopTraining() {
		stopTrainingFlag = true;
	}

	let guessingData = $state('');
	let selectedPlotForCopy = $state(0);
	let decimalPlaces = $state(2);
	let roundingStrategy = $state('round');

	/** @type {any} */
	let guessViolinData = $state(null);
	/** @type {{ from: number; to: number; direction: 'Up' | 'Down' }[]} */
	let guessPnfData = $state([]);
	/** @type {[number, number][]} */
	let guessTimeSeriesData = $state([]);
	/** @type {{ from: number; to: number; direction: 'Up' | 'Down' } | null} */
	let predictedPnfBar = $state(null);

	function copyDataForGuessing() {
		const plot = plots[selectedPlotForCopy];
		if (!plot || !plot.rawData || plot.rawData.length === 0) {
			guessingData = 'No data to copy.';
			return;
		}

		const multiplier = Math.pow(10, decimalPlaces);
		/** @type {(num: number) => number} */
		let roundingFunction;
		switch (roundingStrategy) {
			case 'floor':
				roundingFunction = (num) => Math.floor(num * multiplier) / multiplier;
				break;
			case 'ceil':
				roundingFunction = (num) => Math.ceil(num * multiplier) / multiplier;
				break;
			case 'round':
			default:
				roundingFunction = (num) => Math.round(num * multiplier) / multiplier;
				break;
		}

		const formattedData = plot.rawData.map(roundingFunction);
		guessingData = formattedData.join(', ');
	}

	const TIME_STEP = 20;

	/**
	 * @param {number[] | Float64Array} data
	 * @param {number} timeStep
	 * @returns {[number[][], number[]]}
	 */
	function createSequences(data, timeStep) {
		const X = [];
		const y = [];
		for (let i = 0; i < data.length - timeStep; i++) {
			X.push(Array.from(data.slice(i, i + timeStep)));
			y.push(data[i + timeStep]);
		}
		return [X, y];
	}

	function trainModel() {
		isTraining = true;
		stopTrainingFlag = false;
		trainingStatus = 'Training...';
		trainingProgress = 0;
		finalTrainingStats = null;
		confusionMatrix = null;

		setTimeout(async () => {
			try {
				/** @type {number[][]} */
				const allX = [];
				/** @type {number[]} */
				const allY_reg = [];
				/** @type {number[]} */
				const allY_clf = [];

				plots.forEach((plot, index) => {
					if (plot.rawData.length > 0) {
						const [X, y_reg] = createSequences(plot.rawData, TIME_STEP);
						const y_clf = Array(X.length).fill(index);
						allX.push(...X);
						allY_reg.push(...y_reg);
						allY_clf.push(...y_clf);
					}
				});

				if (allX.length === 0) {
					trainingStatus = 'Error: No data to train on.';
					isTraining = false;
					return;
				}

				const flatX = allX.flat();
				const tensorX = tf.tensor3d(flatX, [allX.length, TIME_STEP, 1]);
				const tensorY_reg = tf.tensor2d(allY_reg, [allY_reg.length, 1]);
				const tensorY_clf = tf.oneHot(tf.tensor1d(allY_clf, 'int32'), plots.length);

				const input = tf.input({ shape: [TIME_STEP, 1] });
				const lstm = /** @type {tf.SymbolicTensor} */ (
					tf.layers.lstm({ units: 32, returnSequences: false }).apply(input)
				);
				const clf_output = tf.layers
					.dense({ units: plots.length, activation: 'softmax', name: 'clf_output' })
					.apply(lstm);
				const reg_output = tf.layers.dense({ units: 1, name: 'reg_output' }).apply(lstm);

				const newModel = tf.model({
					inputs: input,
					outputs: [
						/** @type {tf.SymbolicTensor} */ (clf_output),
						/** @type {tf.SymbolicTensor} */ (reg_output)
					]
				});

				newModel.compile({
					optimizer: 'adam',
					loss: { clf_output: 'categoricalCrossentropy', reg_output: 'meanSquaredError' },
					metrics: { clf_output: 'accuracy', reg_output: 'mae' }
				});

				const history = await newModel.fit(tensorX, [tensorY_clf, tensorY_reg], {
					epochs: epochs,
					batchSize: batchSize,
					callbacks: {
						onEpochEnd: (epoch, logs) => {
							if (stopTrainingFlag) {
								newModel.stopTraining = true;
							}
							if (logs) {
								trainingStatus = `Epoch ${epoch}/${epochs - 1}: loss = ${logs.loss.toFixed(
									4
								)}, acc = ${(logs.clf_output_acc || 0).toFixed(4)}`;
								trainingProgress = epoch + 1;
							}
						}
					}
				});

				if (stopTrainingFlag) {
					trainingStatus = 'Training stopped by user.';
				} else {
					model = newModel;
					trainingStatus = 'Training complete!';
					const lastEpochIndex = history.epoch.length - 1;
					finalTrainingStats = {
						loss: /** @type {number} */ (history.history.loss[lastEpochIndex]),
						acc: /** @type {number} */ (history.history.clf_output_acc[lastEpochIndex]),
						mae: /** @type {number} */ (history.history.reg_output_mae[lastEpochIndex])
					};

					// Compute and store the confusion matrix
					const [pred_clf] = /** @type {tf.Tensor[]} */ (model.predict(tensorX));
					const pred_classes = pred_clf.argMax(-1);
					const true_classes = tf.tensor1d(allY_clf, 'int32');
					confusionMatrix = /** @type {any} */ (
						tf.math.confusionMatrix(true_classes, pred_classes, plots.length).arraySync()
					);
				}
			} catch (/** @type {any} */ e) {
				trainingStatus = `Error: ${e.message}`;
			} finally {
				isTraining = false;
			}
		}, 10);
	}

	async function makePrediction() {
		if (!model) {
			predictionResult = 'Model not trained yet.';
			return;
		}

		const inputData = guessingData
			.split(/[,\\n]/)
			.map(s => parseFloat(s.trim()))
			.filter(n => !isNaN(n));

		if (inputData.length < TIME_STEP) {
			predictionResult = `Not enough data for prediction. Need at least ${TIME_STEP} numbers.`;
			return;
		}

		const sequence = inputData.slice(inputData.length - TIME_STEP);
		const inputTensor = tf.tensor(sequence).reshape([1, TIME_STEP, 1]);
		const [pred_clf, pred_reg] = /** @type {tf.Tensor[]} */ (model.predict(inputTensor));

		const confidence = pred_clf.max().dataSync()[0];
		const predictedIndex = pred_clf.argMax(-1).dataSync()[0];
		const nextValue = pred_reg.dataSync()[0];

		predictionResult = `Predicted Distribution: ${
			plots[predictedIndex].type
		} (Confidence: ${(confidence * 100).toFixed(2)}%), Next Value: ${nextValue.toFixed(4)}`;

		// --- Generate data for post-guess visualizations ---
		// Violin data
		if (inputData.length >= 4) {
			guessViolinData = calculate_violin_data(new Float64Array(inputData));
		} else {
			guessViolinData = null;
		}

		// Time series and P&F data
		const cumulative_sum_values = [];
		/** @type {[number, number][]} */
		const cumulative_sum_pairs = [];
		let current_sum = 0.0;
		for (let i = 0; i < inputData.length; i++) {
			current_sum += inputData[i];
			cumulative_sum_pairs.push([i, current_sum]);
			cumulative_sum_values.push(current_sum);
		}
		guessTimeSeriesData = cumulative_sum_pairs;
		guessPnfData = calculate_pnf_data(new Float64Array(cumulative_sum_values), 1.0, 3);

		// --- Calculate the predicted P&F bar ---
		if (guessPnfData.length > 0) {
			const lastPnfColumn = guessPnfData[guessPnfData.length - 1];
			const lastPrice = cumulative_sum_values[cumulative_sum_values.length - 1];
			const nextPrice = lastPrice + nextValue;

			let nextDirection = lastPnfColumn.direction;
			let from = lastPnfColumn.from;
			if (nextDirection === 'Up') {
				if (nextPrice < lastPrice) { // Reversal
					nextDirection = 'Down';
					from = lastPrice;
				}
			} else { // Down
				if (nextPrice > lastPrice) { // Reversal
					nextDirection = 'Up';
					from = lastPrice;
				}
			}
			predictedPnfBar = { from: from, to: nextPrice, direction: nextDirection };
		} else {
			predictedPnfBar = null;
		}
	}
</script>

<style>
	.ml-container {
		border: 1px solid #ccc;
		padding: 20px;
		margin-top: 20px;
		border-radius: 8px;
	}
	.training,
	.guessing {
		margin-bottom: 20px;
	}
	button {
		margin-right: 10px;
	}
	.training-controls {
		display: flex;
		gap: 15px;
		align-items: center;
		margin-bottom: 15px;
	}
	.training-controls input {
		width: 60px;
	}
	progress {
		width: 100%;
		margin-top: 10px;
	}
	.stats {
		margin-top: 15px;
		border: 1px solid #ddd;
		padding: 10px;
		border-radius: 4px;
	}
	textarea {
		width: 100%;
		padding: 8px;
		box-sizing: border-box;
		border: 1px solid #ccc;
		border-radius: 4px;
		margin-bottom: 10px;
	}
	.post-guess-visuals {
		margin-top: 20px;
		border-top: 1px solid #eee;
		padding-top: 20px;
	}
	.charts-container {
		display: flex;
		flex-wrap: wrap;
		gap: 20px;
		justify-content: center;
	}
	.chart-wrapper {
		border: 1px solid #ddd;
		border-radius: 8px;
		padding: 10px;
	}
    .data-panel {
        width: 100%;
        padding: 10px;
    }
</style>

<div class="ml-container">
	<div class="training">
		<h2>I) Training</h2>
		<div class="training-controls">
			<label for="epochs">Epochs:</label>
			<input id="epochs" type="number" bind:value={epochs} disabled={isTraining} />
			<label for="batchSize">Batch Size:</label>
			<input id="batchSize" type="number" bind:value={batchSize} disabled={isTraining} />
		</div>
		<button onclick={resetModel} disabled={isTraining}>Reset</button>
		<button onclick={trainModel} disabled={isTraining}>Train</button>
		<button onclick={stopTraining} disabled={!isTraining}>Stop</button>
		<p>Status: {trainingStatus}</p>
		{#if isTraining}
			<progress value={trainingProgress} max={epochs}></progress>
		{/if}
		{#if finalTrainingStats && !isTraining}
			<div class="stats">
				<h3>Final Training Stats</h3>
				<p>Loss: {finalTrainingStats.loss.toFixed(4)}</p>
				<p>Accuracy: {finalTrainingStats.acc.toFixed(4)}</p>
				<p>MAE: {finalTrainingStats.mae.toFixed(4)}</p>
			</div>
		{/if}
		{#if confusionMatrix}
			<div class="stats">
				<h3>Confusion Matrix</h3>
				<ConfusionMatrix matrix={confusionMatrix} labels={plots.map(p => p.type)} />
			</div>
		{/if}
	</div>

	<div class="guessing">
		<h2>II) Guessing</h2>
		<div class="training-controls">
			<label for="plot-copy-select">Copy from:</label>
			<select id="plot-copy-select" bind:value={selectedPlotForCopy}>
				{#each plots as plot, i}
					<option value={i}>Plot {i + 1} ({plot.type})</option>
				{/each}
			</select>
			<label for="decimal-places">Decimals:</label>
			<input id="decimal-places" type="number" min="0" bind:value={decimalPlaces} style="width: 60px;" />
			<label for="rounding-strategy">Rounding:</label>
			<select id="rounding-strategy" bind:value={roundingStrategy}>
				<option value="round">Round</option>
				<option value="floor">Floor</option>
				<option value="ceil">Ceil</option>
			</select>
			<button onclick={copyDataForGuessing}>Copy Data</button>
		</div>
		<textarea
			bind:value={guessingData}
			rows="5"
			placeholder="Enter time series data here, separated by commas or newlines..."
		></textarea>
		<button onclick={makePrediction}>Guess</button>
		{#if predictionResult}
			<p>{predictionResult}</p>
		{/if}

		{#if guessViolinData}
			<div class="post-guess-visuals">
				<h3>Visualizations for Your Input</h3>
				<div class="charts-container">
					<div class="chart-wrapper">
						<ViolinPlot data={guessViolinData} title="Input Data Distribution" width={320} height={300} />
					</div>
					<div class="chart-wrapper">
						<TimeSeriesPlot data={guessTimeSeriesData} width={320} height={300} />
					</div>
					<div class="chart-wrapper">
						<PointAndFigureChart data={guessPnfData} boxSize={1.0} width={320} height={300} predictedNextBar={predictedPnfBar} />
					</div>
				</div>
				<div class="data-panel">
					<label for="guessPnfData">Point & Figure Data:</label>
					<textarea id="guessPnfData" readonly rows="4">{JSON.stringify(guessPnfData)}</textarea>
				</div>
			</div>
		{/if}
	</div>
</div>
