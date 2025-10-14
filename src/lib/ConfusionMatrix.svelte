<script>
	/** @type {{ matrix: number[][], labels: string[] }} */
	let { matrix, labels } = $props();
</script>

<style>
	table {
		border-collapse: collapse;
		margin-top: 20px;
		font-size: 12px;
	}
	th, td {
		border: 1px solid #ddd;
		padding: 6px;
		text-align: center;
	}
	th {
		background-color: #f2f2f2;
	}
	.header-cell {
		font-weight: bold;
	}
</style>

<table>
	<thead>
		<tr>
			<th></th>
			{#each labels as label, i}
				<th class="header-cell">Predicted {label}</th>
			{/each}
		</tr>
	</thead>
	<tbody>
		{#each matrix as row, i}
			<tr>
				<td class="header-cell">Actual {labels[i]}</td>
				{#each row as cell, j}
					<td>{cell}</td>
				{/each}
			</tr>
		{/each}
	</tbody>
</table>
