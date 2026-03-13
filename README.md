_____________________________________________
![fixshell](https://github.com/user-attachments/assets/4f2daaa0-d8ba-4957-939e-4190ef6d879f)
_____________________________________________

<div align="center">
  <h1>FIXSHELL</h1>
</div>

Fixshell is a pseudo-reverse shell that combines the ClickFix social engineering technique with a reverse shell using HTTP/S for command polling and Telegram bot as command server. It evades Windows Defender and other signature based security through memory only execution, traffic over standard HTTP/S port and dynamic staging to prevent static signature detection. Fixshell is compatible solely with Windows environments by utilizing PowerShell and is incompatible for Linux or macOS victims.

### Workflow
| Stage | Action | Description |
| :--- | :--- | :--- |
| 1. Landing | GET / | Serves the ClickFix landing page to victim. |
| 2. Verify | GET /api/ok | Validates the visitor and delivers the Windows PowerShell payload. |
| 3. Polling | GET /api/v1/:id | Victim retrieves queued commands from server. |
| 4. Exfiltration | POST /api/v1/:id | Victim sends command results to attacker via Telegram. |

<hr/>

## Setup
### Requirements
* Rust toolchain (Edition 2021)
* A Telegram Bot Token (from [@BotFather](https://t.me/botfather))
* Your Telegram User ID (from [@userinfobot](https://t.me/userinfobot))

### Config
Fixshell looks for environment variables. You can use a `.env` file.
```env
# Telegram bot config
# Get the bot-token from @BotFather 
BOT_TOKEN=12345:example_token

# Get your telegram user-id  from @userinfobot
ADMIN_ID=987654321
```
or <code>mv env_example .env</code> and add your bot token and admin id:

### Run
Compile the release binary and run:
   ```bash
   :$ cargo build --release
   :$ ./target/release/fixshell 
  ```

## Bot Commands
- `/sessions` — List all active victim IDs and redirect url stat.
- `/exec <id> <cmd>` — Run powershell command for specific session.
- `/redirect <url>` — Update the redirect URL in real-time.
- `/help` — show help menu.

## POC
I have used the [cloudflare](https://developers.cloudflare.com/cloudflare-one/networks/connectors/cloudflare-tunnel/) tunnel <code>./cloudflare tunnel --url localhost:6969</code> this resolve the issues like lack of a static IP or ISP restriction on port forwarding.
<table style="width: 100%; border-collapse: collapse;">
  <tr>
    <td align="center"><img src="https://github.com/user-attachments/assets/88a8af42-174d-4982-bd9f-f8b092ed798a" width="100%"/></td>
    <td align="center"><img src="https://github.com/user-attachments/assets/6cb0998d-a4a7-4566-869e-c88e5971a3ff" width="100%"/></td>
  </tr>
  <tr>
    <td align="center"><img src="https://github.com/user-attachments/assets/af485657-2f16-41a4-9667-44eff5ff3461" width="100%"/></td>
    <td align="center"><img src="https://github.com/user-attachments/assets/6c0fab98-7180-4d4a-ada4-0e32e85d82ec" width="100%"/></td>
  </tr>
</table>

<br/>

> [!CAUTION]
> For Educational and Authorized Security Testing Purposes Only. 
> Unauthorized access to computer systems is illegal. The author of Fixshell assumes no liability and is not responsible for any misuse, damage, or legal consequences caused by this software. It is the end-user's responsibility to obey all applicable local, state, and federal laws regarding cybersecurity and privacy. 

<hr />

<div align="center">
  <sub>Developed for Security Research and Red Team Analysis.</sub>
</div>
