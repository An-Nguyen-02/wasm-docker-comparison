

# Start the process and measure execution time
$executionTime = Measure-Command {
    $process = Start-Process -FilePath "wasmtime" -ArgumentList "--dir=. target/wasm32-wasip1/release/my_experiment.wasm" -PassThru -NoNewWindow
    $processId = $process.Id

    # Initialize variables for monitoring
    $cpuUsageSamples = @()
    $peakMemoryUsage = 0

    # Monitor the process until it exits
    while (!$process.HasExited) {
        # Get process information using WMI
        $processInfo = Get-WmiObject -Query "SELECT * FROM Win32_PerfFormattedData_PerfProc_Process WHERE IDProcess = $processId" -ErrorAction SilentlyContinue

        if ($processInfo) {
            # Capture CPU usage percentage
            $cpuUsageSamples += $processInfo.PercentProcessorTime

            # Capture peak memory usage (in MB)
            $currentMemoryUsage = $processInfo.WorkingSetPrivate / 1MB
            if ($currentMemoryUsage -gt $peakMemoryUsage) {
                $peakMemoryUsage = $currentMemoryUsage
            }
        }

        # Wait for a short interval before sampling again
        Start-Sleep -Milliseconds 500
    }

    # Calculate average CPU usage
    if ($cpuUsageSamples.Count -gt 0) {
        $averageCpuUsage = ($cpuUsageSamples | Measure-Object -Average).Average
    } else {
        $averageCpuUsage = 0
    }
}

# Output the results
Write-Output "Execution Time: $($executionTime.TotalSeconds) seconds"
Write-Output "Peak Memory Usage: $peakMemoryUsage MB"
Write-Output "Average CPU Usage: $averageCpuUsage %"