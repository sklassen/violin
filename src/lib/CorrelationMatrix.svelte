<script>
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
    import { pearson_correlation, spearman_correlation, kendall_correlation } from '$lib/vio-pkg/vio.js';

    let correlations = $state({
        pearson: { '1v2': 0, '1v3': 0, '2v3': 0 },
        spearman: { '1v2': 0, '1v3': 0, '2v3': 0 },
        kendall: { '1v2': 0, '1v3': 0, '2v3': 0 },
    });

    $effect(() => {
        if (plots && plots.length === 3 && plots.every(p => p.rawData && p.rawData.length > 0)) {
            const [p1, p2, p3] = plots;
            correlations = {
                pearson: {
                    '1v2': pearson_correlation(new Float64Array(p1.rawData), new Float64Array(p2.rawData)),
                    '1v3': pearson_correlation(new Float64Array(p1.rawData), new Float64Array(p3.rawData)),
                    '2v3': pearson_correlation(new Float64Array(p2.rawData), new Float64Array(p3.rawData)),
                },
                spearman: {
                    '1v2': spearman_correlation(new Float64Array(p1.rawData), new Float64Array(p2.rawData)),
                    '1v3': spearman_correlation(new Float64Array(p1.rawData), new Float64Array(p3.rawData)),
                    '2v3': spearman_correlation(new Float64Array(p2.rawData), new Float64Array(p3.rawData)),
                },
                kendall: {
                    '1v2': kendall_correlation(new Float64Array(p1.rawData), new Float64Array(p2.rawData)),
                    '1v3': kendall_correlation(new Float64Array(p1.rawData), new Float64Array(p3.rawData)),
                    '2v3': kendall_correlation(new Float64Array(p2.rawData), new Float64Array(p3.rawData)),
                },
            };
        }
    });
</script>

<style>
    .correlation-container {
        border: 1px solid #ccc;
        padding: 20px;
        margin-top: 20px;
        border-radius: 8px;
    }
    table {
        width: 100%;
        border-collapse: collapse;
    }
    th, td {
        border: 1px solid #ddd;
        padding: 8px;
        text-align: center;
    }
    th {
        background-color: #f2f2f2;
    }
</style>

<div class="correlation-container">
    <h2>Correlations</h2>
    {#if correlations && correlations.pearson}
        <table>
            <tbody>
                <tr>
                    <th>Correlation</th>
                    <th>Plot 1 vs Plot 2</th>
                    <th>Plot 1 vs Plot 3</th>
                    <th>Plot 2 vs Plot 3</th>
                </tr>
                <tr>
                    <td>Pearson</td>
                    <td>{correlations.pearson['1v2'].toFixed(4)}</td>
                    <td>{correlations.pearson['1v3'].toFixed(4)}</td>
                    <td>{correlations.pearson['2v3'].toFixed(4)}</td>
                </tr>
                <tr>
                    <td>Spearman</td>
                    <td>{correlations.spearman['1v2'].toFixed(4)}</td>
                    <td>{correlations.spearman['1v3'].toFixed(4)}</td>
                    <td>{correlations.spearman['2v3'].toFixed(4)}</td>
                </tr>
                <tr>
                    <td>Kendall</td>
                    <td>{correlations.kendall['1v2'].toFixed(4)}</td>
                    <td>{correlations.kendall['1v3'].toFixed(4)}</td>
                    <td>{correlations.kendall['2v3'].toFixed(4)}</td>
                </tr>
            </tbody>
        </table>
    {:else}
        <p>Generating data to calculate correlations...</p>
    {/if}
</div>
