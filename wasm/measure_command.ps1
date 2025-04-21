# Start the process and measure execution time
$executionTime = Measure-Command {
    $process = Start-Process -FilePath "wasmtime" -ArgumentList "--dir=. target/wasm32-wasip1/release/my_experiment.wasm" -PassThru -NoNewWindow
    $processId = $process.Id
    $cpuUsageSamples = @()

    # Monitoring loop
    while (!$process.HasExited) {
        $processInfo = Get-WmiObject -Query "SELECT * FROM Win32_PerfFormattedData_PerfProc_Process WHERE IDProcess = $processId" -ErrorAction SilentlyContinue
        
        if ($processInfo) {
            # Track CPU samples
            $cpuUsageSamples += $processInfo.PercentProcessorTime
            
        }
        
        Start-Sleep -Milliseconds 500
    }

    # Calculate CPU average
    if ($cpuUsageSamples.Count -gt 0) {
        $averageCpuUsage = ($cpuUsageSamples | Measure-Object -Average).Average
    } else {
        $averageCpuUsage = 0
    }

}

# Output results
Write-Output "Execution Time: $($executionTime.TotalSeconds) seconds"
Write-Output "Average CPU Usage: $averageCpuUsage %"